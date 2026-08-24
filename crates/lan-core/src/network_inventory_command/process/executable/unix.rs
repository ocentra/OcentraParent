use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};

use ocentra_parent_agent_protocol::constants;

pub(super) fn candidates(program: &str) -> Vec<PathBuf> {
    let paths: &[&str] = match program {
        constants::lan_pairing::IP_EXE => &[
            "/usr/sbin/ip",
            "/usr/bin/ip",
            "/sbin/ip",
            "/bin/ip",
            "/system/bin/ip",
        ],
        constants::lan_pairing::PING_EXE => &[
            "/usr/bin/ping",
            "/usr/sbin/ping",
            "/bin/ping",
            "/sbin/ping",
            "/system/bin/ping",
        ],
        constants::lan_pairing::NVIDIA_SMI_EXE => {
            &["/usr/bin/nvidia-smi", "/usr/local/bin/nvidia-smi"]
        }
        "arp" => &["/usr/sbin/arp", "/sbin/arp"],
        "arping" => &[
            "/usr/bin/arping",
            "/usr/sbin/arping",
            "/bin/arping",
            "/sbin/arping",
        ],
        "getent" => &["/usr/bin/getent", "/bin/getent"],
        _ => &[],
    };
    paths.iter().map(PathBuf::from).collect()
}

pub(super) fn canonical_trusted_candidate(candidate: PathBuf) -> Option<PathBuf> {
    let canonical = fs::canonicalize(candidate).ok()?;
    if !is_beneath_protected_root(&canonical) || !has_protected_ancestor_chain(&canonical) {
        return None;
    }
    fs::metadata(&canonical)
        .ok()
        .filter(|metadata| metadata.is_file())
        .map(|_| canonical)
}

fn is_beneath_protected_root(path: &PathBuf) -> bool {
    ["/usr", "/bin", "/sbin", "/system"]
        .into_iter()
        .filter_map(|root| fs::canonicalize(root).ok())
        .any(|root| path.starts_with(root))
}

fn has_protected_ancestor_chain(path: &PathBuf) -> bool {
    path.ancestors().all(|ancestor| {
        fs::symlink_metadata(ancestor)
            .map(|metadata| metadata.uid() == 0 && metadata.mode() & 0o022 == 0)
            .unwrap_or(false)
    })
}
