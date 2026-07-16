use super::{
    NetworkWindowsFirewallLabCommandEvidence, NetworkWindowsFirewallLabCommandKind,
    NetworkWindowsFirewallLabExecutionError,
};

pub(super) fn validate_successful_execution(
    evidence: &[NetworkWindowsFirewallLabCommandEvidence],
) -> Result<(), NetworkWindowsFirewallLabExecutionError> {
    let apply = command(evidence, NetworkWindowsFirewallLabCommandKind::ApplyRule)?;
    let verify_present = command(
        evidence,
        NetworkWindowsFirewallLabCommandKind::VerifyRulePresent,
    )?;
    let rollback = command(evidence, NetworkWindowsFirewallLabCommandKind::RollbackRule)?;
    let verify_removed = command(
        evidence,
        NetworkWindowsFirewallLabCommandKind::VerifyRuleRemoved,
    )?;
    for command in [apply, verify_present, rollback, verify_removed] {
        if command.exit_status != 0 {
            return Err(
                NetworkWindowsFirewallLabExecutionError::CommandEvidenceFailure(command.kind),
            );
        }
    }
    if !verify_present.rule_present_after_command {
        return Err(NetworkWindowsFirewallLabExecutionError::ApplyRuleNotObserved);
    }
    if verify_removed.rule_present_after_command {
        return Err(NetworkWindowsFirewallLabExecutionError::RollbackRuleStillPresent);
    }
    Ok(())
}

fn command(
    evidence: &[NetworkWindowsFirewallLabCommandEvidence],
    kind: NetworkWindowsFirewallLabCommandKind,
) -> Result<&NetworkWindowsFirewallLabCommandEvidence, NetworkWindowsFirewallLabExecutionError> {
    evidence
        .iter()
        .find(|command| command.kind == kind)
        .ok_or(NetworkWindowsFirewallLabExecutionError::MissingCommandEvidence(kind))
}
