use ocentra_child_enforcement_core::tamper_uninstall_artifact_status::tamper_uninstall_artifact_status_typescript;

#[test]
fn tamper_uninstall_artifact_status_generated_typescript_stays_rust_owned_and_self_contained() {
    let generated = tamper_uninstall_artifact_status_typescript();
    assert!(generated.lines().any(
        |line| line == "import { type Infer, Schema, withParser, NonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';"
    ));
    assert!(generated
        .lines()
        .any(|line| line == "} from './tamper_uninstall_artifact_status_support';"));
    assert_eq!(
        generated
            .matches("@ocentra-parent/schema-domain/capabilities';")
            .count(),
        0
    );
    assert_eq!(
        generated
            .matches("@ocentra-parent/schema-domain/family-reference-primitives';")
            .count(),
        0
    );
    assert_eq!(
        generated
            .matches("@ocentra-parent/schema-domain/enforcement-proof-shape';")
            .count(),
        0
    );
    assert_eq!(
        generated.lines().find(|line| *line
            == "} from '@ocentra-parent/schema-domain/tamper-uninstall-artifact-status';"),
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
        .any(|line| line
            == "export const TamperUninstallArtifactStatusReadModelSchema = withParser("));
    assert!(generated
        .lines()
        .any(|line| line == "  'tamper-integrity-audit-contract-proof',"));
}
