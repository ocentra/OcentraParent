#![forbid(unsafe_code)]

mod activity_store;
mod activity_store_error;
mod activity_store_rows;
mod journal;
mod journal_crypto;
mod journal_error;
mod journal_rotation;
mod process_capture;

pub use activity_store::ActivityStore;
pub use activity_store_error::ActivityStoreError;
pub use journal::ActivityJournal;
pub use journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
pub use journal_error::JournalError;
pub use process_capture::{
    collect_process_snapshot, process_observation_event, process_snapshot_events,
    ProcessObservation,
};

pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod activity_store_tests;
#[cfg(test)]
mod journal_tests;

#[cfg(test)]
mod tests {
    use super::crate_name;

    #[test]
    fn crate_name_identifies_agent_core_boundary() {
        assert_eq!(crate_name(), env!("CARGO_PKG_NAME"));
    }
}
