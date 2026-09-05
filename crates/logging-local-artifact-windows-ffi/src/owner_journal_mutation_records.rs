use crate::error::ArtifactError;
use crate::owner_types::{Mutation, MAX_TRANSACTION_MUTATIONS};
use crate::platform::windows::MAX_ARTIFACT_BYTES;

use super::{payload_digest, StagedMutation};

pub(super) fn mutation_records(
    mutations: &[Mutation],
) -> Result<Vec<StagedMutation>, ArtifactError> {
    if mutations.is_empty() || mutations.len() > MAX_TRANSACTION_MUTATIONS {
        return Err(ArtifactError::SizeLimit);
    }
    let mut result = Vec::with_capacity(mutations.len());
    for mutation in mutations {
        let operation = mutation.operation_name().to_owned();
        let payload_length = mutation
            .payload()
            .map(|payload| u64::try_from(payload.len()))
            .transpose()
            .map_err(|_| ArtifactError::SizeLimit)?
            .unwrap_or(0);
        if payload_length > MAX_ARTIFACT_BYTES {
            return Err(ArtifactError::SizeLimit);
        }
        result.push(StagedMutation {
            relative_path: mutation.relative_path().to_owned(),
            operation,
            payload_digest: mutation.payload().map(payload_digest),
            staged_name: None,
            quarantine_name: None,
            target_identity: None,
            installed_identity: None,
        });
    }
    Ok(result)
}
