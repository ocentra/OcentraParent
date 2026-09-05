#[path = "activity_db_value.rs"]
mod activity_db_value;

pub type ActivityDbPath = activity_db_value::ActivityDbPath;

pub fn activity_db_path() -> ActivityDbPath {
    activity_db_value::ActivityDbPath(
        std::env::var(ocentra_parent_agent_protocol::constants::env_var::ACTIVITY_DB_PATH)
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = std::env::temp_dir();
                path.push(
                    ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_FILE_NAME,
                );
                path
            }),
    )
}
