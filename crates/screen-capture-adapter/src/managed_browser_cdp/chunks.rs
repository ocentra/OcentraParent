use crc32fast::Hasher;

use super::super::super::ManagedBrowserCdpScreenCaptureError;

#[derive(Default)]
pub(super) struct StructureState {
    pub(super) seen_ihdr: bool,
    pub(super) seen_idat: bool,
}

pub(super) struct Chunk<'a> {
    pub(super) kind: &'a [u8],
    pub(super) data: &'a [u8],
    pub(super) end: usize,
}

pub(super) fn read<'a>(
    bytes: &'a [u8],
    offset: usize,
) -> Result<Chunk<'a>, ManagedBrowserCdpScreenCaptureError> {
    let length_bytes = bytes
        .get(
            offset
                ..offset
                    .checked_add(4)
                    .ok_or(ManagedBrowserCdpScreenCaptureError::InvalidPng)?,
        )
        .ok_or(ManagedBrowserCdpScreenCaptureError::InvalidPng)?;
    let length_bytes: [u8; 4] = length_bytes
        .try_into()
        .map_err(|_error| ManagedBrowserCdpScreenCaptureError::InvalidPng)?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes))
        .map_err(|_error| ManagedBrowserCdpScreenCaptureError::InvalidPng)?;
    let end = offset
        .checked_add(12)
        .and_then(|value| value.checked_add(length))
        .ok_or(ManagedBrowserCdpScreenCaptureError::InvalidPng)?;
    if end > bytes.len() {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }
    let kind = &bytes[offset + 4..offset + 8];
    let data = &bytes[offset + 8..offset + 8 + length];
    let stored_crc = u32::from_be_bytes(
        bytes[offset + 8 + length..end]
            .try_into()
            .map_err(|_error| ManagedBrowserCdpScreenCaptureError::InvalidPng)?,
    );
    let mut crc = Hasher::new();
    crc.update(kind);
    crc.update(data);
    if crc.finalize() != stored_crc {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }
    Ok(Chunk { kind, data, end })
}

pub(super) fn validate_order(
    state: &mut StructureState,
    chunk: &Chunk<'_>,
) -> Result<(), ManagedBrowserCdpScreenCaptureError> {
    if chunk.kind == b"IHDR" {
        if state.seen_ihdr || chunk.data.len() != 13 || state.seen_idat {
            return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
        }
        state.seen_ihdr = true;
    } else if !state.seen_ihdr {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }
    if chunk.kind == b"IDAT" {
        state.seen_idat = true;
    }
    if chunk.kind == b"acTL" || chunk.kind == b"fcTL" || chunk.kind == b"fdAT" {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }
    Ok(())
}
