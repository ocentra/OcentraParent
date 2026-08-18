use std::io::Cursor;

use png::{Decoder, Limits};

#[path = "structure.rs"]
mod structure;

use super::ManagedBrowserCdpScreenCaptureError;

pub(super) fn decode_png(bytes: &[u8]) -> Result<(u32, u32), ManagedBrowserCdpScreenCaptureError> {
    let declared_dimensions = structure::validate_png_structure(bytes)?;
    let decoder = Decoder::new_with_limits(
        Cursor::new(bytes),
        Limits {
            bytes: super::max_png_bytes(),
        },
    );
    let mut reader = decoder
        .read_info()
        .map_err(|_error| ManagedBrowserCdpScreenCaptureError::InvalidPng)?;
    let output_buffer_size = reader
        .output_buffer_size()
        .ok_or(ManagedBrowserCdpScreenCaptureError::InvalidPng)?;
    if u64::try_from(output_buffer_size).unwrap_or(u64::MAX) > super::max_decoded_bytes() {
        return Err(ManagedBrowserCdpScreenCaptureError::DimensionsOutOfBounds);
    }
    let mut decoded = vec![0; output_buffer_size];
    let output = reader
        .next_frame(&mut decoded)
        .map_err(|_error| ManagedBrowserCdpScreenCaptureError::InvalidPng)?;
    if output.width != declared_dimensions.0 || output.height != declared_dimensions.1 {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }
    super::validate_dimensions(output.width, output.height)?;
    Ok((output.width, output.height))
}
