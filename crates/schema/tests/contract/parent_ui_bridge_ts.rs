use ocentra_schema::parent_ui_bridge_ts::parent_ui_bridge_typescript;

#[test]
fn parent_ui_bridge_typescript_artifact_is_current() {
    assert_eq!(
        include_str!("../../../../apps/portal/generated/parent-ui-bridge.ts"),
        parent_ui_bridge_typescript()
    );
}
