#![forbid(unsafe_code)]

//! Setup, install, and provisioning ownership boundary.
//!
//! This crate owns install journey state, pairing readiness, permission
//! onboarding, recovery, and provisioning contracts. Binary updater mechanics
//! remain in the updater crate.

use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_family_identity_core::{
    authorize_household_action, authorize_session_token_action, authorize_setup_invite,
    evaluate_recovery_operation, DeviceOwnershipScope, DeviceTrustState, HouseholdAuthorityInput,
    HouseholdAuthorizationFailureReason, HouseholdAuthorizationState, HouseholdMembership,
    RecoveryDataCustodyHandoffState, RecoveryKind as FamilyRecoveryKind,
    RecoveryOperation as FamilyRecoveryOperation, RecoveryState as FamilyRecoveryState,
    RecoverySupportChannel, SessionTokenFailureReason, SessionTokenInput, SetupInviteFailureReason,
    SetupInviteInput, SetupInvitePurpose, SetupInviteState, SetupInviteTargetRole,
};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-provisioning-core";
const PROVISIONING_SCHEMA_VERSION: u16 = 1;
const PROVISIONING_READINESS_EVALUATED_EVENT_TYPE: &str = "provisioning.readiness.evaluated";
const PROVISIONING_ACTION_PLANNED_EVENT_TYPE: &str = "provisioning.action.planned";
const PROVISIONING_IDEMPOTENCY_SEPARATOR: &str = ":";
const PROVISIONING_ACTION_PREFIX: &str = "provisioning-action:";
const ERROR_PROVISIONING_ACTION_ID: &str = "provisioning action id";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountReadinessState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "wrong-account")]
    WrongAccount,
    #[serde(rename = "recovery-required")]
    RecoveryRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAppReadinessState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "stale")]
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentDeviceRegistrationState {
    #[serde(rename = "registered")]
    Registered,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildInstallState {
    #[serde(rename = "not-installed")]
    NotInstalled,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "reinstall-required")]
    ReinstallRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildServiceState {
    #[serde(rename = "not-started")]
    NotStarted,
    #[serde(rename = "service-started")]
    ServiceStarted,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildAppReadinessState {
    #[serde(rename = "not-installed")]
    NotInstalled,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "reinstall-required")]
    ReinstallRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionReadinessState {
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PairingLifecycleState {
    #[serde(rename = "generated")]
    Generated,
    #[serde(rename = "displayed")]
    Displayed,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "replayed")]
    Replayed,
    #[serde(rename = "wrong-household")]
    WrongHousehold,
    #[serde(rename = "wrong-device")]
    WrongDevice,
    #[serde(rename = "anonymous-device")]
    AnonymousDevice,
    #[serde(rename = "parent-role-required")]
    ParentRoleRequired,
    #[serde(rename = "stale-signed-hello")]
    StaleSignedHello,
    #[serde(rename = "trusted")]
    Trusted,
    #[serde(rename = "untrusted")]
    Untrusted,
    #[serde(rename = "recovered")]
    Recovered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyBaselineState {
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "stale")]
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataCustodySyncState {
    #[serde(rename = "synced")]
    Synced,
    #[serde(rename = "sync-pending")]
    SyncPending,
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkReachabilityState {
    #[serde(rename = "reachable")]
    Reachable,
    #[serde(rename = "offline-child")]
    OfflineChild,
    #[serde(rename = "lan-unavailable")]
    LanUnavailable,
    #[serde(rename = "direct-entry-required")]
    DirectEntryRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryState {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "lost-parent-device")]
    LostParentDevice,
    #[serde(rename = "child-reinstall")]
    ChildReinstall,
    #[serde(rename = "revoked-child")]
    RevokedChild,
    #[serde(rename = "wrong-account")]
    WrongAccount,
    #[serde(rename = "offline-device")]
    OfflineDevice,
    #[serde(rename = "permission-loss")]
    PermissionLoss,
    #[serde(rename = "stale-code")]
    StaleCode,
    #[serde(rename = "recovered")]
    Recovered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildRuntimeReadinessState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "not-ready")]
    NotReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningOverallState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningManualStepState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningBlockerReason {
    #[serde(rename = "household-member-required")]
    HouseholdMemberRequired,
    #[serde(rename = "wrong-account")]
    WrongAccount,
    #[serde(rename = "account-recovery-required")]
    AccountRecoveryRequired,
    #[serde(rename = "parent-app-missing")]
    ParentAppMissing,
    #[serde(rename = "parent-app-offline")]
    ParentAppOffline,
    #[serde(rename = "parent-app-stale")]
    ParentAppStale,
    #[serde(rename = "parent-device-registration-required")]
    ParentDeviceRegistrationRequired,
    #[serde(rename = "child-device-scope-required")]
    ChildDeviceScopeRequired,
    #[serde(rename = "child-device-trust-required")]
    ChildDeviceTrustRequired,
    #[serde(rename = "permission-missing")]
    PermissionMissing,
    #[serde(rename = "permission-revoked")]
    PermissionRevoked,
    #[serde(rename = "pairing-pending-display")]
    PairingPendingDisplay,
    #[serde(rename = "pairing-pending-acceptance")]
    PairingPendingAcceptance,
    #[serde(rename = "pairing-expired")]
    PairingExpired,
    #[serde(rename = "pairing-revoked")]
    PairingRevoked,
    #[serde(rename = "pairing-replay-rejected")]
    PairingReplayRejected,
    #[serde(rename = "pairing-wrong-household")]
    PairingWrongHousehold,
    #[serde(rename = "pairing-wrong-device")]
    PairingWrongDevice,
    #[serde(rename = "pairing-anonymous-device")]
    PairingAnonymousDevice,
    #[serde(rename = "pairing-parent-role-required")]
    PairingParentRoleRequired,
    #[serde(rename = "pairing-stale-signed-hello")]
    PairingStaleSignedHello,
    #[serde(rename = "pairing-trust-required")]
    PairingTrustRequired,
    #[serde(rename = "policy-baseline-missing")]
    PolicyBaselineMissing,
    #[serde(rename = "policy-baseline-stale")]
    PolicyBaselineStale,
    #[serde(rename = "data-custody-sync-pending")]
    DataCustodySyncPending,
    #[serde(rename = "data-custody-sync-blocked")]
    DataCustodySyncBlocked,
    #[serde(rename = "child-install-not-installed")]
    ChildInstallNotInstalled,
    #[serde(rename = "child-service-not-started")]
    ChildServiceNotStarted,
    #[serde(rename = "child-app-offline")]
    ChildAppOffline,
    #[serde(rename = "child-app-revoked")]
    ChildAppRevoked,
    #[serde(rename = "child-app-reinstall-required")]
    ChildAppReinstallRequired,
    #[serde(rename = "network-offline-child")]
    NetworkOfflineChild,
    #[serde(rename = "network-lan-unavailable")]
    NetworkLanUnavailable,
    #[serde(rename = "network-direct-entry-required")]
    NetworkDirectEntryRequired,
    #[serde(rename = "lost-parent-device-recovery")]
    LostParentDeviceRecovery,
    #[serde(rename = "child-reinstall-recovery")]
    ChildReinstallRecovery,
    #[serde(rename = "revoked-child-recovery")]
    RevokedChildRecovery,
    #[serde(rename = "wrong-account-recovery")]
    WrongAccountRecovery,
    #[serde(rename = "offline-device-recovery")]
    OfflineDeviceRecovery,
    #[serde(rename = "permission-loss-recovery")]
    PermissionLossRecovery,
    #[serde(rename = "stale-code-recovery")]
    StaleCodeRecovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningRecoveryAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "complete-household-membership")]
    CompleteHouseholdMembership,
    #[serde(rename = "restore-parent-session")]
    RestoreParentSession,
    #[serde(rename = "repair-parent-app")]
    RepairParentApp,
    #[serde(rename = "re-register-parent-device")]
    ReRegisterParentDevice,
    #[serde(rename = "reissue-pairing-code")]
    ReissuePairingCode,
    #[serde(rename = "wait-for-pairing-acceptance")]
    WaitForPairingAcceptance,
    #[serde(rename = "re-pair-child-device")]
    RePairChildDevice,
    #[serde(rename = "trust-child-device")]
    TrustChildDevice,
    #[serde(rename = "switch-to-correct-account")]
    SwitchToCorrectAccount,
    #[serde(rename = "wait-for-child-connectivity")]
    WaitForChildConnectivity,
    #[serde(rename = "enter-direct-child-address")]
    EnterDirectChildAddress,
    #[serde(rename = "install-child-app")]
    InstallChildApp,
    #[serde(rename = "start-child-service")]
    StartChildService,
    #[serde(rename = "reinstall-child-app")]
    ReinstallChildApp,
    #[serde(rename = "request-missing-permissions")]
    RequestMissingPermissions,
    #[serde(rename = "regrant-permissions")]
    RegrantPermissions,
    #[serde(rename = "apply-policy-baseline")]
    ApplyPolicyBaseline,
    #[serde(rename = "repair-custody-sync")]
    RepairCustodySync,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningChildRuntimeStartAction {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "do-not-start")]
    DoNotStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningAuditState {
    #[serde(rename = "record")]
    Record,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningReadinessInput {
    pub household_membership: HouseholdMembership,
    pub account_readiness_state: AccountReadinessState,
    pub parent_app_readiness_state: ParentAppReadinessState,
    pub parent_device_registration_state: ParentDeviceRegistrationState,
    pub child_install_state: ChildInstallState,
    pub child_service_state: ChildServiceState,
    pub child_app_readiness_state: ChildAppReadinessState,
    pub child_device_ownership_scope: DeviceOwnershipScope,
    pub device_trust_state: DeviceTrustState,
    pub permission_readiness_state: PermissionReadinessState,
    pub pairing_lifecycle_state: PairingLifecycleState,
    pub policy_baseline_state: PolicyBaselineState,
    pub data_custody_sync_state: DataCustodySyncState,
    pub network_reachability_state: NetworkReachabilityState,
    pub recovery_state: RecoveryState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningFamilyContextInput {
    pub account_matches_invite_target: bool,
    pub setup_invite_input: SetupInviteInput,
    pub pairing_session_input: SessionTokenInput,
    pub household_authority_input: HouseholdAuthorityInput,
    pub recovery_operation: Option<FamilyRecoveryOperation>,
    pub parent_app_readiness_state: ParentAppReadinessState,
    pub parent_device_registration_state: ParentDeviceRegistrationState,
    pub child_install_state: ChildInstallState,
    pub child_service_state: ChildServiceState,
    pub child_app_readiness_state: ChildAppReadinessState,
    pub permission_readiness_state: PermissionReadinessState,
    pub policy_baseline_state: PolicyBaselineState,
    pub data_custody_sync_state: DataCustodySyncState,
    pub network_reachability_state: NetworkReachabilityState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningReadinessDecision {
    pub child_runtime_readiness_state: ChildRuntimeReadinessState,
    pub manual_step_state: ProvisioningManualStepState,
    pub overall_state: ProvisioningOverallState,
    pub blocker_reason: Option<ProvisioningBlockerReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningActionPlan {
    pub child_runtime_start_action: ProvisioningChildRuntimeStartAction,
    pub recovery_action: ProvisioningRecoveryAction,
    pub audit_state: ProvisioningAuditState,
}

macro_rules! provisioning_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue { field: $field });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

provisioning_text_id!(
    ProvisioningReadinessEvaluationId,
    "provisioning.readiness_evaluation_id"
);
provisioning_text_id!(ProvisioningActionPlanId, "provisioning.action_plan_id");
provisioning_text_id!(ProvisioningAggregateId, "provisioning.aggregate_id");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningReadinessEvaluatedEvent {
    pub aggregate_id: ProvisioningAggregateId,
    pub evaluation_id: ProvisioningReadinessEvaluationId,
    pub input: ProvisioningReadinessInput,
    pub decision: ProvisioningReadinessDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningActionPlannedEvent {
    pub aggregate_id: ProvisioningAggregateId,
    pub action_plan_id: ProvisioningActionPlanId,
    pub source_evaluation_id: ProvisioningReadinessEvaluationId,
    pub action_plan: ProvisioningActionPlan,
}

impl DomainEvent for ProvisioningReadinessEvaluatedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        provisioning_event_contract(PROVISIONING_READINESS_EVALUATED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        provisioning_idempotency_key(
            PROVISIONING_READINESS_EVALUATED_EVENT_TYPE,
            &self.evaluation_id,
        )
    }
}

impl DomainEvent for ProvisioningActionPlannedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        provisioning_event_contract(PROVISIONING_ACTION_PLANNED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        provisioning_idempotency_key(PROVISIONING_ACTION_PLANNED_EVENT_TYPE, &self.action_plan_id)
    }
}

pub fn evaluate_provisioning_readiness(
    input: ProvisioningReadinessInput,
) -> ProvisioningReadinessDecision {
    let blocker_reason = provisioning_blocker_reason(input);
    let overall_state = provisioning_overall_state(blocker_reason);

    ProvisioningReadinessDecision {
        child_runtime_readiness_state: if blocker_reason.is_none() {
            ChildRuntimeReadinessState::Ready
        } else {
            ChildRuntimeReadinessState::NotReady
        },
        manual_step_state: if blocker_reason.is_none() {
            ProvisioningManualStepState::NotRequired
        } else {
            ProvisioningManualStepState::Required
        },
        overall_state,
        blocker_reason,
    }
}

pub fn derive_provisioning_readiness_input_from_family_context(
    input: ProvisioningFamilyContextInput,
) -> ProvisioningReadinessInput {
    let authority_decision = authorize_household_action(input.household_authority_input);
    let session_decision = authorize_session_token_action(input.pairing_session_input);
    let recovery_decision = input.recovery_operation.map(evaluate_recovery_operation);

    ProvisioningReadinessInput {
        household_membership: input.household_authority_input.household_membership,
        account_readiness_state: provisioning_account_state_from_family_context(
            input,
            authority_decision,
            recovery_decision,
            session_decision.failure_reason,
        ),
        parent_app_readiness_state: input.parent_app_readiness_state,
        parent_device_registration_state: input.parent_device_registration_state,
        child_install_state: input.child_install_state,
        child_service_state: input.child_service_state,
        child_app_readiness_state: provisioning_child_app_readiness_state(
            input.child_install_state,
            input.child_service_state,
        ),
        child_device_ownership_scope: input.household_authority_input.device_ownership_scope,
        device_trust_state: input.household_authority_input.device_trust_state,
        permission_readiness_state: input.permission_readiness_state,
        pairing_lifecycle_state: provisioning_pairing_state_from_family_context(
            input,
            authority_decision,
            session_decision.failure_reason,
        ),
        policy_baseline_state: input.policy_baseline_state,
        data_custody_sync_state: provisioning_custody_state_from_family_context(
            input,
            recovery_decision,
        ),
        network_reachability_state: input.network_reachability_state,
        recovery_state: provisioning_recovery_state_from_family_context(
            input,
            session_decision.failure_reason,
        ),
    }
}

pub fn evaluate_provisioning_readiness_from_family_context(
    input: ProvisioningFamilyContextInput,
) -> ProvisioningReadinessDecision {
    evaluate_provisioning_readiness(derive_provisioning_readiness_input_from_family_context(
        input,
    ))
}

pub fn plan_provisioning_actions_from_family_context(
    input: ProvisioningFamilyContextInput,
) -> ProvisioningActionPlan {
    plan_provisioning_actions(derive_provisioning_readiness_input_from_family_context(
        input,
    ))
}

pub fn provisioning_readiness_evaluated_event(
    aggregate_id: ProvisioningAggregateId,
    evaluation_id: ProvisioningReadinessEvaluationId,
    input: ProvisioningReadinessInput,
) -> ProvisioningReadinessEvaluatedEvent {
    ProvisioningReadinessEvaluatedEvent {
        aggregate_id,
        evaluation_id,
        input,
        decision: evaluate_provisioning_readiness(input),
    }
}

pub fn plan_provisioning_actions(input: ProvisioningReadinessInput) -> ProvisioningActionPlan {
    let decision = evaluate_provisioning_readiness(input);

    ProvisioningActionPlan {
        child_runtime_start_action: if decision.child_runtime_readiness_state
            == ChildRuntimeReadinessState::Ready
        {
            ProvisioningChildRuntimeStartAction::Start
        } else {
            ProvisioningChildRuntimeStartAction::DoNotStart
        },
        recovery_action: provisioning_recovery_action(decision.blocker_reason),
        audit_state: ProvisioningAuditState::Record,
    }
}

pub fn provisioning_action_planned_event(
    event: ProvisioningReadinessEvaluatedEvent,
) -> ProvisioningActionPlannedEvent {
    ProvisioningActionPlannedEvent {
        aggregate_id: event.aggregate_id,
        action_plan_id: ProvisioningActionPlanId::parse(provisioning_action_ref(
            &event.evaluation_id,
        ))
        .expect(ERROR_PROVISIONING_ACTION_ID),
        source_evaluation_id: event.evaluation_id,
        action_plan: plan_provisioning_actions(event.input),
    }
}

fn provisioning_blocker_reason(
    input: ProvisioningReadinessInput,
) -> Option<ProvisioningBlockerReason> {
    if input.household_membership != HouseholdMembership::Member {
        return Some(ProvisioningBlockerReason::HouseholdMemberRequired);
    }

    match input.account_readiness_state {
        AccountReadinessState::Ready => {}
        AccountReadinessState::WrongAccount => {
            return Some(ProvisioningBlockerReason::WrongAccount);
        }
        AccountReadinessState::RecoveryRequired => {
            return Some(ProvisioningBlockerReason::AccountRecoveryRequired);
        }
    }

    match input.parent_app_readiness_state {
        ParentAppReadinessState::Ready => {}
        ParentAppReadinessState::Missing => {
            return Some(ProvisioningBlockerReason::ParentAppMissing);
        }
        ParentAppReadinessState::Offline => {
            return Some(ProvisioningBlockerReason::ParentAppOffline);
        }
        ParentAppReadinessState::Stale => {
            return Some(ProvisioningBlockerReason::ParentAppStale);
        }
    }

    if input.parent_device_registration_state == ParentDeviceRegistrationState::Missing {
        return Some(ProvisioningBlockerReason::ParentDeviceRegistrationRequired);
    }

    match input.pairing_lifecycle_state {
        PairingLifecycleState::WrongDevice => {
            return Some(ProvisioningBlockerReason::PairingWrongDevice);
        }
        PairingLifecycleState::AnonymousDevice => {
            return Some(ProvisioningBlockerReason::PairingAnonymousDevice);
        }
        PairingLifecycleState::ParentRoleRequired => {
            return Some(ProvisioningBlockerReason::PairingParentRoleRequired);
        }
        PairingLifecycleState::StaleSignedHello => {
            return Some(ProvisioningBlockerReason::PairingStaleSignedHello);
        }
        _ => {}
    }

    if input.child_device_ownership_scope != DeviceOwnershipScope::ChildProfileDevice {
        return Some(ProvisioningBlockerReason::ChildDeviceScopeRequired);
    }

    if input.device_trust_state != DeviceTrustState::Trusted {
        return Some(ProvisioningBlockerReason::ChildDeviceTrustRequired);
    }

    match input.permission_readiness_state {
        PermissionReadinessState::Granted => {}
        PermissionReadinessState::Missing => {
            return Some(ProvisioningBlockerReason::PermissionMissing);
        }
        PermissionReadinessState::Revoked => {
            return Some(ProvisioningBlockerReason::PermissionRevoked);
        }
    }

    match input.pairing_lifecycle_state {
        PairingLifecycleState::Generated | PairingLifecycleState::Displayed => {
            return Some(ProvisioningBlockerReason::PairingPendingDisplay);
        }
        PairingLifecycleState::Accepted => {
            return Some(ProvisioningBlockerReason::PairingPendingAcceptance);
        }
        PairingLifecycleState::Expired => {
            return Some(ProvisioningBlockerReason::PairingExpired);
        }
        PairingLifecycleState::Revoked => {
            return Some(ProvisioningBlockerReason::PairingRevoked);
        }
        PairingLifecycleState::Replayed => {
            return Some(ProvisioningBlockerReason::PairingReplayRejected);
        }
        PairingLifecycleState::WrongHousehold => {
            return Some(ProvisioningBlockerReason::PairingWrongHousehold);
        }
        PairingLifecycleState::WrongDevice => {
            return Some(ProvisioningBlockerReason::PairingWrongDevice);
        }
        PairingLifecycleState::AnonymousDevice => {
            return Some(ProvisioningBlockerReason::PairingAnonymousDevice);
        }
        PairingLifecycleState::ParentRoleRequired => {
            return Some(ProvisioningBlockerReason::PairingParentRoleRequired);
        }
        PairingLifecycleState::StaleSignedHello => {
            return Some(ProvisioningBlockerReason::PairingStaleSignedHello);
        }
        PairingLifecycleState::Untrusted => {
            return Some(ProvisioningBlockerReason::PairingTrustRequired);
        }
        PairingLifecycleState::Trusted | PairingLifecycleState::Recovered => {}
    }

    match input.policy_baseline_state {
        PolicyBaselineState::Applied => {}
        PolicyBaselineState::Missing => {
            return Some(ProvisioningBlockerReason::PolicyBaselineMissing);
        }
        PolicyBaselineState::Stale => {
            return Some(ProvisioningBlockerReason::PolicyBaselineStale);
        }
    }

    match input.data_custody_sync_state {
        DataCustodySyncState::Synced => {}
        DataCustodySyncState::SyncPending => {
            return Some(ProvisioningBlockerReason::DataCustodySyncPending);
        }
        DataCustodySyncState::Blocked => {
            return Some(ProvisioningBlockerReason::DataCustodySyncBlocked);
        }
    }

    match input.child_install_state {
        ChildInstallState::NotInstalled => {
            return Some(ProvisioningBlockerReason::ChildInstallNotInstalled);
        }
        ChildInstallState::Installed => {}
        ChildInstallState::ReinstallRequired => {
            return Some(ProvisioningBlockerReason::ChildAppReinstallRequired);
        }
    }

    match input.child_service_state {
        ChildServiceState::NotStarted => {
            return Some(ProvisioningBlockerReason::ChildServiceNotStarted);
        }
        ChildServiceState::ServiceStarted => {}
        ChildServiceState::Offline => {
            return Some(ProvisioningBlockerReason::ChildAppOffline);
        }
        ChildServiceState::Revoked => {
            return Some(ProvisioningBlockerReason::ChildAppRevoked);
        }
    }

    match input.network_reachability_state {
        NetworkReachabilityState::Reachable => {}
        NetworkReachabilityState::OfflineChild => {
            return Some(ProvisioningBlockerReason::NetworkOfflineChild);
        }
        NetworkReachabilityState::LanUnavailable => {
            return Some(ProvisioningBlockerReason::NetworkLanUnavailable);
        }
        NetworkReachabilityState::DirectEntryRequired => {
            return Some(ProvisioningBlockerReason::NetworkDirectEntryRequired);
        }
    }

    match input.recovery_state {
        RecoveryState::Normal | RecoveryState::Recovered => None,
        RecoveryState::LostParentDevice => {
            Some(ProvisioningBlockerReason::LostParentDeviceRecovery)
        }
        RecoveryState::ChildReinstall => Some(ProvisioningBlockerReason::ChildReinstallRecovery),
        RecoveryState::RevokedChild => Some(ProvisioningBlockerReason::RevokedChildRecovery),
        RecoveryState::WrongAccount => Some(ProvisioningBlockerReason::WrongAccountRecovery),
        RecoveryState::OfflineDevice => Some(ProvisioningBlockerReason::OfflineDeviceRecovery),
        RecoveryState::PermissionLoss => Some(ProvisioningBlockerReason::PermissionLossRecovery),
        RecoveryState::StaleCode => Some(ProvisioningBlockerReason::StaleCodeRecovery),
    }
}

fn provisioning_overall_state(
    blocker_reason: Option<ProvisioningBlockerReason>,
) -> ProvisioningOverallState {
    match blocker_reason {
        None => ProvisioningOverallState::Ready,
        Some(
            ProvisioningBlockerReason::DataCustodySyncPending
            | ProvisioningBlockerReason::ChildAppOffline
            | ProvisioningBlockerReason::NetworkOfflineChild
            | ProvisioningBlockerReason::OfflineDeviceRecovery,
        ) => ProvisioningOverallState::Degraded,
        Some(_) => ProvisioningOverallState::Blocked,
    }
}

fn provisioning_recovery_action(
    blocker_reason: Option<ProvisioningBlockerReason>,
) -> ProvisioningRecoveryAction {
    match blocker_reason {
        None => ProvisioningRecoveryAction::Continue,
        Some(ProvisioningBlockerReason::HouseholdMemberRequired) => {
            ProvisioningRecoveryAction::CompleteHouseholdMembership
        }
        Some(
            ProvisioningBlockerReason::WrongAccount
            | ProvisioningBlockerReason::WrongAccountRecovery,
        ) => ProvisioningRecoveryAction::SwitchToCorrectAccount,
        Some(
            ProvisioningBlockerReason::AccountRecoveryRequired
            | ProvisioningBlockerReason::LostParentDeviceRecovery,
        ) => ProvisioningRecoveryAction::RestoreParentSession,
        Some(
            ProvisioningBlockerReason::ParentAppMissing
            | ProvisioningBlockerReason::ParentAppOffline
            | ProvisioningBlockerReason::ParentAppStale,
        ) => ProvisioningRecoveryAction::RepairParentApp,
        Some(ProvisioningBlockerReason::ParentDeviceRegistrationRequired) => {
            ProvisioningRecoveryAction::ReRegisterParentDevice
        }
        Some(ProvisioningBlockerReason::ChildDeviceScopeRequired) => {
            ProvisioningRecoveryAction::RePairChildDevice
        }
        Some(
            ProvisioningBlockerReason::ChildDeviceTrustRequired
            | ProvisioningBlockerReason::PairingTrustRequired
            | ProvisioningBlockerReason::PairingPendingAcceptance,
        ) => ProvisioningRecoveryAction::TrustChildDevice,
        Some(ProvisioningBlockerReason::PermissionMissing) => {
            ProvisioningRecoveryAction::RequestMissingPermissions
        }
        Some(
            ProvisioningBlockerReason::PermissionRevoked
            | ProvisioningBlockerReason::PermissionLossRecovery,
        ) => ProvisioningRecoveryAction::RegrantPermissions,
        Some(
            ProvisioningBlockerReason::PairingPendingDisplay
            | ProvisioningBlockerReason::PairingExpired
            | ProvisioningBlockerReason::PairingReplayRejected
            | ProvisioningBlockerReason::PairingStaleSignedHello
            | ProvisioningBlockerReason::StaleCodeRecovery,
        ) => ProvisioningRecoveryAction::ReissuePairingCode,
        Some(
            ProvisioningBlockerReason::PairingRevoked
            | ProvisioningBlockerReason::PairingWrongHousehold
            | ProvisioningBlockerReason::PairingWrongDevice
            | ProvisioningBlockerReason::PairingAnonymousDevice
            | ProvisioningBlockerReason::RevokedChildRecovery,
        ) => ProvisioningRecoveryAction::RePairChildDevice,
        Some(ProvisioningBlockerReason::PairingParentRoleRequired) => {
            ProvisioningRecoveryAction::SwitchToCorrectAccount
        }
        Some(
            ProvisioningBlockerReason::PolicyBaselineMissing
            | ProvisioningBlockerReason::PolicyBaselineStale,
        ) => ProvisioningRecoveryAction::ApplyPolicyBaseline,
        Some(
            ProvisioningBlockerReason::DataCustodySyncPending
            | ProvisioningBlockerReason::DataCustodySyncBlocked,
        ) => ProvisioningRecoveryAction::RepairCustodySync,
        Some(ProvisioningBlockerReason::ChildInstallNotInstalled) => {
            ProvisioningRecoveryAction::InstallChildApp
        }
        Some(ProvisioningBlockerReason::ChildServiceNotStarted) => {
            ProvisioningRecoveryAction::StartChildService
        }
        Some(
            ProvisioningBlockerReason::ChildAppOffline
            | ProvisioningBlockerReason::NetworkOfflineChild
            | ProvisioningBlockerReason::NetworkLanUnavailable
            | ProvisioningBlockerReason::OfflineDeviceRecovery,
        ) => ProvisioningRecoveryAction::WaitForChildConnectivity,
        Some(ProvisioningBlockerReason::NetworkDirectEntryRequired) => {
            ProvisioningRecoveryAction::EnterDirectChildAddress
        }
        Some(
            ProvisioningBlockerReason::ChildAppReinstallRequired
            | ProvisioningBlockerReason::ChildReinstallRecovery,
        ) => ProvisioningRecoveryAction::ReinstallChildApp,
        Some(ProvisioningBlockerReason::ChildAppRevoked) => {
            ProvisioningRecoveryAction::RePairChildDevice
        }
    }
}

fn provisioning_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(PROVISIONING_SCHEMA_VERSION)?,
    ))
}

