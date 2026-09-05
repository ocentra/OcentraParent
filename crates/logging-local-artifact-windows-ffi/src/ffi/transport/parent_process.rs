use std::fs::File;
use std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle};

use windows_sys::Win32::Foundation::{HANDLE, STILL_ACTIVE};
use windows_sys::Win32::Storage::FileSystem::{GetFileType, FILE_TYPE_PIPE};
use windows_sys::Win32::System::Pipes::GetNamedPipeClientProcessId;
use windows_sys::Win32::System::Threading::{
    GetExitCodeProcess, GetProcessId, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    PROCESS_SYNCHRONIZE,
};

use super::{ArtifactError, NamedPipeBindError};
use crate::constants::{PARENT_PID_NONZERO, TRANSPORT_NOT_NAMED_PIPE};
use crate::platform::windows::OwnedFile;

#[path = "parent_process_image.rs"]
mod image;

/// The process identity retained before a provider accepts its pipe.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParentProcessIdentity {
    pid: u32,
    creation_time_100ns: u64,
    image_volume_serial_number: u64,
    image_file_id: [u8; 16],
}

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

/// A retained process handle plus the image identity observed through it.
pub struct ParentProcessObservation {
    process: File,
    image_file: OwnedFile,
    identity: ParentProcessIdentity,
}

impl ParentProcessObservation {
    /// Open and observe the process before the listener is made visible.
    pub fn open(pid: u32) -> Result<Self, ArtifactError> {
        if pid == 0 {
            return Err(ArtifactError::InvalidPath(PARENT_PID_NONZERO));
        }
        let handle = unsafe {
            OpenProcess(
                PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_SYNCHRONIZE,
                0,
                pid,
            )
        };
        if handle.is_null() {
            return Err(ArtifactError::OwnershipChanged);
        }
        let process = unsafe { File::from_raw_handle(handle as RawHandle) };
        let actual_pid = unsafe { GetProcessId(process.as_raw_handle() as HANDLE) };
        if actual_pid != pid {
            return Err(ArtifactError::OwnershipChanged);
        }
        let creation_time_100ns = image::query_creation_time(&process)?;
        let image_path = image::query_image_path(&process)?;
        let image_file = OwnedFile::open_process_image_file(&image_path)?;
        let image_metadata = image_file.process_image_metadata()?;
        let identity = ParentProcessIdentity {
            pid,
            creation_time_100ns,
            image_volume_serial_number: image_metadata.identity.volume_serial_number,
            image_file_id: image_metadata.identity.file_id,
        };
        Ok(Self {
            process,
            image_file,
            identity,
        })
    }

    pub fn identity(&self) -> ParentProcessIdentity {
        self.identity
    }

    pub fn is_alive(&self) -> Result<bool, ArtifactError> {
        let mut exit_code = 0u32;
        if unsafe { GetExitCodeProcess(self.process.as_raw_handle() as HANDLE, &mut exit_code) }
            == 0
        {
            return Err(ArtifactError::OwnershipChanged);
        }
        Ok(exit_code == STILL_ACTIVE as u32)
    }

    /// Reobserve PID, creation time, and image identity through the
    /// retained process handle.
    pub fn current(&self) -> Result<ParentProcessIdentity, ArtifactError> {
        let alive = self.is_alive()?;
        if !alive {
            return Err(ArtifactError::OwnershipChanged);
        }
        let pid = unsafe { GetProcessId(self.process.as_raw_handle() as HANDLE) };
        let creation_time_100ns = image::query_creation_time(&self.process)?;
        let image_metadata = self.image_file.process_image_metadata()?;
        let current = ParentProcessIdentity {
            pid,
            creation_time_100ns,
            image_volume_serial_number: image_metadata.identity.volume_serial_number,
            image_file_id: image_metadata.identity.file_id,
        };
        if current != self.identity {
            return Err(ArtifactError::OwnershipChanged);
        }
        Ok(current)
    }

    pub fn bind_named_pipe_client<'a, S>(
        &'a self,
        stream: &'a S,
    ) -> Result<NamedPipeClient<'a>, NamedPipeBindError>
    where
        S: AsRawHandle,
    {
        self.current().map_err(NamedPipeBindError::Parent)?;
        let handle = stream.as_raw_handle() as HANDLE;
        let file_type = unsafe { GetFileType(handle) };
        if file_type == 0 {
            return Err(NamedPipeBindError::Transport(ArtifactError::Io(
                std::io::Error::last_os_error().to_string(),
            )));
        }
        if file_type != FILE_TYPE_PIPE {
            return Err(NamedPipeBindError::Transport(ArtifactError::InvalidPath(
                TRANSPORT_NOT_NAMED_PIPE,
            )));
        }
        let mut client_pid = 0u32;
        if unsafe { GetNamedPipeClientProcessId(handle, &mut client_pid) } == 0 {
            return Err(NamedPipeBindError::Transport(ArtifactError::Io(
                std::io::Error::last_os_error().to_string(),
            )));
        }
        if client_pid != self.identity.pid {
            return Err(NamedPipeBindError::PeerRejected);
        }
        self.current().map_err(NamedPipeBindError::Parent)?;
        Ok(NamedPipeClient {
            stream: stream as &dyn AsRawHandle,
            parent: self,
            client_pid,
        })
    }
}

/// A lifetime-bound proof that one borrowed pipe is connected to the
/// retained parent process observation.
pub struct NamedPipeClient<'a> {
    stream: &'a dyn AsRawHandle,
    parent: &'a ParentProcessObservation,
    client_pid: u32,
}

impl NamedPipeClient<'_> {
    pub fn client_pid(&self) -> u32 {
        self.client_pid
    }

    pub fn parent_identity(&self) -> ParentProcessIdentity {
        self.parent.identity()
    }

    pub fn verify_current(&self) -> Result<(), ArtifactError> {
        self.parent.current()?;
        let handle = self.stream.as_raw_handle() as HANDLE;
        let mut client_pid = 0u32;
        if unsafe { GetNamedPipeClientProcessId(handle, &mut client_pid) } == 0
            || client_pid != self.client_pid
        {
            return Err(ArtifactError::OwnershipChanged);
        }
        Ok(())
    }
}
