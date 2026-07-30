use super::authenticated_delivery_grant_fixture::{issuer, InMemoryMilestoneJournal};
use super::TestResult;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::EventType;
use ocentra_eventing::journal::ndjson::NdjsonEventJournal;
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use ocentra_eventing::journal::EventJournal;

#[test]
fn issuer_refuses_before_and_after_dispatch_journaling_before_an_accepted_milestone_can_orphan(
) -> TestResult {
    let event_type = test_ok!(
        EventType::parse("authenticated-delivery-grant.issuance.milestone"),
        "issuance milestone event type"
    );
    let event_bus = EventBus::with_journal(
        JournalPolicy::before_and_after_dispatch(JournalSelector::EventTypes(vec![event_type])),
        std::sync::Arc::new(InMemoryMilestoneJournal::default())
            as std::sync::Arc<dyn EventJournal>,
    );

    assert_eq!(
        test_ok!(issuer(), "provenance-configured issuer")
            .with_event_bus_issuance_publisher(event_bus)
            .err(),
        Some(EventingError::InvalidHandlerPolicy {
            reason: "authenticated delivery grant issuance requires a before-dispatch-only journal policy so an accepted milestone cannot survive a failed after-dispatch phase".to_owned(),
        }),
        "issuance must reject a two-phase journal before it can record Accepted and then fail its after-dispatch append"
    );
    Ok(())
}

#[test]
fn issuer_rejects_a_before_dispatch_selector_that_omits_issuance_milestones() -> TestResult {
    let other_event = test_ok!(EventType::parse("policy.other"), "other event type");
    let event_bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![other_event])),
        std::sync::Arc::new(InMemoryMilestoneJournal::default())
            as std::sync::Arc<dyn EventJournal>,
    );
    assert_eq!(
        test_ok!(issuer(), "issuer").with_event_bus_issuance_publisher(event_bus).err(),
        Some(EventingError::InvalidHandlerPolicy { reason: "authenticated delivery grant issuance journal selector must cover issuance milestones".to_owned() })
    );
    Ok(())
}

#[test]
fn issuer_rejects_proof_only_ndjson_journal_even_when_before_dispatch() -> TestResult {
    let event_type = test_ok!(
        EventType::parse("authenticated-delivery-grant.issuance.milestone"),
        "issuance event type"
    );
    let path = std::env::temp_dir().join("ocentra-proof-only-issuance-journal.ndjson");
    let event_bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        NdjsonEventJournal::new(&path).shared(),
    );
    assert_eq!(
        test_ok!(issuer(), "issuer").with_event_bus_issuance_publisher(event_bus).err(),
        Some(EventingError::InvalidHandlerPolicy { reason: "authenticated delivery grant issuance requires a production-durable journal capability".to_owned() })
    );
    Ok(())
}
