macro_rules! expected_passive_summary_sources {
    () => {{
        if cfg!(target_os = "windows") {
            vec!["windows-neighbor-table"]
        } else if cfg!(any(target_os = "linux", target_os = "android")) {
            vec!["linux-proc-net-arp", "linux-ip-neigh"]
        } else if cfg!(target_os = "macos") {
            vec!["macos-arp"]
        } else {
            Vec::new()
        }
    }};
}
