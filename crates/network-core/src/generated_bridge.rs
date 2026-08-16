pub fn network_control_catalog_generated_typescript() -> String {
    include_str!("network_control_catalog.ts.txt").to_owned()
}

pub fn network_control_catalog_data_generated_typescript() -> String {
    [
        include_str!("network_control_catalog_data.ts.txt"),
        include_str!("network_control_catalog_data_types.ts.txt"),
        include_str!("network_control_catalog_data_labels.ts.txt"),
        include_str!("network_control_catalog_data_settings_01.ts.txt"),
        include_str!("network_control_catalog_data_settings_02.ts.txt"),
        include_str!("network_control_catalog_data_settings_03.ts.txt"),
        include_str!("network_control_catalog_data_settings_04.ts.txt"),
        include_str!("network_control_catalog_data_settings_05.ts.txt"),
        include_str!("network_control_catalog_data_settings_06.ts.txt"),
        include_str!("network_control_catalog_data_settings_07.ts.txt"),
        include_str!("network_control_catalog_data_settings_08.ts.txt"),
        include_str!("network_control_catalog_data_settings_09.ts.txt"),
        include_str!("network_control_catalog_data_settings_10.ts.txt"),
        include_str!("network_control_catalog_data_settings_11.ts.txt"),
        include_str!("network_control_catalog_data_settings_12.ts.txt"),
        include_str!("network_control_catalog_data_settings_13.ts.txt"),
        include_str!("network_control_catalog_data_capabilities.ts.txt"),
    ]
    .concat()
}

pub fn network_control_catalog_schema_generated_typescript() -> String {
    include_str!("network_control_catalog_schema.ts.txt").to_owned()
}

pub fn network_control_catalog_metadata_generated_typescript() -> String {
    include_str!("network_control_catalog_metadata.ts.txt").to_owned()
}

pub fn network_control_catalog_metadata_text_generated_typescript() -> String {
    include_str!("network_control_catalog_metadata_text.ts.txt").to_owned()
}

pub fn network_control_catalog_metadata_layout_generated_typescript() -> String {
    include_str!("network_control_catalog_metadata_layout.ts.txt").to_owned()
}

pub fn network_control_catalog_metadata_classifiers_generated_typescript() -> String {
    include_str!("network_control_catalog_metadata_classifiers.ts.txt").to_owned()
}

pub fn network_control_catalog_builders_generated_typescript() -> String {
    include_str!("network_control_catalog_builders.ts.txt").to_owned()
}

pub fn network_control_catalog_value_helpers_generated_typescript() -> String {
    include_str!("network_control_catalog_value_helpers.ts.txt").to_owned()
}
