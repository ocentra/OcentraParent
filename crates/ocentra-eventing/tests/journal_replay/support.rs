use ocentra_eventing::bus::EventBus;
use ocentra_eventing::envelope::{EventEnvelope, StoredEventEnvelope};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{EventId, EventType};
use ocentra_eventing::journal::policy::JournalPolicy;
use ocentra_eventing::journal::{EventJournal, JournalAppend};
use std::{
    future::Future,
    path::Path,
    path::PathBuf,
    pin::Pin,
    sync::{Arc, Mutex},
};

use super::super::fixtures::{metadata, subscriber, TestEvent, TEST_SUBSCRIBER, TEST_TARGET};

#[derive(Clone, Debug)]
pub(super) struct TestText(pub(super) String);


#[derive(Clone, Debug)]
pub(super) struct JournalPath(pub(super) PathBuf);

#[derive(Clone, Debug, Default)]
pub(super) struct JournalLine(pub(super) String);

#[derive(Clone, Debug, Default)]
pub(super) struct JournalLines(Vec<JournalLine>);

impl std::ops::Deref for JournalLines {
    type Target = [JournalLine];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl std::ops::DerefMut for JournalLines {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl JournalLines {
    fn push(&mut self, value: JournalLine) {
        self.0.push(value);
    }
}

impl AsRef<Path> for JournalPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

pub(super) fn bus_with_recording_journal(
    policy: JournalPolicy,
    log: Arc<Mutex<JournalLines>>,
) -> EventBus {
    EventBus::with_journal(policy, Arc::new(RecordingJournal { log }))
}

pub(super) async fn subscribe_log_handler(bus: &EventBus, log: Arc<Mutex<JournalLines>>) {
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
                    .push(JournalLine(String::from("handler")));
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

pub(super) fn shared_log() -> Arc<Mutex<JournalLines>> {
    Arc::new(Mutex::new(JournalLines::default()))
}

pub(super) fn snapshot(log: &Arc<Mutex<JournalLines>>) -> JournalLines {
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

pub(super) async fn read_lines(path: JournalPath) -> JournalLines {
    let lines = tokio::fs::read_to_string(path)
        .await
        .expect_value("journal file reads")
        .lines()
        .map(|line| JournalLine(String::from(line)))
        .collect::<Vec<_>>();
    JournalLines(lines)
}

pub(super) async fn write_lines(path: JournalPath, lines: &JournalLines) {
    let mut content = lines
        .iter()
        .map(|line| line.as_ref())
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
    lines[0] = JournalLine(serde_json::to_string(&entry).expect_value("journal value encodes"));
    write_lines(path, &lines).await;
}

pub(super) async fn cleanup(path: JournalPath) {
    let _ = tokio::fs::remove_file(path).await;
}

struct RecordingJournal {
    log: Arc<Mutex<JournalLines>>,
}

impl EventJournal for RecordingJournal {
    fn append<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
    ) -> Pin<Box<dyn Future<Output = Result<JournalAppend, EventingError>> + Send + 'a>> {
        Box::pin(async move {
            let mut log = self.log.lock().expect_value("recording log");
            log.push(JournalLine(format!("journal:{}", envelope.contract.event_type.as_str())));
            Ok(JournalAppend {
                sequence: log.len() as u64,
                previous_hash: None,
                current_hash: None,
            })
        })
    }
}
