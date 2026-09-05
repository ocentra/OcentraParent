#![forbid(unsafe_code)]

//! Account and household membership records owned by the family identity
//! boundary.
//!
//! These records describe account truth that a durable adapter may persist.
//! They do not verify an external provider, mint a session, or grant device
//! authority by themselves.  Authority decisions still require the complete
//! household, device, session, and capability inputs owned by this crate.

use crate::family_identity::{
    ActorAccountState, ChildProfileId, HouseholdId, HouseholdMembershipState, HouseholdRole,
    SetupAuditEvidenceRef,
};
use crate::family_identity_contract_text::{optional_contract_text, required_contract_text};
use crate::household_authority::HouseholdAuthorityAction;
use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

family_identity_text_id!(AccountUserId, "family_identity.account_user_id");
family_identity_text_id!(
    HouseholdMembershipId,
    "family_identity.household_membership_id"
);
family_identity_text_id!(
    SupportAdminActorId,
    "family_identity.support_admin_actor_id"
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountUser {
    pub user_id: AccountUserId,
    pub account_state: ActorAccountState,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdMembership {
    pub membership_id: HouseholdMembershipId,
    pub account_user_id: AccountUserId,
    pub household_id: HouseholdId,
    pub role: HouseholdRole,
    pub state: HouseholdMembershipState,
    pub child_profile_id: Option<ChildProfileId>,
    pub invited_at: String,
    pub joined_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseholdMembershipInput {
    pub membership_id: HouseholdMembershipId,
    pub account_user_id: AccountUserId,
    pub household_id: HouseholdId,
    pub role: HouseholdRole,
    pub state: HouseholdMembershipState,
    pub child_profile_id: Option<ChildProfileId>,
    pub invited_at: String,
    pub joined_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportAdminActor {
    pub actor_id: SupportAdminActorId,
    pub account_user_id: AccountUserId,
    pub action: HouseholdAuthorityAction,
    pub audit_evidence_ref: SetupAuditEvidenceRef,
}

impl AccountUser {
    pub fn new(
        user_id: AccountUserId,
        account_state: ActorAccountState,
        created_at: impl Into<String>,
    ) -> Result<Self, EventingError> {
        Ok(Self {
            user_id,
            account_state,
            created_at: required_contract_text(
                "family_identity.account_user.created_at",
                created_at,
            )?,
        })
    }
}

impl HouseholdMembership {
    pub fn new(input: HouseholdMembershipInput) -> Result<Self, EventingError> {
        let HouseholdMembershipInput {
            membership_id,
            account_user_id,
            household_id,
            role,
            state,
            child_profile_id,
            invited_at,
            joined_at,
        } = input;
        if role == HouseholdRole::ChildProfile && child_profile_id.is_none() {
            return Err(EventingError::InvalidValue {
                field: "family_identity.household_membership.child_profile_id",
                value: String::from("missing"),
            });
        }
        if role != HouseholdRole::ChildProfile && child_profile_id.is_some() {
            return Err(EventingError::InvalidValue {
                field: "family_identity.household_membership.child_profile_id",
                value: String::from("unexpected"),
            });
        }
        if state == HouseholdMembershipState::Active && joined_at.is_none() {
            return Err(EventingError::InvalidValue {
                field: "family_identity.household_membership.joined_at",
                value: String::from("required-for-active-membership"),
            });
        }

        Ok(Self {
            membership_id,
            account_user_id,
            household_id,
            role,
            state,
            child_profile_id,
            invited_at: required_contract_text(
                "family_identity.household_membership.invited_at",
                invited_at,
            )?,
            joined_at: optional_contract_text(
                "family_identity.household_membership.joined_at",
                joined_at,
            )?,
        })
    }

    pub fn validate_household(&self, household_id: &HouseholdId) -> Result<(), EventingError> {
        if &self.household_id != household_id {
            return Err(EventingError::InvalidValue {
                field: "family_identity.household_membership.household_id",
                value: self.household_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.state == HouseholdMembershipState::Active
    }
}

impl SupportAdminActor {
    pub fn new(
        actor_id: SupportAdminActorId,
        account_user_id: AccountUserId,
        action: HouseholdAuthorityAction,
        audit_evidence_ref: SetupAuditEvidenceRef,
    ) -> Self {
        Self {
            actor_id,
            account_user_id,
            action,
            audit_evidence_ref,
        }
    }
}
