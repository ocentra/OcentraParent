use super::policy_delivery_helpers as helpers;
use super::TestResult;
use helpers::{
    attempt, audit_ref, sample_delivery_target, sample_policy_rollback_ref,
    sample_policy_source_document,
};
use ocentra_policy_control_core::policy_delivery::{queue_policy_delivery, PolicyDeliveryId};
use ocentra_policy_control_core::policy_source::{
    compile_domain_policy_artifact, rollback_parent_policy_source_document,
    supersede_parent_policy_source_document, PolicyConsumerDomain, PolicyVersion,
};

#[test]
fn queued_delivery_preserves_superseded_source_lifecycle_metadata() -> TestResult {
    let superseded_source = test_ok!(
        supersede_parent_policy_source_document(
            &sample_policy_source_document()?,
            test_ok!(PolicyVersion::new(4), "policy version"),
            audit_ref("audit-policy-superseded")?,
        ),
        "superseded policy source document"
    );
    let superseded_compiled = test_ok!(
        compile_domain_policy_artifact(&superseded_source, PolicyConsumerDomain::Tracking),
        "compiled superseded artifact"
    );
    let superseded_delivery = test_ok!(
        queue_policy_delivery(
            &superseded_compiled,
            sample_delivery_target()?,
            test_ok!(
                PolicyDeliveryId::parse("delivery-policy-superseded"),
                "policy delivery id"
            ),
            attempt("attempt-superseded-queued")?,
            vec![audit_ref("audit-superseded-queued")?],
        ),
        "queued superseded delivery"
    );

    assert_eq!(
        superseded_delivery.source_audit_reference_ids,
        superseded_source.audit_reference_ids
    );
    assert_eq!(
        test_some!(
            superseded_delivery
                .source_superseded_by_policy_version
                .as_ref(),
            "replacement policy version"
        )
        .value(),
        4
    );
    assert!(superseded_delivery.source_rollback_ref.is_none());
    assert_eq!(
        superseded_delivery.audit_reference_ids,
        vec![audit_ref("audit-superseded-queued")?]
    );
    assert!(superseded_delivery.superseded_by_policy_version.is_none());
    assert!(superseded_delivery.rollback_reference_state.is_none());
    Ok(())
}

#[test]
fn queued_delivery_preserves_rolled_back_source_lifecycle_metadata() -> TestResult {
    let rollback_ref = sample_policy_rollback_ref()?;
    let rolled_back_source = test_ok!(
        rollback_parent_policy_source_document(
            &sample_policy_source_document()?,
            &rollback_ref,
            audit_ref("audit-policy-rolled-back")?,
        ),
        "rolled-back policy source document"
    );
    let rolled_back_compiled = test_ok!(
        compile_domain_policy_artifact(&rolled_back_source, PolicyConsumerDomain::Tracking),
        "compiled rolled-back artifact"
    );
    let rolled_back_delivery = test_ok!(
        queue_policy_delivery(
            &rolled_back_compiled,
            sample_delivery_target()?,
            test_ok!(
                PolicyDeliveryId::parse("delivery-policy-rolled-back"),
                "policy delivery id"
            ),
            attempt("attempt-rolled-back-queued")?,
            vec![audit_ref("audit-rolled-back-queued")?],
        ),
        "queued rolled-back delivery"
    );

    assert_eq!(
        rolled_back_delivery.source_audit_reference_ids,
        rolled_back_source.audit_reference_ids
    );
    assert!(rolled_back_delivery
        .source_superseded_by_policy_version
        .is_none());
    assert_eq!(
        test_some!(
            rolled_back_delivery.source_rollback_ref.as_ref(),
            "source rollback ref"
        )
        .restored_policy_version
        .value(),
        2
    );
    assert_eq!(
        rolled_back_delivery.audit_reference_ids,
        vec![audit_ref("audit-rolled-back-queued")?]
    );
    assert!(rolled_back_delivery.superseded_by_policy_version.is_none());
    assert!(rolled_back_delivery.rollback_reference_state.is_none());
    Ok(())
}
