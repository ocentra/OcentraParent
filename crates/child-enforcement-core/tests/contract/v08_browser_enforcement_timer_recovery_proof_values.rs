use ocentra_child_enforcement_core::v08_browser_enforcement_timer_recovery_proof_values::v08_browser_enforcement_timer_recovery_proof_values_typescript;

#[test]
fn v08_browser_enforcement_timer_recovery_proof_values_typescript_targets_schema_domain_contracts()
{
    let generated = v08_browser_enforcement_timer_recovery_proof_values_typescript();
    let first_lines: Vec<&str> = generated.lines().take(10).collect();

    assert_eq!(
        first_lines,
        vec![
            "/* generated from crates/child-enforcement-core/src/v08_browser_enforcement_timer_recovery_proof_values.rs */",
            "",
            "import {",
            "  ParentControlCapabilityName,",
            "  ParentControlCapabilityStatus,",
            "  type ParentControlCapabilityStatus as ParentControlCapabilityStatusType,",
            "} from '@ocentra-parent/schema-domain/capabilities';",
            "import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';",
            "import {",
            "  V08BrowserEnforcementTimerRecoveryProofReadModelSchema,",
        ]
    );
    assert_eq!(
        generated.lines().find(|line| {
            *line == "    readModelId: 'v0-8-browser-enforcement-timer-recovery-proof',"
        }),
        Some("    readModelId: 'v0-8-browser-enforcement-timer-recovery-proof',")
    );
    assert!(generated
        .lines()
        .any(|line| line == "  V08BrowserEnforcementTimerRecoveryProofReadModelSchema,"));
}
