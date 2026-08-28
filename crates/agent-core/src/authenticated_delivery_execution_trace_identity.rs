use super::AuthenticatedAdapterExecutionTrace;

impl AuthenticatedAdapterExecutionTrace {
    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    pub fn grant_fingerprint(&self) -> &str {
        &self.grant_fingerprint
    }

    pub fn issuer_key_id(&self) -> &str {
        &self.issuer_key_id
    }

    pub fn nonce_digest(&self) -> &str {
        &self.nonce_digest
    }

    pub fn correlation_id(&self) -> &str {
        &self.correlation_id
    }

    pub fn issuer_actor_id(&self) -> &str {
        &self.issuer_actor_id
    }

    pub fn household_id(&self) -> &str {
        &self.household_id
    }

    pub fn parent_device_id(&self) -> &str {
        &self.parent_device_id
    }

    pub fn child_profile_id(&self) -> &str {
        &self.child_profile_id
    }

    pub fn target_device_id(&self) -> &str {
        &self.target_device_id
    }

    pub fn policy_decision_id(&self) -> &str {
        &self.policy_decision_id
    }

    pub fn policy_version(&self) -> &str {
        &self.policy_version
    }

    pub fn action_id(&self) -> &str {
        &self.action_id
    }

    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }
}
