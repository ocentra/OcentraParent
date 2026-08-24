use std::io;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

use interprocess::os::windows::named_pipe::{pipe_mode, PipeListener, PipeListenerOptions};
use interprocess::os::windows::security_descriptor::SecurityDescriptor;
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;
use windows_service::service::{
    ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::ServiceStatusHandle;

use crate::custody::BrokerCustodyService;
use crate::BrokerError;

type PipeListenerType = PipeListener<pipe_mode::Bytes, pipe_mode::Bytes>;

pub(super) fn run() -> Result<(), BrokerError> {
    let stopping = Arc::new(AtomicBool::new(false));
    let status_handle = super::service_control::register(&stopping)?;
    set_status(
        &status_handle,
        ServiceState::StartPending,
        ServiceControlAccept::empty(),
        ServiceExitCode::NO_ERROR,
    )?;
    let result = run_registered(&status_handle, &stopping);
    match result {
        Ok(()) => set_status(
            &status_handle,
            ServiceState::Stopped,
            ServiceControlAccept::empty(),
            ServiceExitCode::NO_ERROR,
        ),
        Err(error) => {
            // SCM is the service's production diagnostic boundary. Publish a
            // terminal non-zero state even when startup fails before a pipe is
            // created, then return the original error to the dispatcher path.
            let _status_result = set_status(
                &status_handle,
                ServiceState::Stopped,
                ServiceControlAccept::empty(),
                super::service_status::exit_code(&error),
            );
            Err(error)
        }
    }
}

fn run_registered(
    status_handle: &ServiceStatusHandle,
    stopping: &AtomicBool,
) -> Result<(), BrokerError> {
    // Capability-only preflight must fail before broker admission selects the
    // storage path or creates SQLite, its journal, or the writer lock.
    BrokerCustodyService::preflight_service_start()?;
    let custody = BrokerCustodyService::open();
    // Do not report Running or publish a pipe endpoint while the required
    // process/token admission adapter is unavailable. A transport listener
    // with a fail-closed peer path would still misrepresent service health.
    custody.peer_admission_available()?;
    let sddl = custody.broker_pipe_sddl()?;
    let listener = create_listener(&sddl)?;
    set_status(
        status_handle,
        ServiceState::Running,
        ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
        ServiceExitCode::NO_ERROR,
    )?;

    serve_until_stopped(&listener, stopping, &custody)?;
    set_status(
        status_handle,
        ServiceState::StopPending,
        ServiceControlAccept::empty(),
        ServiceExitCode::NO_ERROR,
    )
}

fn create_listener(
    sddl: &crate::custody::BrokerPipeSecurityDescriptor,
) -> Result<PipeListenerType, BrokerError> {
    let descriptor = SecurityDescriptor::deserialize(&sddl.0).map_err(map_transport_error)?;
    PipeListenerOptions::new()
        .path(BrokerPipeName::fixed().as_path())
        .nonblocking(true)
        .accept_remote(false)
        .inheritable(false)
        .security_descriptor(Some(descriptor))
        .create_duplex::<pipe_mode::Bytes>()
        .map_err(map_transport_error)
}

fn serve_until_stopped(
    listener: &PipeListenerType,
    stopping: &AtomicBool,
    custody: &BrokerCustodyService,
) -> Result<(), BrokerError> {
    while !stopping.load(Ordering::Acquire) {
        let Some(mut stream) = super::service_accept::accept_until(listener, stopping)? else {
            continue;
        };
        stream.set_nonblocking(true).map_err(map_transport_error)?;
        let deadline = super::service_accept::connection_deadline()?;
        // A malformed or abandoned peer is isolated to this connection. The
        // SCM service remains available for later authenticated clients.
        let _ = super::peer::serve(&mut stream, deadline, custody);
    }
    Ok(())
}

fn set_status(
    status_handle: &ServiceStatusHandle,
    state: ServiceState,
    controls_accepted: ServiceControlAccept,
    exit_code: ServiceExitCode,
) -> Result<(), BrokerError> {
    status_handle
        .set_service_status(ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: state,
            controls_accepted,
            exit_code,
            checkpoint: 0,
            wait_hint: Duration::from_secs(5),
            process_id: None,
        })
        .map_err(map_service_error)
}

fn map_service_error<E>(_error: E) -> BrokerError {
    BrokerError::Transport
}

fn map_transport_error(_error: io::Error) -> BrokerError {
    BrokerError::Transport
}
