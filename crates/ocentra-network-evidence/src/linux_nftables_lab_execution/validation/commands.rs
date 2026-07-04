use super::{
    CommandEvidenceFlags, NetworkLinuxNftablesLabCommandEvidence,
    NetworkLinuxNftablesLabCommandKind,
};

pub(super) fn command_flags(
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
) -> CommandEvidenceFlags {
    CommandEvidenceFlags {
        create_table: has_kind(evidence, NetworkLinuxNftablesLabCommandKind::CreateTable),
        create_chain: has_kind(evidence, NetworkLinuxNftablesLabCommandKind::CreateChain),
        add_rule: has_kind(evidence, NetworkLinuxNftablesLabCommandKind::AddRule),
        verify_present: has_kind(
            evidence,
            NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
        ),
        delete_table: has_kind(evidence, NetworkLinuxNftablesLabCommandKind::DeleteTable),
        verify_removed: has_kind(
            evidence,
            NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
        ),
    }
}

fn has_kind(
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
    kind: NetworkLinuxNftablesLabCommandKind,
) -> bool {
    evidence.iter().any(|command| command.kind == kind)
}
