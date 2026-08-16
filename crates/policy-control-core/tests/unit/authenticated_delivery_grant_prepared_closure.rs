use super::authenticated_delivery_grant::IssuanceFixture;
use super::authenticated_delivery_grant_fixture::issuer;
use super::TestResult;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::ids::{EventId, EventType};
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use ocentra_eventing::journal::production_file::ProductionFileEventJournal;
use ocentra_policy_control_core::authenticated_delivery_grant::issuance_milestone::{
    AuthenticatedDeliveryGrantIssuanceMilestone, AuthenticatedDeliveryGrantIssuanceOutcome,
    AuthenticatedDeliveryGrantIssuanceRejection,
};
use ocentra_policy_control_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantIssuanceError;

#[test]
fn async_prepared_publication_failure_is_closed_by_a_durable_rejected_milestone() -> TestResult {
    let runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "event runtime"
    );
    let journal_path = std::env::temp_dir().join(format!(
        "ocentra-policy-control-prepared-failure-{}.ndjson",
        EventId::generated().as_str()
    ));
    let event_type = test_ok!(
        EventType::parse("authenticated-delivery-grant.issuance.milestone"),
        "issuance milestone event type"
    );
    let journal = ProductionFileEventJournal::new(journal_path);
    let event_bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        journal.clone().shared(),
    );
    let issuer = test_ok!(issuer(), "provenance-configured issuer")
        .with_event_bus_issuance_publisher(event_bus.clone())
        .map_err(|error| format!("event publisher: {error:?}"))?;
    journal.inject_next_sync_failure_for_debug();

    runtime.block_on(async {
        assert_eq!(
            issuer.issue_async(IssuanceFixture::new().request()).await,
            Err(AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed),
            "a failed Prepared append must keep the issuance failed even when its terminal closure succeeds"
        );
        let milestones = event_bus.journal().await;
        assert_eq!(
            milestones.len(),
            2,
            "the recovered journal must retain the failed Prepared record and its terminal closure"
        );
        let prepared = milestones[0].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
        assert_eq!(
            prepared.payload.outcome,
            AuthenticatedDeliveryGrantIssuanceOutcome::Prepared,
            "the transient durable-sync failure must occur after the Prepared record is written"
        );
        let terminal = milestones[1].decode::<AuthenticatedDeliveryGrantIssuanceMilestone>()?;
        assert_eq!(
            terminal.payload.outcome,
            AuthenticatedDeliveryGrantIssuanceOutcome::Rejected,
            "a failed Prepared append must close the same issuance attempt as Rejected"
        );
        assert_eq!(
            terminal.payload.attempt_id, prepared.payload.attempt_id,
            "the durable Rejected closure must retain the Prepared attempt identity"
        );
        assert_eq!(
            terminal.payload.rejection,
            Some(AuthenticatedDeliveryGrantIssuanceRejection::MilestonePublication),
            "the terminal closure must retain the failed publication reason"
        );
        assert!(terminal.payload.redaction_state);
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;
    Ok(())
}
