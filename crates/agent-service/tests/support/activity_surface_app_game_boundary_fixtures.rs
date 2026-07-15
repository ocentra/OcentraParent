use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{
    AppGameEvidenceClaim, AppGameIdentity, APP_GAME_CATALOG_READY,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_FOREGROUND_FOREGROUND,
    APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_RUNNING, APP_GAME_SCHEMA_VERSION,
};

use super::activity_surface_common_fixtures::TEST_THIRD_OBSERVED_AT;

pub(crate) fn evidence_claim(evidence: ActivityEvidenceRef) -> AppGameEvidenceClaim {
    AppGameEvidenceClaim {
        schema_version: APP_GAME_SCHEMA_VERSION,
        claim_id: "app-evidence-claim-1".to_string(),
        observed_at: TEST_THIRD_OBSERVED_AT.to_string(),
        claim_kind: "inventory".to_string(),
        observation_mode: "processSnapshot".to_string(),
        display_name: "Ocentra Game".to_string(),
        identity_strength: "catalogMatched".to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_FOREGROUND.to_string(),
        inventory_entry_id: Some("game-inventory-1".to_string()),
        process_identity: Some("proc-game-1".to_string()),
        launcher_ref: Some("steam".to_string()),
        catalog_ref: Some("catalog-game-1".to_string()),
        confidence: 0.93,
        evidence: vec![evidence],
    }
}

pub(crate) fn identity_row(evidence: ActivityEvidenceRef) -> AppGameIdentity {
    AppGameIdentity {
        schema_version: APP_GAME_SCHEMA_VERSION,
        identity_id: "app-identity-1".to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_GAME.to_string(),
        display_label: "Ocentra Game".to_string(),
        parent_label: None,
        confidence: "deterministic".to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        application_token_ref: None,
        executable_path_ref: None,
        publisher_signature_ref: None,
        file_hash_ref: None,
        launcher_ref: Some("steam".to_string()),
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: Some("catalog-game-1".to_string()),
        child_game_evidence_claim_id: Some("app-evidence-claim-1".to_string()),
        evidence: vec![evidence],
    }
}
