use ocentra_eventing::bus::EventBus;
use ocentra_eventing::envelope::{EventEnvelope, StoredEventEnvelope};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{EventId, EventType};
use ocentra_eventing::journal::ndjson::NdjsonJournalRecord;
use ocentra_eventing::journal::policy::JournalPolicy;
use ocentra_eventing::journal::{
    EventJournal, JournalAppend, JournalAppendDurability, JournalHashVersion,
};
use std::{
    ffi::OsStr,
    future::Future,
    path::Path,
    path::PathBuf,
    pin::Pin,
    sync::{Arc, Mutex},
};

use super::super::fixtures::{
    metadata, subscriber, TestEvent, TestText, TEST_SUBSCRIBER, TEST_TARGET,
};

#[derive(Clone, Debug)]
pub(super) struct JournalPath(pub(super) PathBuf);

impl AsRef<Path> for JournalPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<OsStr> for JournalPath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

pub(super) fn bus_with_recording_journal(
    policy: JournalPolicy,
    log: Arc<Mutex<Vec<String>>>,
) -> EventBus {
    EventBus::with_journal(policy, Arc::new(RecordingJournal { log }))
}

pub(super) async fn subscribe_log_handler(bus: &EventBus, log: Arc<Mutex<Vec<String>>>) {
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let log = Arc::clone(&log);
            async move {
                log.lock()
                    .expect_value("recording log")
                    .push(String::from("handler"));
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");
}

pub(super) fn stored_event(event: TestEvent) -> StoredEventEnvelope {
    EventEnvelope::from_event(event, metadata(TestText(TEST_TARGET.to_owned())))
        .expect_value("envelope builds")
        .store()
        .expect_value("stored envelope builds")
}

pub(super) fn event_type(value: TestText) -> EventType {
    EventType::parse(value.0).expect_value("event type parses")
}

pub(super) fn shared_log() -> Arc<Mutex<Vec<String>>> {
    Arc::new(Mutex::new(Vec::new()))
}

pub(super) fn snapshot(log: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
    log.lock().expect_value("recording log").clone()
}

pub(super) fn journal_path(label: TestText) -> JournalPath {
    let label = label.0;
    JournalPath(std::env::temp_dir().join(format!(
        "ocentra-eventing-{label}-{}-{}.ndjson",
        std::process::id(),
        EventId::generated().as_str()
    )))
}

pub(super) async fn read_lines(path: JournalPath) -> Vec<String> {
    tokio::fs::read_to_string(path)
        .await
        .expect_value("journal file reads")
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            NdjsonJournalRecord::parse(line, index + 1)
                .expect_value("journal record decodes")
                .entry()
                .map(|_| String::from(line))
        })
        .collect()
}

pub(super) async fn write_lines(path: JournalPath, lines: &[String]) {
    let mut content = lines
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("\n");
    content.push('\n');
    tokio::fs::write(path, content)
        .await
        .expect_value("journal file writes");
}

pub(super) async fn tamper_first_journal_payload_label(path: JournalPath, label: TestText) {
    let label = label.0;
    let mut lines = read_lines(path.clone()).await;
    let mut entry: serde_json::Value =
        serde_json::from_str(&lines[0]).expect_value("journal line decodes as value");
    entry["envelope"]["payload"]["label"] = serde_json::Value::String(label);
    lines[0] = serde_json::to_string(&entry).expect_value("journal value encodes");
    write_lines(path, &lines).await;
}

pub(super) async fn cleanup(path: JournalPath) {
    let _ = tokio::fs::remove_file(path).await;
}

pub(super) async fn cleanup_idempotent_journal(path: JournalPath) {
    let lock_path = append_lock_path(&path);
    cleanup(path).await;
    let _cleanup_lock = tokio::fs::remove_file(lock_path).await;
}

pub(super) fn append_lock_path(path: &JournalPath) -> PathBuf {
    let mut lock_path = path.0.as_os_str().to_os_string();
    lock_path.push(".append.lock");
    PathBuf::from(lock_path)
}

struct RecordingJournal {
    log: Arc<Mutex<Vec<String>>>,
}

impl EventJournal for RecordingJournal {
    fn append<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
    ) -> Pin<Box<dyn Future<Output = Result<JournalAppend, EventingError>> + Send + 'a>> {
        Box::pin(async move {
            let mut log = self.log.lock().expect_value("recording log");
            log.push(format!("journal:{}", envelope.contract.event_type.as_str()));
            Ok(JournalAppend {
                sequence: log.len() as u64,
                previous_hash: None,
                current_hash: None,
                hash_version: JournalHashVersion::V2,
                durability: JournalAppendDurability::Synchronized,
                requested_durability: JournalAppendDurability::Synchronized,
                synchronization_hash: None,
            })
        })
    }
}
