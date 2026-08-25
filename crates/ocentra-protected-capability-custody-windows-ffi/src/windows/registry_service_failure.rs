//! Bounded SCM failure-action observations.

use super::config;
use crate::{Error, InputFault, Result, ServiceFailureAction, WindowsText, MAX_BUFFER_BYTES};
use std::ptr;
use windows_sys::Win32::System::Services::{
    SC_ACTION, SC_ACTION_NONE, SC_ACTION_REBOOT, SC_ACTION_RESTART, SC_ACTION_RUN_COMMAND,
    SC_HANDLE, SERVICE_CONFIG_FAILURE_ACTIONS, SERVICE_CONFIG_FAILURE_ACTIONS_FLAG,
    SERVICE_FAILURE_ACTIONSW, SERVICE_FAILURE_ACTIONS_FLAG,
};

const MAX_FAILURE_ACTIONS: usize = 64;

pub(super) struct FailureActionsSnapshot {
    pub(super) reset_period: u32,
    pub(super) reboot_message: Option<WindowsText>,
    pub(super) command: Option<WindowsText>,
    pub(super) actions: Vec<ServiceFailureAction>,
    pub(super) on_non_crash_failures: bool,
}

pub(super) fn query_failure_actions(handle: SC_HANDLE) -> Result<FailureActionsSnapshot> {
    let buffer = config::query_service_config2(handle, SERVICE_CONFIG_FAILURE_ACTIONS)?;
    if buffer.len() < core::mem::size_of::<SERVICE_FAILURE_ACTIONSW>() {
        return Err(Error::InvalidInput(
            InputFault::ServiceFailureActionsSizeInvalid,
        ));
    }
    let value = unsafe { ptr::read_unaligned(buffer.as_ptr() as *const SERVICE_FAILURE_ACTIONSW) };
    let count = usize::try_from(value.cActions)?;
    if count > MAX_FAILURE_ACTIONS {
        return Err(Error::BufferTooLarge);
    }
    let actions = copy_actions(value.lpsaActions, count, &buffer)?;
    let on_non_crash_failures = query_failure_actions_flag(handle)?;
    Ok(FailureActionsSnapshot {
        reset_period: value.dwResetPeriod,
        reboot_message: config::strings::copy_optional_wide_ptr(value.lpRebootMsg, &buffer)?,
        command: config::strings::copy_optional_wide_ptr(value.lpCommand, &buffer)?,
        actions,
        on_non_crash_failures,
    })
}

fn query_failure_actions_flag(handle: SC_HANDLE) -> Result<bool> {
    let buffer = config::query_service_config2(handle, SERVICE_CONFIG_FAILURE_ACTIONS_FLAG)?;
    if buffer.len() != core::mem::size_of::<SERVICE_FAILURE_ACTIONS_FLAG>() {
        return Err(Error::InvalidInput(
            InputFault::ServiceFailureActionsFlagSizeInvalid,
        ));
    }
    Ok(
        unsafe { ptr::read_unaligned(buffer.as_ptr() as *const SERVICE_FAILURE_ACTIONS_FLAG) }
            .fFailureActionsOnNonCrashFailures
            != 0,
    )
}

fn copy_actions(
    pointer: *mut SC_ACTION,
    count: usize,
    buffer: &[u8],
) -> Result<Vec<ServiceFailureAction>> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if pointer.is_null() {
        return Err(Error::InvalidInput(InputFault::ServiceFailureActionMissing));
    }
    let base = buffer.as_ptr() as usize;
    let end = base
        .checked_add(buffer.len())
        .ok_or(Error::BufferTooLarge)?;
    let start = pointer as usize;
    let bytes = count
        .checked_mul(core::mem::size_of::<SC_ACTION>())
        .ok_or(Error::BufferTooLarge)?;
    let actions_end = start.checked_add(bytes).ok_or(Error::BufferTooLarge)?;
    if start < base
        || actions_end > end
        || !start.is_multiple_of(core::mem::align_of::<SC_ACTION>())
        || bytes > MAX_BUFFER_BYTES
    {
        return Err(Error::InvalidInput(
            InputFault::ServiceFailureActionOutsideBuffer,
        ));
    }
    let raw = unsafe { core::slice::from_raw_parts(pointer, count) };
    let mut normalized = Vec::with_capacity(count);
    for action in raw {
        if !matches!(
            action.Type,
            SC_ACTION_NONE | SC_ACTION_RESTART | SC_ACTION_REBOOT | SC_ACTION_RUN_COMMAND
        ) {
            return Err(Error::InvalidInput(
                InputFault::ServiceFailureActionTypeUnknown,
            ));
        }
        normalized.push(ServiceFailureAction {
            action_type: action.Type,
            delay_ms: action.Delay,
        });
    }
    Ok(normalized)
}
