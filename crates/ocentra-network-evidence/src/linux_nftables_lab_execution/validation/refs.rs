use crate::lab_execution_common::{
    is_test_net_remote_address as is_test_net_remote_address_impl, normalize_ref,
};

use super::{
    NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
    NetworkLinuxNftablesLabExecutionError,
};

pub(super) fn normalize_required(value: &str) -> Option<String> {
    normalize_ref(value)
}

pub(super) fn normalize_lab_ref(
    value: &str,
) -> Result<String, NetworkLinuxNftablesLabExecutionError> {
    normalize_required(value).ok_or(NetworkLinuxNftablesLabExecutionError::EmptyLabRef)
}

pub(super) fn normalize_table_name(
    value: &str,
) -> Result<String, NetworkLinuxNftablesLabExecutionError> {
    let table_name =
        normalize_required(value).ok_or(NetworkLinuxNftablesLabExecutionError::EmptyTableName)?;
    if !table_name.starts_with("ocentra_parent_lab_") {
        return Err(NetworkLinuxNftablesLabExecutionError::UnsafeTableName);
    }
    Ok(table_name)
}

pub(super) fn normalize_chain_name(
    value: &str,
) -> Result<String, NetworkLinuxNftablesLabExecutionError> {
    let chain_name =
        normalize_required(value).ok_or(NetworkLinuxNftablesLabExecutionError::EmptyChainName)?;
    if !chain_name.starts_with("ocentra_parent_lab_") {
        return Err(NetworkLinuxNftablesLabExecutionError::UnsafeChainName);
    }
    Ok(chain_name)
}

pub(super) fn normalize_target_remote_address(
    value: &str,
) -> Result<String, NetworkLinuxNftablesLabExecutionError> {
    let target_remote_address = normalize_required(value)
        .ok_or(NetworkLinuxNftablesLabExecutionError::EmptyTargetRemoteAddress)?;
    if !is_test_net_remote_address(&target_remote_address) {
        return Err(NetworkLinuxNftablesLabExecutionError::UnsafeTargetRemoteAddress);
    }
    Ok(target_remote_address)
}

fn is_test_net_remote_address(value: &str) -> bool {
    is_test_net_remote_address_impl(value)
}

pub(super) fn normalize_command_evidence(
    evidence: Vec<NetworkLinuxNftablesLabCommandEvidence>,
) -> Result<Vec<NetworkLinuxNftablesLabCommandEvidence>, NetworkLinuxNftablesLabExecutionError> {
    let mut normalized = Vec::new();
    for mut command in evidence {
        if has_kind(&normalized, command.kind) {
            return Err(
                NetworkLinuxNftablesLabExecutionError::DuplicateCommandEvidence(command.kind),
            );
        }
        command.command_ref = normalize_ref(&command.command_ref).ok_or(
            NetworkLinuxNftablesLabExecutionError::EmptyCommandRef(command.kind),
        )?;
        command.output_sha256 = normalize_ref(&command.output_sha256)
            .ok_or(NetworkLinuxNftablesLabExecutionError::EmptyCommandOutputHash(command.kind))?;
        normalized.push(command);
    }
    Ok(normalized)
}

fn has_kind(
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
    kind: NetworkLinuxNftablesLabCommandKind,
) -> bool {
    evidence.iter().any(|command| command.kind == kind)
}
