//! Safe parent-process custody for the native transport boundary.
//!
//! A pipe PID is only a lookup value and is vulnerable to PID reuse. The
//! provider must retain this observation before accepting a listener, then
//! bind the pipe to the retained process handle and image identity. No raw
//! process or pipe handle leaves this module.

use crate::error::ArtifactError;

/// Failure classes for binding an accepted stream to the retained parent.
///
/// Only `PeerRejected` means that the stream was proven to be a different
/// process. The other variants require the transport owner to terminate or
/// surface the failure instead of retrying an unclassified native error.
#[derive(Debug)]
pub enum NamedPipeBindError {
    PeerRejected,
    Parent(ArtifactError),
    Transport(ArtifactError),
}

#[cfg(windows)]
#[path = "parent_process.rs"]
mod windows;
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParentProcessIdentity {
    pid: u32,
    creation_time_100ns: u64,
    image_volume_serial_number: u64,
    image_file_id: [u8; 16],
}

#[cfg(windows)]
impl ParentProcessIdentity {
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn creation_time_100ns(&self) -> u64 {
        self.creation_time_100ns
    }

    pub fn image_volume_serial_number(&self) -> u64 {
        self.image_volume_serial_number
    }

    pub fn image_file_id(&self) -> [u8; 16] {
        self.image_file_id
    }
}

#[cfg(windows)]
pub struct ParentProcessObservation {
    inner: windows::ParentProcessObservation,
}

#[cfg(windows)]
impl ParentProcessObservation {
    pub fn open(pid: u32) -> Result<Self, ArtifactError> {
        windows::ParentProcessObservation::open(pid).map(|inner| Self { inner })
    }

    pub fn identity(&self) -> ParentProcessIdentity {
        identity_from_windows(self.inner.identity())
    }

    pub fn is_alive(&self) -> Result<bool, ArtifactError> {
        self.inner.is_alive()
    }

    pub fn current(&self) -> Result<ParentProcessIdentity, ArtifactError> {
        self.inner.current().map(identity_from_windows)
    }

    pub fn bind_named_pipe_client<'a, S>(
        &'a self,
        stream: &'a S,
    ) -> Result<NamedPipeClient<'a>, NamedPipeBindError>
    where
        S: std::os::windows::io::AsRawHandle,
    {
        self.inner
            .bind_named_pipe_client(stream)
            .map(|inner| NamedPipeClient { inner })
    }
}

#[cfg(windows)]
pub struct NamedPipeClient<'a> {
    inner: windows::NamedPipeClient<'a>,
}

#[cfg(windows)]
impl NamedPipeClient<'_> {
    pub fn client_pid(&self) -> u32 {
        self.inner.client_pid()
    }

    pub fn parent_identity(&self) -> ParentProcessIdentity {
        identity_from_windows(self.inner.parent_identity())
    }

    pub fn verify_current(&self) -> Result<(), ArtifactError> {
        self.inner.verify_current()
    }
}

#[cfg(windows)]
fn identity_from_windows(identity: windows::ParentProcessIdentity) -> ParentProcessIdentity {
    ParentProcessIdentity {
        pid: identity.pid(),
        creation_time_100ns: identity.creation_time_100ns(),
        image_volume_serial_number: identity.image_volume_serial_number(),
        image_file_id: identity.image_file_id(),
    }
}

#[cfg(not(windows))]
use std::marker::PhantomData;

#[cfg(not(windows))]
pub struct ParentProcessObservation;

#[cfg(not(windows))]
pub struct NamedPipeClient<'a> {
    _marker: PhantomData<&'a ()>,
}

#[cfg(not(windows))]
impl ParentProcessObservation {
    pub fn open(_pid: u32) -> Result<Self, ArtifactError> {
        Err(ArtifactError::UnsupportedPlatform)
    }
}
