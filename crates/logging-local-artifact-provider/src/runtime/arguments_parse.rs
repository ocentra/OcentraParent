use std::env;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

use super::{finish_arguments, Arguments, RootPath, RuntimeError};
use crate::protocol;
use crate::transport::endpoint::PipeName;

pub(super) fn parse_arguments() -> Result<Arguments, RuntimeError> {
    let mut arguments = env::args_os().skip(1);
    let mut argument_count = 0_usize;
    let mut pipe_name = None;
    let mut root = None;
    let mut parent_pid = None;

    while let Some(option) = arguments.next() {
        argument_count = argument_count.saturating_add(1);
        if argument_count > super::MAXIMUM_ARGUMENTS {
            return Err(RuntimeError::Arguments);
        }
        let option = option.to_str().ok_or(RuntimeError::Arguments)?;
        let value = arguments.next().ok_or(RuntimeError::Arguments)?;
        argument_count = argument_count.saturating_add(1);
        if argument_count > super::MAXIMUM_ARGUMENTS {
            return Err(RuntimeError::Arguments);
        }
        match protocol::text::TextId::argument(option) {
            Some(protocol::text::TextId::CliPipeName) => {
                set_pipe_name(&mut pipe_name, ArgumentValue(value))?
            }
            Some(protocol::text::TextId::CliRoot) => set_root(&mut root, ArgumentValue(value))?,
            Some(protocol::text::TextId::CliParentPid) => {
                set_parent_pid(&mut parent_pid, ArgumentValue(value))?
            }
            _ => return Err(RuntimeError::Arguments),
        }
    }

    finish_arguments(pipe_name, root, parent_pid)
}

/// A single opaque operating-system argument consumed by the startup boundary.
///
/// BRAND-INVARIANT: the value is consumed exactly once by the option-specific
/// parser and never escapes as an unvalidated path, identifier, or pid.
struct ArgumentValue(OsString);

fn reject_duplicate<T>(slot: &Option<T>) -> Result<(), RuntimeError> {
    if slot.is_some() {
        return Err(RuntimeError::Arguments);
    }
    Ok(())
}

fn set_pipe_name(slot: &mut Option<PipeName>, value: ArgumentValue) -> Result<(), RuntimeError> {
    reject_duplicate(slot)?;
    let value = value.0;
    *slot = Some(PipeName::parse(&value).map_err(|_error| RuntimeError::Arguments)?);
    Ok(())
}

fn set_root(slot: &mut Option<RootPath>, value: ArgumentValue) -> Result<(), RuntimeError> {
    reject_duplicate(slot)?;
    let value = value.0;
    let wide: Vec<u16> = value.encode_wide().collect();
    let path = PathBuf::from(&value);
    if wide.is_empty()
        || wide.len() > super::MAXIMUM_ROOT_PATH_UTF16
        || wide.contains(&0)
        || !path.is_absolute()
    {
        return Err(RuntimeError::Arguments);
    }
    *slot = Some(RootPath(path));
    Ok(())
}

fn set_parent_pid(slot: &mut Option<u32>, value: ArgumentValue) -> Result<(), RuntimeError> {
    reject_duplicate(slot)?;
    let value = value.0;
    let text = value.to_str().ok_or(RuntimeError::Arguments)?;
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(RuntimeError::Arguments);
    }
    let parsed = text
        .parse::<u32>()
        .map_err(|_error| RuntimeError::Arguments)?;
    if parsed == 0 {
        return Err(RuntimeError::Arguments);
    }
    *slot = Some(parsed);
    Ok(())
}
