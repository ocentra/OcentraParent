use ocentra_schema::authenticated_delivery_managed_process::AuthenticatedManagedProcessTargetBinding;

use super::{AuthenticatedOwnedProcessTerminationTarget, OwnedProcessTerminationTarget};

impl AuthenticatedOwnedProcessTerminationTarget {
    pub(super) fn from_local_binding(
        binding: &AuthenticatedManagedProcessTargetBinding,
        process_id: u32,
        process_name: String,
        executable_path_ref: String,
        process_start_time: u64,
    ) -> Self {
        Self {
            pid: process_id,
            expected_process_name: process_name,
            grant_fingerprint: binding.grant_fingerprint.clone(),
            issuer_key_id: binding.issuer_key_id.clone(),
            issuer_actor_id: binding.issuer_actor_id.clone(),
            household_id: binding.household_id.clone(),
            parent_device_id: binding.parent_device_id.clone(),
            child_profile_id: binding.child_profile_id.clone(),
            target_device_id: binding.target_device_id.clone(),
            policy_decision_id: binding.policy_decision_id.clone(),
            policy_version: binding.policy_version.clone(),
            action_id: binding.action_id.clone(),
            capability_id: binding.capability_id.clone(),
            managed_process_identity: binding.managed_process_identity.clone(),
            expected_executable_path_ref: executable_path_ref,
            process_start_time,
        }
    }

    pub(crate) fn raw_target(&self) -> OwnedProcessTerminationTarget {
        OwnedProcessTerminationTarget {
            pid: self.pid,
            expected_process_name: self.expected_process_name.clone(),
        }
    }

    pub(crate) fn grant_fingerprint(&self) -> &str {
        &self.grant_fingerprint
    }

    pub(crate) fn issuer_key_id(&self) -> &str {
        &self.issuer_key_id
    }

    pub(crate) fn issuer_actor_id(&self) -> &str {
        &self.issuer_actor_id
    }

    pub(crate) fn household_id(&self) -> &str {
        &self.household_id
    }

    pub(crate) fn parent_device_id(&self) -> &str {
        &self.parent_device_id
    }

    pub(crate) fn child_profile_id(&self) -> &str {
        &self.child_profile_id
    }

    pub(crate) fn target_device_id(&self) -> &str {
        &self.target_device_id
    }

    pub(crate) fn policy_decision_id(&self) -> &str {
        &self.policy_decision_id
    }

    pub(crate) fn policy_version(&self) -> &str {
        &self.policy_version
    }

    pub(crate) fn action_id(&self) -> &str {
        &self.action_id
    }

    pub(crate) fn capability_id(&self) -> &str {
        &self.capability_id
    }

    pub(crate) fn pid(&self) -> u32 {
        self.pid
    }

    pub(crate) fn expected_process_name(&self) -> &str {
        &self.expected_process_name
    }

    pub(crate) fn expected_executable_path(&self) -> &str {
        &self.expected_executable_path_ref
    }

    pub(crate) fn process_start_time(&self) -> u64 {
        self.process_start_time
    }

    pub(crate) fn managed_process_identity(&self) -> &str {
        &self.managed_process_identity
    }
}
