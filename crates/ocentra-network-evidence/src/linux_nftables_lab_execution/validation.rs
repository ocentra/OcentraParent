use super::types::{
    NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
    NetworkLinuxNftablesLabExecutionError, NetworkLinuxNftablesLabExecutionInput,
    NetworkLinuxNftablesLabExecutionState, NetworkLinuxNftablesLabUnsupportedClaims,
};
use crate::linux_adapter_gate::NetworkLinuxAdapterGateProof;

mod claims;
mod commands;
mod gate;
mod normalization;
mod refs;
mod state;

pub struct NormalizedLabExecutionInput {
    pub lab_ref: String,
    pub gate_proof: NetworkLinuxAdapterGateProof,
    pub table_name: String,
    pub chain_name: String,
    pub target_remote_address: String,
    pub wsl_host_observed: bool,
    pub root_permission_observed: bool,
    pub nft_tool_observed: bool,
    pub command_evidence: Vec<NetworkLinuxNftablesLabCommandEvidence>,
}

#[derive(Clone, Copy)]
pub struct CommandEvidenceFlags {
    pub create_table: bool,
    pub create_chain: bool,
    pub add_rule: bool,
    pub verify_present: bool,
    pub delete_table: bool,
    pub verify_removed: bool,
}

pub fn normalize_input(
    input: NetworkLinuxNftablesLabExecutionInput,
) -> Result<NormalizedLabExecutionInput, NetworkLinuxNftablesLabExecutionError> {
    normalization::normalize_input(input)
}

pub fn command_flags(evidence: &[NetworkLinuxNftablesLabCommandEvidence]) -> CommandEvidenceFlags {
    commands::command_flags(evidence)
}

pub fn execution_state(
    wsl_host_observed: bool,
    root_permission_observed: bool,
    nft_tool_observed: bool,
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
    flags: CommandEvidenceFlags,
) -> Result<NetworkLinuxNftablesLabExecutionState, NetworkLinuxNftablesLabExecutionError> {
    state::execution_state(
        wsl_host_observed,
        root_permission_observed,
        nft_tool_observed,
        evidence,
        flags,
    )
}
