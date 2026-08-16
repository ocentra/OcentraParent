use crate::{
    NetworkAppInventoryEntry, NetworkProcessAppCorrelationInput, NetworkProcessCorrelationError,
    NetworkProcessSnapshot,
};

pub(crate) fn matched_process_snapshot(
    input: &NetworkProcessAppCorrelationInput,
) -> Option<&NetworkProcessSnapshot> {
    let observed_pid = input.flow.observed_pid?;
    input
        .process_snapshots
        .iter()
        .find(|snapshot| snapshot.pid == observed_pid)
}

pub(crate) fn matched_app_inventory<'a>(
    snapshot: &NetworkProcessSnapshot,
    inventory: &'a [NetworkAppInventoryEntry],
) -> Option<&'a NetworkAppInventoryEntry> {
    inventory
        .iter()
        .find(|app| {
            same_optional(
                app.executable_path.as_ref(),
                snapshot.executable_path.as_ref(),
            )
        })
        .or_else(|| {
            inventory
                .iter()
                .find(|app| same_optional(app.process_name.as_ref(), Some(&snapshot.process_name)))
        })
}

pub(crate) fn validate_process_correlation_input(
    input: &NetworkProcessAppCorrelationInput,
) -> Result<(), NetworkProcessCorrelationError> {
    if input.flow.flow_ref.trim().is_empty() {
        return Err(NetworkProcessCorrelationError::EmptyFlowRef);
    }

    validate_snapshots(&input.process_snapshots)?;
    validate_inventory(&input.app_inventory)?;
    Ok(())
}

pub(crate) fn non_empty_option(value: Option<&String>) -> Option<&str> {
    value
        .map(String::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn validate_snapshots(
    snapshots: &[NetworkProcessSnapshot],
) -> Result<(), NetworkProcessCorrelationError> {
    for snapshot in snapshots {
        if snapshot.source_ref.trim().is_empty() {
            return Err(NetworkProcessCorrelationError::EmptyProcessSnapshotRef);
        }
        if snapshot.process_name.trim().is_empty() {
            return Err(NetworkProcessCorrelationError::EmptyProcessSnapshotName);
        }
    }
    Ok(())
}

fn validate_inventory(
    inventory: &[NetworkAppInventoryEntry],
) -> Result<(), NetworkProcessCorrelationError> {
    for app in inventory {
        if app.source_ref.trim().is_empty() {
            return Err(NetworkProcessCorrelationError::EmptyAppInventoryRef);
        }
        if app.app_id.trim().is_empty() {
            return Err(NetworkProcessCorrelationError::EmptyAppInventoryId);
        }
        if app.display_name.trim().is_empty() {
            return Err(NetworkProcessCorrelationError::EmptyAppInventoryDisplayName);
        }
    }
    Ok(())
}

fn same_optional(left: Option<&String>, right: Option<&String>) -> bool {
    left.and_then(|value| non_empty_option(Some(value)))
        .zip(right.and_then(|value| non_empty_option(Some(value))))
        .is_some_and(|(left, right)| left.eq_ignore_ascii_case(right))
}
