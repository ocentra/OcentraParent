use ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeJournalPath;
use ocentra_parent_agent_protocol::constants;

pub fn network_runtime_journal_path() -> NetworkRuntimeJournalPath {
    let path = std::env::var(constants::env_var::NETWORK_RUNTIME_JOURNAL_PATH)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = crate::activity_store_path::activity_db_path().0;
            path.set_file_name(
                constants::activity_store::DEFAULT_NETWORK_RUNTIME_JOURNAL_FILE_NAME,
            );
            path
        });
    NetworkRuntimeJournalPath::new(path)
}
