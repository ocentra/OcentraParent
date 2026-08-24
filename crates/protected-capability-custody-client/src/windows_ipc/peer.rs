use std::io as std_io;

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream};
use ocentra_protected_capability_custody_core::broker_admission::BrokerExecutableGuard;
use ocentra_protected_capability_custody_protocol::handshake::{
    UntrustedBrokerHello, UntrustedClientHello,
};
use sysinfo::{Pid, System};

use super::io;
use crate::ClientError;

type PipeStream = DuplexPipeStream<pipe_mode::Bytes>;

#[derive(Clone, Copy)]
pub(super) struct ProcessIdentity {
    pub(super) process_id: u32,
    pub(super) process_epoch: u64,
    pub(super) session_id: u32,
}

pub(super) fn current_process_identity() -> Result<ProcessIdentity, ClientError> {
    let identity = process_identity(std::process::id())?;
    if identity.session_id == 0 {
        return Err(ClientError::PeerAuthentication);
    }
    Ok(identity)
}

fn process_identity(process_id: u32) -> Result<ProcessIdentity, ClientError> {
    if process_id == 0 {
        return Err(ClientError::PeerAuthentication);
    }
    let system = System::new_all();
    let process = system
        .process(Pid::from_u32(process_id))
        .ok_or(ClientError::PeerAuthentication)?;
    let process_epoch = process.start_time();
    let session_id = process
        .session_id()
        .map(Pid::as_u32)
        .ok_or(ClientError::PeerAuthentication)?;
    if process_epoch == 0 {
        return Err(ClientError::PeerAuthentication);
    }
    Ok(ProcessIdentity {
        process_id,
        process_epoch,
        session_id,
    })
}

pub(super) fn authenticate_pipe_server(
    stream: &PipeStream,
    broker_executable: &BrokerExecutableGuard,
) -> Result<(), ClientError> {
    let server_process_id = stream
        .server_process_id()
        .map_err(io::map_transport_error)?;
    let server_session_id = stream
        .server_session_id()
        .map_err(io::map_transport_error)?;
    let observed = process_identity(server_process_id)?;
    if observed.session_id != server_session_id
        || !process_executable_matches(server_process_id, broker_executable)?
    {
        return Err(ClientError::PeerAuthentication);
    }
    Ok(())
}

pub(super) fn authenticate_broker_hello(
    stream: &PipeStream,
    broker_executable: &BrokerExecutableGuard,
    client_hello: &UntrustedClientHello,
    broker_hello: &UntrustedBrokerHello,
) -> Result<(), ClientError> {
    let server_process_id = stream
        .server_process_id()
        .map_err(io::map_transport_error)?;
    let server_session_id = stream
        .server_session_id()
        .map_err(io::map_transport_error)?;
    let observed = process_identity(server_process_id)?;
    if !broker_hello.matches_client(client_hello)
        || broker_hello.broker_process_id() != server_process_id
        || broker_hello.broker_session_id() != server_session_id
        || broker_hello.broker_epoch() != observed.process_epoch
        || broker_hello.broker_session_id() != observed.session_id
        || !process_executable_matches(server_process_id, broker_executable)?
    {
        return Err(ClientError::PeerAuthentication);
    }
    Ok(())
}

fn process_executable_matches(
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

fn map_broker_path_error(_error: std_io::Error) -> ClientError {
    ClientError::BrokerUnavailable
}
