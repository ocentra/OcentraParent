use ocentra_child_enforcement_core::v08_browser_enforcement_timer_recovery_proof_values::v08_browser_enforcement_timer_recovery_proof_values_typescript;

#[test]
fn v08_browser_enforcement_timer_recovery_proof_values_typescript_targets_schema_domain_contracts()
{
    let generated = v08_browser_enforcement_timer_recovery_proof_values_typescript();

    assert!(generated
        .contains("@ocentra-parent/schema-domain/v0-8-browser-enforcement-timer-recovery-proof"));
    assert!(generated.contains("V08BrowserEnforcementTimerRecoveryProofReadModel"));
    assert!(generated.contains("v0-8-browser-enforcement-timer-recovery-proof"));
    assert!(!generated.contains("@ocentra-parent/enforcement-domain"));
}
