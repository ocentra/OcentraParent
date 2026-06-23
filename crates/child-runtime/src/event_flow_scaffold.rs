use std::sync::{Arc, Mutex, MutexGuard};

use ocentra_eventing::error::EventingError;

pub(crate) fn clear_optional_event<E>(
    value: &Arc<Mutex<Option<E>>>,
    _poisoned_message: &'static str,
) {
    *lock_recover(value) = None;
}

pub(crate) fn record_optional_event<E>(
    value: &Arc<Mutex<Option<E>>>,
    event: E,
    _poisoned_message: &'static str,
) {
    *lock_recover(value) = Some(event);
}

pub(crate) fn optional_event<E>(value: &Arc<Mutex<Option<E>>>) -> Option<E>
where
    E: Clone,
{
    lock_recover(value).clone()
}

pub(crate) fn required_optional_event<E>(
    value: &Arc<Mutex<Option<E>>>,
    _poisoned_message: &'static str,
    missing_field: &'static str,
    missing_value: &'static str,
) -> Result<E, EventingError>
where
    E: Clone,
{
    lock_recover(value)
        .clone()
        .ok_or_else(|| EventingError::InvalidValue {
            field: missing_field,
            value: missing_value.to_string(),
        })
}

pub(crate) fn record_event<E>(
    value: &Arc<Mutex<Vec<E>>>,
    event: E,
    _poisoned_message: &'static str,
) {
    lock_recover(value).push(event);
}

pub(crate) fn latest_event<E>(
    value: &Arc<Mutex<Vec<E>>>,
    _poisoned_message: &'static str,
    missing_field: &'static str,
    missing_value: &'static str,
) -> Result<E, EventingError>
where
    E: Clone,
{
    lock_recover(value)
        .last()
        .cloned()
        .ok_or_else(|| EventingError::InvalidValue {
            field: missing_field,
            value: missing_value.to_string(),
        })
}

pub(crate) fn lock_recover<T>(value: &Arc<Mutex<T>>) -> MutexGuard<'_, T> {
    value
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
