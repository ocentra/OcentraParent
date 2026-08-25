use std::{
    io,
    time::{SystemTime, UNIX_EPOCH},
};

use super::removal_record::ChildAgentRemovalRecord;

pub(super) fn validate_record(record: &ChildAgentRemovalRecord) -> io::Result<()> {
    if record.audit.iter().any(|entry| {
        entry.audit_ref.trim().is_empty()
            || entry.parent_authorization_ref.trim().is_empty()
            || entry.household_id.trim().is_empty()
            || entry.child_profile_id.trim().is_empty()
            || entry.target_device_id.trim().is_empty()
            || entry.recorded_at_unix_seconds == 0
    }) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "child removal audit state is incomplete",
        ));
    }
    if record
        .tamper_signals
        .iter()
        .any(|signal| signal.signal_ref.trim().is_empty() || signal.observed_at_unix_seconds == 0)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "child tamper signal state is incomplete",
        ));
    }
    Ok(())
}

pub(super) fn current_unix_seconds() -> io::Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)
        .map(|duration| duration.as_secs())
        .and_then(|seconds| {
            (seconds > 0)
                .then_some(seconds)
                .ok_or_else(|| io::Error::other("system clock produced an unusable timestamp"))
        })
}

pub(super) fn non_empty_ref(value: &str) -> io::Result<String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "parent authorization reference must not be empty",
        ));
    }
    Ok(value.to_owned())
}

pub(super) fn non_empty_signal_ref(value: &str) -> io::Result<String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "tamper signal reference must not be empty",
        ));
    }
    Ok(value.to_owned())
}
