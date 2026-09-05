use std::collections::HashSet;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use ocentra_parent_logging_core::local_artifact_mutation::{
    LocalArtifactMutation, LocalArtifactMutationSession,
};

use super::{FailureDisposition, LeaseRequirement, LeaseState, OperationExecution, ProviderError};
use crate::protocol::{self, ProviderPayload, ValidatedMutation, ValidatedRequest};

pub(super) fn apply_transaction(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    mutations: &[ValidatedMutation],
) -> Result<OperationExecution, ProviderError> {
    super::super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Required)?;
    let native_mutations = transaction_mutations(mutations)?;
    let request_id = request.request_id().text();
    let receipt = session
        .apply_transaction(&request_id, &native_mutations)
        .map_err(|error| super::super::map_owner_error(&error))?;
    super::super::values::transaction_result(
        &receipt,
        request,
        super::super::MutationCount(native_mutations.len()),
    )
}

fn transaction_mutations(
    mutations: &[ValidatedMutation],
) -> Result<Vec<LocalArtifactMutation>, ProviderError> {
    let mut paths = HashSet::with_capacity(mutations.len());
    let mut native = Vec::with_capacity(mutations.len());
    for mutation in mutations {
        let relative_path = mutation_path(mutation);
        let path = relative_path.text();
        let duplicate_key = path.to_lowercase();
        if !paths.insert(duplicate_key) {
            return Err(ProviderError::new(
                protocol::text::DUPLICATE_TRANSACTION_TARGET,
                FailureDisposition::Continue,
            ));
        }
        native.push(to_native_mutation(mutation)?);
    }
    Ok(native)
}

fn mutation_path(mutation: &ValidatedMutation) -> &crate::protocol::ProviderRelativePath {
    match mutation {
        ValidatedMutation::Replace { relative_path, .. }
        | ValidatedMutation::Remove { relative_path }
        | ValidatedMutation::RemoveTree { relative_path } => relative_path,
    }
}

fn to_native_mutation(
    mutation: &ValidatedMutation,
) -> Result<LocalArtifactMutation, ProviderError> {
    match mutation {
        ValidatedMutation::Replace {
            relative_path,
            payload_base64,
        } => {
            let payload = decode_payload(payload_base64, super::super::PayloadKind::Replace)?;
            Ok(LocalArtifactMutation::Replace {
                relative_path: relative_path.text(),
                payload,
            })
        }
        ValidatedMutation::Remove { relative_path } => Ok(LocalArtifactMutation::Remove {
            relative_path: relative_path.text(),
        }),
        ValidatedMutation::RemoveTree { relative_path } => Ok(LocalArtifactMutation::RemoveTree {
            relative_path: relative_path.text(),
        }),
    }
}

pub(super) fn decode_payload(
    encoded: &ProviderPayload,
    kind: super::super::PayloadKind,
) -> Result<Vec<u8>, ProviderError> {
    let encoded_text = encoded.text();
    let maximum_encoded_bytes = kind
        .maximum_bytes()
        .checked_add(2)
        .and_then(|bytes| bytes.checked_div(3))
        .and_then(|groups| groups.checked_mul(4))
        .ok_or_else(|| {
            ProviderError::new(
                protocol::text::PAYLOAD_BOUND_OVERFLOW,
                FailureDisposition::Terminate,
            )
        })?;
    if encoded_text.len() > maximum_encoded_bytes
        || (!kind.allows_empty() && encoded_text.is_empty())
    {
        return Err(ProviderError::new(
            protocol::text::PAYLOAD_BOUND,
            FailureDisposition::Continue,
        ));
    }
    let decoded = STANDARD.decode(encoded_text.as_bytes()).map_err(|_error| {
        ProviderError::new(
            protocol::text::PAYLOAD_NOT_BASE64,
            FailureDisposition::Continue,
        )
    })?;
    if decoded.len() > kind.maximum_bytes() || (!kind.allows_empty() && decoded.is_empty()) {
        return Err(ProviderError::new(
            protocol::text::PAYLOAD_BOUND,
            FailureDisposition::Continue,
        ));
    }
    if STANDARD.encode(&decoded) != encoded_text {
        return Err(ProviderError::new(
            protocol::text::PAYLOAD_NOT_CANONICAL,
            FailureDisposition::Continue,
        ));
    }
    Ok(decoded)
}
