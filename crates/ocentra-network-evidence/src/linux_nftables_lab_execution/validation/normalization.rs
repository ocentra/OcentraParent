use super::{
    claims, gate, refs, NetworkLinuxNftablesLabExecutionError,
    NetworkLinuxNftablesLabExecutionInput, NormalizedLabExecutionInput,
};

pub(super) fn normalize_input(
    input: NetworkLinuxNftablesLabExecutionInput,
) -> Result<NormalizedLabExecutionInput, NetworkLinuxNftablesLabExecutionError> {
    claims::reject_unsupported_claims(&input.unsupported_claims)?;
    gate::validate_gate_proof(&input.gate_proof)?;
    Ok(NormalizedLabExecutionInput {
        lab_ref: refs::normalize_lab_ref(&input.lab_ref)?,
        gate_proof: input.gate_proof,
        table_name: refs::normalize_table_name(&input.table_name)?,
        chain_name: refs::normalize_chain_name(&input.chain_name)?,
        target_remote_address: refs::normalize_target_remote_address(&input.target_remote_address)?,
        wsl_host_observed: input.wsl_host_observed,
        root_permission_observed: input.root_permission_observed,
        nft_tool_observed: input.nft_tool_observed,
        command_evidence: refs::normalize_command_evidence(input.command_evidence)?,
    })
}
