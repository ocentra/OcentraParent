use ocentra_child_enforcement_core::tamper_uninstall_artifact_status::tamper_uninstall_artifact_status_typescript;

#[test]
fn tamper_uninstall_artifact_status_generated_typescript_targets_schema_domain_contracts() {
    let generated = tamper_uninstall_artifact_status_typescript();
    let first_lines: Vec<&str> = generated.lines().take(10).collect();

    assert_eq!(
        first_lines,
        vec![
            "/* generated from crates/child-enforcement-core/src/tamper_uninstall_artifact_status.rs */",
            "",
            "import { ParentControlCapabilityName, ParentControlCapabilityStatus } from '@ocentra-parent/schema-domain/capabilities';",
            "import type {",
            "  ParentControlCapabilityNameSchema,",
            "  ParentControlCapabilityStatusSchema,",
            "  ParentControlPlatformSchema,",
            "} from '@ocentra-parent/schema-domain/capabilities';",
            "import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';",
            "import {",
        ]
    );
    assert_eq!(
        generated
            .lines()
            .find(|line| *line == "  readModelId: 'tamper-uninstall-artifact-status-proof',"),
        Some("  readModelId: 'tamper-uninstall-artifact-status-proof',")
    );
    assert_eq!(
        generated
            .lines()
            .find(|line| *line == "  readModelId: 'tamper-uninstall-artifact-status-proof',"),
        Some("  readModelId: 'tamper-uninstall-artifact-status-proof',")
    );
    assert!(generated
        .lines()
        .any(|line| line == "  TamperUninstallArtifactStatusReadModelSchema,"));
    assert!(generated
        .lines()
        .any(|line| line == "  'tamper-integrity-audit-contract-proof',"));
}
