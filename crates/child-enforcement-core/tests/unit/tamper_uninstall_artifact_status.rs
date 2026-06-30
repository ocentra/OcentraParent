use ocentra_child_enforcement_core::tamper_uninstall_artifact_status::tamper_uninstall_artifact_status_typescript;

#[test]
fn tamper_uninstall_artifact_status_generated_typescript_targets_schema_domain_contracts() {
    let generated = tamper_uninstall_artifact_status_typescript();

    assert!(generated.contains("@ocentra-parent/schema-domain/tamper-uninstall-artifact-status"));
    assert!(generated.contains("TamperUninstallArtifactStatusReadModel"));
    assert!(generated.contains("tamper-uninstall-artifact-status-proof"));
    assert!(!generated.contains("@ocentra-parent/enforcement-domain"));
}
