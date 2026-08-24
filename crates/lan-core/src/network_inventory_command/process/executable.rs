use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::constants;

#[cfg(unix)]
#[path = "executable/unix.rs"]
mod platform;
#[cfg(windows)]
#[path = "executable/windows.rs"]
mod platform;

const ALLOWLISTED_NAMES: [&str; 8] = [
    constants::lan_pairing::POWERSHELL_EXE,
    constants::lan_pairing::IP_EXE,
    constants::lan_pairing::PING_EXE,
    constants::lan_pairing::NBTSTAT_EXE,
    constants::lan_pairing::NVIDIA_SMI_EXE,
    "arp",
    "arping",
    "getent",
];

pub(super) fn resolve_trusted_executable(program: &str) -> Option<PathBuf> {
    if !is_plain_allowlisted_name(program) {
        return None;
    }

    platform::candidates(program)
        .into_iter()
        .find_map(platform::canonical_trusted_candidate)
}

fn is_plain_allowlisted_name(program: &str) -> bool {
    !program.is_empty()
        && Path::new(program)
            .file_name()
            .and_then(|name| name.to_str())
            == Some(program)
        && ALLOWLISTED_NAMES.contains(&program)
}

#[cfg(not(any(unix, windows)))]
mod platform {
    use std::path::PathBuf;

    pub(super) fn candidates(_program: &str) -> Vec<PathBuf> {
        Vec::new()
    }

    pub(super) fn canonical_trusted_candidate(_candidate: PathBuf) -> Option<PathBuf> {
        None
    }
}
