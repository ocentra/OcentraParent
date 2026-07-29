use super::TestResult;
use ocentra_eventing::bus::subscriber::EventSubscriber;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::envelope::StoredEventEnvelope;
use ocentra_eventing::ids::{EventType, SubscriberId, TargetHandler};
use ocentra_eventing::journal::ndjson::NdjsonEventJournal;
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use ocentra_eventing::journal::{EventJournal, JournalAppend, JournalAppendFuture};
use ocentra_policy_control_core::authenticated_delivery_grant::authority::AuthenticatedDeliveryGrantAuthoritySigner;
use ocentra_policy_control_core::authenticated_delivery_grant::issuance_milestone::AuthenticatedDeliveryGrantIssuanceMilestone;
use ocentra_policy_control_core::authenticated_delivery_grant::step_up::ParentStepUpProofSigner;
use ocentra_policy_control_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantIssuanceError, AuthenticatedDeliveryGrantIssuer,
};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Default)]
pub(super) struct InMemoryMilestoneJournal {
    next_sequence: AtomicU64,
}

impl EventJournal for InMemoryMilestoneJournal {
    fn append<'a>(&'a self, _envelope: &'a StoredEventEnvelope) -> JournalAppendFuture<'a> {
        Box::pin(async move {
            Ok(JournalAppend {
                sequence: self.next_sequence.fetch_add(1, Ordering::Relaxed) + 1,
                previous_hash: None,
                current_hash: None,
            })
        })
    }
}

pub(super) fn issuer_without_milestone_publisher(
) -> Result<AuthenticatedDeliveryGrantIssuer, AuthenticatedDeliveryGrantIssuanceError> {
    let authority = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let step_up = ParentStepUpProofSigner::from_platform_key([8; 32]);
    AuthenticatedDeliveryGrantIssuer::from_platform_key_with_provenance_verifiers(
        "parent-key-1",
        [3; 32],
        authority.verifying_key(),
        step_up.verifying_key(),
    )
    .map(|issuer| issuer.with_trusted_issuance_now_for_debug_test("2026-07-28T00:01:00Z"))
}

pub(super) fn issuer(
) -> Result<AuthenticatedDeliveryGrantIssuer, AuthenticatedDeliveryGrantIssuanceError> {
    let event_type = EventType::parse("authenticated-delivery-grant.issuance.milestone")
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)?;
    let event_bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        Arc::new(InMemoryMilestoneJournal::default()),
    );
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)?;
    runtime
        .block_on(subscribe_issuance_milestone_persistence(&event_bus))
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)?;
    issuer_without_milestone_publisher()
        .and_then(|issuer| {
            issuer
                .with_event_bus_issuance_publisher(event_bus)
                .map_err(|_error| {
                    AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed
                })
        })
        .map(|issuer| issuer.with_trusted_issuance_now_for_debug_test("2026-07-28T00:01:00Z"))
}

pub(super) async fn subscribe_issuance_milestone_persistence(
    event_bus: &EventBus,
) -> Result<(), ocentra_eventing::error::EventingError> {
    event_bus
        .subscribe::<AuthenticatedDeliveryGrantIssuanceMilestone, _, _>(
            EventSubscriber::new(
                SubscriberId::parse("policy-control.issuance-milestone-persistence")?,
                EventType::parse("authenticated-delivery-grant.issuance.milestone")?,
                TargetHandler::parse("policy-control.issuance-milestone-persistence")?,
            ),
            |_| async { Ok(()) },
        )
        .await
        .map(|_| ())
}

pub(super) fn durable_milestone_bus(
    journal_path: &std::path::Path,
) -> Result<EventBus, ocentra_eventing::error::EventingError> {
    let event_type = EventType::parse("authenticated-delivery-grant.issuance.milestone")?;
    Ok(EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        NdjsonEventJournal::new(journal_path).shared(),
    ))
}

pub(super) fn assert_durable_milestone_count(
    journal_path: &std::path::Path,
    expected_count: usize,
    description: &str,
) -> TestResult {
    let journal = std::fs::read_to_string(journal_path)?;
    assert_eq!(journal.lines().count(), expected_count, "{description}");
    std::fs::remove_file(journal_path)?;
    Ok(())
}
