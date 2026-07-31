use super::authenticated_delivery_grant::IssuanceFixture;
use super::authenticated_delivery_grant_fixture::{
    durable_milestone_bus, issuance_fixture_with_expiry, issuer,
    issuer_without_milestone_publisher, FailingMilestoneJournal, ForgedV3MilestoneJournal,
    LegacyV2MilestoneJournal,
};
use super::TestResult;
use ocentra_eventing::bus::subscriber::EventSubscriber;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::ids::{CorrelationId, EventId, EventType, SubscriberId, TargetHandler};
use ocentra_eventing::journal::ndjson::{
    JournalFlushPolicy, JournalHashChain, NdjsonEventJournal, NdjsonJournalOptions,
};
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use ocentra_eventing::testkit::EventRecorder;
use ocentra_family_identity_core::household_authority_proof::HouseholdAuthorityProofSigner;
use ocentra_family_identity_core::parent_step_up_proof::ParentStepUpProofSigner;
use ocentra_policy_control_core::authenticated_delivery_grant::authority::AuthenticatedDeliveryGrantAuthoritySigner;
use ocentra_policy_control_core::authenticated_delivery_grant::issuance_milestone::{
    AuthenticatedDeliveryGrantIssuanceMilestone, AuthenticatedDeliveryGrantIssuanceOutcome,
    AuthenticatedDeliveryGrantIssuanceRejection,
};
use ocentra_policy_control_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantIssuanceError, AuthenticatedDeliveryGrantIssuer,
};
use std::sync::Arc;

#[test]
fn issuer_requires_current_parent_authority_and_produces_verifiable_grant() -> TestResult {
    let issuer = test_ok!(issuer(), "valid test key id");
    let grant = test_ok!(
        issuer.issue(IssuanceFixture::new().request()),
        "current authority can issue"
    );
    let signature = test_ok!(
        ed25519_dalek::Signature::from_slice(&grant.signature),
        "signature bytes"
    );
    assert!(issuer
        .verifying_key()
        .verify_strict(&grant.signing_bytes(), &signature)
        .is_ok());
    Ok(())
}

#[test]
fn issuer_allows_unbound_action_device_when_signed_target_child_is_bound() -> TestResult {
    let issuer = test_ok!(issuer(), "provenance-configured issuer");
    let grant = test_ok!(
        issuer.issue(IssuanceFixture::new().request()),
        "unbound action device with separately bound target child can issue"
    );
    assert_eq!(grant.child_profile_id, "child-1");
    assert_eq!(grant.target_device_id, "child-device-1");
    Ok(())
}

#[test]
fn issuer_rejects_ndjson_proof_journal_before_issuance() -> TestResult {
    let journal_path = std::env::temp_dir().join(format!(
        "ocentra-policy-control-issuance-milestone-{}.ndjson",
        EventId::generated().as_str()
    ));
    let event_type = test_ok!(
        EventType::parse("authenticated-delivery-grant.issuance.milestone"),
        "event type"
    );
    let event_bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain())
            .shared(),
    );
    assert!(test_ok!(issuer_without_milestone_publisher(), "issuer")
        .with_event_bus_issuance_publisher(event_bus)
        .is_err());
    Ok(())
}

#[test]
fn issuer_rejects_buffered_ndjson_proof_journal_before_issuance() -> TestResult {
    let journal_path = std::env::temp_dir().join(format!(
        "ocentra-policy-control-buffered-issuance-{}.ndjson",
        EventId::generated().as_str()
    ));
    let event_type = test_ok!(
        EventType::parse("authenticated-delivery-grant.issuance.milestone"),
        "event type"
    );
    let event_bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        NdjsonEventJournal::with_options(
            &journal_path,
            NdjsonJournalOptions {
                hash_chain: JournalHashChain::Enabled,
                flush: JournalFlushPolicy::Buffered,
            },
        )
        .shared(),
    );
    assert!(test_ok!(issuer_without_milestone_publisher(), "issuer")
        .with_event_bus_issuance_publisher(event_bus)
        .is_err());
    Ok(())
}

