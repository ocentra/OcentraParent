use std::error::Error;
use std::primitive::str as TestStr;
use std::string::String as TestString;

use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::activity_surface::source_status::ActivityAppGameSourceStatusRow;
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivitySurfaceRequest, ActivitySurfaceScope, ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameLauncherEvidenceRow,
    AppGameRuntimeEvidenceRow, AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CATALOG_NOT_LOADED, APP_GAME_CATALOG_READY, APP_GAME_CLASSIFICATION_KNOWN_APP,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
    APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED, APP_GAME_FOREGROUND_FOREGROUND,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT,
    APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD, APP_GAME_INVENTORY_SOURCE_SHORTCUT,
    APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE, APP_GAME_INVENTORY_STATE_INSTALLED,
    APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW, APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
    APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_RUNTIME_RUNNING, APP_GAME_SCHEMA_VERSION, APP_GAME_TEST_DISPLAY_LABEL,
    APP_GAME_TITLE_CAPTURE_TITLE_REF,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use ocentra_parent_agent_service::test_support::{
    build_activity_app_use_read_model_from_app_game_for_test,
    build_activity_games_read_model_for_test,
};

const APP_GAME_INVENTORY_STATE_DETECTABLE: &TestStr = "detectable";
const APP_GAME_LAUNCHER_KIND_STEAM: &TestStr = "steam";
const APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY: &TestStr = "launcherOnly";
const APP_GAME_OBSERVATION_MODE_LAUNCHER_MANIFEST: &TestStr = "launcherManifest";
const APP_GAME_TEST_CATALOG_REF: &TestStr = "catalog-ref-ocentra-game";
const APP_GAME_TEST_EXECUTABLE_PATH_REF: &TestStr = "path-ref-ocentra-fixture";
const APP_GAME_TEST_FOREGROUND_EVIDENCE_ID: &TestStr = "foreground-evidence-window-4242";
const APP_GAME_TEST_GAME_DISPLAY_LABEL: &TestStr = "Ocentra Game Fixture";
const APP_GAME_TEST_LAUNCHER_EVIDENCE_ID: &TestStr = "launcher-evidence-steam-only";
const APP_GAME_TEST_LAUNCHER_PROCESS_ID: u64 = 5150;
const APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY: &TestStr = "process-5150";
const APP_GAME_TEST_LAUNCHER_PROCESS_NAME: &TestStr = "steam.exe";
const APP_GAME_TEST_LAUNCHER_REF: &TestStr = "launcher-ref-ocentra";
const APP_GAME_TEST_PROCESS_ID: u64 = 4242;
const APP_GAME_TEST_PROCESS_IDENTITY: &TestStr = "process-4242";
const APP_GAME_TEST_PROCESS_NAME: &TestStr = "ocentra-fixture.exe";
const APP_GAME_TEST_REGISTRY_SOURCE_REF: &TestStr = "source-registry-native-app";
const APP_GAME_TEST_RUNTIME_EVIDENCE_ID: &TestStr = "runtime-evidence-process-4242";
const APP_GAME_TEST_SECOND_SHORTCUT_SOURCE_REF: &TestStr = "source-second-start-menu-shortcut";
const APP_GAME_TEST_STORE_GAME_SOURCE_REF: &TestStr = "source-store-package-game";

