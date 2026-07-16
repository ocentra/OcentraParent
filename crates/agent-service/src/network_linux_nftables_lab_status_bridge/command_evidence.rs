use ocentra_network_evidence::linux_nftables_lab_execution::types::{
    NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
};
use ocentra_parent_agent_protocol::constants;

pub(super) fn command_evidence_rows() -> Vec<NetworkLinuxNftablesLabCommandEvidence> {
    vec![
        NetworkLinuxNftablesLabCommandEvidence {
            kind: NetworkLinuxNftablesLabCommandKind::CreateTable,
            command_ref: constants::network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256: constants::network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_OUTPUT_SHA256
                .to_string(),
            table_present_after_command: true,
            chain_present_after_command: false,
            rule_present_after_command: false,
        },
        NetworkLinuxNftablesLabCommandEvidence {
            kind: NetworkLinuxNftablesLabCommandKind::CreateChain,
            command_ref: constants::network_flow::TEST_LINUX_NFTABLES_CREATE_CHAIN_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256: constants::network_flow::TEST_LINUX_NFTABLES_CREATE_CHAIN_OUTPUT_SHA256
                .to_string(),
            table_present_after_command: true,
            chain_present_after_command: true,
            rule_present_after_command: false,
        },
        NetworkLinuxNftablesLabCommandEvidence {
            kind: NetworkLinuxNftablesLabCommandKind::AddRule,
            command_ref: constants::network_flow::TEST_LINUX_NFTABLES_ADD_RULE_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256: constants::network_flow::TEST_LINUX_NFTABLES_ADD_RULE_OUTPUT_SHA256
                .to_string(),
            table_present_after_command: true,
            chain_present_after_command: true,
            rule_present_after_command: true,
        },
        NetworkLinuxNftablesLabCommandEvidence {
            kind: NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
            command_ref: constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_RULE_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256: constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_RULE_OUTPUT_SHA256
                .to_string(),
            table_present_after_command: true,
            chain_present_after_command: true,
            rule_present_after_command: true,
        },
        NetworkLinuxNftablesLabCommandEvidence {
            kind: NetworkLinuxNftablesLabCommandKind::DeleteTable,
            command_ref: constants::network_flow::TEST_LINUX_NFTABLES_DELETE_TABLE_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256: constants::network_flow::TEST_LINUX_NFTABLES_DELETE_TABLE_OUTPUT_SHA256
                .to_string(),
            table_present_after_command: false,
            chain_present_after_command: false,
            rule_present_after_command: false,
        },
        NetworkLinuxNftablesLabCommandEvidence {
            kind: NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
            command_ref: constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_REMOVED_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256:
                constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_REMOVED_OUTPUT_SHA256
                    .to_string(),
            table_present_after_command: false,
            chain_present_after_command: false,
            rule_present_after_command: false,
        },
    ]
}
