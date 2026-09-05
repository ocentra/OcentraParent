use super::app_game_identity_validation::{ensure, is_blank, validate_evidence_refs};
use super::{
    AppGameIdentityMergeProof, APP_GAME_IDENTITY_DETERMINISTIC_REF_APPLICATION_TOKEN_REF,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_APP_USER_MODEL_ID,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_BUNDLE_ID, APP_GAME_IDENTITY_DETERMINISTIC_REF_CATALOG_REF,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_CHILD_GAME_EVIDENCE_CLAIM_ID,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_DESKTOP_ENTRY_ID,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_EXECUTABLE_PATH_REF,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_LAUNCHER_APP_ID,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_LAUNCHER_MANIFEST_ID,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_PACKAGE_ID,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_PUBLISHER_SIGNATURE_REF,
    APP_GAME_IDENTITY_DETERMINISTIC_REF_STORE_ID, APP_GAME_SCHEMA_VERSION,
};

pub(super) fn validate(merge: &AppGameIdentityMergeProof) -> Result<(), &'static str> {
    ensure(
        merge.schema_version == APP_GAME_SCHEMA_VERSION,
        "app game identity merge schema version is unsupported",
    )?;
    ensure(
        !is_blank(&merge.merge_id),
        "app game identity merge id must not be empty",
    )?;
    validate_source_identities(merge)?;
    ensure(
        merge.merge_confidence.is_finite() && (0.0..=1.0).contains(&merge.merge_confidence),
        "app game identity merge confidence must be between zero and one",
    )?;
    validate_merge_authority(merge)?;
    validate_evidence_refs(
        &merge.evidence,
        "app game identity merge must cite at least one evidence ref",
    )?;
    super::app_game_identity_validation::validate(&merge.target_identity)
}

fn validate_source_identities(merge: &AppGameIdentityMergeProof) -> Result<(), &'static str> {
    ensure(
        merge.source_identity_ids.len() >= 2,
        "app game identity merge must cite source identities",
    )?;
    ensure(
        merge.source_identity_ids.iter().all(|id| !is_blank(id)),
        "app game identity merge source id must not be empty",
    )?;
    merge
        .source_identity_ids
        .iter()
        .enumerate()
        .try_for_each(|(index, source_id)| {
            ensure(
                !merge.source_identity_ids[..index].contains(source_id),
                "app game identity merge source ids must be distinct",
            )?;
            ensure(
                source_id != &merge.target_identity.identity_id,
                "app game identity merge sources must not include the target",
            )
        })
}

fn validate_merge_authority(merge: &AppGameIdentityMergeProof) -> Result<(), &'static str> {
    ensure(
        !merge.conflicting_file_hash_refs,
        "conflicting file hashes must block app game identity merge",
    )?;
    ensure(
        merge.merge_confidence <= 0.3 || !merge.shared_deterministic_refs.is_empty(),
        "non-weak app game identity merge must share deterministic refs",
    )?;
    ensure(
        !merge.parent_label_changed
            || (merge.target_identity.parent_label.is_some()
                && !merge.shared_deterministic_refs.is_empty()),
        "parent labels must not create an app game identity merge",
    )?;
    ensure(
        merge
            .shared_deterministic_refs
            .iter()
            .all(|kind| !is_blank(kind) && deterministic_ref_kind_is_known(kind)),
        "app game identity merge contains an unsupported deterministic ref kind",
    )
}

fn deterministic_ref_kind_is_known(kind: &str) -> bool {
    [
        APP_GAME_IDENTITY_DETERMINISTIC_REF_PACKAGE_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_BUNDLE_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_APP_USER_MODEL_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_DESKTOP_ENTRY_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_APPLICATION_TOKEN_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_EXECUTABLE_PATH_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_PUBLISHER_SIGNATURE_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_LAUNCHER_APP_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_LAUNCHER_MANIFEST_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_STORE_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_CATALOG_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_CHILD_GAME_EVIDENCE_CLAIM_ID,
    ]
    .contains(&kind)
}
