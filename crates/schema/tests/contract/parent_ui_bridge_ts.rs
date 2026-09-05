use ocentra_schema::parent_ui_bridge_ts::{
    parent_ui_bridge_typescript, parent_ui_bridge_validation_primitives_typescript,
    parent_ui_bridge_validation_typescript,
};

#[test]
fn parent_ui_bridge_typescript_artifact_is_current() {
    assert_eq!(
        include_str!("../../../../apps/portal/generated/parent-ui-bridge.ts"),
        parent_ui_bridge_typescript()
    );
}

#[test]
fn parent_ui_bridge_validation_artifact_is_current() {
    assert_eq!(
        include_str!("../../../../apps/portal/generated/parent-ui-bridge-validation.ts"),
        parent_ui_bridge_validation_typescript()
    );
}

#[test]
fn parent_ui_bridge_validation_primitives_artifact_is_current() {
    assert_eq!(
        include_str!("../../../../apps/portal/generated/parent-ui-bridge-validation-primitives.ts"),
        parent_ui_bridge_validation_primitives_typescript()
    );
}
