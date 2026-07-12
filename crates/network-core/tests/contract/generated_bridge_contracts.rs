use ocentra_network_core::generated_bridge::{
    network_control_catalog_builders_generated_typescript,
    network_control_catalog_data_generated_typescript,
    network_control_catalog_generated_typescript,
    network_control_catalog_metadata_classifiers_generated_typescript,
    network_control_catalog_metadata_generated_typescript,
    network_control_catalog_metadata_layout_generated_typescript,
    network_control_catalog_metadata_text_generated_typescript,
    network_control_catalog_schema_generated_typescript,
    network_control_catalog_value_helpers_generated_typescript,
};

#[test]
fn network_generated_helpers_remain_rust_owned_and_marked() {
    let generated_catalog = network_control_catalog_generated_typescript();
    let generated_data = network_control_catalog_data_generated_typescript();
    let generated_schema = network_control_catalog_schema_generated_typescript();
    let generated_metadata = network_control_catalog_metadata_generated_typescript();
    let generated_metadata_text = network_control_catalog_metadata_text_generated_typescript();
    let generated_metadata_layout = network_control_catalog_metadata_layout_generated_typescript();
    let generated_metadata_classifiers =
        network_control_catalog_metadata_classifiers_generated_typescript();
    let generated_builders = network_control_catalog_builders_generated_typescript();
    let generated_value_helpers = network_control_catalog_value_helpers_generated_typescript();

    assert!(generated_catalog.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog.ts.txt */"
    ));
    assert!(generated_data.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog_data.ts.txt */"
    ));
    assert!(generated_schema.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog_schema.ts.txt */"
    ));
    assert!(generated_metadata.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog_metadata.ts.txt */"
    ));
    assert!(generated_metadata_text.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog_metadata_text.ts.txt */"
    ));
    assert!(generated_metadata_layout.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog_metadata_layout.ts.txt */"
    ));
    assert!(generated_metadata_classifiers.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog_metadata_classifiers.ts.txt */"
    ));
    assert!(generated_builders.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog_builders.ts.txt */"
    ));
    assert!(generated_value_helpers.starts_with(
        "/* generated from crates/network-core/src/network_control_catalog_value_helpers.ts.txt */"
    ));

    assert_eq!(
        generated_metadata
            .lines()
            .find(|line| *line == "export const runtimeOwnerFor = networkRuntimeOwnerFor;"),
        Some("export const runtimeOwnerFor = networkRuntimeOwnerFor;")
    );
    assert_eq!(
        generated_catalog
            .lines()
            .find(|line| *line == "export const policyLaneFor = networkPolicyLaneFor;"),
        Some("export const policyLaneFor = networkPolicyLaneFor;")
    );
}

#[test]
fn network_generated_catalog_surfaces_stay_checked_in() {
    let checked_in_catalog =
        include_str!("../../../../packages/schema-domain/src/network-control-catalog.ts");
    let checked_in_data =
        include_str!("../../../../packages/schema-domain/src/network-control-catalog-data.ts");
    let checked_in_schema =
        include_str!("../../../../packages/schema-domain/src/network-control-catalog-schema.ts");
    let checked_in_metadata =
        include_str!("../../../../packages/schema-domain/src/network-control-catalog-metadata.ts");
    let checked_in_metadata_text = include_str!(
        "../../../../packages/schema-domain/src/network-control-catalog-metadata-text.ts"
    );
    let checked_in_metadata_layout = include_str!(
        "../../../../packages/schema-domain/src/network-control-catalog-metadata-layout.ts"
    );
    let checked_in_metadata_classifiers = include_str!(
        "../../../../packages/schema-domain/src/network-control-catalog-metadata-classifiers.ts"
    );
    let checked_in_builders =
        include_str!("../../../../packages/schema-domain/src/network-control-catalog-builders.ts");
    let checked_in_value_helpers = include_str!(
        "../../../../packages/schema-domain/src/network-control-catalog-value-helpers.ts"
    );

    assert_eq!(
        checked_in_catalog,
        network_control_catalog_generated_typescript()
    );
    assert_eq!(
        checked_in_data,
        network_control_catalog_data_generated_typescript()
    );
    assert_eq!(
        checked_in_schema,
        network_control_catalog_schema_generated_typescript()
    );
    assert_eq!(
        checked_in_metadata,
        network_control_catalog_metadata_generated_typescript()
    );
    assert_eq!(
        checked_in_metadata_text,
        network_control_catalog_metadata_text_generated_typescript()
    );
    assert_eq!(
        checked_in_metadata_layout,
        network_control_catalog_metadata_layout_generated_typescript()
    );
    assert_eq!(
        checked_in_metadata_classifiers,
        network_control_catalog_metadata_classifiers_generated_typescript()
    );
    assert_eq!(
        checked_in_builders,
        network_control_catalog_builders_generated_typescript()
    );
    assert_eq!(
        checked_in_value_helpers,
        network_control_catalog_value_helpers_generated_typescript()
    );
}
