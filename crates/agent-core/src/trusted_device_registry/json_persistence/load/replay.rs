use std::collections::VecDeque;

use serde_json::Value;

pub(super) fn strict_replay_ids(value: &Value, key: &str) -> Option<VecDeque<String>> {
    let ids = match value.get(key) {
        Some(ids) => serde_json::from_value(ids.clone()).ok()?,
        None => VecDeque::new(),
    };
    let mut bounded = VecDeque::new();
    for id in ids {
        super::super::super::replay::remember_bounded_replay_id(&mut bounded, id);
    }
    Some(bounded)
}
