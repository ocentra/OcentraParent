use super::authenticated_delivery_grant::{
    issuer, subscribe_issuance_milestone_persistence, IssuanceFixture,
};
use super::TestResult;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::ids::{EventId, EventType};
use ocentra_eventing::journal::ndjson::NdjsonEventJournal;
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use sha2::{Digest, Sha256};

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
fn issuer_flushes_an_accepted_milestone_to_the_configured_durable_journal() -> TestResult {
    let journal_path = std::env::temp_dir().join(format!(
        "ocentra-policy-control-issuance-milestone-{}.ndjson",
        EventId::generated().as_str()
    ));
    let runtime = test_ok!(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        "durable journal runtime"
    );
    let grant = runtime.block_on(async {
        let event_type = test_ok!(
            EventType::parse("authenticated-delivery-grant.issuance.milestone"),
            "issuance milestone event type"
        );
        let event_bus = EventBus::with_journal(
            JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
            NdjsonEventJournal::new(&journal_path).shared(),
        );
        test_ok!(
            subscribe_issuance_milestone_persistence(&event_bus).await,
            "durable issuance milestone subscriber registers"
        );
        let issuer = test_ok!(issuer(), "provenance-configured issuer")
            .with_event_bus_issuance_publisher(event_bus)
            .map_err(|error| format!("event publisher: {error:?}"))?;
        let grant = test_ok!(
            issuer.issue_async(IssuanceFixture::new().request()).await,
            "accepted issuance must wait for durable milestone persistence"
        );
        assert_eq!(grant.target_device_id, "child-device-1");
        Ok::<_, Box<dyn std::error::Error>>(grant)
    })?;
    let mut fingerprint = Sha256::new();
    fingerprint.update(b"ocentra.authenticated-delivery-grant.audit-fingerprint.v1\0");
    fingerprint.update(grant.signing_bytes());
    fingerprint.update(&grant.signature);
    let expected_fingerprint = format!("{:x}", fingerprint.finalize());
    drop(runtime);

    let journal = std::fs::read_to_string(&journal_path)?;
    assert_eq!(
        journal.lines().count(),
        1,
        "one accepted issuance must durably write exactly one before-dispatch milestone"
    );
    assert!(
        journal.contains("authenticated-delivery-grant.issuance.milestone"),
        "the durable record must identify the issuance milestone contract"
    );
    assert!(
        journal.contains(&expected_fingerprint),
        "the durable milestone must bind exactly to the returned signed grant without storing raw bindings"
    );
    std::fs::remove_file(journal_path)?;
    Ok(())
}
