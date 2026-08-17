use super::super::{
    LanHouseholdMeshIngressAuthorization, LanHouseholdMeshIngressAuthorizationScope,
};

impl LanHouseholdMeshIngressAuthorization {
    pub fn receipt_id(&self) -> &str {
        &self.receipt_id
    }

    pub fn scope(&self) -> LanHouseholdMeshIngressAuthorizationScope {
        self.scope
    }

    pub fn family_hash(&self) -> &str {
        &self.family_hash
    }

    pub fn child_device_id(&self) -> &str {
        &self.child_device_id
    }

    pub fn target_device_id(&self) -> &str {
        &self.target_device_id
    }
}
