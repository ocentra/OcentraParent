use ocentra_eventing::envelope::StoredEventEnvelope;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{EventId, IdempotencyKey, JournalHash};
use ocentra_eventing::journal::ndjson::{
    NdjsonEventJournal, NdjsonJournalEntry, NdjsonJournalOptions,
};
use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_eventing::journal::{
    EventJournal, JournalAppend, JournalAppendDurability, JournalHashVersion,
};
use ocentra_eventing::replay::ReplayFilter;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    super::fixtures::{test_event, TestText},
    support::{cleanup, journal_path, read_lines, stored_event},
};

#[tokio::test]
async fn hash_chain_replays_and_appends_a_legacy_v1_ndjson_entry_into_v3() {
    let path = journal_path(TestText("legacy-v1-hash-chain".to_owned()));
    let legacy_envelope = unique_stored_event("legacy journal event", 58);
    let legacy_hash = legacy_hash_entry(
        1,
        None,
        &legacy_envelope,
        JournalDispatchPhase::AfterDispatch,
    );
    let legacy_entry = NdjsonJournalEntry {
        append: JournalAppend {
            sequence: 1,
            previous_hash: None,
            current_hash: Some(legacy_hash.clone()),
            hash_version: JournalHashVersion::LegacyV1,
            durability: JournalAppendDurability::Buffered,
            requested_durability: JournalAppendDurability::Buffered,
            synchronization_hash: None,
        },
        phase: JournalDispatchPhase::AfterDispatch,
        envelope: legacy_envelope.clone(),
    };
    let mut encoded = serde_json::to_value(legacy_entry).expect_value("legacy entry encodes");
    encoded["append"]
        .as_object_mut()
        .expect_value("append remains an object")
        .remove("hash_version");
    tokio::fs::write(
        &path.0,
        format!(
            "{}\n",
            serde_json::to_string(&encoded).expect_value("legacy wire entry encodes")
        ),
    )
    .await
    .expect_value("legacy journal writes");

    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let replay = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("legacy journal replays");
    let append = journal
        .append(&unique_stored_event("v3 journal event", 59))
        .await
        .expect_value("v3 append follows legacy journal");
    let lines = read_lines(path.clone()).await;
    let current_entry: NdjsonJournalEntry =
        serde_json::from_str(&lines[1]).expect_value("current line decodes");

    assert_eq!(replay.records.len(), 1);
    assert_eq!(replay.records[0].envelope, legacy_envelope);
    assert_eq!(append.previous_hash, Some(legacy_hash));
    assert_eq!(current_entry.append.hash_version, JournalHashVersion::V3);
    assert_eq!(current_entry.append.current_hash, append.current_hash);
    cleanup(path).await;
}

#[tokio::test]
async fn hash_chain_replays_and_appends_a_legacy_v2_ndjson_entry_into_v3() {
    let path = journal_path(TestText("legacy-v2-hash-chain".to_owned()));
    let legacy_envelope = unique_stored_event("legacy v2 journal event", 60);
    let legacy_hash = v2_hash_entry(
        1,
        None,
        &legacy_envelope,
        JournalDispatchPhase::AfterDispatch,
        JournalAppendDurability::Synchronized,
    );
    let legacy_entry = NdjsonJournalEntry {
        append: JournalAppend {
            sequence: 1,
            previous_hash: None,
            current_hash: Some(legacy_hash.clone()),
            hash_version: JournalHashVersion::V2,
            durability: JournalAppendDurability::Synchronized,
            requested_durability: JournalAppendDurability::Buffered,
            synchronization_hash: None,
        },
        phase: JournalDispatchPhase::AfterDispatch,
        envelope: legacy_envelope.clone(),
    };
    let mut encoded = serde_json::to_value(legacy_entry).expect_value("legacy v2 entry encodes");
    encoded["append"]
        .as_object_mut()
        .expect_value("legacy v2 append remains an object")
        .remove("requested_durability");
    tokio::fs::write(
        &path.0,
        format!(
            "{}\n",
            serde_json::to_string(&encoded).expect_value("legacy v2 wire entry encodes")
        ),
    )
    .await
    .expect_value("legacy v2 journal writes");

    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let replay = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("legacy v2 journal replays");
    let append = journal
        .append(&unique_stored_event("v3 journal event after v2", 61))
        .await
        .expect_value("v3 append follows legacy v2 journal");
    let lines = read_lines(path.clone()).await;
    let current_entry: NdjsonJournalEntry =
        serde_json::from_str(&lines[1]).expect_value("current v3 line decodes");

    assert_eq!(replay.records.len(), 1);
    assert_eq!(replay.records[0].envelope, legacy_envelope);
    assert_eq!(append.previous_hash, Some(legacy_hash));
    assert_eq!(current_entry.append.hash_version, JournalHashVersion::V3);
    assert_eq!(current_entry.append.current_hash, append.current_hash);
    cleanup(path).await;
}

#[derive(Serialize)]
struct LegacyJournalHashInput<'a> {
    sequence: u64,
    previous_hash: Option<&'a JournalHash>,
    phase: JournalDispatchPhase,
    envelope: &'a StoredEventEnvelope,
}

#[derive(Serialize)]
struct V2JournalHashInput<'a> {
    version: u8,
    sequence: u64,
    previous_hash: Option<&'a JournalHash>,
    phase: JournalDispatchPhase,
    durability: JournalAppendDurability,
    envelope: &'a StoredEventEnvelope,
}

fn legacy_hash_entry(
    sequence: u64,
    previous_hash: Option<&JournalHash>,
    envelope: &StoredEventEnvelope,
    phase: JournalDispatchPhase,
) -> JournalHash {
    let bytes = serde_json::to_vec(&LegacyJournalHashInput {
        sequence,
        previous_hash,
        phase,
        envelope,
    })
    .expect_value("legacy hash input encodes");
    JournalHash::parse(format!("journal-hash:{:x}", Sha256::digest(bytes)))
        .expect_value("legacy hash parses")
}

fn v2_hash_entry(
    sequence: u64,
    previous_hash: Option<&JournalHash>,
    envelope: &StoredEventEnvelope,
    phase: JournalDispatchPhase,
    durability: JournalAppendDurability,
) -> JournalHash {
    let bytes = serde_json::to_vec(&V2JournalHashInput {
        version: 2,
        sequence,
        previous_hash,
        phase,
        durability,
        envelope,
    })
    .expect_value("v2 hash input encodes");
    JournalHash::parse(format!("journal-hash:{:x}", Sha256::digest(bytes)))
        .expect_value("v2 hash parses")
}

fn unique_stored_event(label: &str, index: usize) -> StoredEventEnvelope {
    let mut event = stored_event(test_event(TestText(format!("{label} {index}"))));
    event.event_id =
        EventId::parse(format!("idempotent-event-{index}")).expect_value("unique event id");
    event.idempotency_key = IdempotencyKey::parse(format!("idempotent-key-{index}"))
        .expect_value("unique idempotency key");
    event
}
