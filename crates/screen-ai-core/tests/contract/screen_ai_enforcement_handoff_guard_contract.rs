use ocentra_screen_ai_core::screen_ai_enforcement_handoff_guard::screen_ai_enforcement_handoff_guard_generated_typescript;

#[test]
fn screen_ai_enforcement_handoff_guard_contract_stays_rust_owned_and_replaces_schema_domain_owner()
{
    let source = screen_ai_enforcement_handoff_guard_generated_typescript();

    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/screen-ai-enforcement-handoff-guard-proof';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("export type ScreenAiEnforcementHandoffGuardInput = {")
            .count(),
        1
    );
    assert_eq!(
        source
            .matches("export type ScreenAiEnforcementHandoffGuardPayload = {")
            .count(),
        1
    );
    assert_eq!(
        source
            .matches("export function buildScreenAiEnforcementHandoffGuardPayloadGenerated(")
            .count(),
        1
    );
}
