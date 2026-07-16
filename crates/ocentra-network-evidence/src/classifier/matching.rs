use super::{
    NetworkActivityClassification, NetworkActivityClassifierInput, NetworkClassifierBasis,
};
use crate::{CategoryFreshnessState, NetworkCategory, NetworkEvidenceGrade};

pub(super) fn browser_confirmed_cdn(
    input: &NetworkActivityClassifierInput,
) -> Option<NetworkActivityClassification> {
    let hint = input.cdn_hint.as_ref()?;
    let confirmation = input.browser_confirmation.as_ref()?;
    if hint.category_hint != confirmation.category || !target_category(hint.category_hint) {
        return None;
    }

    Some(NetworkActivityClassification {
        category: hint.category_hint,
        basis: NetworkClassifierBasis::BrowserConfirmedCdn,
        confidence_percent: hint.confidence_percent.max(85),
        evidence_refs: vec![hint.source_ref.clone(), confirmation.source_ref.clone()],
        browser_confirmation_required: false,
        evidence_grade: NetworkEvidenceGrade::C,
        exact_url_available: false,
        decrypted_payload_available: false,
    })
}

pub(super) fn browser_confirmed_process(
    input: &NetworkActivityClassifierInput,
) -> Option<NetworkActivityClassification> {
    let hint = input.process_hint.as_ref()?;
    let confirmation = input.browser_confirmation.as_ref()?;
    if hint.category_hint != confirmation.category || !target_category(hint.category_hint) {
        return None;
    }

    Some(NetworkActivityClassification {
        category: hint.category_hint,
        basis: NetworkClassifierBasis::BrowserConfirmedProcess,
        confidence_percent: hint.confidence_percent.max(80),
        evidence_refs: vec![hint.source_ref.clone(), confirmation.source_ref.clone()],
        browser_confirmation_required: false,
        evidence_grade: NetworkEvidenceGrade::C,
        exact_url_available: false,
        decrypted_payload_available: false,
    })
}

pub(super) fn fresh_enough(freshness: CategoryFreshnessState) -> bool {
    matches!(freshness, CategoryFreshnessState::Fresh { .. })
}

pub(super) fn target_category(category: NetworkCategory) -> bool {
    matches!(
        category,
        NetworkCategory::Social
            | NetworkCategory::Video
            | NetworkCategory::Game
            | NetworkCategory::CloudGaming
    )
}