fn provisioning_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, PROVISIONING_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn provisioning_action_ref(evaluation_id: &ProvisioningReadinessEvaluationId) -> String {
    let mut value = String::from(PROVISIONING_ACTION_PREFIX);
    value.push_str(evaluation_id.as_str());
    value
}

fn provisioning_child_app_readiness_state(
    child_install_state: ChildInstallState,
    child_service_state: ChildServiceState,
) -> ChildAppReadinessState {
    match child_install_state {
        ChildInstallState::NotInstalled => ChildAppReadinessState::NotInstalled,
        ChildInstallState::ReinstallRequired => ChildAppReadinessState::ReinstallRequired,
        ChildInstallState::Installed => match child_service_state {
            ChildServiceState::NotStarted => ChildAppReadinessState::Installed,
            ChildServiceState::ServiceStarted => ChildAppReadinessState::Ready,
            ChildServiceState::Offline => ChildAppReadinessState::Offline,
            ChildServiceState::Revoked => ChildAppReadinessState::Revoked,
        },
    }
}

fn provisioning_account_state_from_family_context(
    input: ProvisioningFamilyContextInput,
    authority_decision: ocentra_family_identity_core::HouseholdAuthorityDecision,
    recovery_decision: Option<ocentra_family_identity_core::RecoveryDecision>,
    session_failure_reason: Option<SessionTokenFailureReason>,
) -> AccountReadinessState {
    if !input.account_matches_invite_target {
        return AccountReadinessState::WrongAccount;
    }

    if matches!(
        authority_decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::AccountNotActive)
    ) || matches!(
        session_failure_reason,
        Some(
            SessionTokenFailureReason::SessionLoggedOut
                | SessionTokenFailureReason::SessionRevoked
                | SessionTokenFailureReason::SessionGloballyRevoked
                | SessionTokenFailureReason::SessionNotFresh
        )
    ) {
        return AccountReadinessState::RecoveryRequired;
    }

    if matches!(
        input.recovery_operation,
        Some(operation) if operation.state != FamilyRecoveryState::Completed
    ) {
        let recovery_blocks_custody_first = matches!(
            recovery_decision,
            Some(decision)
                if decision.data_custody_handoff_state != RecoveryDataCustodyHandoffState::None
        ) || matches!(
            input.recovery_operation,
            Some(operation)
                if operation.support_channel == RecoverySupportChannel::SupportAssisted
        );

        if !recovery_blocks_custody_first {
            return AccountReadinessState::RecoveryRequired;
        }
    }

    AccountReadinessState::Ready
}

