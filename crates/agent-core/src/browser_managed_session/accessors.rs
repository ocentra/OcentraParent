use std::path::PathBuf;

use super::{BrowserManagedLaunch, ManagedBrowserLaunchAuthority};

impl BrowserManagedLaunch {
    pub fn process_id(&self) -> u32 {
        self.process_id
    }

    pub fn bridge_port(&self) -> u16 {
        self.bridge_port
    }

    pub fn browser_family(&self) -> ocentra_parent_agent_protocol::browser::BrowserFamily {
        self.browser_family
    }

    pub fn browser_channel(&self) -> ocentra_parent_agent_protocol::browser::BrowserChannel {
        self.browser_channel
    }

    pub fn profile_path_ref(&self) -> &str {
        &self.profile_path_ref
    }

    pub fn bridge_endpoint_ref(&self) -> &str {
        &self.bridge_endpoint_ref
    }

    pub(crate) fn cdp_authority(&self) -> &ManagedBrowserLaunchAuthority {
        &self.cdp_authority
    }
}

impl ManagedBrowserLaunchAuthority {
    pub(crate) fn managed_browser_session_id(&self) -> &str {
        &self.managed_browser_session_id
    }

    pub(crate) fn profile_id(&self) -> &str {
        &self.profile_id
    }

    pub(crate) fn process_id(&self) -> u32 {
        self.process_id
    }

    pub(crate) fn bridge_port(&self) -> u16 {
        self.bridge_port
    }

    pub(crate) fn browser_family(&self) -> ocentra_parent_agent_protocol::browser::BrowserFamily {
        self.browser_family
    }

    pub(crate) fn browser_channel(&self) -> ocentra_parent_agent_protocol::browser::BrowserChannel {
        self.browser_channel
    }

    pub(crate) fn executable_path(&self) -> &PathBuf {
        &self.executable_path
    }

    pub(crate) fn profile_path(&self) -> &PathBuf {
        &self.profile_path
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn created_at_epoch_ms(&self) -> u64 {
        self.created_at_epoch_ms
    }

    pub(crate) fn expires_at_epoch_ms(&self) -> u64 {
        self.expires_at_epoch_ms
    }
}
