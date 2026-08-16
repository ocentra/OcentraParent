use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_APPX_ATTRIBUTE_DISPLAY_NAME, APP_GAME_APPX_ATTRIBUTE_ID, APP_GAME_APPX_ATTRIBUTE_NAME,
    APP_GAME_APPX_ATTRIBUTE_PUBLISHER, APP_GAME_APPX_ELEMENT_APPLICATION,
    APP_GAME_APPX_ELEMENT_DISPLAY_NAME, APP_GAME_APPX_ELEMENT_IDENTITY,
    APP_GAME_APPX_ELEMENT_VISUAL_ELEMENTS, APP_GAME_CONFIDENCE_STORE_PACKAGE_MANIFEST,
    APP_GAME_INVENTORY_STATE_INSTALLED,
};
use ocentra_parent_agent_protocol::constants;

use super::app_game_windows_store_inventory::WindowsStorePackageInventoryRecord;

pub fn record_from_manifest_xml(
    observed_at: &str,
    source_ref: String,
    manifest: &str,
) -> Option<WindowsStorePackageInventoryRecord> {
    let document = roxmltree::Document::parse(manifest).ok()?;
    let identity = document
        .descendants()
        .find(|node| node.has_tag_name(APP_GAME_APPX_ELEMENT_IDENTITY))?;
    let package_name = required_attribute(identity, APP_GAME_APPX_ATTRIBUTE_NAME)?;
    let application = document
        .descendants()
        .find(|node| node.has_tag_name(APP_GAME_APPX_ELEMENT_APPLICATION));
    let application_id =
        application.and_then(|node| optional_attribute(node, APP_GAME_APPX_ATTRIBUTE_ID));
    let display_label = display_label(&document).unwrap_or_else(|| package_name.clone());

    Some(WindowsStorePackageInventoryRecord {
        observed_at: observed_at.to_string(),
        source_ref,
        display_label,
        package_id: Some(package_name.clone()),
        bundle_id: optional_attribute(identity, APP_GAME_APPX_ATTRIBUTE_PUBLISHER),
        app_user_model_id: application_id.map(|id| app_user_model_id(&package_name, &id)),
        store_id: None,
        catalog_ref: None,
        category_kind: None,
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        confidence: APP_GAME_CONFIDENCE_STORE_PACKAGE_MANIFEST,
        evidence: Vec::new(),
    })
}

fn display_label(document: &roxmltree::Document<'_>) -> Option<String> {
    visual_elements_display_name(document).or_else(|| properties_display_name(document))
}

fn visual_elements_display_name(document: &roxmltree::Document<'_>) -> Option<String> {
    document
        .descendants()
        .find(|node| node.has_tag_name(APP_GAME_APPX_ELEMENT_VISUAL_ELEMENTS))
        .and_then(|node| optional_attribute(node, APP_GAME_APPX_ATTRIBUTE_DISPLAY_NAME))
}

fn properties_display_name(document: &roxmltree::Document<'_>) -> Option<String> {
    document
        .descendants()
        .find(|node| node.has_tag_name(APP_GAME_APPX_ELEMENT_DISPLAY_NAME))
        .and_then(|node| node.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn required_attribute(node: roxmltree::Node<'_, '_>, attribute: &str) -> Option<String> {
    optional_attribute(node, attribute)
}

fn optional_attribute(node: roxmltree::Node<'_, '_>, attribute: &str) -> Option<String> {
    node.attribute(attribute)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn app_user_model_id(package_name: &str, application_id: &str) -> String {
    let mut id = String::from(package_name);
    id.push(constants::delimiter::BANG);
    id.push_str(application_id);
    id
}
