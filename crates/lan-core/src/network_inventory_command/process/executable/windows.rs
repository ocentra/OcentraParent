use std::{fs, path::PathBuf};

use ocentra_parent_agent_protocol::constants;

const WINDOWS_SYSTEM32: &str = r"C:\Windows\System32";

pub(super) fn candidates(program: &str) -> Vec<PathBuf> {
    let system32 = PathBuf::from(WINDOWS_SYSTEM32);
    match program {
        constants::lan_pairing::POWERSHELL_EXE => vec![system32
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe")],
        constants::lan_pairing::PING_EXE => vec![system32.join("ping.exe")],
        constants::lan_pairing::NBTSTAT_EXE => vec![system32.join("nbtstat.exe")],
        _ => Vec::new(),
    }
}

pub(super) fn canonical_trusted_candidate(candidate: PathBuf) -> Option<PathBuf> {
    let trusted_root = fs::canonicalize(WINDOWS_SYSTEM32).ok()?;
    let canonical = fs::canonicalize(candidate).ok()?;
    let metadata = fs::metadata(&canonical).ok()?;
    (metadata.is_file() && canonical.starts_with(trusted_root)).then_some(canonical)
}
