//! Private fixed TPM handles and operation codes.

use super::super::{
    FIXED_COUNTER_INDEX, TPM_CC_NV_INCREMENT, TPM_CC_NV_READ, TPM_HT_POLICY_SESSION,
    TPM_HT_TRANSIENT, TPM_RH_NULL,
};
use crate::{Error, Result};

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct NvIndex(u32);

impl NvIndex {
    pub(crate) const fn fixed_counter() -> Self {
        Self(FIXED_COUNTER_INDEX)
    }

    pub(crate) fn raw(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct PermanentHandle(u32);

impl PermanentHandle {
    pub(crate) const fn null() -> Self {
        Self(TPM_RH_NULL)
    }

    pub(crate) fn raw(self) -> u32 {
        self.0
    }
}

pub(crate) struct SessionHandle(u32);

impl SessionHandle {
    pub(crate) fn from_policy_response(raw: u32) -> Result<Self> {
        if raw & 0xff00_0000 != TPM_HT_POLICY_SESSION {
            return Err(Error::MalformedTpm);
        }
        Ok(Self(raw))
    }

    pub(crate) fn raw(&self) -> u32 {
        self.0
    }
}

pub(crate) struct TransientHandle(u32);

impl TransientHandle {
    pub(crate) fn from_response(raw: u32) -> Result<Self> {
        if raw & 0xff00_0000 != TPM_HT_TRANSIENT {
            return Err(Error::MalformedTpm);
        }
        Ok(Self(raw))
    }

    pub(crate) fn raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum FixedNvOperation {
    Read,
    Increment,
}

impl FixedNvOperation {
    pub(crate) fn command_code(self) -> u32 {
        match self {
            Self::Read => TPM_CC_NV_READ,
            Self::Increment => TPM_CC_NV_INCREMENT,
        }
    }
}
