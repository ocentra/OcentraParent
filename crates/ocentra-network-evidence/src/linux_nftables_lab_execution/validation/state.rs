use super::{
    CommandEvidenceFlags, NetworkLinuxNftablesLabCommandEvidence,
    NetworkLinuxNftablesLabCommandKind, NetworkLinuxNftablesLabExecutionError,
    NetworkLinuxNftablesLabExecutionState,
};

pub(super) fn execution_state(
    wsl_host_observed: bool,
    root_permission_observed: bool,
    nft_tool_observed: bool,
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
    flags: CommandEvidenceFlags,
) -> Result<NetworkLinuxNftablesLabExecutionState, NetworkLinuxNftablesLabExecutionError> {
    if !wsl_host_observed || !nft_tool_observed {
        return Ok(NetworkLinuxNftablesLabExecutionState::Unavailable);
    }
    if !root_permission_observed
        || evidence.is_empty()
        || !flags.create_table
        || !flags.create_chain
        || !flags.add_rule
        || !flags.verify_present
        || !flags.delete_table
        || !flags.verify_removed
    {
        return Ok(NetworkLinuxNftablesLabExecutionState::ManualRequired);
    }
    validate_successful_execution(evidence)?;
    Ok(NetworkLinuxNftablesLabExecutionState::ExecutedAndRolledBack)
}

fn validate_successful_execution(
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
) -> Result<(), NetworkLinuxNftablesLabExecutionError> {
    let verify_present = command(
        evidence,
        NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
    )?;
    let verify_removed = command(
        evidence,
        NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
    )?;
    for kind in required_command_kinds() {
        let command = command(evidence, kind)?;
        if command.exit_status != 0 {
            return Err(NetworkLinuxNftablesLabExecutionError::CommandEvidenceFailure(kind));
        }
    }
    if !verify_present.table_present_after_command
        || !verify_present.chain_present_after_command
        || !verify_present.rule_present_after_command
    {
        return Err(NetworkLinuxNftablesLabExecutionError::RuleNotObserved);
    }
    if verify_removed.table_present_after_command {
        return Err(NetworkLinuxNftablesLabExecutionError::RollbackTableStillPresent);
    }
    Ok(())
}

fn required_command_kinds() -> [NetworkLinuxNftablesLabCommandKind; 6] {
    [
        NetworkLinuxNftablesLabCommandKind::CreateTable,
        NetworkLinuxNftablesLabCommandKind::CreateChain,
        NetworkLinuxNftablesLabCommandKind::AddRule,
        NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
        NetworkLinuxNftablesLabCommandKind::DeleteTable,
        NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
    ]
}

fn command(
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
    kind: NetworkLinuxNftablesLabCommandKind,
) -> Result<&NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabExecutionError> {
    evidence
        .iter()
        .find(|command| command.kind == kind)
        .ok_or(NetworkLinuxNftablesLabExecutionError::MissingCommandEvidence(kind))
}
