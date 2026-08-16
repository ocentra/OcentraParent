use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryCategoryCandidate, APP_GAME_INVENTORY_CATEGORY_GAME,
    APP_GAME_PRODUCT_NATIVE_GAME,
};

use super::super::WindowsStorePackageInventoryRecord;

pub(crate) fn category_candidates_for_record(
    record: &WindowsStorePackageInventoryRecord,
    product_kind: &str,
) -> Vec<AppGameInventoryCategoryCandidate> {
    if product_kind == APP_GAME_PRODUCT_NATIVE_GAME {
        vec![AppGameInventoryCategoryCandidate {
            category_kind: APP_GAME_INVENTORY_CATEGORY_GAME.to_string(),
            confidence: record.confidence,
            catalog_ref: record.catalog_ref.clone(),
            evidence: record.evidence.clone(),
        }]
    } else {
        Vec::new()
    }
}
