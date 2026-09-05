use crate::support::extract_typescript_block;
use ocentra_schema::parent_ui_bridge_ts::{
    parent_ui_bridge_typescript, portal_contracts_typescript,
};

const OPEN_BRACE: char = 123u8 as char;
const CLOSE_BRACE: char = 125u8 as char;

#[test]
fn generated_agent_protocol_literals_cover_rust_owned_transport_enums() {
    let parent_generated = parent_ui_bridge_typescript();
    let portal_generated = portal_contracts_typescript();
    let transport_source = include_str!("../../../../../crates/agent-protocol/src/transport.rs");

    for (generated, object_name, enum_name) in [
        (&parent_generated, "ParentAgentCommand", "AgentCommandName"),
        (&parent_generated, "ParentAgentEvent", "AgentEventName"),
        (
            &portal_generated,
            "GeneratedPortalAgentCommand",
            "AgentCommandName",
        ),
        (
            &portal_generated,
            "GeneratedPortalAgentEvent",
            "AgentEventName",
        ),
    ] {
        let object = extract_typescript_block(
            crate::contract_text!(generated),
            crate::text_boundary!(
                &format!("export const {object_name} = {OPEN_BRACE}"),
                &format!("{CLOSE_BRACE} as const;")
            ),
        );
        let enum_start = crate::support::option_or_unreachable(
            transport_source.find(&format!("pub enum {enum_name} {OPEN_BRACE}")),
            crate::assert_context!("expected transport enum to exist"),
        );
        let enum_body = crate::support::option_or_unreachable(
            transport_source[enum_start..].split_once(OPEN_BRACE),
            crate::assert_context!("expected transport enum body to start"),
        )
        .1;
        let enum_body = crate::support::option_or_unreachable(
            enum_body.split_once(&format!("\n{CLOSE_BRACE}")),
            crate::assert_context!("expected transport enum body to end"),
        )
        .0;
        let variants: Vec<&'static str> = enum_body
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                let variant = trimmed.strip_suffix(',')?;
                variant
                    .chars()
                    .next()
                    .filter(char::is_ascii_uppercase)
                    .map(|_| variant)
            })
            .collect();

        for variant in &variants {
            let generated_key = crate::support::option_or_unreachable(
                variant.strip_prefix("Agent"),
                crate::assert_context!("agent protocol variant must use Agent prefix"),
            );
            assert!(
                object.0.contains(&format!("{generated_key}: ")),
                "{object_name} omits {variant}"
            );
        }
        assert_eq!(
            object.0.matches(": \"agent.").count(),
            variants.len(),
            "{object_name} must not contain extra or duplicate protocol literals"
        );
    }
}
