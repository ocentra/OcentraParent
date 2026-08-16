use crate::linux_adapter_gate::{
    NetworkLinuxAdapterGateProof, NetworkLinuxAdapterGateState, NetworkLinuxAdapterKind,
};

use super::NetworkLinuxNftablesLabExecutionError;

pub(super) fn validate_gate_proof(
    gate_proof: &NetworkLinuxAdapterGateProof,
) -> Result<(), NetworkLinuxNftablesLabExecutionError> {
    if gate_proof.gate_state != NetworkLinuxAdapterGateState::DistroProofReady
        || !gate_proof.distro_proof_ready
    {
        return Err(NetworkLinuxNftablesLabExecutionError::GateProofNotDistroReady);
    }
    if gate_proof.adapter_kind != NetworkLinuxAdapterKind::Nftables {
        return Err(NetworkLinuxNftablesLabExecutionError::UnsupportedAdapterKind);
    }
    Ok(())
}