#[test]
fn issuer_rejects_a_v3_synchronized_receipt_without_authenticated_completion() -> TestResult {
    let runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "forged V3 receipt runtime"
    );
    runtime.block_on(async {
        let event_type = test_ok!(
            EventType::parse("authenticated-delivery-grant.issuance.milestone"),
            "issuance milestone event type"
        );
        let journal: Arc<dyn ocentra_eventing::journal::EventJournal> =
            Arc::new(ForgedV3MilestoneJournal::default());
        let event_bus = EventBus::with_journal(
            JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
            journal,
        );
        let issuer = test_ok!(
            issuer_without_milestone_publisher(),
            "provenance-configured issuer"
        )
        .with_event_bus_issuance_publisher(event_bus)
        .map_err(|error| format!("event publisher: {error:?}"))?;
        assert_eq!(
            issuer.issue_async(IssuanceFixture::new().request()).await,
            Err(AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed),
            "a V3 receipt without its authenticated completion marker cannot authorize a grant"
        );
        Ok::<_, Box<dyn std::error::Error>>(())
    })
}

#[test]
fn issuer_rejects_a_legacy_v2_synchronized_receipt_without_completion_proof() -> TestResult {
    let runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "legacy V2 receipt runtime"
    );
    runtime.block_on(async {
        let event_type = test_ok!(
            EventType::parse("authenticated-delivery-grant.issuance.milestone"),
            "issuance milestone event type"
        );
        let journal: Arc<dyn ocentra_eventing::journal::EventJournal> =
            Arc::new(LegacyV2MilestoneJournal::default());
        let event_bus = EventBus::with_journal(
            JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
            journal,
        );
        let issuer = test_ok!(
            issuer_without_milestone_publisher(),
            "provenance-configured issuer"
        )
        .with_event_bus_issuance_publisher(event_bus)
        .map_err(|error| format!("event publisher: {error:?}"))?;
        assert_eq!(
            issuer.issue_async(IssuanceFixture::new().request()).await,
            Err(AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed),
            "a legacy V2 synchronized field without a verifiable completion proof cannot authorize a grant"
        );
        Ok::<_, Box<dyn std::error::Error>>(())
    })
}

#[test]
fn issuer_requires_a_durable_publisher_for_rejected_attempts_too() -> TestResult {
    let issuer = test_ok!(
        issuer_without_milestone_publisher(),
        "provenance-configured issuer without publisher"
    );
    let fixture = IssuanceFixture::new();
    let mut request = fixture.request();
    request.signed_authority_bindings.signature.clear();
    assert_eq!(
        issuer.issue(request),
        Err(AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed),
        "a rejected attempt without a durable record must fail closed"
    );
    Ok(())
}

#[test]
fn issuer_closes_a_prepared_attempt_when_accepted_terminal_append_fails() -> TestResult {
    let runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "terminal append failure runtime"
    );
    runtime.block_on(async {
        let event_type = test_ok!(
            EventType::parse("authenticated-delivery-grant.issuance.milestone"),
            "issuance milestone event type"
        );
        let journal = Arc::new(FailingMilestoneJournal::fail_once_on(2));
        let cloned_journal: Arc<FailingMilestoneJournal> = Arc::clone(&journal);
        let event_journal: Arc<dyn ocentra_eventing::journal::EventJournal> = cloned_journal;
        let event_bus = EventBus::with_journal(
            JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
            event_journal,
        );
        let issuer = test_ok!(issuer_without_milestone_publisher(), "issuer")
            .with_event_bus_issuance_publisher(event_bus)
            .map_err(|error| format!("event publisher: {error:?}"))?;

        assert_eq!(
            issuer.issue_async(IssuanceFixture::new().request()).await,
            Err(AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed),
            "an accepted terminal append failure must fail closed"
        );

        let persisted = test_ok!(journal.persisted(), "persisted terminal closure");
        assert_eq!(
            persisted.len(),
            2,
            "a prepared attempt must receive a durable rejected terminal closure after its accepted append fails"
        );
        let prepared = persisted[0].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
        let rejected = persisted[1].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
        assert_eq!(prepared.payload.outcome, AuthenticatedDeliveryGrantIssuanceOutcome::Prepared);
        assert_eq!(rejected.payload.outcome, AuthenticatedDeliveryGrantIssuanceOutcome::Rejected);
        assert_eq!(
            rejected.payload.rejection,
            Some(AuthenticatedDeliveryGrantIssuanceRejection::MilestonePublication),
            "publication failure must not be misattributed to authority provenance"
        );
        assert_eq!(
            test_ok!(issuance_attempt_id(&persisted[0]), "prepared attempt id"),
            test_ok!(issuance_attempt_id(&persisted[1]), "rejected attempt id"),
            "prepared and rejected terminal milestones must share one lifecycle attempt id"
        );
        Ok::<(), Box<dyn std::error::Error>>(())
    })
}

