#[path = "policy_preview_helpers.rs"]
mod helpers;

use super::TestResult;
use helpers::sample_preview_request;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewFindingKind;
use ocentra_policy_control_core::policy_authority::PolicyManualReviewState;
use ocentra_policy_control_core::policy_preview::{
    preview_parent_policy_before_save, PolicyPreviewRequest, PolicyPreviewSaveState,
};
use ocentra_policy_control_core::policy_source::PolicyVersion;
#[test]
fn policy_preview_request_serde_rejects_zero_schema_version() -> TestResult {
    let mut value = test_ok!(
        serde_json::to_value(sample_preview_request(4, Some(4), false)?),
        "preview request"
    );
    value["schema_version"] = serde_json::json!(0);

    let error = test_err!(
        serde_json::from_value::<PolicyPreviewRequest>(value),
        "policy preview schema version zero must be rejected"
    );

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
    Ok(())
}

#[test]
fn stale_current_document_version_is_visible_and_blocks_save() -> TestResult {
    let request = sample_preview_request(4, Some(5), true)?;
    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview stale-source result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    assert_eq!(result.findings.len(), 1);
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::StaleSourceDocument
    );
    assert_eq!(result.findings[0].rule_ids, Vec::new());
    assert_eq!(result.findings[0].schedule_ids, Vec::new());
    assert_eq!(
        result.findings[0].explanation_code.as_str(),
        "stale-policy-version"
    );
    Ok(())
}

#[test]
fn matching_current_document_version_stays_ready_to_save() -> TestResult {
    let request = sample_preview_request(4, Some(4), true)?;
    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview matching-source result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::ReadyToSave);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
    assert!(result.findings.is_empty());
    assert_eq!(
        result.policy_version,
        test_ok!(PolicyVersion::new(4), "policy version")
    );
    Ok(())
}