fn provisioning_pairing_state_from_family_context(
    input: ProvisioningFamilyContextInput,
    authority_decision: ocentra_family_identity_core::HouseholdAuthorityDecision,
    session_failure_reason: Option<SessionTokenFailureReason>,
) -> PairingLifecycleState {
    let invite_failure_reason = provisioning_setup_invite_failure_reason(input.setup_invite_input);
    let awaiting_parent_trust_confirmation = input.setup_invite_input.invite_state
        == SetupInviteState::Accepted
        && input.household_authority_input.device_trust_state == DeviceTrustState::Pending;

    if matches!(
        session_failure_reason,
        Some(SessionTokenFailureReason::TokenReplayRejected)
    ) || matches!(
        invite_failure_reason,
        Some(SetupInviteFailureReason::InviteReplayRejected)
    ) {
        return PairingLifecycleState::Replayed;
    }

    if matches!(
        session_failure_reason,
        Some(SessionTokenFailureReason::TokenExpired | SessionTokenFailureReason::TokenNotYetValid)
    ) {
        return PairingLifecycleState::StaleSignedHello;
    }

    if input.setup_invite_input.invite_state == SetupInviteState::Expired {
        return PairingLifecycleState::Expired;
    }

    if input.setup_invite_input.invite_state == SetupInviteState::Revoked
        || (matches!(
            authority_decision.failure_reason,
            Some(HouseholdAuthorizationFailureReason::DeviceNotTrusted)
        ) && !awaiting_parent_trust_confirmation)
    {
        return PairingLifecycleState::Revoked;
    }

    if !input.account_matches_invite_target {
        return PairingLifecycleState::Untrusted;
    }

    if matches!(
        invite_failure_reason,
        Some(SetupInviteFailureReason::WrongHousehold)
    ) || matches!(
        authority_decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ExternalHousehold)
    ) {
        return PairingLifecycleState::WrongHousehold;
    }

    if matches!(
        authority_decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::WrongDeviceScope)
    ) {
        return PairingLifecycleState::WrongDevice;
    }

    if matches!(
        authority_decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ChildProfileNotBound)
    ) {
        return PairingLifecycleState::AnonymousDevice;
    }

    if matches!(
        authority_decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::RoleNotAuthorized)
    ) {
        return PairingLifecycleState::ParentRoleRequired;
    }

    if authority_decision.authorization_state == HouseholdAuthorizationState::Rejected {
        if awaiting_parent_trust_confirmation {
            return PairingLifecycleState::Accepted;
        }
        return PairingLifecycleState::Untrusted;
    }

    if matches!(
        input.recovery_operation,
        Some(operation) if operation.state == FamilyRecoveryState::Completed
    ) {
        return PairingLifecycleState::Recovered;
    }

    if matches!(
        input.recovery_operation,
        Some(operation)
            if operation.state == FamilyRecoveryState::PendingIdentityProof
                || operation.state == FamilyRecoveryState::OwnerApprovalRequired
                || operation.state == FamilyRecoveryState::Approved
    ) {
        return PairingLifecycleState::Trusted;
    }

    if input.setup_invite_input.invite_state == SetupInviteState::Accepted {
        return PairingLifecycleState::Trusted;
    }

    PairingLifecycleState::Displayed
}

