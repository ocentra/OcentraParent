use super::AuthenticatedAdapterExecutionTrace;

impl AuthenticatedAdapterExecutionTrace {
    pub fn process_id(&self) -> u32 {
        self.process_id
    }

    pub fn expected_process_name(&self) -> &str {
        &self.expected_process_name
    }

    pub fn expected_executable_path(&self) -> &str {
        &self.expected_executable_path_ref
    }

    pub fn process_start_time(&self) -> u64 {
        self.process_start_time
    }

    pub fn managed_process_identity(&self) -> &str {
        &self.managed_process_identity
    }

    pub fn observed_process_id(&self) -> Option<u32> {
        self.observed_process_id
    }

    pub fn observed_process_name(&self) -> Option<&str> {
        self.observed_process_name.as_deref()
    }

    pub fn observed_executable_path(&self) -> Option<&str> {
        self.observed_executable_path_ref.as_deref()
    }

    pub fn observed_process_start_time(&self) -> Option<u64> {
        self.observed_process_start_time
    }

    pub fn adapter_result(&self) -> &str {
        &self.adapter_result
    }

    pub fn adapter_status(&self) -> &str {
        &self.adapter_status
    }

    pub fn completed_at(&self) -> Option<&str> {
        self.completed_at.as_deref()
    }

    pub fn rollback_required(&self) -> bool {
        self.rollback_required
    }

    pub fn rollback_state(&self) -> &str {
        &self.rollback_state
    }
}
