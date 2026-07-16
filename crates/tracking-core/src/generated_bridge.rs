pub fn tracking_runtime_generated_typescript() -> String {
    [
        include_str!("tracking_runtime_generated_contracts.ts.txt"),
        include_str!("tracking_runtime_generated.ts.txt"),
    ]
    .join("\n\n")
}

pub fn tracking_retention_runtime_generated_typescript() -> String {
    include_str!("tracking_runtime_generated.payload.txt").to_owned()
}

pub fn tracking_local_place_store_generated_typescript() -> String {
    include_str!("tracking_runtime_generated_expected_place_support.payload.txt").to_owned()
}

pub fn tracking_poi_provider_adapter_generated_typescript() -> String {
    include_str!("tracking_runtime_generated_geofence_support.payload.txt").to_owned()
}

pub fn tracking_policy_compiler_runtime_proof_generated_typescript() -> String {
    include_str!("tracking_runtime_generated_expected_place_decision.payload.txt").to_owned()
}

pub fn tracking_control_catalog_schema_generated_typescript() -> String {
    include_str!("tracking_control_catalog_schema.ts.txt").to_owned()
}

pub fn tracking_control_catalog_metadata_generated_typescript() -> String {
    include_str!("../../../packages/schema-domain/src/tracking-control-catalog-metadata.ts")
        .to_owned()
}

pub fn tracking_control_catalog_metadata_classifiers_generated_typescript() -> String {
    include_str!(
        "../../../packages/schema-domain/src/tracking-control-catalog-metadata-classifiers.ts"
    )
    .to_owned()
}

pub fn tracking_control_catalog_metadata_layout_generated_typescript() -> String {
    include_str!("../../../packages/schema-domain/src/tracking-control-catalog-metadata-layout.ts")
        .to_owned()
}

pub fn tracking_control_catalog_data_generated_typescript() -> String {
    include_str!("../../../packages/schema-domain/src/tracking-control-catalog-data.ts").to_owned()
}

pub fn tracking_control_catalog_options_generated_typescript() -> String {
    include_str!("../../../packages/schema-domain/src/tracking-control-catalog-options.ts")
        .to_owned()
}

pub fn tracking_control_catalog_build_generated_typescript() -> String {
    include_str!("../../../packages/schema-domain/src/tracking-control-catalog-build.ts").to_owned()
}

pub fn tracking_control_catalog_generated_typescript() -> String {
    include_str!("../../../packages/schema-domain/src/tracking-control-catalog.ts").to_owned()
}
