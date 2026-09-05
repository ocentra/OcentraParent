#![cfg(test)]

use super::{DpapiKeySealingContext, DpapiKeySealingError};

fn valid_context() -> DpapiKeySealingContext {
    DpapiKeySealingContext {
        family_id: "family-secret".to_owned(),
        trust_subject: "subject-secret".to_owned(),
        device_ref: "device-secret".to_owned(),
        device_role: "parent".to_owned(),
        lifecycle_generation: 1,
        installation_binding_generation: 1,
    }
}

#[test]
fn context_rejects_empty_identity_or_zero_generation() {
    let mut empty_identity = valid_context();
    empty_identity.family_id.clear();
    assert_eq!(
        empty_identity.validate(),
        Err(DpapiKeySealingError::InvalidBinding)
    );

    let mut zero_generation = valid_context();
    zero_generation.lifecycle_generation = 0;
    assert_eq!(
        zero_generation.validate(),
        Err(DpapiKeySealingError::InvalidBinding)
    );
}

#[test]
fn context_debug_redacts_trust_identities() {
    let debug = format!("{:?}", valid_context());
    assert_eq!(
        debug,
        "DpapiKeySealingContext { family_id: \"[redacted]\", trust_subject: \"[redacted]\", device_ref: \"[redacted]\", device_role: \"[redacted]\", lifecycle_generation: 1, installation_binding_generation: 1 }"
    );
}
