#[path = "arguments_parse.rs"]
mod parser;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use super::RuntimeError;
use crate::protocol;
use crate::transport::endpoint::PipeName;

const MAXIMUM_ARGUMENTS: usize = 6;
const MAXIMUM_ROOT_PATH_UTF16: usize = 32 * 1024;
const HASH_BUFFER_BYTES: usize = 1024 * 1024;

#[derive(Debug)]
pub(super) struct Arguments {
    pub(super) pipe_name: PipeName,
    pub(super) root: RootPath,
    pub(super) parent_pid: u32,
}

/// A validated absolute root path owned by the provider startup boundary.
///
/// BRAND-INVARIANT: the path is an absolute, NUL-free Windows path within the
/// startup UTF-16 length bound.
#[derive(Debug)]
pub(super) struct RootPath(PathBuf);

impl From<RootPath> for PathBuf {
    fn from(value: RootPath) -> Self {
        value.0
    }
}

/// The digest of the running provider executable.
///
/// BRAND-INVARIANT: the value is lowercase hexadecimal encoded SHA-256 output.
#[derive(Debug)]
pub(super) struct BinaryDigest(String);

impl From<BinaryDigest> for String {
    fn from(value: BinaryDigest) -> Self {
        value.0
    }
}

pub(super) fn parse_arguments() -> Result<Arguments, RuntimeError> {
    parser::parse_arguments()
}

pub(super) fn finish_arguments(
    pipe_name: Option<PipeName>,
    root: Option<RootPath>,
    parent_pid: Option<u32>,
) -> Result<Arguments, RuntimeError> {
    Ok(Arguments {
        pipe_name: pipe_name.ok_or(RuntimeError::Arguments)?,
        root: root.ok_or(RuntimeError::Arguments)?,
        parent_pid: parent_pid.ok_or(RuntimeError::Arguments)?,
    })
}

pub(super) fn hash_current_executable() -> Result<BinaryDigest, RuntimeError> {
    let path = env::current_exe().map_err(|_error| RuntimeError::Startup)?;
    let mut file = File::open(path).map_err(|_error| RuntimeError::Startup)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; HASH_BUFFER_BYTES];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|_error| RuntimeError::Startup)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    let digest = hasher.finalize();
    Ok(BinaryDigest(protocol::hex_encode(&digest)))
}
