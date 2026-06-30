use ocentra_screen_ai_core::{
    screen_ai_enforcement_handoff_guard::screen_ai_enforcement_handoff_guard_generated_typescript,
    screen_family_ai_hub_routing::screen_family_ai_hub_routing_generated_typescript,
    screen_intelligence_router::screen_intelligence_router_generated_typescript,
};

#[test]
fn screen_generated_helpers_remain_rust_owned_and_marked() {
    let generated = [
        (
            screen_intelligence_router_generated_typescript(),
            "planScreenIntelligenceRouteGenerated",
        ),
        (
            screen_family_ai_hub_routing_generated_typescript(),
            "planScreenFamilyAiHubRouteGenerated",
        ),
        (
            screen_ai_enforcement_handoff_guard_generated_typescript(),
            "buildScreenAiEnforcementHandoffGuardPayloadGenerated",
        ),
    ];

    for (source, marker) in generated {
        assert!(source.starts_with("/* generated from crates/screen-ai-core/src/"));
        assert!(source.contains(marker));
    }
}
