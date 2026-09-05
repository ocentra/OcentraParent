use ocentra_app_game_windows_local_policy_ffi::error::AppGameWindowsLocalPolicyError;

#[cfg(windows)]
#[test]
fn no_argument_observer_returns_only_bounded_local_policy_values(
) -> Result<(), AppGameWindowsLocalPolicyError> {
    let observation = ocentra_app_game_windows_local_policy_ffi::observe_local_policy()?;
    assert_eq!(u8::from(observation.probe_supported()), 1);
    assert_eq!(
        observation
            .app_locker_collection_count()
            .min(ocentra_app_game_windows_local_policy_ffi::APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT),
        observation.app_locker_collection_count()
    );
    assert_eq!(
        observation.app_locker_rule_count().min(
            ocentra_app_game_windows_local_policy_ffi::APP_GAME_WINDOWS_LOCAL_POLICY_MAX_RULE_COUNT
        ),
        observation.app_locker_rule_count()
    );
    Ok(())
}

#[cfg(not(windows))]
#[test]
fn no_argument_observer_is_unsupported_without_platform_spawn() {
    assert_eq!(
        ocentra_app_game_windows_local_policy_ffi::observe_local_policy(),
        Err(AppGameWindowsLocalPolicyError::UnsupportedPlatform)
    );
}
