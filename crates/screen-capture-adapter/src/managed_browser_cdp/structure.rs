#[path = "chunks.rs"]
mod chunks;

use super::ManagedBrowserCdpScreenCaptureError;

const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

pub(super) fn validate_png_structure(
    bytes: &[u8],
) -> Result<(u32, u32), ManagedBrowserCdpScreenCaptureError> {
    if bytes.len() > super::super::max_png_bytes() || bytes.len() < PNG_SIGNATURE.len() {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }
    if bytes.get(..PNG_SIGNATURE.len()) != Some(PNG_SIGNATURE.as_slice()) {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }

    let mut offset = PNG_SIGNATURE.len();
    let mut state = chunks::StructureState::default();
    let mut dimensions = None;
    while offset < bytes.len() {
        let chunk = chunks::read(bytes, offset)?;
        chunks::validate_order(&mut state, &chunk)?;
        if chunk.kind == b"IHDR" {
            dimensions = Some(ihdr_dimensions(chunk.data)?);
        }
        offset = chunk.end;
        if chunk.kind == b"IEND" {
            if chunk.data.is_empty() && state.seen_idat && offset == bytes.len() {
                return dimensions.ok_or(ManagedBrowserCdpScreenCaptureError::InvalidPng);
            }
            return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
        }
    }
    Err(ManagedBrowserCdpScreenCaptureError::InvalidPng)
}

fn ihdr_dimensions(data: &[u8]) -> Result<(u32, u32), ManagedBrowserCdpScreenCaptureError> {
    if data.len() != 13 {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }
    let width = u32::from_be_bytes(data[0..4].try_into().unwrap());
    let height = u32::from_be_bytes(data[4..8].try_into().unwrap());
    super::super::validate_dimensions(width, height)?;
    Ok((width, height))
}
