use std::{fs, path::Path};

use ocentra_parent_agent_protocol::constants;

pub(crate) fn shortcut_target_from_path(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    browser_windows_shortcut_target_from_bytes(&bytes)
}

fn browser_windows_shortcut_target_from_bytes(bytes: &[u8]) -> Option<String> {
    if read_u32(bytes, 0)? != constants::browser::SHORTCUT_LINK_HEADER_SIZE {
        return None;
    }
    let link_flags = read_u32(bytes, constants::browser::SHORTCUT_LINK_FLAGS_OFFSET)?;
    if link_flags & constants::browser::SHORTCUT_LINK_FLAGS_HAS_LINK_INFO == 0 {
        return None;
    }
    link_info_target(bytes, constants::browser::SHORTCUT_LINK_INFO_SECTION_OFFSET)
}

pub(crate) fn link_info_target(bytes: &[u8], offset: usize) -> Option<String> {
    let size = read_u32(
        bytes,
        offset + constants::browser::SHORTCUT_LINK_INFO_SIZE_OFFSET,
    )? as usize;
    if size < constants::browser::SHORTCUT_LINK_INFO_MIN_SIZE {
        return None;
    }
    let end = offset.checked_add(size)?;
    if end > bytes.len() {
        return None;
    }
    let flags = read_u32(
        bytes,
        offset + constants::browser::SHORTCUT_LINK_INFO_FLAGS_OFFSET,
    )?;
    if flags & constants::browser::SHORTCUT_LINK_INFO_LOCAL_BASE_PATH_FLAG == 0 {
        return None;
    }
    let local_base_path_offset = read_u32(
        bytes,
        offset + constants::browser::SHORTCUT_LINK_INFO_LOCAL_BASE_PATH_OFFSET,
    )? as usize;
    let target_offset = offset.checked_add(local_base_path_offset)?;
    if target_offset >= end {
        return None;
    }
    read_null_terminated_ansi(&bytes[target_offset..end])
}

pub(crate) fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes(slice.try_into().ok()?))
}

pub(crate) fn read_null_terminated_ansi(bytes: &[u8]) -> Option<String> {
    let end = bytes.iter().position(|byte| *byte == 0)?;
    if end == 0 {
        return None;
    }
    String::from_utf8(bytes[..end].to_vec()).ok()
}
