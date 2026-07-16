use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphNodeKind;
use ocentra_parent_agent_protocol::activity_memory_graph::ACTIVITY_MEMORY_GRAPH_GAME_LABEL_HINT;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::activity_store_memory_graph_rows::MemoryGraphStoreRow;

pub(crate) fn node_kind(row: &MemoryGraphStoreRow) -> Option<ActivityMemoryGraphNodeKind> {
    match row.kind.as_str() {
        constants::activity_event_kind::URL_OBSERVED => {
            Some(ActivityMemoryGraphNodeKind::BrowserUrl)
        }
        constants::activity_event_kind::VIDEO_OBSERVED => Some(ActivityMemoryGraphNodeKind::Video),
        constants::activity_event_kind::WINDOW_FOCUSED if looks_like_game(row) => {
            Some(ActivityMemoryGraphNodeKind::Game)
        }
        constants::activity_event_kind::WINDOW_FOCUSED => Some(ActivityMemoryGraphNodeKind::App),
        _ => None,
    }
}

pub(crate) fn looks_like_game(row: &MemoryGraphStoreRow) -> bool {
    node_label(row)
        .map(|label| {
            label
                .to_ascii_lowercase()
                .contains(ACTIVITY_MEMORY_GRAPH_GAME_LABEL_HINT)
        })
        .unwrap_or(false)
}

pub(crate) fn node_label(row: &MemoryGraphStoreRow) -> Option<String> {
    if row.kind == constants::activity_event_kind::URL_OBSERVED {
        return string_field(&row.fields, constants::field::URL)
            .or_else(|| row.subject_display_name.clone());
    }
    if row.kind == constants::activity_event_kind::VIDEO_OBSERVED {
        return row
            .subject_display_name
            .clone()
            .or_else(|| string_field(&row.fields, constants::field::TITLE))
            .or_else(|| string_field(&row.fields, constants::field::URL));
    }
    row.subject_display_name
        .clone()
        .or_else(|| string_field(&row.fields, constants::field::WINDOW_TITLE))
        .or_else(|| string_field(&row.fields, constants::field::APP_NAME))
        .or_else(|| string_field(&row.fields, constants::field::PROCESS_NAME))
}

pub(crate) fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
