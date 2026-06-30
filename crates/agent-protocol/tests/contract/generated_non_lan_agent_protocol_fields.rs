use ocentra_parent_agent_protocol::constants;

const GENERATED_NON_LAN_FIELDS_TS: &str = include_str!(
    "../../../../packages/agent-protocol-domain/src/generated/non-lan-agent-protocol-fields.ts"
);

#[test]
fn generated_non_lan_agent_protocol_fields_stay_aligned_with_rust_constants() {
    assert_generated_field(
        "AppGameAdapterExecutionReadinessReadModel",
        constants::field::APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL,
    );
    assert_generated_field(
        "ParentAssistantProviderStatus",
        constants::parent_assistant::FIELD_PROVIDER_STATUS,
    );
    assert_generated_field(
        "ParentAssistantRunCancelResult",
        constants::parent_assistant::FIELD_RUN_CANCEL_RESULT,
    );
    assert_generated_field(
        "ParentAssistantThreadResponse",
        constants::parent_assistant::FIELD_THREAD_RESPONSE,
    );
}

fn assert_generated_field(field_name: &str, expected_value: &str) {
    let expected_line = format!("{field_name}: '{expected_value}'");
    assert!(
        GENERATED_NON_LAN_FIELDS_TS.contains(&expected_line),
        "expected generated TS helper to contain `{expected_line}`"
    );
}
