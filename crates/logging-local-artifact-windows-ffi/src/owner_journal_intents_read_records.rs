#[path = "owner_journal_intents_read_parse.rs"]
mod parse_records;
#[path = "owner_journal_intents_read_references.rs"]
mod references;

use std::path::Path;

use crate::error::ArtifactError;

use super::*;

pub(super) fn parse(
    directory: &Path,
    entries: &super::IntentEntryNames,
) -> Result<Vec<IntentRecord>, ArtifactError> {
    parse_records::parse(directory, entries)
}

pub(super) fn referenced(
    records: &[IntentRecord],
) -> Result<super::ReferencedTemps, ArtifactError> {
    references::referenced(records)
}
