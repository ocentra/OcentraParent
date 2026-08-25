//! Service Control Manager observation composition.

#[path = "registry_service_config.rs"]
mod config;
#[path = "registry_service_extended.rs"]
mod extended;
#[path = "registry_service_failure.rs"]
mod failure;
#[path = "registry_service_security.rs"]
mod security;

use super::super::handles::last_error;
use super::super::handles::{ScManagerInner, ServiceInner};
use crate::{Error, OwnedScManager, OwnedService, Result, ServiceName, ServiceObservation};
use std::ptr;
use windows_sys::Win32::Storage::FileSystem::READ_CONTROL;
use windows_sys::Win32::System::Services::{
    OpenSCManagerW, OpenServiceW, SC_MANAGER_CONNECT, SERVICE_QUERY_CONFIG,
};

impl OwnedScManager {
    pub fn open() -> Result<Self> {
        let handle = unsafe { OpenSCManagerW(ptr::null(), ptr::null(), SC_MANAGER_CONNECT) };
        if handle.is_null() {
            return Err(Error::Win32(last_error()));
        }
        Ok(Self {
            inner: ScManagerInner { handle },
        })
    }

    pub fn open_service(&self, name: &ServiceName) -> Result<OwnedService> {
        let name_wide = name.wide_nul()?;
        let handle = unsafe {
            OpenServiceW(
                self.inner.handle,
                name_wide.as_ptr(),
                SERVICE_QUERY_CONFIG | READ_CONTROL,
            )
        };
        if handle.is_null() {
            return Err(Error::Win32(last_error()));
        }
        Ok(OwnedService {
            inner: ServiceInner { handle },
            service_name: name.text(),
        })
    }
}

impl OwnedService {
    pub fn observation(&self) -> Result<ServiceObservation> {
        self.reobserve()
    }

    /// Repeat the complete observation on the same retained SCM service handle.
    pub fn reobserve(&self) -> Result<ServiceObservation> {
        let config = config::query_service_config(self.inner.handle)?;
        let failure = failure::query_failure_actions(self.inner.handle)?;
        Ok(ServiceObservation {
            service_name: self.service_name.clone(),
            service_type: config.service_type,
            start_type: config.start_type,
            error_control: config.error_control,
            binary_path: config.binary_path,
            load_order_group: config.load_order_group,
            tag_id: config.tag_id,
            dependencies: config.dependencies,
            start_name: config.start_name,
            display_name: config.display_name,
            service_sid_type: config::query_service_sid_type(self.inner.handle)?,
            required_privileges: config::query_required_privileges(self.inner.handle)?,
            delayed_auto_start: config::query_delayed_auto_start(self.inner.handle)?,
            launch_protected: extended::query_launch_protected(self.inner.handle)?,
            failure_actions_reset_period: failure.reset_period,
            failure_actions_reboot_message: failure.reboot_message,
            failure_actions_command: failure.command,
            failure_actions: failure.actions,
            failure_actions_on_non_crash_failures: failure.on_non_crash_failures,
            security: security::query_service_security(self.inner.handle)?,
        })
    }
}
