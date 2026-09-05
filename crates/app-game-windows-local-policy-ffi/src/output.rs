use serde::Deserialize;

use crate::observation::{
    AppGameWindowsLocalPolicyObservation, AppGameWindowsLocalPolicyObservationValues,
};
use crate::{
    error::AppGameWindowsLocalPolicyError, Result,
    APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT,
    APP_GAME_WINDOWS_LOCAL_POLICY_MAX_RULE_COUNT, APP_GAME_WINDOWS_LOCAL_POLICY_OUTPUT_MAX_BYTES,
};

const OUTPUT_SCHEMA_VERSION: u16 = 1;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LocalPolicyWire {
    schema_version: u16,
    probe_supported: bool,
    app_id_service_query_succeeded: bool,
    app_id_service_present: bool,
    app_id_service_running: bool,
    app_locker_policy_readable: bool,
    app_locker_collection_count: u64,
    app_locker_rule_count: u64,
    device_guard_query_succeeded: bool,
    device_guard_configured: bool,
    device_guard_running: bool,
    app_control_configured: bool,
    app_control_audit_only: bool,
    app_control_policy_reports_enforced: bool,
}

pub(super) fn parse(output: &[u8]) -> Result<AppGameWindowsLocalPolicyObservation> {
    if output.len() > APP_GAME_WINDOWS_LOCAL_POLICY_OUTPUT_MAX_BYTES {
        return Err(AppGameWindowsLocalPolicyError::OutputTooLarge);
    }
    let text = core::str::from_utf8(output)
        .map_err(|_utf8_error| AppGameWindowsLocalPolicyError::OutputInvalidUtf8)?;
    let wire: LocalPolicyWire = serde_json::from_str(text)
        .map_err(|_json_error| AppGameWindowsLocalPolicyError::OutputInvalidJson)?;
    validate(&wire)?;
    Ok(AppGameWindowsLocalPolicyObservation::from_values(
        AppGameWindowsLocalPolicyObservationValues {
            probe_supported: wire.probe_supported,
            app_id_service_query_succeeded: wire.app_id_service_query_succeeded,
            app_id_service_present: wire.app_id_service_present,
            app_id_service_running: wire.app_id_service_running,
            app_locker_policy_readable: wire.app_locker_policy_readable,
            app_locker_collection_count: wire.app_locker_collection_count,
            app_locker_rule_count: wire.app_locker_rule_count,
            device_guard_query_succeeded: wire.device_guard_query_succeeded,
            device_guard_configured: wire.device_guard_configured,
            device_guard_running: wire.device_guard_running,
            app_control_configured: wire.app_control_configured,
            app_control_audit_only: wire.app_control_audit_only,
            app_control_policy_reports_enforced: wire.app_control_policy_reports_enforced,
        },
    ))
}

fn validate(wire: &LocalPolicyWire) -> Result<()> {
    if wire.schema_version != OUTPUT_SCHEMA_VERSION {
        return Err(AppGameWindowsLocalPolicyError::OutputInvalidSchemaVersion);
    }
    let app_id_valid = wire.app_id_service_query_succeeded
        || (!wire.app_id_service_present && !wire.app_id_service_running);
    let app_id_relationship_valid = !wire.app_id_service_running || wire.app_id_service_present;
    let app_locker_valid = wire.app_locker_policy_readable
        || (wire.app_locker_collection_count == 0 && wire.app_locker_rule_count == 0);
    let app_locker_bounded = wire.app_locker_collection_count
        <= APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT
        && wire.app_locker_rule_count <= APP_GAME_WINDOWS_LOCAL_POLICY_MAX_RULE_COUNT;
    let device_guard_valid = wire.device_guard_query_succeeded
        || (!wire.device_guard_configured && !wire.device_guard_running);
    let device_guard_relationship_valid =
        !wire.device_guard_running || wire.device_guard_configured;
    let app_control_valid = (!wire.app_control_audit_only
        || !wire.app_control_policy_reports_enforced)
        && (wire.app_control_configured
            || (!wire.app_control_audit_only && !wire.app_control_policy_reports_enforced))
        && (!wire.app_control_configured || wire.device_guard_query_succeeded);
    let unsupported_probe_is_empty = wire.probe_supported || observations_are_empty(wire);
    if unsupported_probe_is_empty
        && app_id_valid
        && app_id_relationship_valid
        && app_locker_valid
        && app_locker_bounded
        && device_guard_valid
        && device_guard_relationship_valid
        && app_control_valid
    {
        return Ok(());
    }
    Err(AppGameWindowsLocalPolicyError::OutputInvalidInvariant)
}

fn observations_are_empty(wire: &LocalPolicyWire) -> bool {
    let booleans = [
        wire.app_id_service_query_succeeded,
        wire.app_id_service_present,
        wire.app_id_service_running,
        wire.app_locker_policy_readable,
        wire.device_guard_query_succeeded,
        wire.device_guard_configured,
        wire.device_guard_running,
        wire.app_control_configured,
        wire.app_control_audit_only,
        wire.app_control_policy_reports_enforced,
    ];
    booleans.into_iter().all(|value| !value)
        && wire.app_locker_collection_count == 0
        && wire.app_locker_rule_count == 0
}
