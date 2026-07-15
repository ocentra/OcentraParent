use super::*;

#[path = "parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_a.rs"]
mod parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_a;
#[path = "parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_b.rs"]
mod parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_b;
#[path = "parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_c.rs"]
mod parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_c;
#[path = "parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_d.rs"]
mod parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_d;

fn activity_surface_decoder_template() -> String {
    [
        parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_a::ACTIVITY_SURFACE_DECODER_TYPESCRIPT_TEMPLATE_A,
        parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_b::ACTIVITY_SURFACE_DECODER_TYPESCRIPT_TEMPLATE_B,
        parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_c::ACTIVITY_SURFACE_DECODER_TYPESCRIPT_TEMPLATE_C,
        parent_agent_protocol_bridge_ts_part2_typescript_decoder_template_d::ACTIVITY_SURFACE_DECODER_TYPESCRIPT_TEMPLATE_D,
    ]
    .join("")
}

pub(super) fn activity_surface_decoder_typescript(
    types: &ActivitySurfaceTypescriptNames,
) -> String {
    let replacements = activity_surface_decoder_replacements(types);
    replace_tokens(activity_surface_decoder_template(), replacements.as_slice())
}

fn activity_surface_decoder_replacements(
    types: &ActivitySurfaceTypescriptNames,
) -> Vec<(&'static str, &str)> {
    let mut replacements = activity_surface_decoder_type_replacements(types);
    replacements.extend(activity_surface_decoder_read_model_replacements(types));
    replacements.extend(activity_surface_decoder_schema_replacements(types));
    replacements
}

fn activity_surface_decoder_type_replacements(
    types: &ActivitySurfaceTypescriptNames,
) -> Vec<(&'static str, &str)> {
    vec![
        ("__SCHEMA_VERSION_CONST__", &types.schema_version_const),
        ("__SCOPE_KIND_CONST__", &types.scope_kind_const),
        ("__SCOPE_KIND_TYPE__", &types.scope_kind_type),
        ("__REPORT_FREQUENCY_CONST__", &types.report_frequency_const),
        ("__REPORT_FREQUENCY_TYPE__", &types.report_frequency_type),
        ("__SECTION_KIND_CONST__", &types.section_kind_const),
        ("__SECTION_KIND_TYPE__", &types.section_kind_type),
        ("__READ_MODEL_STATE_CONST__", &types.read_model_state_const),
        ("__READ_MODEL_STATE_TYPE__", &types.read_model_state_type),
        (
            "__SOURCE_REACHABILITY_CONST__",
            &types.source_reachability_const,
        ),
        (
            "__SOURCE_REACHABILITY_TYPE__",
            &types.source_reachability_type,
        ),
        (
            "__SAVED_REPORT_STATE_CONST__",
            &types.saved_report_state_const,
        ),
        (
            "__SAVED_REPORT_STATE_TYPE__",
            &types.saved_report_state_type,
        ),
        ("__CUSTODY_LABEL_CONST__", &types.custody_label_const),
        ("__CUSTODY_LABEL_TYPE__", &types.custody_label_type),
        ("__SOURCE_LABEL_CONST__", &types.source_label_const),
        ("__SOURCE_LABEL_TYPE__", &types.source_label_type),
        ("__EVIDENCE_KIND_CONST__", &types.evidence_kind_const),
        ("__EVIDENCE_KIND_TYPE__", &types.evidence_kind_type),
        (
            "__READ_MODEL_KIND_NAME_CONST__",
            &types.read_model_kind_name_const,
        ),
        ("__READ_MODEL_KIND_TYPE__", &types.read_model_kind_type),
        ("__PARSER_TYPE__", &types.parser_type),
        ("__EVIDENCE_REF_TYPE__", &types.evidence_ref_type),
        ("__SCOPE_TYPE__", &types.scope_type),
        ("__REQUEST_TYPE__", &types.request_type),
        ("__SOURCE_STATE_TYPE__", &types.source_state_type),
        ("__SECTION_TYPE__", &types.section_type),
        ("__SAVED_METADATA_TYPE__", &types.saved_metadata_type),
        (
            "__SOURCE_STATE_SUMMARY_TYPE__",
            &types.source_state_summary_type,
        ),
        ("__REPORT_DOCUMENT_TYPE__", &types.report_document_type),
        ("__HISTORY_ITEM_TYPE__", &types.history_item_type),
        ("__HISTORY_LIST_TYPE__", &types.history_list_type),
        ("__SOURCE_STATUS_ROW_TYPE__", &types.source_status_row_type),
        ("__TAB_READ_MODEL_TYPE__", &types.tab_read_model_type),
    ]
}

fn activity_surface_decoder_read_model_replacements(
    types: &ActivitySurfaceTypescriptNames,
) -> Vec<(&'static str, &str)> {
    vec![
        ("__SCREEN_ROW_TYPE__", &types.screen_row_type),
        ("__APP_USE_ROW_TYPE__", &types.app_use_row_type),
        ("__BROWSER_ROW_TYPE__", &types.browser_row_type),
        ("__GAMES_ROW_TYPE__", &types.games_row_type),
        ("__NETWORK_ROW_TYPE__", &types.network_row_type),
        ("__SCREEN_READ_MODEL_TYPE__", &types.screen_read_model_type),
        (
            "__APP_USE_READ_MODEL_TYPE__",
            &types.app_use_read_model_type,
        ),
        (
            "__BROWSER_READ_MODEL_TYPE__",
            &types.browser_read_model_type,
        ),
        ("__GAMES_READ_MODEL_TYPE__", &types.games_read_model_type),
        (
            "__NETWORK_READ_MODEL_TYPE__",
            &types.network_read_model_type,
        ),
        (
            "__SURFACE_READ_MODEL_TYPE__",
            &types.surface_read_model_type,
        ),
    ]
}

fn activity_surface_decoder_schema_replacements(
    types: &ActivitySurfaceTypescriptNames,
) -> Vec<(&'static str, &str)> {
    vec![
        (
            "__READ_MODEL_STATE_SCHEMA_CONST__",
            &types.read_model_state_schema_const,
        ),
        ("__REQUEST_SCHEMA_CONST__", &types.request_schema_const),
        (
            "__REPORT_DOCUMENT_SCHEMA_CONST__",
            &types.report_document_schema_const,
        ),
        (
            "__HISTORY_LIST_SCHEMA_CONST__",
            &types.history_list_schema_const,
        ),
        (
            "__SCREEN_READ_MODEL_SCHEMA_CONST__",
            &types.screen_read_model_schema_const,
        ),
        (
            "__APP_USE_READ_MODEL_SCHEMA_CONST__",
            &types.app_use_read_model_schema_const,
        ),
        (
            "__BROWSER_READ_MODEL_SCHEMA_CONST__",
            &types.browser_read_model_schema_const,
        ),
        (
            "__GAMES_READ_MODEL_SCHEMA_CONST__",
            &types.games_read_model_schema_const,
        ),
        (
            "__NETWORK_READ_MODEL_SCHEMA_CONST__",
            &types.network_read_model_schema_const,
        ),
        ("__HELPER_PREFIX__", &types.helper_prefix),
    ]
}
