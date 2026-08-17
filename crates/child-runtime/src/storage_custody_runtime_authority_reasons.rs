use super::ChildStorageCustodyAuthorityError;

pub(crate) fn authority_error_reason(error: ChildStorageCustodyAuthorityError) -> &'static str {
    match error {
        ChildStorageCustodyAuthorityError::InvalidBinding => "current authority binding is invalid",
        ChildStorageCustodyAuthorityError::InvalidGeneration => {
            "current authority/session generation is invalid"
        }
        ChildStorageCustodyAuthorityError::StaleOrRevoked => {
            "current authority is stale or revoked; no custody effect was executed"
        }
        ChildStorageCustodyAuthorityError::EffectNotGranted => {
            "current authority does not grant this custody effect"
        }
    }
}
