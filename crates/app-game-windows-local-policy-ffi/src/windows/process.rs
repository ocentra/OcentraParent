use std::ffi::OsString;
use std::os::windows::process::CommandExt;
use std::os::windows::{ffi::OsStringExt, io::AsRawHandle};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use windows_sys::Win32::Foundation::{GetLastError, HANDLE};
use windows_sys::Win32::System::Threading::{QueryFullProcessImageNameW, CREATE_NO_WINDOW};

use super::path_security::TrustedPowerShell;
use super::process_output::{OutputReaders, ProcessOutput};
use super::wire;
use crate::{
    error::AppGameWindowsLocalPolicyError, observation::AppGameWindowsLocalPolicyObservation,
    parse_local_policy_output, Result,
};

const PROCESS_DEADLINE: Duration = Duration::from_secs(5);
const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(5);
const MAX_PROCESS_PATH_CHARS: usize = 32 * 1024;
const SYSTEM_ROOT_ENV: &str = "SystemRoot";
const PS_MODULE_PATH_ENV: &str = "PSModulePath";

struct ProcessImagePath(PathBuf);

impl ProcessImagePath {
    fn matches(&self, expected: &Path) -> bool {
        let expected = expected.as_os_str().to_string_lossy();
        let observed = self.0.as_os_str().to_string_lossy();
        expected.eq_ignore_ascii_case(&observed)
    }
}

pub(super) fn observe(trusted: &TrustedPowerShell) -> Result<AppGameWindowsLocalPolicyObservation> {
    trusted.verify_current()?;
    let deadline = Instant::now() + PROCESS_DEADLINE;
    let mut child = spawn(trusted)?;
    verify_process_image(&child, trusted.executable())?;
    let stdout = child
        .stdout
        .take()
        .ok_or(AppGameWindowsLocalPolicyError::ProcessSpawn(0))?;
    let stderr = child
        .stderr
        .take()
        .ok_or(AppGameWindowsLocalPolicyError::ProcessSpawn(0))?;
    let readers = OutputReaders::start(stdout, stderr)?;
    let status = wait_until(&mut child, deadline)?;
    let output = readers.finish(deadline)?;
    trusted.verify_current()?;
    validate_process_result(status, &output)?;
    parse_local_policy_output(&output.stdout)
}

fn spawn(trusted: &TrustedPowerShell) -> Result<Child> {
    let mut command = Command::new(trusted.executable());
    command
        .args(wire::POWERSHELL_ARGUMENTS)
        .env_clear()
        .env(SYSTEM_ROOT_ENV, trusted.system_root())
        .env(PS_MODULE_PATH_ENV, trusted.module_path())
        .current_dir(trusted.system_directory())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW);
    command
        .spawn()
        .map_err(|error| AppGameWindowsLocalPolicyError::ProcessSpawn(os_error_code(&error)))
}

fn wait_until(child: &mut Child, deadline: Instant) -> Result<ExitStatus> {
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| AppGameWindowsLocalPolicyError::ProcessReap(os_error_code(&error)))?
        {
            return Ok(status);
        }
        if Instant::now() >= deadline {
            terminate_and_reap(child)?;
            return Err(AppGameWindowsLocalPolicyError::ProcessTimeout);
        }
        std::thread::sleep(PROCESS_POLL_INTERVAL);
    }
}

fn terminate_and_reap(child: &mut Child) -> Result<()> {
    let kill_result = child.kill();
    let reap_result = child.wait();
    if let Err(error) = kill_result {
        return Err(AppGameWindowsLocalPolicyError::ProcessKill(os_error_code(
            &error,
        )));
    }
    reap_result
        .map(|_| ())
        .map_err(|error| AppGameWindowsLocalPolicyError::ProcessReap(os_error_code(&error)))
}

fn validate_process_result(status: ExitStatus, output: &ProcessOutput) -> Result<()> {
    if !status.success() {
        return Err(AppGameWindowsLocalPolicyError::ProcessFailed(
            status.code().unwrap_or(-1),
        ));
    }
    if !output.stderr.is_empty() {
        return Err(AppGameWindowsLocalPolicyError::UnexpectedStandardError);
    }
    Ok(())
}

fn verify_process_image(child: &Child, expected: &Path) -> Result<()> {
    let observed = query_process_image(child.as_raw_handle() as HANDLE)?;
    if observed.matches(expected) {
        return Ok(());
    }
    Err(AppGameWindowsLocalPolicyError::ProcessIdentityMismatch)
}

fn query_process_image(process: HANDLE) -> Result<ProcessImagePath> {
    let mut buffer = vec![0u16; MAX_PROCESS_PATH_CHARS];
    let mut length = u32::try_from(buffer.len())
        .map_err(|_size_error| AppGameWindowsLocalPolicyError::ProcessIdentityMismatch)?;
    let ok = unsafe { QueryFullProcessImageNameW(process, 0, buffer.as_mut_ptr(), &mut length) };
    if ok == 0 {
        return Err(AppGameWindowsLocalPolicyError::WindowsApi(unsafe {
            GetLastError()
        }));
    }
    let length = usize::try_from(length)
        .map_err(|_size_error| AppGameWindowsLocalPolicyError::ProcessIdentityMismatch)?;
    if length == 0 || length > buffer.len() {
        return Err(AppGameWindowsLocalPolicyError::ProcessIdentityMismatch);
    }
    Ok(ProcessImagePath(PathBuf::from(OsString::from_wide(
        &buffer[..length],
    ))))
}

fn os_error_code(error: &std::io::Error) -> u32 {
    error
        .raw_os_error()
        .and_then(|value| u32::try_from(value).ok())
        .unwrap_or(0)
}
