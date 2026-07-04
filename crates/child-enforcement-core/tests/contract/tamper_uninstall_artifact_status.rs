use ocentra_child_enforcement_core::tamper_uninstall_artifact_status::tamper_uninstall_artifact_status_typescript;

#[test]
fn tamper_uninstall_artifact_status_generated_typescript_stays_rust_owned_and_self_contained() {
    let generated = tamper_uninstall_artifact_status_typescript();
    let first_lines: Vec<&str> = generated.lines().take(11).collect();

    assert_eq!(
        first_lines,
        vec![
            "/* generated from crates/child-enforcement-core/src/tamper_uninstall_artifact_status.rs */",
            "",
            "import { type Infer, Schema, withParser, NonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';",
            "import { ParentControlCapabilityName, ParentControlCapabilityStatus } from '@ocentra-parent/schema-domain/capabilities';",
            "import type {",
            "  ParentControlCapabilityNameSchema,",
            "  ParentControlCapabilityStatusSchema,",
            "  ParentControlPlatformSchema,",
            "} from '@ocentra-parent/schema-domain/capabilities';",
            "import {",
            "  ParentContractSchemaVersion,",
        ]
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
            .find(|line| *line == "} from '@ocentra-parent/schema-domain/tamper-uninstall-artifact-status';"),
        None
    );
    assert_eq!(
        generated
            .lines()
            .find(|line| *line == "  readModelId: 'tamper-uninstall-artifact-status-proof',"),
        Some("  readModelId: 'tamper-uninstall-artifact-status-proof',")
    );
    assert!(generated
        .lines()
        .any(|line| line == "export const TamperUninstallArtifactStatusReadModelSchema = withParser("));
    assert!(generated
        .lines()
        .any(|line| line == "  'tamper-integrity-audit-contract-proof',"));
}