fn provisioning_custody_state_from_family_context(
    input: ProvisioningFamilyContextInput,
    recovery_decision: Option<ocentra_family_identity_core::RecoveryDecision>,
) -> DataCustodySyncState {
    if matches!(
        recovery_decision,
        Some(decision)
            if decision.data_custody_handoff_state != RecoveryDataCustodyHandoffState::None
    ) {
        return DataCustodySyncState::Blocked;
    }

    if matches!(
        input.recovery_operation,
        Some(operation)
            if operation.support_channel == RecoverySupportChannel::SupportAssisted
                && operation.state != FamilyRecoveryState::Completed
    ) {
        return DataCustodySyncState::Blocked;
    }

    input.data_custody_sync_state
}

fn provisioning_recovery_state_from_family_context(
    input: ProvisioningFamilyContextInput,
    session_failure_reason: Option<SessionTokenFailureReason>,
) -> RecoveryState {
    let child_app_readiness_state = provisioning_child_app_readiness_state(
        input.child_install_state,
        input.child_service_state,
    );

    if let Some(recovery_operation) = input.recovery_operation {
        if recovery_operation.state == FamilyRecoveryState::Completed {
            return RecoveryState::Recovered;
        }

        return match recovery_operation.kind {
            FamilyRecoveryKind::ForgotLogin => RecoveryState::WrongAccount,
            FamilyRecoveryKind::LostParentDevice => RecoveryState::LostParentDevice,
            FamilyRecoveryKind::CompromisedAccount => RecoveryState::PermissionLoss,
            FamilyRecoveryKind::ChildReinstall => RecoveryState::ChildReinstall,
            FamilyRecoveryKind::HouseholdTransfer => RecoveryState::RevokedChild,
        };
    }

    if matches!(
        session_failure_reason,
        Some(SessionTokenFailureReason::TokenExpired | SessionTokenFailureReason::TokenNotYetValid)
    ) || input.setup_invite_input.invite_state == SetupInviteState::Expired
    {
        return RecoveryState::StaleCode;
    }

    if !input.account_matches_invite_target {
        return RecoveryState::WrongAccount;
    }

    match child_app_readiness_state {
        ChildAppReadinessState::NotInstalled | ChildAppReadinessState::Installed => {}
        ChildAppReadinessState::Revoked => return RecoveryState::RevokedChild,
        ChildAppReadinessState::ReinstallRequired => return RecoveryState::ChildReinstall,
        ChildAppReadinessState::Offline => return RecoveryState::OfflineDevice,
        ChildAppReadinessState::Ready => {}
    }

    if input.permission_readiness_state != PermissionReadinessState::Granted {
        return RecoveryState::PermissionLoss;
    }

    if input.network_reachability_state != NetworkReachabilityState::Reachable {
        return RecoveryState::OfflineDevice;
    }

    RecoveryState::Normal
}

