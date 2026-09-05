use crate::activity::ActivityEvidenceRef;

use super::app_game_identity_validation::{ensure, is_blank};
use super::{
    AppGameEvidenceClaim, APP_GAME_CATALOG_NOT_LOADED, APP_GAME_CATALOG_PERMISSION_LIMITED,
    APP_GAME_CATALOG_READY, APP_GAME_CATALOG_STALE, APP_GAME_CATALOG_UNAVAILABLE,
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_KNOWN_APP,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
    APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_POSSIBLY_GAME, APP_GAME_CLASSIFICATION_STALE,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_CLASSIFICATION_UNSUPPORTED_PLATFORM,
    APP_GAME_EVIDENCE_CLAIM_KIND_AI_DIGEST, APP_GAME_EVIDENCE_CLAIM_KIND_CATALOG,
    APP_GAME_EVIDENCE_CLAIM_KIND_FOREGROUND, APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY,
    APP_GAME_EVIDENCE_CLAIM_KIND_LAUNCHER, APP_GAME_EVIDENCE_CLAIM_KIND_RUNTIME,
    APP_GAME_EVIDENCE_CLAIM_KIND_SESSION, APP_GAME_FOREGROUND_ADAPTER_ERROR,
    APP_GAME_FOREGROUND_BACKGROUND, APP_GAME_FOREGROUND_DEGRADED, APP_GAME_FOREGROUND_FOREGROUND,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_FOREGROUND_PERMISSION_LIMITED,
    APP_GAME_FOREGROUND_UNKNOWN, APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED,
    APP_GAME_IDENTITY_STRENGTH_CHILD_GAME_PROOF, APP_GAME_IDENTITY_STRENGTH_DISPLAY_NAME_ONLY,
    APP_GAME_IDENTITY_STRENGTH_LAUNCHER_CLAIMED, APP_GAME_IDENTITY_STRENGTH_OBSERVED_PROCESS,
    APP_GAME_IDENTITY_STRENGTH_PLATFORM_MANAGED, APP_GAME_IDENTITY_STRENGTH_WEAK,
    APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW, APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN,
    APP_GAME_OBSERVATION_MODE_LAUNCHER_MANIFEST, APP_GAME_OBSERVATION_MODE_PROCESS_EXIT,
    APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT, APP_GAME_OBSERVATION_MODE_PROCESS_START,
    APP_GAME_RUNTIME_ADAPTER_ERROR, APP_GAME_RUNTIME_DEGRADED, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_RUNTIME_NOT_RUNNING, APP_GAME_RUNTIME_PERMISSION_LIMITED, APP_GAME_RUNTIME_RUNNING,
    APP_GAME_RUNTIME_STALE, APP_GAME_RUNTIME_UNAVAILABLE, APP_GAME_RUNTIME_UNKNOWN,
    APP_GAME_SCHEMA_VERSION,
};

pub(super) fn validate(claim: &AppGameEvidenceClaim) -> Result<(), &'static str> {
    validate_identity_fields(claim)?;
    validate_observation_states(claim)?;
    validate_optional_references(claim)?;
    ensure(
        claim.confidence.is_finite() && (0.0..=1.0).contains(&claim.confidence),
        "app game evidence claim confidence",
    )?;
    validate_evidence_ref_values(&claim.evidence)?;
    validate_claim_boundaries(claim)
}

fn validate_identity_fields(claim: &AppGameEvidenceClaim) -> Result<(), &'static str> {
    ensure(
        claim.schema_version == APP_GAME_SCHEMA_VERSION,
        "app game evidence claim schema version is unsupported",
    )?;
    ensure(
        !is_blank(&claim.claim_id),
        "app game evidence claim id must not be empty",
    )?;
    ensure(
        !is_blank(&claim.observed_at),
        "app game evidence claim observed-at must not be empty",
    )?;
    ensure(
        !is_blank(&claim.display_name),
        "app game evidence claim display name must not be empty",
    )?;
    ensure(
        [
            APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY,
            APP_GAME_EVIDENCE_CLAIM_KIND_RUNTIME,
            APP_GAME_EVIDENCE_CLAIM_KIND_FOREGROUND,
            APP_GAME_EVIDENCE_CLAIM_KIND_LAUNCHER,
            APP_GAME_EVIDENCE_CLAIM_KIND_SESSION,
            APP_GAME_EVIDENCE_CLAIM_KIND_CATALOG,
            APP_GAME_EVIDENCE_CLAIM_KIND_AI_DIGEST,
        ]
        .contains(&claim.claim_kind.as_str()),
        "app game evidence claim kind is unsupported",
    )?;
    ensure(
        [
            APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW,
            APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
            APP_GAME_OBSERVATION_MODE_PROCESS_START,
            APP_GAME_OBSERVATION_MODE_PROCESS_EXIT,
            APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN,
            APP_GAME_OBSERVATION_MODE_LAUNCHER_MANIFEST,
        ]
        .contains(&claim.observation_mode.as_str()),
        "app game evidence claim observation mode is unsupported",
    )
}

