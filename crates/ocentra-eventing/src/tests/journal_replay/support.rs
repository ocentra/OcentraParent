use crate::ExpectValue;
use std::{
    future::Future,
    path::PathBuf,
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::{
    EventBus, EventEnvelope, EventJournal, EventType, EventingError, JournalAppend, JournalPolicy,
    StoredEventEnvelope,
};

use super::super::fixtures::{metadata, subscriber, TestEvent, TEST_SUBSCRIBER, TEST_TARGET};

pub(super) fn bus_with_recording_journal(
    policy: JournalPolicy,
    log: Arc<Mutex<Vec<String>>>,
) -> EventBus {
    EventBus::with_journal(policy, Arc::new(RecordingJournal { log }))
}

pub(super) async fn subscribe_log_handler(bus: &EventBus, log: Arc<Mutex<Vec<String>>>) {
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |_| {
        let log = Arc::clone(&log);
        async move {
            log.lock()
                .expect_value("recording log")
                .push(String::from("handler"));
            Ok(())
        }
    })
    .await
    .expect_value("subscriber registers");
}

pub(super) fn stored_event(event: TestEvent) -> StoredEventEnvelope {
    EventEnvelope::from_event(event, metadata(TEST_TARGET))
        .expect_value("envelope builds")
        .store()
        .expect_value("stored envelope builds")
}

pub(super) fn event_type(value: &str) -> EventType {
    EventType::parse(value).expect_value("event type parses")
}

pub(super) fn shared_log() -> Arc<Mutex<Vec<String>>> {
    Arc::new(Mutex::new(Vec::new()))
}

pub(super) fn snapshot(log: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
    log.lock().expect_value("recording log").clone()
}

pub(super) fn journal_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-eventing-{label}-{}-{}.ndjson",
        std::process::id(),
        crate::EventId::generated().as_str()
    ))
}

pub(super) async fn read_lines(path: &PathBuf) -> Vec<String> {
    tokio::fs::read_to_string(path)
        .await
        .expect_value("journal file reads")
        .lines()
        .map(String::from)
        .collect()
}

pub(super) async fn write_lines(path: &PathBuf, lines: &[String]) {
    let mut content = lines.join("\n");
    content.push('\n');
    tokio::fs::write(path, content)
        .await
        .expect_value("journal file writes");
}

pub(super) async fn tamper_first_journal_payload_label(path: &PathBuf, label: &str) {
    let mut lines = read_lines(path).await;
    let mut entry: serde_json::Value =
        serde_json::from_str(&lines[0]).expect_value("journal line decodes as value");
    entry["envelope"]["payload"]["label"] = serde_json::Value::String(label.to_string());
    lines[0] = serde_json::to_string(&entry).expect_value("journal value encodes");
    write_lines(path, &lines).await;
}

pub(super) async fn cleanup(path: &PathBuf) {
    let _ = tokio::fs::remove_file(path).await;
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
            })
        })
    }
}
