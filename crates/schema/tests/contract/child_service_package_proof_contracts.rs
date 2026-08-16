use std::fs;
use std::path::PathBuf;

#[test]
fn child_service_package_proof_contracts_generated_typescript_matches_checked_in_files() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let checks = [
        (
            ocentra_schema::child_service_package_proof_contracts_ts::child_linux_service_package_proof_typescript(),
            manifest_dir.join("../../packages/schema-domain/src/child-linux-service-package-proof.ts"),
            "/* generated from crates/schema/src/child_linux_service_package_proof_ts.rs */",
        ),
        (
            ocentra_schema::child_service_package_proof_contracts_ts::child_macos_service_package_proof_typescript(),
            manifest_dir.join("../../packages/schema-domain/src/child-macos-service-package-proof.ts"),
            "/* generated from crates/schema/src/child_macos_service_package_proof_ts.rs */",
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