fn provisioning_setup_invite_failure_reason(
    input: SetupInviteInput,
) -> Option<SetupInviteFailureReason> {
    if input.invite_state == SetupInviteState::Pending {
        return authorize_setup_invite(input).failure_reason;
    }

    if !input.single_use {
        return Some(SetupInviteFailureReason::InviteNotSingleUse);
    }

    if !provisioning_setup_purpose_matches_target_role(input.purpose, input.target_role) {
        return Some(SetupInviteFailureReason::WrongTargetRole);
    }

    if input.household_membership != HouseholdMembership::Member {
        return Some(SetupInviteFailureReason::WrongHousehold);
    }

    None
}

fn provisioning_setup_purpose_matches_target_role(
    purpose: SetupInvitePurpose,
    target_role: SetupInviteTargetRole,
) -> bool {
    matches!(
        (purpose, target_role),
        (
            SetupInvitePurpose::CoParentInvite,
            SetupInviteTargetRole::CoParentGuardian
        ) | (
            SetupInvitePurpose::ObserverInvite,
            SetupInviteTargetRole::Observer
        ) | (
            SetupInvitePurpose::ChildDevicePairing,
            SetupInviteTargetRole::ChildDeviceAgent
        ) | (
            SetupInvitePurpose::HouseholdTransfer,
            SetupInviteTargetRole::ParentOwner
        )
    )
}
