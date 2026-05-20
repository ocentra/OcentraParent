#![forbid(unsafe_code)]

mod journal;
mod journal_crypto;
mod journal_error;
mod journal_rotation;

pub use journal::ActivityJournal;
pub use journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
pub use journal_error::JournalError;

pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

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
