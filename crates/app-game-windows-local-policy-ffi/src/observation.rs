#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppGameWindowsLocalPolicyObservationState {
    Ready,
    Partial,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameWindowsLocalPolicyObservation {
    state: AppGameWindowsLocalPolicyObservationState,
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

impl AppGameWindowsLocalPolicyObservation {
    pub(crate) fn from_values(values: AppGameWindowsLocalPolicyObservationValues) -> Self {
        let succeeded = u8::from(values.app_id_service_query_succeeded)
            + u8::from(values.app_locker_policy_readable)
            + u8::from(values.device_guard_query_succeeded);
        let state = if values.probe_supported && succeeded == 3 {
            AppGameWindowsLocalPolicyObservationState::Ready
        } else if values.probe_supported && succeeded > 0 {
            AppGameWindowsLocalPolicyObservationState::Partial
        } else {
            AppGameWindowsLocalPolicyObservationState::Unavailable
        };
        Self {
            state,
            probe_supported: values.probe_supported,
            app_id_service_query_succeeded: values.app_id_service_query_succeeded,
            app_id_service_present: values.app_id_service_present,
            app_id_service_running: values.app_id_service_running,
            app_locker_policy_readable: values.app_locker_policy_readable,
            app_locker_collection_count: values.app_locker_collection_count,
            app_locker_rule_count: values.app_locker_rule_count,
            device_guard_query_succeeded: values.device_guard_query_succeeded,
            device_guard_configured: values.device_guard_configured,
            device_guard_running: values.device_guard_running,
            app_control_configured: values.app_control_configured,
            app_control_audit_only: values.app_control_audit_only,
            app_control_policy_reports_enforced: values.app_control_policy_reports_enforced,
        }
    }

    pub const fn state(&self) -> AppGameWindowsLocalPolicyObservationState {
        self.state
    }

    pub const fn probe_supported(&self) -> bool {
        self.probe_supported
    }

    pub const fn app_id_service_query_succeeded(&self) -> bool {
        self.app_id_service_query_succeeded
    }

    pub const fn app_id_service_present(&self) -> bool {
        self.app_id_service_present
    }

    pub const fn app_id_service_running(&self) -> bool {
        self.app_id_service_running
    }

    pub const fn app_locker_policy_readable(&self) -> bool {
        self.app_locker_policy_readable
    }

    pub const fn app_locker_collection_count(&self) -> u64 {
        self.app_locker_collection_count
    }

    pub const fn app_locker_rule_count(&self) -> u64 {
        self.app_locker_rule_count
    }

    pub const fn device_guard_query_succeeded(&self) -> bool {
        self.device_guard_query_succeeded
    }

    pub const fn device_guard_configured(&self) -> bool {
        self.device_guard_configured
    }

    pub const fn device_guard_running(&self) -> bool {
        self.device_guard_running
    }

    pub const fn app_control_configured(&self) -> bool {
        self.app_control_configured
    }

    pub const fn app_control_audit_only(&self) -> bool {
        self.app_control_audit_only
    }

    pub const fn app_control_policy_reports_enforced(&self) -> bool {
        self.app_control_policy_reports_enforced
    }
}

#[derive(Clone, Copy)]
pub(crate) struct AppGameWindowsLocalPolicyObservationValues {
    pub(crate) probe_supported: bool,
    pub(crate) app_id_service_query_succeeded: bool,
    pub(crate) app_id_service_present: bool,
    pub(crate) app_id_service_running: bool,
    pub(crate) app_locker_policy_readable: bool,
    pub(crate) app_locker_collection_count: u64,
    pub(crate) app_locker_rule_count: u64,
    pub(crate) device_guard_query_succeeded: bool,
    pub(crate) device_guard_configured: bool,
    pub(crate) device_guard_running: bool,
    pub(crate) app_control_configured: bool,
    pub(crate) app_control_audit_only: bool,
    pub(crate) app_control_policy_reports_enforced: bool,
}
