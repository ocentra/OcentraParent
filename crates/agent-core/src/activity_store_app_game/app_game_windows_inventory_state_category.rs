use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryCategoryCandidate, APP_GAME_INVENTORY_CATEGORY_GAME,
    APP_GAME_INVENTORY_CATEGORY_LAUNCHER, APP_GAME_PRODUCT_LAUNCHER, APP_GAME_PRODUCT_NATIVE_GAME,
};

use super::super::WindowsInstalledAppInventoryRecord;

pub(crate) fn category_candidates_for_record(
    record: &WindowsInstalledAppInventoryRecord,
    product_kind: &str,
) -> Vec<AppGameInventoryCategoryCandidate> {
    if product_kind == APP_GAME_PRODUCT_NATIVE_GAME {
        vec![category_candidate(record, APP_GAME_INVENTORY_CATEGORY_GAME)]
    } else if product_kind == APP_GAME_PRODUCT_LAUNCHER {
        vec![category_candidate(
            record,
            APP_GAME_INVENTORY_CATEGORY_LAUNCHER,
        )]
    } else {
        Vec::new()
    }
}

fn category_candidate(
    record: &WindowsInstalledAppInventoryRecord,
    category_kind: &str,
) -> AppGameInventoryCategoryCandidate {
    AppGameInventoryCategoryCandidate {
        category_kind: category_kind.to_string(),
        confidence: record.confidence,
        catalog_ref: record.catalog_ref.clone(),
        evidence: record.evidence.clone(),
    }
}