fn validate_observation_states(claim: &AppGameEvidenceClaim) -> Result<(), &'static str> {
    ensure(
        [
            APP_GAME_IDENTITY_STRENGTH_DISPLAY_NAME_ONLY,
            APP_GAME_IDENTITY_STRENGTH_WEAK,
            APP_GAME_IDENTITY_STRENGTH_OBSERVED_PROCESS,
            APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED,
            APP_GAME_IDENTITY_STRENGTH_LAUNCHER_CLAIMED,
            APP_GAME_IDENTITY_STRENGTH_PLATFORM_MANAGED,
            APP_GAME_IDENTITY_STRENGTH_CHILD_GAME_PROOF,
        ]
        .contains(&claim.identity_strength.as_str()),
        "app game evidence claim identity strength is unsupported",
    )?;
    ensure(
        valid_classification_state(&claim.classification_state),
        "app game evidence claim classification state is unsupported",
    )?;
    ensure(
        [
            APP_GAME_CATALOG_READY,
            APP_GAME_CATALOG_UNAVAILABLE,
            APP_GAME_CATALOG_NOT_LOADED,
            APP_GAME_CATALOG_STALE,
            APP_GAME_CATALOG_PERMISSION_LIMITED,
        ]
        .contains(&claim.catalog_ready_state.as_str()),
        "app game evidence claim catalog state is unsupported",
    )?;
    ensure(
        valid_runtime_state(&claim.runtime_state),
        "app game evidence claim runtime state is unsupported",
    )?;
    ensure(
        valid_foreground_state(&claim.foreground_state),
        "app game evidence claim foreground state is unsupported",
    )
}

fn valid_classification_state(state: &str) -> bool {
    [
        APP_GAME_CLASSIFICATION_KNOWN_APP,
        APP_GAME_CLASSIFICATION_KNOWN_GAME,
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
        APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME,
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
        APP_GAME_CLASSIFICATION_UNSUPPORTED_PLATFORM,
        APP_GAME_CLASSIFICATION_STALE,
        APP_GAME_CLASSIFICATION_ADAPTER_ERROR,
    ]
    .contains(&state)
}

fn valid_runtime_state(state: &str) -> bool {
    [
        APP_GAME_RUNTIME_RUNNING,
        APP_GAME_RUNTIME_NOT_RUNNING,
        APP_GAME_RUNTIME_NOT_CLAIMED,
        APP_GAME_RUNTIME_UNKNOWN,
        APP_GAME_RUNTIME_PERMISSION_LIMITED,
        APP_GAME_RUNTIME_UNAVAILABLE,
        APP_GAME_RUNTIME_DEGRADED,
        APP_GAME_RUNTIME_STALE,
        APP_GAME_RUNTIME_ADAPTER_ERROR,
    ]
    .contains(&state)
}

fn valid_foreground_state(state: &str) -> bool {
    [
        APP_GAME_FOREGROUND_FOREGROUND,
        APP_GAME_FOREGROUND_BACKGROUND,
        APP_GAME_FOREGROUND_UNKNOWN,
        APP_GAME_FOREGROUND_PERMISSION_LIMITED,
        APP_GAME_FOREGROUND_DEGRADED,
        APP_GAME_FOREGROUND_ADAPTER_ERROR,
        APP_GAME_FOREGROUND_NOT_CLAIMED,
    ]
    .contains(&state)
}

fn validate_optional_references(claim: &AppGameEvidenceClaim) -> Result<(), &'static str> {
    [
        claim.inventory_entry_id.as_deref(),
        claim.process_identity.as_deref(),
        claim.launcher_ref.as_deref(),
        claim.catalog_ref.as_deref(),
    ]
    .into_iter()
    .try_for_each(|value| {
        ensure(
            value.is_none_or(|value| !is_blank(value)),
            "app game optional reference must not be empty",
        )
    })
}

fn validate_evidence_ref_values(evidence: &[ActivityEvidenceRef]) -> Result<(), &'static str> {
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

fn validate_claim_boundaries(claim: &AppGameEvidenceClaim) -> Result<(), &'static str> {
    ensure(
        claim.identity_strength != APP_GAME_IDENTITY_STRENGTH_DISPLAY_NAME_ONLY
            || (claim.confidence <= 0.3
                && claim.inventory_entry_id.is_none()
                && claim.process_identity.is_none()
                && claim.launcher_ref.is_none()
                && claim.catalog_ref.is_none()),
        "display-name-only app game evidence must remain weak and unlinked",
    )?;
    ensure(
        claim.claim_kind != APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY
            || (claim.runtime_state == APP_GAME_RUNTIME_NOT_CLAIMED
                && claim.foreground_state == APP_GAME_FOREGROUND_NOT_CLAIMED),
        "inventory evidence must not claim runtime or foreground use",
    )?;
    ensure(
        claim.claim_kind != APP_GAME_EVIDENCE_CLAIM_KIND_LAUNCHER
            || claim.classification_state != APP_GAME_CLASSIFICATION_KNOWN_GAME
            || claim.identity_strength == APP_GAME_IDENTITY_STRENGTH_CHILD_GAME_PROOF,
        "launcher evidence must cite child-game proof before known-game classification",
    )
}