#[test]
fn app_use_read_model_groups_app_game_source_status_rows_without_launcher_claims(
) -> Result<(), Box<dyn Error>> {
    let read_model = build_activity_app_use_read_model_from_app_game_for_test(
        surface_request(),
        Some(service_model()?),
    );
    let source_rows = &read_model.rows[0].source_status_rows;

    assert_eq!(source_rows.len(), 4);
    assert_source_status(
        source_rows,
        APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD,
        1,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    );
    assert_source_status(
        source_rows,
        APP_GAME_INVENTORY_SOURCE_SHORTCUT,
        1,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    assert_source_status(
        source_rows,
        APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
        1,
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
    );
    assert_source_status(
        source_rows,
        APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW,
        1,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )?;
    assert!(!source_rows
        .iter()
        .any(|row| row.source_kind == APP_GAME_OBSERVATION_MODE_LAUNCHER_MANIFEST));
    Ok(())
}

#[test]
fn games_read_model_groups_game_inventory_runtime_foreground_and_launcher_source_status_rows(
) -> Result<(), Box<dyn Error>> {
    let read_model =
        build_activity_games_read_model_for_test(surface_request(), Some(service_model()?));
    let source_rows = &read_model.rows[0].source_status_rows;

    assert_eq!(source_rows.len(), 4);
    assert_source_status(
        source_rows,
        APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE,
        1,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    );
    assert_source_status(
        source_rows,
        APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
        1,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    assert_source_status(
        source_rows,
        APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW,
        1,
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
    );
    assert_source_status(
        source_rows,
        APP_GAME_OBSERVATION_MODE_LAUNCHER_MANIFEST,
        1,
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
    )?;
    Ok(())
}

fn assert_source_status(
    rows: &[ActivityAppGameSourceStatusRow],
    source_kind: &TestStr,
    row_count: u64,
    last_observed_at: &TestStr,
) -> Result<(), Box<dyn Error>> {
    let row = rows
        .iter()
        .find(|row| row.source_kind == source_kind)
        .ok_or_else(|| std::io::Error::other(constants::error::ACTIVITY_STORE_QUERIES))?;

    assert_eq!(row.state, ActivityReadModelState::Ready);
    assert_eq!(row.row_count, row_count);
    assert_eq!(row.last_observed_at.as_deref(), Some(last_observed_at));
    assert_eq!(row.capability_status, APP_GAME_CAPABILITY_STATUS_AVAILABLE);
    assert_eq!(row.evidence.len(), 1);
    Ok(())
}

fn service_model() -> Result<AppGameServiceReadModel, Box<dyn Error>> {
    Ok(AppGameServiceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        custody_label:
            ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE
                .to_string(),
        replay_state:
            ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED
                .to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        inventory_returned: 3,
        running_now_returned: 2,
        foreground_now_returned: 2,
        launcher_returned: 1,
        daily_rollup_returned: 0,
        evidence_claim_returned: 0,
        identity_returned: 0,
        approval_authority_returned: 0,
        approval_action_result_returned: 0,
        platform_authority_matrix_returned: 0,
        ai_classifier_result_returned: 0,
        inventory_rows: service_inventory_rows(),
        running_now_rows: service_runtime_rows(),
        foreground_now_rows: service_foreground_rows(),
        launcher_rows: vec![launcher_row()],
        daily_rollups: Vec::new(),
        evidence_claim_rows: Vec::new(),
        identity_rows: Vec::new(),
        approval_authority_rows: Vec::new(),
        approval_action_result_rows: Vec::new(),
        platform_authority_matrices: Vec::new(),
        ai_classifier_result_rows: Vec::new(),
    })
}

fn service_inventory_rows() -> Vec<AppGameInventoryEvidenceRow> {
    vec![
        inventory_row(
            APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD,
            APP_GAME_TEST_REGISTRY_SOURCE_REF,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            APP_GAME_PRODUCT_NATIVE_APP,
            APP_GAME_CLASSIFICATION_KNOWN_APP,
        ),
        inventory_row(
            APP_GAME_INVENTORY_SOURCE_SHORTCUT,
            APP_GAME_TEST_SECOND_SHORTCUT_SOURCE_REF,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            APP_GAME_PRODUCT_NATIVE_APP,
            APP_GAME_CLASSIFICATION_KNOWN_APP,
        ),
        inventory_row(
            APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE,
            APP_GAME_TEST_STORE_GAME_SOURCE_REF,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            APP_GAME_PRODUCT_NATIVE_GAME,
            APP_GAME_CLASSIFICATION_KNOWN_GAME,
        ),
    ]
}

fn service_runtime_rows() -> Vec<AppGameRuntimeEvidenceRow> {
    vec![
        runtime_row(
            APP_GAME_TEST_RUNTIME_EVIDENCE_ID,
            APP_GAME_TEST_PROCESS_IDENTITY,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
            APP_GAME_CLASSIFICATION_KNOWN_APP,
        ),
        runtime_row(
            APP_GAME_TEST_LAUNCHER_EVIDENCE_ID,
            APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            APP_GAME_CLASSIFICATION_KNOWN_GAME,
        ),
    ]
}

fn service_foreground_rows() -> Vec<AppGameForegroundEvidenceRow> {
    vec![
        foreground_row(
            APP_GAME_TEST_FOREGROUND_EVIDENCE_ID,
            APP_GAME_TEST_PROCESS_IDENTITY,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            APP_GAME_CLASSIFICATION_KNOWN_APP,
        ),
        foreground_row(
            APP_GAME_TEST_LAUNCHER_EVIDENCE_ID,
            APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
            APP_GAME_CLASSIFICATION_KNOWN_GAME,
        ),
    ]
}

fn inventory_row(
    source_kind: &TestStr,
    source_ref: &TestStr,
    observed_at: &TestStr,
    product_kind: &TestStr,
    classification_state: &TestStr,
) -> AppGameInventoryEvidenceRow {
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: source_ref.to_string(),
        observed_at: observed_at.to_string(),
        source_kind: source_kind.to_string(),
        source_ref: source_ref.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT.to_string(),
        product_kind: product_kind.to_string(),
        display_label: inventory_label(product_kind),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        inventory_state: inventory_state(product_kind),
        classification_state: classification_state.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.84,
        category_candidates: Vec::new(),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: vec![evidence_ref(source_ref)],
    }
}

