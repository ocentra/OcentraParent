#[path = "windows/path_security.rs"]
mod path_security;
#[path = "windows/process.rs"]
mod process;
#[path = "windows/process_output.rs"]
mod process_output;
#[path = "windows/security.rs"]
mod security;
#[path = "windows/security_acl.rs"]
mod security_acl;
#[path = "windows/security_sid.rs"]
mod security_sid;
#[path = "windows/system_path.rs"]
mod system_path;
#[path = "windows/wire.rs"]
mod wire;

use crate::{observation::AppGameWindowsLocalPolicyObservation, Result};

pub(super) fn observe() -> Result<AppGameWindowsLocalPolicyObservation> {
    let paths = system_path::trusted_paths()?;
    let trusted = path_security::TrustedPowerShell::open(paths)?;
    process::observe(&trusted)
}
