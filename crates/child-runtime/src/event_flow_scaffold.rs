use std::sync::{Arc, Mutex};

use ocentra_eventing::error::EventingError;

pub(crate) fn clear_optional_event<E>(
    value: &Arc<Mutex<Option<E>>>,
    poisoned_message: &'static str,
) {
    *value.lock().expect(poisoned_message) = None;
}

pub(crate) fn record_optional_event<E>(
    value: &Arc<Mutex<Option<E>>>,
    event: E,
    poisoned_message: &'static str,
) {
    *value.lock().expect(poisoned_message) = Some(event);
}

pub(crate) fn optional_event<E>(value: &Arc<Mutex<Option<E>>>) -> Option<E>
where
    E: Clone,
{
    value.lock().ok()?.clone()
}

pub(crate) fn required_optional_event<E>(
    value: &Arc<Mutex<Option<E>>>,
    poisoned_message: &'static str,
    missing_field: &'static str,
    missing_value: &'static str,
) -> Result<E, EventingError>
where
    E: Clone,
{
    value
        .lock()
        .expect(poisoned_message)
        .clone()
        .ok_or_else(|| EventingError::InvalidValue {
            field: missing_field,
            value: missing_value.to_string(),
        })
}

pub(crate) fn record_event<E>(
    value: &Arc<Mutex<Vec<E>>>,
    event: E,
    poisoned_message: &'static str,
) {
    value.lock().expect(poisoned_message).push(event);
}

pub(crate) fn latest_event<E>(
    value: &Arc<Mutex<Vec<E>>>,
    poisoned_message: &'static str,
    missing_field: &'static str,
    missing_value: &'static str,
) -> Result<E, EventingError>
where
    E: Clone,
{
    value
        .lock()
        .expect(poisoned_message)
        .last()
        .cloned()
        .ok_or_else(|| EventingError::InvalidValue {
            field: missing_field,
            value: missing_value.to_string(),
        })
}
