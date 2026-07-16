use ocentra_child_enforcement_core::v08_browser_enforcement_timer_recovery_proof_values::v08_browser_enforcement_timer_recovery_proof_values_typescript;

#[test]
fn v08_browser_enforcement_timer_recovery_proof_values_typescript_stays_rust_owned_and_self_contained(
) {
    let generated = v08_browser_enforcement_timer_recovery_proof_values_typescript();
    let first_lines: Vec<&str> = generated.lines().take(10).collect();

    assert_eq!(
        first_lines,
        vec![
            "import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';",
            "import {",
            "  ParentControlCapabilityName,",
            "  ParentControlCapabilityNameSchema,",
            "  ParentControlCapabilityStatus,",
            "  ParentControlCapabilityStatusSchema,",
            "  ParentControlPlatformSchema,",
            "} from './capabilities';",
            "import {",
            "  ParentContractSchemaVersion,",
        ]
    );
    assert_eq!(
        generated.lines().find(|line| {
            *line == "    readModelId: 'v0-8-browser-enforcement-timer-recovery-proof',"
        }),
        Some("    readModelId: 'v0-8-browser-enforcement-timer-recovery-proof',")
    );
    assert_eq!(
        generated
            .lines()
            .find(|line| *line == "  ParentContractSchemaVersionSchema,"),
        Some("  ParentContractSchemaVersionSchema,")
    );
    assert_eq!(
        generated
            .lines()
            .find(|line| *line == "} from '@ocentra-parent/schema-domain/v0-8-browser-enforcement-timer-recovery-proof';"),
        None
    );
    assert!(generated.lines().any(|line| line
        == "export const V08BrowserEnforcementTimerRecoveryProofReadModelSchema = withParser("));
}
