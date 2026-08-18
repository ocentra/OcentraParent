use std::{
    path::Path,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::constants;
use sha2::{Digest, Sha256};

use super::{
    BrowserManagedLaunch, BrowserManagedLaunchConfig, BrowserManagedLaunchError,
    ManagedBrowserLaunchAuthority,
};

const MANAGED_BROWSER_SESSION_SECRET_BYTES: usize = 32;

pub(super) fn launch_managed_browser(
    config: BrowserManagedLaunchConfig,
) -> Result<BrowserManagedLaunch, BrowserManagedLaunchError> {
    let executable_binding = managed_path_binding(&config.executable_path);
    let profile_binding = managed_path_binding(&config.profile_dir);
    let profile_id = managed_profile_id(&config.profile_dir)
        .ok_or(BrowserManagedLaunchError::UnownedProfileRejected)?;
    let created_at_epoch_ms = unix_epoch_millis()?;
    let mut session_secret = [0u8; MANAGED_BROWSER_SESSION_SECRET_BYTES];
    getrandom::fill(&mut session_secret).map_err(|_error| BrowserManagedLaunchError::Io)?;
    if session_secret.iter().all(|byte| *byte == 0) {
        return Err(BrowserManagedLaunchError::Io);
    }
    let generation = session_secret
        .iter()
        .take(8)
        .fold(0u64, |value, byte| (value << 8) | u64::from(*byte))
        .max(1);
    let expires_at_epoch_ms = created_at_epoch_ms
        .checked_add(
            ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_AUTHORITY_TTL_MS,
        )
        .ok_or(BrowserManagedLaunchError::Io)?;
    let plan = super::launch::managed_browser_launch_plan(config)?;
    let child = Command::new(&plan.executable_path)
        .args(&plan.args)
        .spawn()
        .map_err(|_error| BrowserManagedLaunchError::Io)?;

    Ok(BrowserManagedLaunch {
        process_id: child.id(),
        bridge_port: plan.bridge_port,
        browser_family: plan.browser_family,
        browser_channel: plan.browser_channel,
        profile_path_ref: plan.profile_path_ref,
        bridge_endpoint_ref: plan.bridge_endpoint_ref,
        cdp_authority: ManagedBrowserLaunchAuthority {
            managed_browser_session_id: managed_browser_session_id(&session_secret),
            profile_id,
            session_secret,
            generation,
            created_at_epoch_ms,
            expires_at_epoch_ms,
            executable_binding,
            profile_binding,
        },
    })
}

pub(super) fn managed_path_binding(path: &Path) -> String {
    let normalized = std::fs::canonicalize(path)
        .unwrap_or_else(|_error| path.to_path_buf())
        .to_string_lossy()
        .to_ascii_lowercase();
    let mut value = String::new();
    for byte in Sha256::digest(normalized.as_bytes()) {
        value.push_str(&format!("{byte:02x}"));
    }
    value
}

fn managed_profile_id(path: &Path) -> Option<String> {
    let profile_id = path.file_name()?.to_string_lossy().into_owned();
    super::launch::managed_profile_path_owned(path).then_some(profile_id)
}

fn unix_epoch_millis() -> Result<u64, BrowserManagedLaunchError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_error| BrowserManagedLaunchError::Io)
        .and_then(|duration| {
            u64::try_from(duration.as_millis()).map_err(|_error| BrowserManagedLaunchError::Io)
        })
}

fn managed_browser_session_id(secret: &[u8; MANAGED_BROWSER_SESSION_SECRET_BYTES]) -> String {
    let mut value = String::from(constants::browser::SESSION_ID_PREFIX_MANAGED);
    value.push('-');
    for byte in secret {
        value.push_str(&format!("{byte:02x}"));
    }
    value
}
