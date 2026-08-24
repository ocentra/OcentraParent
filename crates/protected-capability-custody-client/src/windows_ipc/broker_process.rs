use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};

use ocentra_protected_capability_custody_core::broker_admission::BrokerExecutableGuard;
use ocentra_protected_capability_custody_protocol::constants::BROKER_PIPE_ARGUMENT;
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;
use sysinfo::{Pid, System};

use crate::ClientError;

pub(super) struct BrokerChild {
    process: Child,
    reaped: bool,
}

impl BrokerChild {
    fn new(process: Child) -> Self {
        Self {
            process,
            reaped: false,
        }
    }

    pub(super) fn id(&self) -> u32 {
        self.process.id()
    }

    pub(super) fn take_stdin(&mut self) -> Result<ChildStdin, ClientError> {
        self.process
            .stdin
            .take()
            .ok_or(ClientError::BrokerUnavailable)
    }

    pub(super) fn wait(&mut self) -> Result<ExitStatus, std::io::Error> {
        let status = self.process.wait()?;
        self.reaped = true;
        Ok(status)
    }
}

impl Drop for BrokerChild {
    fn drop(&mut self) {
        if !self.reaped {
            drop(self.process.kill());
            drop(self.process.wait());
        }
    }
}

pub(super) fn fixed_broker_path() -> Result<BrokerExecutableGuard, ClientError> {
    BrokerExecutableGuard::open_client_sibling().map_err(map_broker_guard_error)
}

pub(super) fn spawn_broker(
    broker_path: &BrokerExecutableGuard,
    pipe_name: &BrokerPipeName,
) -> Result<BrokerChild, ClientError> {
    Command::new(broker_path.path())
        .arg(BROKER_PIPE_ARGUMENT)
        .arg(pipe_name.as_os_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(BrokerChild::new)
        .map_err(map_broker_spawn_error)
}

pub(super) fn process_executable_matches(
    process_id: u32,
    expected: &BrokerExecutableGuard,
) -> Result<bool, ClientError> {
    expected
        .revalidate()
        .map_err(|_| ClientError::PeerAuthentication)?;
    let system = System::new_all();
    let observed = system
        .process(Pid::from_u32(process_id))
        .and_then(sysinfo::Process::exe)
        .ok_or(ClientError::PeerAuthentication)?;
    let observed = dunce::canonicalize(observed).map_err(map_broker_path_error)?;
    Ok(observed == expected.path())
}

fn map_broker_guard_error(
    _error: ocentra_protected_capability_custody_core::broker_admission::BrokerRuntimeError,
) -> ClientError {
    ClientError::BrokerUnavailable
}

fn map_broker_path_error(_error: std::io::Error) -> ClientError {
    ClientError::BrokerUnavailable
}

fn map_broker_spawn_error(_error: std::io::Error) -> ClientError {
    ClientError::BrokerUnavailable
}