#[test]
fn issuer_rejects_an_oversized_issuer_key_id_at_construction() -> TestResult {
    let authority = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let household_authority = HouseholdAuthorityProofSigner::from_platform_key([6; 32]);
    let step_up = ParentStepUpProofSigner::from_platform_key([8; 32]);
    assert!(matches!(
        AuthenticatedDeliveryGrantIssuer::from_platform_key_with_provenance_verifiers(
            "k".repeat(513),
            [3; 32],
            authority.verifying_key(),
            household_authority.verifying_key(),
            step_up.verifying_key(),
        ),
        Err(AuthenticatedDeliveryGrantIssuanceError::InvalidIssuerKeyId)
    ));
    Ok(())
}

#[test]
fn issuer_records_prepare_then_rejection_when_durable_publish_exhausts_lifetime() -> TestResult {
    let journal_path = std::env::temp_dir().join(format!(
        "ocentra-policy-control-post-publish-lifetime-{}.ndjson",
        EventId::generated().as_str()
    ));
    let runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "post-publish lifetime runtime"
    );
    runtime.block_on(async {
        let event_bus = test_ok!(
            durable_milestone_bus(&journal_path),
            "durable milestone bus"
        );
        let issuer = test_ok!(issuer_without_milestone_publisher(), "issuer")
            .with_event_bus_issuance_publisher(event_bus.clone())
            .map_err(|error| format!("event publisher: {error:?}"))?;
        let fixture = test_ok!(
            issuance_fixture_with_expiry(IssuanceFixture::new(), "2026-07-28T00:01:20Z"),
            "post-publish lifetime fixture"
        );
        assert_eq!(
            issuer.issue_async(fixture.request()).await,
            Err(AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp),
            "a grant with less than the post-publication minimum lifetime must not return"
        );
        let journal = event_bus.journal().await;
        assert_eq!(
            journal.len(),
            2,
            "prepare and rejection must both be durable"
        );
        let prepare = journal[0].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
        let terminal = journal[1].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
        assert_eq!(
            prepare.payload.outcome,
            AuthenticatedDeliveryGrantIssuanceOutcome::Prepared
        );
        assert_eq!(
            terminal.payload.outcome,
            AuthenticatedDeliveryGrantIssuanceOutcome::Rejected
        );
        assert_eq!(
            terminal.payload.rejection,
            Some(AuthenticatedDeliveryGrantIssuanceRejection::Timestamp)
        );
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;
    Ok(())
}

#[test]
fn issuer_rechecks_lifetime_immediately_before_accepted_dispatch_and_closes_rejected() -> TestResult
{
    let runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "post-accepted lifetime runtime"
    );
    runtime.block_on(async {
        let event_type = test_ok!(
            EventType::parse("authenticated-delivery-grant.issuance.milestone"),
            "issuance milestone event type"
        );
        let journal = Arc::new(FailingMilestoneJournal::fail_once_on(u64::MAX));
        let cloned_journal: Arc<FailingMilestoneJournal> = Arc::clone(&journal);
        let event_journal: Arc<dyn ocentra_eventing::journal::EventJournal> = cloned_journal;
        let event_bus = EventBus::with_journal(
            JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
            event_journal,
        );
        let recorder = EventRecorder::<AuthenticatedDeliveryGrantIssuanceMilestone>::attach(
            &event_bus,
            EventSubscriber::new(
                test_ok!(SubscriberId::parse("policy-test-recorder"), "subscriber id"),
                test_ok!(
                    EventType::parse("authenticated-delivery-grant.issuance.milestone"),
                    "subscriber event type"
                ),
                test_ok!(
                    TargetHandler::parse("policy-test-recorder"),
                    "subscriber target"
                ),
            ),
        )
        .await?;
        let issuer = test_ok!(issuer_without_milestone_publisher(), "issuer")
            .with_trusted_issuance_now_sequence_for_debug_test([
                "2026-07-28T00:01:00Z",
                "2026-07-28T00:01:31Z",
            ])
            .with_event_bus_issuance_publisher(event_bus)
            .map_err(|error| format!("event publisher: {error:?}"))?;
        let fixture = test_ok!(
            issuance_fixture_with_expiry(IssuanceFixture::new(), "2026-07-28T00:01:30Z"),
            "post-accepted lifetime fixture"
        );

        assert_eq!(
            issuer.issue_async(fixture.request()).await,
            Err(AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp),
            "an accepted dispatch must not return a grant after its remaining lifetime has elapsed"
        );

        let dispatched = recorder.recorded().await;
        assert_eq!(
            dispatched
                .iter()
                .map(|envelope| envelope.payload.outcome)
                .collect::<Vec<_>>(),
            vec![
                AuthenticatedDeliveryGrantIssuanceOutcome::Prepared,
                AuthenticatedDeliveryGrantIssuanceOutcome::Rejected,
            ],
            "a slow publish boundary must not dispatch Accepted before the lifetime recheck"
        );
        let persisted = test_ok!(journal.persisted(), "persisted post-accepted closure");
        assert_eq!(persisted.len(), 2);
        let outcomes = persisted
            .iter()
            .map(|envelope| {
                envelope
                    .decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()
                    .map(|milestone| milestone.payload.outcome)
            })
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(
            outcomes,
            vec![
                AuthenticatedDeliveryGrantIssuanceOutcome::Prepared,
                AuthenticatedDeliveryGrantIssuanceOutcome::Rejected,
            ],
            "a publish-boundary expiry must leave only an explicit durable rejected closure"
        );
        let attempt_id = test_ok!(issuance_attempt_id(&persisted[0]), "prepared attempt id");
        for envelope in persisted.iter().skip(1) {
            assert_eq!(
                test_ok!(issuance_attempt_id(envelope), "terminal attempt id"),
                attempt_id,
                "every milestone in one lifecycle must retain the same attempt id"
            );
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    })
}

#[test]
fn issuer_derives_durable_correlation_from_verified_authority_not_caller_context() -> TestResult {
    let journal_path = std::env::temp_dir().join(format!(
        "ocentra-policy-control-derived-correlation-{}.ndjson",
        EventId::generated().as_str()
    ));
    let runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "derived correlation journal runtime"
    );
    runtime.block_on(async {
        let event_bus = test_ok!(
            durable_milestone_bus(&journal_path),
            "controlled durable bus"
        );
        let issuer = test_ok!(
            issuer_without_milestone_publisher(),
            "provenance-configured issuer"
        )
        .with_event_bus_issuance_publisher(event_bus.clone())
        .map_err(|error| format!("event publisher: {error:?}"))?;
        let fixture = IssuanceFixture::new();
        let mut request = fixture.request();
        let caller_correlation_id = test_ok!(
            CorrelationId::parse("c".repeat(513)),
            "oversized correlation remains syntactically valid"
        );
        request.correlation_id = caller_correlation_id.clone();
        let _grant = test_ok!(
            issuer.issue_async(request).await,
            "verified authority derives a bounded trusted audit correlation"
        );
        let journal = event_bus.journal().await;
        assert_eq!(
            journal.len(),
            2,
            "accepted issuance must leave durable prepare and terminal milestones"
        );
        assert_ne!(
            journal[0].correlation_id, caller_correlation_id,
            "untrusted caller context must not become the issuance audit chain"
        );
        let milestone = journal[1].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
        assert_eq!(
            milestone.payload.outcome,
            AuthenticatedDeliveryGrantIssuanceOutcome::Accepted
        );
        Ok::<_, Box<dyn std::error::Error>>(())
    })?;
    Ok(())
}

fn issuance_attempt_id(
    envelope: &ocentra_eventing::envelope::StoredEventEnvelope,
) -> Result<String, ocentra_eventing::error::EventingError> {
    let milestone = envelope.decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
    Ok(milestone.payload.attempt_id.as_str().to_owned())
}
