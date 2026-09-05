use crate::activity::ActivityEvidenceRef;

use super::{
    AppGameIdentity, APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_KNOWN_APP,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
    APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_POSSIBLY_GAME, APP_GAME_CLASSIFICATION_STALE,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_CLASSIFICATION_UNSUPPORTED_PLATFORM,
    APP_GAME_IDENTITY_CONFIDENCE_AI_ASSISTED, APP_GAME_IDENTITY_CONFIDENCE_CANDIDATE,
    APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC, APP_GAME_IDENTITY_CONFIDENCE_PARENT_LABELED,
    APP_GAME_IDENTITY_CONFIDENCE_WEAK, APP_GAME_PRODUCT_LAUNCHER, APP_GAME_PRODUCT_NATIVE_APP,
    APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE, APP_GAME_SCHEMA_VERSION,
};

pub(super) fn validate(identity: &AppGameIdentity) -> Result<(), &'static str> {
    validate_labels_and_states(identity)?;
    validate_optional_references(identity)?;
    validate_evidence_refs(
        &identity.evidence,
        "app game identity must cite at least one evidence ref",
    )?;
    validate_reference_strength(identity)
}

fn validate_labels_and_states(identity: &AppGameIdentity) -> Result<(), &'static str> {
    ensure(
        identity.schema_version == APP_GAME_SCHEMA_VERSION,
        "app game identity schema version is unsupported",
    )?;
    ensure(
        !is_blank(&identity.identity_id),
        "app game identity id must not be empty",
    )?;
    ensure(
        !is_blank(&identity.display_label),
        "app game identity display label must not be empty",
    )?;
    ensure(
        [
            APP_GAME_PRODUCT_NATIVE_APP,
            APP_GAME_PRODUCT_NATIVE_GAME,
            APP_GAME_PRODUCT_LAUNCHER,
            APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE,
        ]
        .contains(&identity.product_kind.as_str()),
        "app game identity product kind is unsupported",
    )?;
    ensure(
        [
            APP_GAME_IDENTITY_CONFIDENCE_WEAK,
            APP_GAME_IDENTITY_CONFIDENCE_CANDIDATE,
            APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC,
            APP_GAME_IDENTITY_CONFIDENCE_PARENT_LABELED,
            APP_GAME_IDENTITY_CONFIDENCE_AI_ASSISTED,
        ]
        .contains(&identity.confidence.as_str()),
        "app game identity confidence is unsupported",
    )?;
    ensure(
        valid_classification_state(&identity.classification_state),
        "app game identity classification state is unsupported",
    )
}

fn valid_classification_state(state: &str) -> bool {
    [
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
        APP_GAME_CLASSIFICATION_KNOWN_APP,
        APP_GAME_CLASSIFICATION_KNOWN_GAME,
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
        APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
        APP_GAME_CLASSIFICATION_UNSUPPORTED_PLATFORM,
        APP_GAME_CLASSIFICATION_STALE,
        APP_GAME_CLASSIFICATION_ADAPTER_ERROR,
    ]
    .contains(&state)
}

fn validate_optional_references(identity: &AppGameIdentity) -> Result<(), &'static str> {
    [
        ("parentLabel", identity.parent_label.as_deref()),
        ("packageId", identity.package_id.as_deref()),
        ("bundleId", identity.bundle_id.as_deref()),
        ("appUserModelId", identity.app_user_model_id.as_deref()),
        ("desktopEntryId", identity.desktop_entry_id.as_deref()),
        (
            "applicationTokenRef",
            identity.application_token_ref.as_deref(),
        ),
        ("executablePathRef", identity.executable_path_ref.as_deref()),
        (
            "publisherSignatureRef",
            identity.publisher_signature_ref.as_deref(),
        ),
        ("fileHashRef", identity.file_hash_ref.as_deref()),
        ("launcherRef", identity.launcher_ref.as_deref()),
        ("launcherAppId", identity.launcher_app_id.as_deref()),
        (
            "launcherManifestId",
            identity.launcher_manifest_id.as_deref(),
        ),
        ("storeId", identity.store_id.as_deref()),
        ("catalogRef", identity.catalog_ref.as_deref()),
        (
            "childGameEvidenceClaimId",
            identity.child_game_evidence_claim_id.as_deref(),
        ),
    ]
    .into_iter()
    .try_for_each(|(field, value)| {
        let error = if field == "parentLabel" {
            "app game identity parent label must not be empty"
        } else {
            "app game identity reference must not be empty"
        };
        ensure(value.is_none_or(|value| !is_blank(value)), error)
    })
}

