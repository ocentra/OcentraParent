use std::ffi::OsStr;
use std::fmt;
use std::path::Path;

use crate::constants::{BROKER_PIPE_NAME, BROKER_PIPE_NAME_PREFIX, DEBUG_BROKER_PIPE_NAME};
use crate::types::{Nonce, ProtocolError};

pub struct BrokerPipeName(String);

impl BrokerPipeName {
    pub fn fixed() -> Self {
        Self(BROKER_PIPE_NAME.to_owned())
    }

    pub fn from_nonce(pipe_nonce: Nonce) -> Self {
        let mut value = String::with_capacity(BROKER_PIPE_NAME_PREFIX.len() + 64);
        value.push_str(BROKER_PIPE_NAME_PREFIX);
        for byte in pipe_nonce.as_bytes() {
            value.push(char::from_digit(u32::from(byte >> 4), 16).unwrap_or('0'));
            value.push(char::from_digit(u32::from(byte & 0x0f), 16).unwrap_or('0'));
        }
        Self(value)
    }

    pub fn try_from_untrusted(value: String) -> Result<Self, ProtocolError> {
        let Some(suffix) = value.strip_prefix(BROKER_PIPE_NAME_PREFIX) else {
            return Err(ProtocolError::InvalidDomain);
        };
        if suffix.len() != 64
            || !suffix
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        {
            return Err(ProtocolError::InvalidDomain);
        }
        Ok(Self(value))
    }

    pub fn as_os_str(&self) -> &OsStr {
        OsStr::new(&self.0)
    }

    pub fn as_path(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl PartialEq for BrokerPipeName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for BrokerPipeName {}

impl fmt::Debug for BrokerPipeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(DEBUG_BROKER_PIPE_NAME)
    }
}
