//! Opaque owned wrapper types shared by platform implementations.

#[cfg(not(windows))]
use core::marker::PhantomData;

#[cfg(windows)]
use crate::windows::cng_handles::{PcpProviderInner, PcpSigningKeyInner};
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

#[cfg(windows)]
pub struct OwnedPcpProvider {
    pub(crate) inner: PcpProviderInner,
}

#[cfg(not(windows))]
pub struct OwnedPcpProvider {
    pub(crate) _marker: PhantomData<*mut ()>,
}

#[cfg(windows)]
pub struct OwnedPcpSigningKey {
    pub(crate) inner: PcpSigningKeyInner,
}

#[cfg(not(windows))]
pub struct OwnedPcpSigningKey {
    pub(crate) _marker: PhantomData<*mut ()>,
}

#[cfg(not(windows))]
pub struct OwnedTbsContext {
    pub(crate) _marker: PhantomData<*mut ()>,
}