fn validate_reference_strength(identity: &AppGameIdentity) -> Result<(), &'static str> {
    let has_raw_reference = has_raw_reference(identity);
    ensure(
        has_raw_reference
            || (identity.confidence == APP_GAME_IDENTITY_CONFIDENCE_WEAK
                && identity.classification_state == APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
                && identity.product_kind == APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE),
        "display-name-only app game identity must remain weak and unknown",
    )?;
    let deterministic_confidence = identity.confidence
        == APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC
        || identity.confidence == APP_GAME_IDENTITY_CONFIDENCE_PARENT_LABELED;
    ensure(
        !deterministic_confidence
            || has_deterministic_reference(identity)
            || (identity.product_kind == APP_GAME_PRODUCT_LAUNCHER && has_raw_reference),
        "deterministic app game identity must include an identity reference",
    )?;
    ensure(
        !has_only_launcher_references(identity)
            || (identity.product_kind == APP_GAME_PRODUCT_LAUNCHER
                && identity.classification_state != APP_GAME_CLASSIFICATION_KNOWN_GAME),
        "launcher-only app game identity cannot claim a known game",
    )
}

pub(super) fn validate_evidence_refs(
    evidence: &[ActivityEvidenceRef],
    empty_message: &'static str,
) -> Result<(), &'static str> {
    ensure(!evidence.is_empty(), empty_message)?;
    ensure(
        evidence
            .iter()
            .all(|reference| !is_blank(&reference.evidence_id)),
        "app game evidence ref id must not be empty",
    )?;
    ensure(
        evidence.iter().all(|reference| {
            reference
                .digest
                .as_deref()
                .is_none_or(|value| !is_blank(value))
                && reference
                    .uri
                    .as_deref()
                    .is_none_or(|value| !is_blank(value))
        }),
        "app game evidence ref values must not be empty",
    )
}

fn has_raw_reference(identity: &AppGameIdentity) -> bool {
    has_deterministic_reference(identity)
        || identity.launcher_ref.is_some()
        || identity.launcher_app_id.is_some()
        || identity.launcher_manifest_id.is_some()
}

fn has_deterministic_reference(identity: &AppGameIdentity) -> bool {
    identity.package_id.is_some()
        || identity.bundle_id.is_some()
        || identity.app_user_model_id.is_some()
        || identity.desktop_entry_id.is_some()
        || identity.application_token_ref.is_some()
        || identity.executable_path_ref.is_some()
        || identity.publisher_signature_ref.is_some()
        || identity.file_hash_ref.is_some()
        || identity.store_id.is_some()
        || identity.catalog_ref.is_some()
        || identity.child_game_evidence_claim_id.is_some()
}

fn has_only_launcher_references(identity: &AppGameIdentity) -> bool {
    !has_deterministic_reference(identity)
        && (identity.launcher_ref.is_some()
            || identity.launcher_app_id.is_some()
            || identity.launcher_manifest_id.is_some())
}

pub(super) fn is_blank(value: &str) -> bool {
    value.trim().is_empty()
}

pub(super) fn ensure(condition: bool, error: &'static str) -> Result<(), &'static str> {
    condition.then_some(()).ok_or(error)
}