fn runtime_row(
    evidence_id: &TestStr,
    process_identity: &TestStr,
    observed_at: &TestStr,
    classification_state: &TestStr,
) -> AppGameRuntimeEvidenceRow {
    AppGameRuntimeEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        runtime_evidence_id: evidence_id.to_string(),
        observed_at: observed_at.to_string(),
        process_identity: process_identity.to_string(),
        process_id: APP_GAME_TEST_PROCESS_ID,
        parent_process_id: None,
        process_name: APP_GAME_TEST_PROCESS_NAME.to_string(),
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        publisher_signature_ref: None,
        file_hash_ref: None,
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        started_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        exited_at: None,
        running_duration_ms: 0,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: classification_state.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.82,
        evidence: vec![evidence_ref(evidence_id)],
    }
}

fn foreground_row(
    evidence_id: &TestStr,
    process_identity: &TestStr,
    observed_at: &TestStr,
    classification_state: &TestStr,
) -> AppGameForegroundEvidenceRow {
    AppGameForegroundEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        foreground_evidence_id: evidence_id.to_string(),
        observed_at: observed_at.to_string(),
        process_identity: process_identity.to_string(),
        process_id: APP_GAME_TEST_PROCESS_ID,
        process_name: APP_GAME_TEST_PROCESS_NAME.to_string(),
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        window_ref: None,
        window_title_ref: None,
        title_capture_state: APP_GAME_TITLE_CAPTURE_TITLE_REF.to_string(),
        foreground_started_at: Some(observed_at.to_string()),
        foreground_ended_at: None,
        foreground_duration_ms: 0,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_FOREGROUND.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW.to_string(),
        classification_state: classification_state.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        content_knowledge_state: APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED.to_string(),
        confidence: 0.84,
        evidence: vec![evidence_ref(evidence_id)],
    }
}

fn launcher_row() -> AppGameLauncherEvidenceRow {
    AppGameLauncherEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        launcher_evidence_id: APP_GAME_TEST_LAUNCHER_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string(),
        launcher_kind: APP_GAME_LAUNCHER_KIND_STEAM.to_string(),
        launcher_ref: APP_GAME_TEST_LAUNCHER_REF.to_string(),
        launcher_inventory_entry_id: None,
        launcher_manifest_id: None,
        launcher_app_id: None,
        launcher_process_identity: Some(APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY.to_string()),
        launcher_process_id: Some(APP_GAME_TEST_LAUNCHER_PROCESS_ID),
        launcher_process_name: Some(APP_GAME_TEST_LAUNCHER_PROCESS_NAME.to_string()),
        child_process_identity: None,
        child_inventory_entry_id: None,
        child_game_evidence_claim_id: None,
        catalog_ref: None,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_LAUNCHER_MANIFEST.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_NOT_LOADED.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        game_proof_state: APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY.to_string(),
        confidence: 0.74,
        evidence: vec![evidence_ref(APP_GAME_TEST_LAUNCHER_EVIDENCE_ID)],
    }
}

fn evidence_ref(evidence_id: &TestStr) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: None,
    }
}

fn inventory_label(product_kind: &TestStr) -> TestString {
    if product_kind == APP_GAME_PRODUCT_NATIVE_GAME {
        APP_GAME_TEST_GAME_DISPLAY_LABEL.to_string()
    } else {
        APP_GAME_TEST_DISPLAY_LABEL.to_string()
    }
}

fn inventory_state(product_kind: &TestStr) -> TestString {
    if product_kind == APP_GAME_PRODUCT_NATIVE_GAME {
        APP_GAME_INVENTORY_STATE_DETECTABLE.to_string()
    } else {
        APP_GAME_INVENTORY_STATE_INSTALLED.to_string()
    }
}

fn surface_request() -> ActivitySurfaceRequest {
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Family,
            family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
            device_id: None,
        },
        requested_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string(),
    }
}
