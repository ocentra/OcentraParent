use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceVerificationFailureReason,
    ParentPresenceVerificationInput,
};

use super::open_parent_presence_test_port;

const EXPIRY: &str = "2099-01-01T00:00:00.000Z";
const SCOPE_ENV: &str = "OCENTRA_PARENT_PRESENCE_PROBE_SCOPE";
const STORE_ENV: &str = "OCENTRA_PARENT_PRESENCE_PROBE_STORE";
const OUTCOME_ENV: &str = "OCENTRA_PARENT_PRESENCE_PROBE_OUTCOME";
const READY_ENV: &str = "OCENTRA_PARENT_PRESENCE_PROBE_READY";
const START_ENV: &str = "OCENTRA_PARENT_PRESENCE_PROBE_START";
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
        assert!(matches!(fs::create_dir_all(&root), Ok(())));
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

fn input(
    scope: &str,
) -> Result<
    (ParentPresenceChallenge, ParentPresenceVerificationInput),
    ocentra_eventing::error::EventingError,
> {
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
        correlation_id: CorrelationId::parse("parent-presence-cross-process-correlation")?,
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
    Ok((challenge, verification))
}

fn worker(
    scope: &str,
    store: &Path,
    outcome: &Path,
    ready: &Path,
    start: &Path,
) -> std::io::Result<Command> {
    let mut command = Command::new(std::env::current_exe()?);
    command
        .arg("parent_presence_cross_process_worker")
        .arg("--nocapture")
        .env(SCOPE_ENV, scope)
        .env(STORE_ENV, store)
        .env(OUTCOME_ENV, outcome)
        .env(READY_ENV, ready)
        .env(START_ENV, start)
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    Ok(command)
}

fn wait_until(predicate: impl Fn() -> bool) -> std::io::Result<()> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !predicate() {
        if Instant::now() >= deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "cross-process synchronization timed out",
            ));
        }
        thread::sleep(Duration::from_millis(5));
    }
    Ok(())
}

#[test]
fn parent_presence_replay_is_durable_across_processes_and_restart(
) -> Result<(), Box<dyn std::error::Error>> {
    let scope = format!(
        "cross-process-{}",
        NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed)
    );
    let store = Store::new();
    let mut issuer = open_parent_presence_test_port(&store.path)
        .map_err(|_error| std::io::Error::other("issuer store unavailable"))?;
    let (challenge, _) = input(&scope)?;
    assert_eq!(issuer.issue_challenge(challenge), Ok(()));
    let first_path = store.outcome("first");
    let second_path = store.outcome("second");
    let first_ready = store.outcome("first-ready");
    let second_ready = store.outcome("second-ready");
    let start = store.outcome("start");
    let first = worker(&scope, &store.path, &first_path, &first_ready, &start)?.spawn()?;
    let second = worker(&scope, &store.path, &second_path, &second_ready, &start)?.spawn()?;
    wait_until(|| first_ready.exists() && second_ready.exists())?;
    fs::write(&start, "start")?;
    let first = first.wait_with_output()?;
    let second = second.wait_with_output()?;
    assert!(
        first.status.success(),
        "first worker failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(
        second.status.success(),
        "second worker failed: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    let outcomes = [
        fs::read_to_string(&first_path)?,
        fs::read_to_string(&second_path)?,
    ];
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome.starts_with("accepted:"))
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome.starts_with("replay-rejected:"))
            .count(),
        1
    );
    let concurrent_decision_ids = outcomes
        .iter()
        .filter_map(|outcome| outcome.split_once(':').map(|(_result, id)| id))
        .collect::<Vec<_>>();
    assert_eq!(concurrent_decision_ids.len(), 2);
    assert_ne!(concurrent_decision_ids[0], concurrent_decision_ids[1]);
    let restart_path = store.outcome("restart");
    let restart_ready = store.outcome("restart-ready");
    let restart_start = store.outcome("restart-start");
    fs::write(&restart_start, "start")?;
    let restart = worker(
        &scope,
        &store.path,
        &restart_path,
        &restart_ready,
        &restart_start,
    )?
    .output()?;
    assert!(restart.status.success());
    let restart_outcome = fs::read_to_string(restart_path)?;
    assert!(restart_outcome.starts_with("replay-rejected:"));
    let restart_id = restart_outcome
        .split_once(':')
        .map(|(_result, id)| id)
        .ok_or("restart outcome has no decision id")?;
    assert!(concurrent_decision_ids
        .iter()
        .all(|decision_id| *decision_id != restart_id));
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
    let ready = PathBuf::from(std::env::var_os(READY_ENV).ok_or("missing process ready path")?);
    let start = PathBuf::from(std::env::var_os(START_ENV).ok_or("missing process start path")?);
    fs::write(ready, "ready")?;
    wait_until(|| start.exists())?;
    let mut port = open_parent_presence_test_port(store)
        .map_err(|_error| std::io::Error::other("worker store unavailable"))?;
    let (_, verification) = input(&scope)?;
    let result = port.verify_and_consume(verification);
    let result_label = if result.is_ok() {
        "accepted"
    } else if result == Err(ParentPresenceVerificationFailureReason::ReplayRejected) {
        "replay-rejected"
    } else {
        "rejected"
    };
    let decision_id = port
        .take_custody_artifact()
        .ok_or("worker produced no custody artifact")?
        .decision_id;
    fs::write(outcome, format!("{result_label}:{}", decision_id.as_str()))?;
    Ok(())
}
