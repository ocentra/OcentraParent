use ocentra_network_core::generated_bridge::network_control_catalog_metadata_generated_typescript;

#[test]
fn network_generated_helpers_remain_rust_owned_and_marked() {
    let generated = network_control_catalog_metadata_generated_typescript();
    let first_export = generated
        .lines()
        .find(|line| line.starts_with("export function "));
    let runtime_owner_signature = generated
        .lines()
        .find(|line| line == &"export function runtimeOwnerFor(");

    assert!(generated.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog_metadata.ts.txt */"
    ));
    assert_eq!(first_export, Some("export function policyLaneFor(sectionTitle: string, groupTitle: string, sourceText: string) {"));
    assert_eq!(
        runtime_owner_signature,
        Some("export function runtimeOwnerFor(")
    );
}

#[test]
fn network_generated_catalog_metadata_surface_stays_checked_in() {
    let checked_in =
        include_str!("../../../../packages/schema-domain/src/network-control-catalog-metadata.ts");

    assert_eq!(
        checked_in,
        network_control_catalog_metadata_generated_typescript()
    );
}
