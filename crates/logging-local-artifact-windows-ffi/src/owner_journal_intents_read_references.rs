use crate::error::ArtifactError;

use super::super::reconcile;
use super::super::*;

pub(super) fn referenced(
    records: &[IntentRecord],
) -> Result<super::super::ReferencedTemps, ArtifactError> {
    let mut referenced = super::super::ReferencedTemps(std::collections::HashSet::new());
    for record in records {
        append_references(record, &mut referenced)?;
    }
    Ok(referenced)
}

fn append_references(
    record: &IntentRecord,
    referenced: &mut super::super::ReferencedTemps,
) -> Result<(), ArtifactError> {
    match record {
        IntentRecord::Append { temp_name, .. } => {
            append_optional_temp(temp_name.as_deref(), referenced)
        }
        IntentRecord::Replace { temp_name, .. } => reconcile::insert_temp(temp_name, referenced),
        IntentRecord::Remove { .. } | IntentRecord::RemoveTree { .. } => Ok(()),
        IntentRecord::Transaction { staged, .. } => append_staged(staged, referenced),
    }
}

fn append_optional_temp<N>(
    temp_name: Option<&N>,
    referenced: &mut super::super::ReferencedTemps,
) -> Result<(), ArtifactError>
where
    N: descriptors::generated_names::GeneratedNameInput + ?Sized,
{
    temp_name
        .map(|name| reconcile::insert_temp(name, referenced))
        .transpose()
        .map(|_| ())
}

fn append_staged(
    staged: &[StagedMutation],
    referenced: &mut super::super::ReferencedTemps,
) -> Result<(), ArtifactError> {
    for item in staged {
        append_optional_stage(item.staged_name.as_deref(), referenced)?;
    }
    Ok(())
}

fn append_optional_stage<N>(
    staged_name: Option<&N>,
    referenced: &mut super::super::ReferencedTemps,
) -> Result<(), ArtifactError>
where
    N: descriptors::generated_names::GeneratedNameInput + ?Sized,
{
    staged_name
        .map(|name| reconcile::insert_stage(name, referenced))
        .transpose()
        .map(|_| ())
}
