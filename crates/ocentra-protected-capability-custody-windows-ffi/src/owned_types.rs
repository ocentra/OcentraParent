//! Opaque owned wrapper types shared by platform implementations.

#[cfg(not(windows))]
use core::marker::PhantomData;

#[cfg(windows)]
use crate::windows::handles::{
    ProcessInner, RegistryChainInner, ScManagerInner, ServiceInner, TbsContextInner, TokenInner,
};

#[cfg(windows)]
pub struct OwnedRegistryChain {
    pub(crate) inner: RegistryChainInner,
}

#[cfg(not(windows))]
pub struct OwnedRegistryChain {
    pub(crate) _marker: PhantomData<*mut ()>,
}

#[cfg(windows)]
pub struct OwnedProcess {
    pub(crate) inner: ProcessInner,
}

#[cfg(not(windows))]
pub struct OwnedProcess {
    pub(crate) _marker: PhantomData<*mut ()>,
}

#[cfg(windows)]
pub struct OwnedToken {
    pub(crate) inner: TokenInner,
}

#[cfg(not(windows))]
pub struct OwnedToken {
    pub(crate) _marker: PhantomData<*mut ()>,
}

#[cfg(windows)]
pub struct OwnedScManager {
    pub(crate) inner: ScManagerInner,
}

#[cfg(not(windows))]
pub struct OwnedScManager {
    pub(crate) _marker: PhantomData<*mut ()>,
}

#[cfg(windows)]
pub struct OwnedService {
    pub(crate) inner: ServiceInner,
    pub(crate) service_name: crate::WindowsText,
}

#[cfg(not(windows))]
pub struct OwnedService {
    pub(crate) _marker: PhantomData<*mut ()>,
}

#[cfg(windows)]
pub struct OwnedTbsContext {
    pub(crate) inner: TbsContextInner,
}

#[cfg(not(windows))]
pub struct OwnedTbsContext {
    pub(crate) _marker: PhantomData<*mut ()>,
}

pub struct OwnedTpmNvIndex {
    #[cfg(windows)]
    pub(crate) context: OwnedTbsContext,
    pub(crate) index: crate::TpmNvIndex,
    #[cfg(not(windows))]
    pub(crate) _marker: PhantomData<*mut ()>,
}

impl OwnedTpmNvIndex {
    pub fn index(&self) -> crate::TpmNvIndex {
        self.index
    }
}

impl crate::TpmNvIndex {
    pub fn try_from_raw(value: u32) -> crate::Result<Self> {
        const NV_HANDLE_PREFIX: u32 = 0x0100_0000;
        const HANDLE_TYPE_MASK: u32 = 0xff00_0000;
        if value & HANDLE_TYPE_MASK != NV_HANDLE_PREFIX {
            return Err(crate::Error::InvalidInput(
                crate::InputFault::TpmNvIndexInvalid,
            ));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub(crate) fn raw(self) -> u32 {
        self.0
    }
}
