use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceVerificationFailureReason,
    ParentPresenceVerificationInput, ParentPresenceVerificationPort,
};

const EXPIRY: &str = "2099-01-01T00:00:00.000Z";
const SCOPE_ENV: &str = "OCENTRA_PARENT_PRESENCE_PROBE_SCOPE";
const STORE_ENV: &str = "OCENTRA_PARENT_PRESENCE_PROBE_STORE";
const OUTCOME_ENV: &str = "OCENTRA_PARENT_PRESENCE_PROBE_OUTCOME";
static NEXT_CASE_ID: AtomicU64 = AtomicU64::new(1);

struct Store {
    root: PathBuf,
    path: PathBuf,
}
impl Store {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "ocentra-parent-presence-process-{}-{}",
            std::process::id(),
            NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed)
        ));
        Self {
            path: root.join("parent-presence.sqlite"),
            root,
        }
    }
    fn outcome(&self, label: &str) -> PathBuf {
        self.root.join(format!("{label}.outcome"))
    }
}
impl Drop for Store {
    fn drop(&mut self) {
        let _cleanup = fs::remove_dir_all(&self.root);
    }
}

fn input(scope: &str) -> (ParentPresenceChallenge, ParentPresenceVerificationInput) {
    let challenge_ref = format!("{scope}-challenge");
    let nonce_ref = format!("{scope}-nonce");
    let family_id = format!("{scope}-family");
    let parent_account_id = format!("{scope}-parent");
    let action_device_id = format!("{scope}-device");
    let action_device_child_profile_id = Some(format!("{scope}-action-child"));
    let target_child_profile_id = Some(format!("{scope}-target-child"));
    let challenge = ParentPresenceChallenge {
        challenge_ref: challenge_ref.clone(),
        nonce_ref: nonce_ref.clone(),
        family_id: family_id.clone(),
        parent_account_id: parent_account_id.clone(),
        privileged_action: HouseholdAuthorityAction::PairChildDevice,
        action_device_id: action_device_id.clone(),
        action_device_child_profile_id: action_device_child_profile_id.clone(),
        target_child_profile_id: target_child_profile_id.clone(),
        expires_at: EXPIRY.to_owned(),
    };
    let verification = ParentPresenceVerificationInput {
        challenge_ref,
        assertion: ParentStepUpAssertionSnapshot {
            family_id,
            parent_account_id,
            action_device_id,
            action_device_child_profile_id,
            target_child_profile_id,
            action: HouseholdAuthorityAction::PairChildDevice,
            nonce: nonce_ref,
            expires_at: EXPIRY.to_owned(),
        },
    };
    (challenge, verification)
}

fn worker(scope: &str, store: &Path, outcome: &Path) -> std::io::Result<Command> {
    let mut command = Command::new(std::env::current_exe()?);
    command
        .arg("parent_presence_cross_process_worker")
        .arg("--nocapture")
        .env(SCOPE_ENV, scope)
        .env(STORE_ENV, store)
        .env(OUTCOME_ENV, outcome)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    Ok(command)
}

#[test]
fn parent_presence_replay_is_durable_across_processes_and_restart(
) -> Result<(), Box<dyn std::error::Error>> {
    let scope = format!(
        "cross-process-{}",
        NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed)
    );
    let store = Store::new();
    let mut issuer = ParentPresenceVerificationPort::open(&store.path)
        .map_err(|_error| std::io::Error::other("issuer store unavailable"))?;
    let (challenge, _) = input(&scope);
    assert_eq!(issuer.issue_challenge(challenge), Ok(()));
    let first_path = store.outcome("first");
    let second_path = store.outcome("second");
    let first = worker(&scope, &store.path, &first_path)?
        .spawn()?
        .wait_with_output()?;
    let second = worker(&scope, &store.path, &second_path)?
        .spawn()?
        .wait_with_output()?;
    assert!(first.status.success());
    assert!(second.status.success());
    let outcomes = [
        fs::read_to_string(&first_path)?,
        fs::read_to_string(&second_path)?,
    ];
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome.as_str() == "accepted")
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome.as_str() == "replay-rejected")
            .count(),
        1
    );
    let restart_path = store.outcome("restart");
    let restart = worker(&scope, &store.path, &restart_path)?.output()?;
    assert!(restart.status.success());
    assert_eq!(fs::read_to_string(restart_path)?, "replay-rejected");
    Ok(())
}

#[test]
fn parent_presence_cross_process_worker() -> Result<(), Box<dyn std::error::Error>> {
    let Ok(scope) = std::env::var(SCOPE_ENV) else {
        return Ok(());
    };
    let store = PathBuf::from(std::env::var_os(STORE_ENV).ok_or("missing process store path")?);
    let outcome =
        PathBuf::from(std::env::var_os(OUTCOME_ENV).ok_or("missing process outcome path")?);
    let mut port = ParentPresenceVerificationPort::open(store)
        .map_err(|_error| std::io::Error::other("worker store unavailable"))?;
    let (_, verification) = input(&scope);
    let result = port.verify_and_consume(verification);
    let value = if result.is_ok() {
        "accepted"
    } else if result == Err(ParentPresenceVerificationFailureReason::ReplayRejected) {
        "replay-rejected"
    } else {
        "rejected"
    };
    fs::write(outcome, value)?;
    Ok(())
}
