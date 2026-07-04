use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateRequest;
use tokio::sync::Mutex as AsyncMutex;

#[path = "browser_policy_runtime_flow.rs"]
mod browser_policy_runtime_flow;

use crate::browser_policy_store::browser_policy_store_path_from_env;

#[derive(Clone, Debug)]
pub(crate) struct BrowserPolicyRuntime {
    persistence: BrowserPolicyPersistence,
    io_lock: Arc<AsyncMutex<()>>,
}

#[derive(Clone, Debug)]
enum BrowserPolicyPersistence {
    LocalJson(PathBuf),
}

impl BrowserPolicyRuntime {
    pub(crate) fn from_env() -> Self {
        Self::for_store_path(browser_policy_store_path_from_env().0)
    }

    pub(crate) fn for_store_path(path: impl AsRef<Path>) -> Self {
        Self {
            persistence: BrowserPolicyPersistence::LocalJson(path.as_ref().to_path_buf()),
            io_lock: Arc::new(AsyncMutex::new(())),
        }
    }

    pub(crate) async fn handle_request(
        &self,
        request: BrowserPolicyUpdateRequest,
    ) -> ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse {
        let _guard = self.io_lock.lock().await;
        browser_policy_runtime_flow::handle_browser_policy_update_request(self, request).await
    }
}
