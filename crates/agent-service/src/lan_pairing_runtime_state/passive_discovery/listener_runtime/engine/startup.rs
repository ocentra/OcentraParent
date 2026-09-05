use super::super::receive::is_running;
use super::PassiveDiscoveryListenerRuntime;

impl PassiveDiscoveryListenerRuntime {
    pub(in super::super) fn prepare_startup(&mut self) -> bool {
        let ready_to_bind = is_running(&self.runtime.passive_discovery_listener_state);
        if !ready_to_bind {
            return false;
        }
        self.bind_due_listeners();
        self.persist_capability();
        self.active_listener_count() > 0
    }
}
