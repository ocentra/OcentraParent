use std::{
    future::Future,
    path::PathBuf,
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::bus::EventBus;
use crate::envelope::EventEnvelope;
use crate::journal::{EventJournal, JournalAppend, policy::JournalPolicy};
use crate::error::EventingError;
use crate::ids::EventType;
use crate::envelope::StoredEventEnvelope;
use crate::sync::lock_unpoison;

use super::super::fixtures::{metadata, subscriber, TestEvent, TEST_SUBSCRIBER, TEST_TARGET};

pub(super) fn bus_with_recording_journal(
    policy: JournalPolicy,
    log: Arc<Mutex<Vec<String>>>,
) -> EventBus {
    EventBus::with_journal(policy, Arc::new(RecordingJournal { log }))
}

pub(super) async fn subscribe_log_handler(
    bus: &EventBus,
    log: Arc<Mutex<Vec<String>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |_| {
        let log = Arc::clone(&log);
        async move {
            lock_unpoison(&log).push(String::from("handler"));
            Ok(())
        }
    })
    .await
    ?;
    Ok(())
}

pub(super) fn stored_event(event: TestEvent) -> Result<StoredEventEnvelope, EventingError> {
    EventEnvelope::from_event(event, metadata(TEST_TARGET))?.store()
}

pub(super) fn event_type(value: &str) -> Result<EventType, EventingError> {
    EventType::parse(value)
}

pub(super) fn shared_log() -> Arc<Mutex<Vec<String>>> {
    Arc::new(Mutex::new(Vec::new()))
}

pub(super) fn snapshot(log: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
    lock_unpoison(log).clone()
}

pub(super) fn journal_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-eventing-{label}-{}-{}.ndjson",
        std::process::id(),
        crate::ids::EventId::generated().as_str()
    ))
}

pub(super) async fn read_lines(
    path: &PathBuf,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(tokio::fs::read_to_string(path).await?.lines().map(String::from).collect())
}

pub(super) async fn write_lines(
    path: &PathBuf,
    lines: &[String],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut content = lines.join("\n");
    content.push('\n');
    tokio::fs::write(path, content).await?;
    Ok(())
}

pub(super) async fn tamper_first_journal_payload_label(
    path: &PathBuf,
    label: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut lines = read_lines(path).await?;
    let mut entry: serde_json::Value = serde_json::from_str(&lines[0])?;
    entry["envelope"]["payload"]["label"] = serde_json::Value::String(label.to_string());
    lines[0] = serde_json::to_string(&entry)?;
    write_lines(path, &lines).await?;
    Ok(())
}

pub(super) async fn cleanup(path: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = tokio::fs::remove_file(path).await;
    Ok(())
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
            let mut log = lock_unpoison(&self.log);
            log.push(format!("journal:{}", envelope.contract.event_type.as_str()));
            Ok(JournalAppend {
                sequence: log.len() as u64,
                previous_hash: None,
                current_hash: None,
            })
        })
    }
}
