use std::collections::BTreeSet;

use ocentra_eventing::envelope::StoredEventEnvelope;
use ocentra_eventing::journal::ndjson::NdjsonEventJournal;
use ocentra_eventing::replay::ReplayFilter;

use crate::app_game_unknown_approval_event::{
    unknown_approval_event_type, AppGameUnknownApprovalEvent,
};
use crate::app_game_unknown_approval_reducer::apply_unknown_approval_event;
use crate::app_game_unknown_approval_types::{
    AppGameUnknownApprovalError, AppGameUnknownApprovalSnapshot,
};

pub(crate) struct AppGameUnknownApprovalHistory {
    pub(crate) events: Vec<(u64, AppGameUnknownApprovalEvent)>,
    pub(crate) snapshot: Option<AppGameUnknownApprovalSnapshot>,
}

pub(crate) async fn read_unknown_approval_history(
    journal: &NdjsonEventJournal,
    request_id: &str,
) -> Result<AppGameUnknownApprovalHistory, AppGameUnknownApprovalError> {
    if !journal.path().exists() {
        return Ok(AppGameUnknownApprovalHistory {
            events: Vec::new(),
            snapshot: None,
        });
    }
    let report = journal
        .replay_projection(ReplayFilter::for_event_type(unknown_approval_event_type()?))
        .await?;
    let mut events = Vec::new();
    for record in report.records {
        if record.envelope.aggregate_key.as_str() != request_id {
            continue;
        }
        events.push((record.sequence, decode_event(&record.envelope)?));
    }
    let snapshot = project_unknown_approval_events(&events)?;
    Ok(AppGameUnknownApprovalHistory { events, snapshot })
}

fn decode_event(
    envelope: &StoredEventEnvelope,
) -> Result<AppGameUnknownApprovalEvent, AppGameUnknownApprovalError> {
    Ok(envelope.decode::<AppGameUnknownApprovalEvent>()?.payload)
}

fn project_unknown_approval_events(
    events: &[(u64, AppGameUnknownApprovalEvent)],
) -> Result<Option<AppGameUnknownApprovalSnapshot>, AppGameUnknownApprovalError> {
    let mut snapshot = None;
    let mut transition_ids = BTreeSet::new();
    for (_sequence, event) in events {
        if !transition_ids.insert(event.transition_id.as_str()) {
            return Err(AppGameUnknownApprovalError::DuplicateTransition {
                transition_id: event.transition_id.clone(),
            });
        }
        snapshot = Some(apply_unknown_approval_event(snapshot.as_ref(), event)?);
    }
    Ok(snapshot)
}
