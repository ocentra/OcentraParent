use std::io::{self, Read};

use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;

use super::super::super::{endpoint, PipeStream, TransportError};

pub(super) fn read<const N: usize>(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    prefix: &mut [u8; N],
) -> Result<(), TransportError> {
    let mut offset = 0_usize;
    while offset < prefix.len() {
        endpoint::verify_client(parent, stream)?;
        match stream.read(&mut prefix[offset..]) {
            Ok(0) => return Err(TransportError::Io),
            Ok(read) => offset += read,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(super::super::POLL_INTERVAL);
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => return Err(TransportError::Io),
        }
    }
    Ok(())
}
