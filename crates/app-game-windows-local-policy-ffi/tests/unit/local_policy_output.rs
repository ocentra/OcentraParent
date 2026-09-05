use ocentra_app_game_windows_local_policy_ffi::error::AppGameWindowsLocalPolicyError;
use ocentra_app_game_windows_local_policy_ffi::observation::AppGameWindowsLocalPolicyObservationState;
use ocentra_app_game_windows_local_policy_ffi::{
    parse_local_policy_output, APP_GAME_WINDOWS_LOCAL_POLICY_OUTPUT_MAX_BYTES,
};

const READY_OUTPUT: &str = r#"{"schemaVersion":1,"probeSupported":true,"appIdServiceQuerySucceeded":true,"appIdServicePresent":true,"appIdServiceRunning":true,"appLockerPolicyReadable":true,"appLockerCollectionCount":4,"appLockerRuleCount":12,"deviceGuardQuerySucceeded":true,"deviceGuardConfigured":true,"deviceGuardRunning":true,"appControlConfigured":true,"appControlAuditOnly":true,"appControlPolicyReportsEnforced":false}"#;

#[test]
fn strict_output_parser_returns_only_reduced_ready_observations(
) -> Result<(), AppGameWindowsLocalPolicyError> {
    let observation = parse_local_policy_output(READY_OUTPUT.as_bytes())?;
    assert_eq!(
        observation.state(),
        AppGameWindowsLocalPolicyObservationState::Ready
    );
    assert_eq!(u8::from(observation.probe_supported()), 1);
    assert_eq!(u8::from(observation.app_id_service_query_succeeded()), 1);
    assert_eq!(u8::from(observation.app_id_service_present()), 1);
    assert_eq!(u8::from(observation.app_id_service_running()), 1);
    assert_eq!(u8::from(observation.app_locker_policy_readable()), 1);
    assert_eq!(observation.app_locker_collection_count(), 4);
    assert_eq!(observation.app_locker_rule_count(), 12);
    assert_eq!(u8::from(observation.device_guard_query_succeeded()), 1);
    assert_eq!(u8::from(observation.device_guard_configured()), 1);
    assert_eq!(u8::from(observation.device_guard_running()), 1);
    assert_eq!(u8::from(observation.app_control_configured()), 1);
    assert_eq!(u8::from(observation.app_control_audit_only()), 1);
    assert_eq!(
        u8::from(observation.app_control_policy_reports_enforced()),
        0
    );
    Ok(())
}

#[test]
fn strict_output_parser_preserves_partial_query_state() -> Result<(), AppGameWindowsLocalPolicyError>
{
    let output = READY_OUTPUT
        .replace(
            "\"appLockerPolicyReadable\":true",
            "\"appLockerPolicyReadable\":false",
        )
        .replace(
            "\"appLockerCollectionCount\":4",
            "\"appLockerCollectionCount\":0",
        )
        .replace("\"appLockerRuleCount\":12", "\"appLockerRuleCount\":0");
    let observation = parse_local_policy_output(output.as_bytes())?;
    assert_eq!(
        observation.state(),
        AppGameWindowsLocalPolicyObservationState::Partial
    );
    assert_eq!(observation.app_locker_collection_count(), 0);
    assert_eq!(observation.app_locker_rule_count(), 0);
    Ok(())
}

#[test]
fn strict_output_parser_rejects_unknown_private_shape_and_invalid_utf8() {
    let unknown = READY_OUTPUT.replace(
        "\"schemaVersion\":1",
        "\"schemaVersion\":1,\"rawRuleXml\":\"private\"",
    );
    assert_eq!(
        parse_local_policy_output(unknown.as_bytes()),
        Err(AppGameWindowsLocalPolicyError::OutputInvalidJson)
    );
    assert_eq!(
        parse_local_policy_output(&[0xff]),
        Err(AppGameWindowsLocalPolicyError::OutputInvalidUtf8)
    );
}

#[test]
fn strict_output_parser_rejects_oversize_and_impossible_observations() {
    let oversized = vec![b'x'; APP_GAME_WINDOWS_LOCAL_POLICY_OUTPUT_MAX_BYTES + 1];
    assert_eq!(
        parse_local_policy_output(&oversized),
        Err(AppGameWindowsLocalPolicyError::OutputTooLarge)
    );
    let impossible = READY_OUTPUT.replace(
        "\"appIdServicePresent\":true",
        "\"appIdServicePresent\":false",
    );
    assert_eq!(
        parse_local_policy_output(impossible.as_bytes()),
        Err(AppGameWindowsLocalPolicyError::OutputInvalidInvariant)
    );
    let unsupported_with_observations =
        READY_OUTPUT.replace("\"probeSupported\":true", "\"probeSupported\":false");
    assert_eq!(
        parse_local_policy_output(unsupported_with_observations.as_bytes()),
        Err(AppGameWindowsLocalPolicyError::OutputInvalidInvariant)
    );
}
