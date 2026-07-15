use std::fs;
use std::path::PathBuf;

#[test]
fn child_android_proof_contracts_generated_typescript_matches_checked_in_files() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let checks = [
        (
            ocentra_schema::child_android_proof_contracts_ts::child_android_device_proof_artifact_gate_typescript(),
            manifest_dir.join("../../packages/schema-domain/src/child-android-device-proof-artifact-gate.ts"),
            "/* generated from crates/schema/src/child_android_device_proof_artifact_gate_ts.rs */",
        ),
        (
            ocentra_schema::child_android_proof_contracts_ts::child_android_lifecycle_proof_typescript(),
            manifest_dir.join("../../packages/schema-domain/src/child-android-lifecycle-proof.ts"),
            "/* generated from crates/schema/src/child_android_lifecycle_proof_ts.rs */",
        ),
        (
            ocentra_schema::child_android_proof_contracts_ts::child_android_permission_capability_proof_typescript(),
            manifest_dir.join("../../packages/schema-domain/src/child-android-permission-capability-proof.ts"),
            "/* generated from crates/schema/src/child_android_permission_capability_proof_ts.rs */",
        ),
        (
            ocentra_schema::child_android_proof_contracts_ts::child_android_privileged_capability_proof_typescript(),
            manifest_dir.join("../../packages/schema-domain/src/child-android-privileged-capability-proof.ts"),
            "/* generated from crates/schema/src/child_android_privileged_capability_proof_ts.rs */",
        ),
        (
            ocentra_schema::child_android_proof_contracts_ts::child_android_service_protocol_proof_typescript(),
            manifest_dir.join("../../packages/schema-domain/src/child-android-service-protocol-proof.ts"),
            "/* generated from crates/schema/src/child_android_service_protocol_proof_ts.rs */",
        ),
        (
            ocentra_schema::child_android_proof_contracts_ts::child_android_storage_protocol_proof_typescript(),
            manifest_dir.join("../../packages/schema-domain/src/child-android-storage-protocol-proof.ts"),
            "/* generated from crates/schema/src/child_android_storage_protocol_proof_ts.rs */",
        ),
    ];

    for (generated, path, header) in checks {
        let checked_in = match fs::read_to_string(path) {
            Ok(value) => value,
            Err(_) => std::process::abort(),
        };
        assert_eq!(generated, checked_in);
        assert!(generated.starts_with(header));
    }
}
