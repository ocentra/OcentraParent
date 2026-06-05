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
                .expect("recording log")
                .push(String::from("handler"));
            Ok(())
        }
    })
    .await
    .expect("subscriber registers");
}

pub(super) fn stored_event(event: TestEvent) -> StoredEventEnvelope {
    EventEnvelope::from_event(event, metadata(TEST_TARGET))
        .expect("envelope builds")
        .store()
        .expect("stored envelope builds")
}

pub(super) fn event_type(value: &str) -> EventType {
    EventType::parse(value).expect("event type parses")
}

pub(super) fn shared_log() -> Arc<Mutex<Vec<String>>> {
    Arc::new(Mutex::new(Vec::new()))
}

pub(super) fn snapshot(log: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
    log.lock().expect("recording log").clone()
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
        .expect("journal file reads")
        .lines()
        .map(String::from)
        .collect()
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
            let mut log = self.log.lock().expect("recording log");
            log.push(format!("journal:{}", envelope.contract.event_type.as_str()));
            Ok(JournalAppend {
                sequence: log.len() as u64,
                previous_hash: None,
                current_hash: None,
            })
        })
    }
}
