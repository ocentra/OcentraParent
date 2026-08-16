use crate::support::assert_contract_contains;
use ocentra_schema::parent_ui_bridge::{
    PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION, PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX,
    PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET, PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE,
};
use ocentra_schema::parent_ui_bridge_ts::parent_ui_bridge_typescript;

#[test]
fn screen_settings_command_runtime_literals_are_rust_owned() {
    assert_eq!(PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION, 1);
    assert_eq!(
        PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX,
        "screen-settings-request-"
    );
    assert_eq!(PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET, "get");
    assert_eq!(PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE, "replace");
}

#[test]
fn portal_bridge_generates_screen_settings_command_encoder() {
    let generated = parent_ui_bridge_typescript();
    let runtime_schema_version =
        format!("SchemaVersion: {PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION}");
    let runtime_request_id_prefix =
        format!("RequestIdPrefix: '{PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX}'");
    let get_kind_value = format!("Get: '{PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET}'");
    let replace_kind_value = format!("Replace: '{PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE}'");

    assert_contract_contains(
        crate::contract_text!(&generated),
        crate::contract_text!("export const ParentScreenSettingsCommandRuntime = {"),
    );
    assert_contract_contains(
        crate::contract_text!(&generated),
        crate::contract_text!(&runtime_schema_version),
    );
    assert_contract_contains(
        crate::contract_text!(&generated),
        crate::contract_text!(&runtime_request_id_prefix),
    );
    assert_contract_contains(
        crate::contract_text!(&generated),
        crate::contract_text!(&get_kind_value),
    );
    assert_contract_contains(
        crate::contract_text!(&generated),
        crate::contract_text!(&replace_kind_value),
    );
    assert_contract_contains(
        crate::contract_text!(&generated),
        crate::contract_text!("export function parentScreenSettingsGetCommandDraft("),
    );
    assert_contract_contains(
        crate::contract_text!(&generated),
        crate::contract_text!("export function parentScreenSettingsReplaceCommandDraft(input: {"),
    );
    assert_contract_contains(
        crate::contract_text!(&generated),
        crate::contract_text!(
            "[ParentUiActionPayloadField.ScreenSettingsRequest]: JSON.stringify(request),"
        ),
    );
    assert_contract_contains(
        crate::contract_text!(&generated),
        crate::contract_text!("[ParentUiActionPayloadField.ScreenSettingsUpdateKind]: kind"),
    );
}
