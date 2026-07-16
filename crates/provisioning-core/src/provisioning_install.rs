#![forbid(unsafe_code)]

//! Setup, install, and provisioning ownership boundary.
//!
//! This crate owns install journey state, pairing readiness, permission
//! onboarding, recovery, and provisioning contracts. Binary updater mechanics
//! remain in the updater crate.

use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_family_identity_core::family_identity::{
    DeviceOwnershipScope, DeviceTrustState, HouseholdMembershipState,
};
use ocentra_family_identity_core::household_authority::{
    authorize_household_action, HouseholdAuthorityDecision, HouseholdAuthorityInput,
    HouseholdAuthorizationFailureReason, HouseholdAuthorizationState,
};
use ocentra_family_identity_core::session_lifecycle::{
    authorize_session_token_action, SessionTokenFailureReason, SessionTokenInput,
};
use ocentra_family_identity_core::setup_lifecycle::{
    authorize_setup_invite, device_trust_state_for_recovery_operation, evaluate_recovery_operation,
    RecoveryDataCustodyHandoffState, RecoveryDecision, RecoveryKind as FamilyRecoveryKind,
    RecoveryOperation as FamilyRecoveryOperation, RecoveryState as FamilyRecoveryState,
    RecoverySupportChannel, SetupInviteFailureReason, SetupInviteInput, SetupInvitePurpose,
    SetupInviteState, SetupInviteTargetRole,
};
use serde::{Deserialize, Serialize};

mod family_account;
mod family_context;
mod family_pairing;
mod family_recovery;
mod readiness_actions;
mod readiness_blockers;
mod readiness_logic;

pub const CRATE_NAME: &str = "ocentra-provisioning-core";
const PROVISIONING_SCHEMA_VERSION: u16 = 1;
const PROVISIONING_READINESS_EVALUATED_EVENT_TYPE: &str = "provisioning.readiness.evaluated";
const PROVISIONING_ACTION_PLANNED_EVENT_TYPE: &str = "provisioning.action.planned";
const PROVISIONING_IDEMPOTENCY_SEPARATOR: &str = ":";
const PROVISIONING_ACTION_PREFIX: &str = "provisioning-action:";

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
    pub membership_state: HouseholdMembershipState,
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
                parse_provisioning_text_id(value, $field).map(Self)
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

fn parse_provisioning_text_id(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }
    Ok(value)
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
        provisioning_event_contract(EventType::parse(
            PROVISIONING_READINESS_EVALUATED_EVENT_TYPE,
        )?)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        provisioning_idempotency_key(
            &EventType::parse(PROVISIONING_READINESS_EVALUATED_EVENT_TYPE)?,
            &self.evaluation_id,
        )
    }
}

impl DomainEvent for ProvisioningActionPlannedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        provisioning_event_contract(EventType::parse(PROVISIONING_ACTION_PLANNED_EVENT_TYPE)?)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        provisioning_idempotency_key(
            &EventType::parse(PROVISIONING_ACTION_PLANNED_EVENT_TYPE)?,
            &self.action_plan_id,
        )
    }
}

pub fn evaluate_provisioning_readiness(
    input: ProvisioningReadinessInput,
) -> ProvisioningReadinessDecision {
    let blocker_reason = readiness_logic::provisioning_blocker_reason(input);
    let overall_state = readiness_logic::provisioning_overall_state(blocker_reason);

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
    family_context::derive_provisioning_readiness_input_from_family_context(input)
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
        recovery_action: readiness_logic::provisioning_recovery_action(decision.blocker_reason),
        audit_state: ProvisioningAuditState::Record,
    }
}

pub fn provisioning_action_planned_event(
    event: ProvisioningReadinessEvaluatedEvent,
) -> ProvisioningActionPlannedEvent {
    ProvisioningActionPlannedEvent {
        aggregate_id: event.aggregate_id,
        action_plan_id: ProvisioningActionPlanId(provisioning_action_ref(&event.evaluation_id)),
        source_evaluation_id: event.evaluation_id,
        action_plan: plan_provisioning_actions(event.input),
    }
}

fn provisioning_event_contract(event_type: EventType) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        event_type,
        SchemaVersion::new(PROVISIONING_SCHEMA_VERSION)?,
    ))
}

fn provisioning_idempotency_key(
    event_type: &EventType,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type.as_str(),
        PROVISIONING_IDEMPOTENCY_SEPARATOR,
        unique_ref
    ))
}

fn provisioning_action_ref(evaluation_id: &ProvisioningReadinessEvaluationId) -> String {
    let mut value = String::from(PROVISIONING_ACTION_PREFIX);
    value.push_str(evaluation_id.as_str());
    value
}
