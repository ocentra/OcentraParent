use ocentra_network_evidence::linux_nftables_lab_execution::types::{
    NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
};
use ocentra_parent_agent_protocol::constants;

pub(super) fn command_evidence_rows() -> Vec<NetworkLinuxNftablesLabCommandEvidence> {
    vec![
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::CreateTable,
            CommandRefText(constants::network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_COMMAND_REF),
            OutputSha256Text(
                constants::network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_OUTPUT_SHA256,
            ),
            true,
            false,
            false,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::CreateChain,
            CommandRefText(constants::network_flow::TEST_LINUX_NFTABLES_CREATE_CHAIN_COMMAND_REF),
            OutputSha256Text(
                constants::network_flow::TEST_LINUX_NFTABLES_CREATE_CHAIN_OUTPUT_SHA256,
            ),
            true,
            true,
            false,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::AddRule,
            CommandRefText(constants::network_flow::TEST_LINUX_NFTABLES_ADD_RULE_COMMAND_REF),
            OutputSha256Text(constants::network_flow::TEST_LINUX_NFTABLES_ADD_RULE_OUTPUT_SHA256),
            true,
            true,
            true,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
            CommandRefText(constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_RULE_COMMAND_REF),
            OutputSha256Text(
                constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_RULE_OUTPUT_SHA256,
            ),
            true,
            true,
            true,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::DeleteTable,
            CommandRefText(constants::network_flow::TEST_LINUX_NFTABLES_DELETE_TABLE_COMMAND_REF),
            OutputSha256Text(
                constants::network_flow::TEST_LINUX_NFTABLES_DELETE_TABLE_OUTPUT_SHA256,
            ),
            false,
            false,
            false,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
            CommandRefText(constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_REMOVED_COMMAND_REF),
            OutputSha256Text(
                constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_REMOVED_OUTPUT_SHA256,
            ),
            false,
            false,
            false,
        ),
    ]
}

struct CommandRefText<'a>(&'a str);

struct OutputSha256Text<'a>(&'a str);

fn command_evidence(
    kind: NetworkLinuxNftablesLabCommandKind,
    command_ref: CommandRefText<'_>,
    output_sha256: OutputSha256Text<'_>,
    table_present_after_command: bool,
    chain_present_after_command: bool,
    rule_present_after_command: bool,
) -> NetworkLinuxNftablesLabCommandEvidence {
    NetworkLinuxNftablesLabCommandEvidence {
        kind,
        command_ref: command_ref.0.to_string(),
        exit_status: 0,
        output_sha256: output_sha256.0.to_string(),
        table_present_after_command,
        chain_present_after_command,
        rule_present_after_command,
    }
}
