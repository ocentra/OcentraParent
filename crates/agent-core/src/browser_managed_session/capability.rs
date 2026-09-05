use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::constants;

use super::{
    BrowserManagedLaunch, BrowserManagedLaunchConfig, BrowserManagedLaunchError,
    ManagedBrowserLaunchAuthority,
};

pub(super) fn launch_managed_browser(
    config: BrowserManagedLaunchConfig,
) -> Result<BrowserManagedLaunch, BrowserManagedLaunchError> {
    let profile_path = normalized_path(&config.profile_dir);
    let profile_id = managed_profile_id(&config.profile_dir)
        .ok_or(BrowserManagedLaunchError::UnownedProfileRejected)?;
    let plan = super::launch::managed_browser_launch_plan(config)?;
    let executable_path = normalized_path(&plan.executable_path);
    let created_at_epoch_ms = unix_epoch_millis()?;
    let generation = created_at_epoch_ms.max(1);
    let expires_at_epoch_ms = created_at_epoch_ms
        .checked_add(
            ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_AUTHORITY_TTL_MS,
        )
        .ok_or(BrowserManagedLaunchError::Io)?;
    let mut child = Command::new(&plan.executable_path)
        .args(&plan.args)
        .spawn()
        .map_err(|_error| BrowserManagedLaunchError::Io)?;
    let process_id = child.id();
    let endpoint = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), plan.bridge_port);
    if !verify_endpoint_after_spawn(endpoint, process_id, &executable_path) {
        let _ = child.kill();
        let _ = child.wait();
        return Err(BrowserManagedLaunchError::ManualRequired);
    }

    Ok(BrowserManagedLaunch {
        process_id,
        bridge_port: plan.bridge_port,
        browser_family: plan.browser_family,
        browser_channel: plan.browser_channel,
        profile_path_ref: plan.profile_path_ref,
        bridge_endpoint_ref: plan.bridge_endpoint_ref,
        cdp_authority: ManagedBrowserLaunchAuthority {
            managed_browser_session_id: managed_browser_session_id(process_id, generation),
            profile_id,
            process_id,
            bridge_port: plan.bridge_port,
            browser_family: plan.browser_family,
            browser_channel: plan.browser_channel,
            executable_path,
            profile_path,
            generation,
            created_at_epoch_ms,
            expires_at_epoch_ms,
        },
    })
}

fn verify_endpoint_after_spawn(
    endpoint: SocketAddr,
    process_id: u32,
    executable_path: &Path,
) -> bool {
    let deadline = Instant::now()
        .checked_add(Duration::from_millis(
            constants::browser::DEVTOOLS_TIMEOUT_MS,
        ))
        .unwrap_or_else(Instant::now);
    loop {
        if crate::browser_bridge_capture::verify_managed_browser_cdp_endpoint(
            endpoint,
            process_id,
            executable_path,
        )
        .is_ok()
        {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(25));
    }
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

fn managed_browser_session_id(process_id: u32, generation: u64) -> String {
    format!(
        "{}-{}-{}",
        constants::browser::SESSION_ID_PREFIX_MANAGED,
        process_id,
        generation
    )
}

fn normalized_path(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_error| path.to_path_buf())
}
