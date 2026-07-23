use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use ocentra_family_identity_core::household_authority::HouseholdAuthorityAction;
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
};

use super::open_parent_presence_test_port;

const STORE_ENV: &str = "OCENTRA_PARENT_NONCE_STORE";
const SCOPE_ENV: &str = "OCENTRA_PARENT_NONCE_SCOPE";
const CHALLENGE_ENV: &str = "OCENTRA_PARENT_NONCE_CHALLENGE";
const OUTCOME_ENV: &str = "OCENTRA_PARENT_NONCE_OUTCOME";
const READY_ENV: &str = "OCENTRA_PARENT_NONCE_READY";
const START_ENV: &str = "OCENTRA_PARENT_NONCE_START";

fn challenge(scope: &str, challenge_label: &str) -> ParentPresenceChallenge {
    ParentPresenceChallenge {
        challenge_ref: format!("{scope}-{challenge_label}-challenge"),
        nonce_ref: format!("{scope}-single-use-nonce"),
        family_id: format!("{scope}-family"),
        parent_account_id: format!("{scope}-parent"),
        privileged_action: HouseholdAuthorityAction::PairChildDevice,
        action_device_id: format!("{scope}-device"),
        action_device_child_profile_id: Some(format!("{scope}-action-child")),
        target_child_profile_id: Some(format!("{scope}-target-child")),
        expires_at: "2099-01-01T00:00:00.000Z".to_owned(),
    }
}

fn worker(
    store: &Path,
    scope: &str,
    challenge_label: &str,
    outcome: &Path,
    ready: &Path,
    start: &Path,
) -> std::io::Result<Command> {
    let mut command = Command::new(std::env::current_exe()?);
    command
        .arg("parent_presence_nonce_process_worker")
        .arg("--nocapture")
        .env(STORE_ENV, store)
        .env(SCOPE_ENV, scope)
        .env(CHALLENGE_ENV, challenge_label)
        .env(OUTCOME_ENV, outcome)
        .env(READY_ENV, ready)
        .env(START_ENV, start)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    Ok(command)
}

fn wait_for(path: &Path) -> std::io::Result<()> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !path.exists() {
        if Instant::now() >= deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "nonce process synchronization timed out",
            ));
        }
        thread::sleep(Duration::from_millis(5));
    }
    Ok(())
}

#[test]
fn parent_presence_nonce_is_single_use_across_concurrent_processes_and_restart(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::temp_dir().join(format!(
        "ocentra-parent-nonce-process-{}",
        std::process::id()
    ));
    fs::create_dir_all(&root)?;
    let store = root.join("parent-presence.sqlite");
    drop(
        open_parent_presence_test_port(&store)
            .map_err(|_error| std::io::Error::other("nonce store unavailable"))?,
    );
    let scope = format!("nonce-process-{}", std::process::id());
    let first_outcome = root.join("first.outcome");
    let second_outcome = root.join("second.outcome");
    let first_ready = root.join("first.ready");
    let second_ready = root.join("second.ready");
    let start = root.join("start");
    let first = worker(
        &store,
        &scope,
        "first",
        &first_outcome,
        &first_ready,
        &start,
    )?
    .spawn()?;
    let second = worker(
        &store,
        &scope,
        "second",
        &second_outcome,
        &second_ready,
        &start,
    )?
    .spawn()?;
    wait_for(&first_ready)?;
    wait_for(&second_ready)?;
    fs::write(&start, "start")?;
    assert!(first.wait_with_output()?.status.success());
    assert!(second.wait_with_output()?.status.success());
    let outcomes = [
        fs::read_to_string(first_outcome)?,
        fs::read_to_string(second_outcome)?,
    ];
    assert_eq!(
        outcomes
            .iter()
            .filter(|value| value.as_str() == "issued")
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|value| value.as_str() == "duplicate-nonce")
            .count(),
        1
    );

    let restart_outcome = root.join("restart.outcome");
    let restart_ready = root.join("restart.ready");
    let restart_start = root.join("restart.start");
    fs::write(&restart_start, "start")?;
    assert!(worker(
        &store,
        &scope,
        "restart",
        &restart_outcome,
        &restart_ready,
        &restart_start
    )?
    .output()?
    .status
    .success());
    assert_eq!(fs::read_to_string(restart_outcome)?, "duplicate-nonce");
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn parent_presence_nonce_process_worker() -> Result<(), Box<dyn std::error::Error>> {
    let Some(store) = std::env::var_os(STORE_ENV).map(PathBuf::from) else {
        return Ok(());
    };
    let scope = std::env::var(SCOPE_ENV)?;
    let challenge_label = std::env::var(CHALLENGE_ENV)?;
    let outcome = PathBuf::from(std::env::var_os(OUTCOME_ENV).ok_or("missing nonce outcome")?);
    let ready = PathBuf::from(std::env::var_os(READY_ENV).ok_or("missing nonce ready")?);
    let start = PathBuf::from(std::env::var_os(START_ENV).ok_or("missing nonce start")?);
    fs::write(ready, "ready")?;
    wait_for(&start)?;
    let mut port = open_parent_presence_test_port(store)
        .map_err(|_error| std::io::Error::other("nonce worker store unavailable"))?;
    let result = port.issue_challenge(challenge(&scope, &challenge_label));
    let value = match result {
        Ok(()) => "issued",
        Err(ParentPresenceChallengeIssuanceFailureReason::DuplicateNonceRef) => "duplicate-nonce",
        Err(_) => "rejected",
    };
    fs::write(outcome, value)?;
    Ok(())
}
