use super::super::receive::is_running;
use super::PassiveDiscoveryListenerRuntime;

impl PassiveDiscoveryListenerRuntime {
    pub(in super::super) fn prepare_startup(&mut self) -> bool {
        let ready_to_bind = self
            .listener_state
            .upgrade()
            .is_some_and(|listener_state| is_running(&listener_state));
        if !ready_to_bind {
            return false;
        }
        self.bind_due_listeners();
        self.persist_capability();
        self.active_listener_count() > 0
    }
}
