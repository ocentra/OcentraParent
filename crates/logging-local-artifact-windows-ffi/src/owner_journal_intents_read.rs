use std::collections::HashSet;
use std::path::Path;

use crate::error::ArtifactError;

use super::*;

pub(super) struct IntentEntryNames(pub(super) Vec<String>);

pub(super) struct ReferencedTemps(pub(super) HashSet<String>);

#[path = "owner_journal_intents_read_entries.rs"]
mod entries;
#[path = "owner_journal_intents_read_reconcile.rs"]
mod reconcile;
#[path = "owner_journal_intents_read_records.rs"]
mod records;

pub(super) fn read_intents(
    root: &Path,
    intent_directory: &OwnedFile,
) -> Result<Vec<IntentRecord>, ArtifactError> {
    let directory = root
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(INTENTS_DIRECTORY);
    let names = entries::collect(&directory)?;
    let records = records::parse(&directory, &names)?;
    let referenced = records::referenced(&records)?;
    reconcile::unreferenced(&directory, intent_directory, &names, &referenced)?;
    Ok(records)
}

fn generated_intent_temp_name<N>(name: &N) -> bool
where
    N: descriptors::generated_names::GeneratedNameInput + ?Sized,
{
    name.is_generated_intent_temp_name()
}

fn generated_intent_stage_name<N>(name: &N) -> bool
where
    N: descriptors::generated_names::GeneratedNameInput + ?Sized,
{
    name.is_generated_intent_stage_name()
}
