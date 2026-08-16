#![forbid(unsafe_code)]

use crate::family_identity::{
    ChildCustodyLabel, ChildProfile, ChildProfileId, DeviceId, DeviceRegistration,
    DeviceRouteStateLabel, DeviceTrustState, HouseholdId, HouseholdProfile, HouseholdRole,
    ObserverPermission, ObserverPermissionId, ParentControllerLease, ParentControllerLeaseId,
    ParentMember, ParentMemberId,
};
use crate::family_identity_contract_text::{optional_contract_text, required_contract_text};
use crate::household_authority::{HouseholdAuthorityAction, ParentControllerLeaseState};
use ocentra_eventing::error::EventingError;

impl HouseholdProfile {
    pub fn new(
        household_id: HouseholdId,
        display_name: impl Into<String>,
        created_at: impl Into<String>,
        parent_member_ids: Vec<ParentMemberId>,
        child_profile_ids: Vec<ChildProfileId>,
    ) -> Result<Self, EventingError> {
        Ok(Self {
            household_id,
            display_name: required_contract_text(
                "family_identity.household_profile.display_name",
                display_name,
            )?,
            created_at: required_contract_text(
                "family_identity.household_profile.created_at",
                created_at,
            )?,
            parent_member_ids,
            child_profile_ids,
        })
    }
}

impl ParentMember {
    pub fn new(
        member_id: ParentMemberId,
        household_id: HouseholdId,
        role: HouseholdRole,
        invite_state: crate::family_identity::HouseholdMembershipState,
        joined_at: impl Into<String>,
    ) -> Result<Self, EventingError> {
        if let Some(error) =
            crate::family_identity_profiles_validation::parent_member_role_failure(role)
        {
            return Err(error);
        }

        Ok(Self {
            member_id,
            household_id,
            role,
            invite_state,
            joined_at: required_contract_text(
                "family_identity.parent_member.joined_at",
                joined_at,
            )?,
        })
    }
}

impl ChildProfile {
    pub fn new(
        child_id: ChildProfileId,
        household_id: HouseholdId,
        display_name: impl Into<String>,
        device_ids: Vec<DeviceId>,
        custody_label: ChildCustodyLabel,
    ) -> Result<Self, EventingError> {
        Ok(Self {
            child_id,
            household_id,
            display_name: required_contract_text(
                "family_identity.child_profile.display_name",
                display_name,
            )?,
            device_ids,
            custody_label,
        })
    }
}

impl DeviceRegistration {
    pub fn new(
        device_id: DeviceId,
        child_id: ChildProfileId,
        household_id: HouseholdId,
        trust_state: DeviceTrustState,
        role_label: HouseholdRole,
        route_state: DeviceRouteStateLabel,
        stale_since: Option<String>,
    ) -> Result<Self, EventingError> {
        Ok(Self {
            device_id,
            child_id,
            household_id,
            trust_state,
            role_label,
            route_state,
            stale_since: optional_contract_text(
                "family_identity.device_registration.stale_since",
                stale_since,
            )?,
        })
    }

    pub fn validate_child_profile(
        &self,
        child_profile: &ChildProfile,
    ) -> Result<(), EventingError> {
        if let Some(error) =
            crate::family_identity_profiles_validation::device_registration_child_profile_failure(
                self,
                child_profile,
            )
        {
            return Err(error);
        }

        Ok(())
    }
}

impl ParentControllerLease {
    pub fn new(
        lease_id: ParentControllerLeaseId,
        parent_member_id: ParentMemberId,
        device_id: DeviceId,
        issued_at: impl Into<String>,
        expires_at: impl Into<String>,
        granted_actions: Vec<HouseholdAuthorityAction>,
        revocation_state: ParentControllerLeaseState,
    ) -> Result<Self, EventingError> {
        Ok(Self {
            lease_id,
            parent_member_id,
            device_id,
            issued_at: required_contract_text(
                "family_identity.parent_controller_lease.issued_at",
                issued_at,
            )?,
            expires_at: required_contract_text(
                "family_identity.parent_controller_lease.expires_at",
                expires_at,
            )?,
            granted_actions,
            revocation_state,
        })
    }

    pub fn ensure_reusable(&self) -> Result<(), EventingError> {
        if let Some(error) =
            crate::family_identity_profiles_validation::parent_controller_lease_reusable_failure(
                self,
            )
        {
            return Err(error);
        }

        Ok(())
    }
}

impl ObserverPermission {
    pub fn new(
        perm_id: ObserverPermissionId,
        parent_member_id: ParentMemberId,
        household_id: HouseholdId,
        granted_scopes: Vec<HouseholdAuthorityAction>,
        is_write_blocked: bool,
    ) -> Result<Self, EventingError> {
        if let Some(error) =
            crate::family_identity_profiles_validation::observer_permission_failure_reason(
                &granted_scopes,
                is_write_blocked,
            )
        {
            return Err(error);
        }

        Ok(Self {
            perm_id,
            parent_member_id,
            household_id,
            granted_scopes,
            is_write_blocked,
        })
    }
}
