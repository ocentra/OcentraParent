use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;
use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationOwner;

use crate::operations::{self, LeaseState, ProviderError};
use crate::protocol::{self, ReadyFrame};
use crate::transport::{self, TransportError};

#[path = "runtime/arguments.rs"]
mod arguments;
#[path = "runtime/connection.rs"]
mod connection;

#[derive(Debug)]
pub(crate) enum RuntimeError {
    Arguments,
    Startup,
    Protocol,
    Provider(ProviderError),
    Transport(TransportError),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Provider(error) => std::fmt::Debug::fmt(error, formatter),
            _ => std::fmt::Debug::fmt(self, formatter),
        }
    }
}

impl std::error::Error for RuntimeError {}

pub(crate) enum ConnectionDisposition {
    Close,
    Shutdown(Vec<u8>),
}

pub(crate) fn run() -> Result<(), RuntimeError> {
    let arguments = arguments::parse_arguments()?;
    let parent = ParentProcessObservation::open(arguments.parent_pid)
        .map_err(|_error| RuntimeError::Startup)?;
    parent.current().map_err(|_error| RuntimeError::Startup)?;

    let binary_sha256 = arguments::hash_current_executable()?;
    let root = std::path::PathBuf::from(arguments.root);
    let owner = LocalArtifactMutationOwner::open(&root)
        .map_err(|error| RuntimeError::Provider(operations::map_owner_error(&error)))?;
    let mut session = owner
        .session()
        .map_err(|error| RuntimeError::Provider(operations::map_owner_error(&error)))?;
    let provider_instance_id = operations::random_identifier()
        .map_err(RuntimeError::Provider)?
        .into_text();
    let root_stat = session
        .stat(&protocol::text::TextId::Empty.text())
        .map_err(|error| RuntimeError::Provider(operations::map_owner_error(&error)))?
        .ok_or_else(|| {
            RuntimeError::Provider(ProviderError::new(
                protocol::text::ROOT_IDENTITY_CHANGED,
                operations::FailureDisposition::Terminate,
            ))
        })?;
    if !root_stat.is_directory() {
        return Err(RuntimeError::Provider(ProviderError::new(
            protocol::text::ROOT_NOT_DIRECTORY,
            operations::FailureDisposition::Terminate,
        )));
    }
    let ready = ReadyFrame {
        protocol_version: protocol::PROTOCOL_VERSION,
        provider_instance_id,
        binary_sha256: binary_sha256.into(),
        root_identity: operations::wire_identity(root_stat.identity()),
    };

    // The listener is not visible until the native session has acquired its
    // durable lock and completed startup recovery.
    let listener = transport::bind(&arguments.pipe_name).map_err(RuntimeError::Transport)?;
    let mut lease = LeaseState::default();
    loop {
        let mut stream = transport::accept(&listener, &parent).map_err(RuntimeError::Transport)?;
        match connection::serve_connection(&parent, &mut stream, &mut session, &mut lease, &ready)?
        {
            ConnectionDisposition::Close => {}
            ConnectionDisposition::Shutdown(response) => {
                drop(session);
                drop(owner);
                transport::write_frame(&parent, &mut stream, &response)
                    .map_err(RuntimeError::Transport)?;
                return Ok(());
            }
        }
    }
}
