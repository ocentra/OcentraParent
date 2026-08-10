use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantContext,
    RemoteAccessGrantDisclosureState, RemoteAccessGrantError, RemoteAccessGrantParentGrant,
    RemoteAccessGrantState, RemoteAccessGrantTransition,
};
use ocentra_schema::remote_capability_fabric::{RemoteActorRole, RemoteRoute};

const HOUSEHOLD: &str = "household-alpha";
const PARENT: &str = "parent-alpha";
const CHILD: &str = "child-alpha";
const ROUTE: RemoteRoute = RemoteRoute::LocalNetwork;

fn context() -> RemoteAccessGrantContext<'static> {
    context_for("attempt-default")
}

fn context_for(attempt_ref: &'static str) -> RemoteAccessGrantContext<'static> {
    RemoteAccessGrantContext {
        household_ref: HOUSEHOLD,
        actor_ref: PARENT,
        child_device_ref: CHILD,
        route: ROUTE,
        attempt_ref,
        parent_authorized: true,
        child_disclosed: true,
        parent_grant_approved: true,
    }
}

fn paired_grant() -> RemoteAccessGrant {
    let mut grant = RemoteAccessGrant::request(
        "grant-alpha",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-alpha",
    )
    .expect_value("grant request");
    grant
        .transition(RemoteAccessGrantTransition::ConfirmParent, context())
        .result
        .expect_value("parent confirmation");
    grant
        .transition(RemoteAccessGrantTransition::Pair, context())
        .result
        .expect_value("pairing");
    grant
}

#[test]
fn pairing_creates_standing_disclosed_access_until_revoke_or_remove() {
    let mut grant = paired_grant();
    assert_eq!(grant.state(), RemoteAccessGrantState::Paired);
    assert_eq!(
        grant.disclosure_state(),
        RemoteAccessGrantDisclosureState::Disclosed
    );

    grant
        .transition(RemoteAccessGrantTransition::Activate, context())
        .result
        .expect_value("activate");
    grant
        .transition(RemoteAccessGrantTransition::Pause, context())
        .result
        .expect_value("pause");
    grant
        .transition(RemoteAccessGrantTransition::RequestReconnect, context())
        .result
        .expect_value("reconnect request");
    grant
        .transition(RemoteAccessGrantTransition::Reconnect, context())
        .result
        .expect_value("reconnect");
    assert_eq!(grant.state(), RemoteAccessGrantState::Active);

    grant
        .transition(RemoteAccessGrantTransition::Revoke, context())
        .result
        .expect_value("revoke");
    assert!(!grant.can_reconnect());
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Reconnect, context())
            .result,
        Err(RemoteAccessGrantError::ReconnectDenied)
    );
}

#[test]
fn removed_device_cannot_reconnect_or_be_reactivated() {
    let mut grant = paired_grant();
    grant
        .transition(RemoteAccessGrantTransition::Activate, context())
        .result
        .expect_value("activate");
    grant
        .transition(RemoteAccessGrantTransition::RemoveDevice, context())
        .result
        .expect_value("remove device");

    assert_eq!(grant.state(), RemoteAccessGrantState::Removed);
    assert!(!grant.can_reconnect());
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Activate, context())
            .result,
        Err(RemoteAccessGrantError::InvalidTransition)
    );
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Reconnect, context())
            .result,
        Err(RemoteAccessGrantError::ReconnectDenied)
    );
}

#[test]
fn revoke_and_remove_require_parent_authority() {
    let mut grant = paired_grant();
    let mut unauthorized = context();
    unauthorized.parent_authorized = false;

    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Revoke, unauthorized)
            .result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::RemoveDevice, unauthorized)
            .result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
    assert_eq!(grant.state(), RemoteAccessGrantState::Paired);
}

#[test]
fn pairing_rejects_wrong_actor_household_device_and_undisclosed_child() {
    let mut grant = RemoteAccessGrant::request(
        "grant-alpha",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-alpha",
    )
    .expect_value("grant request");

    let mut wrong_actor = context();
    wrong_actor.actor_ref = "parent-other";
    wrong_actor.parent_authorized = false;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::ConfirmParent, wrong_actor)
            .result,
        Err(RemoteAccessGrantError::WrongActor)
    );

    let mut wrong_household = context();
    wrong_household.household_ref = "household-other";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::ConfirmParent, wrong_household)
            .result,
        Err(RemoteAccessGrantError::WrongHousehold)
    );

    let mut wrong_device = context();
    wrong_device.child_device_ref = "child-other";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::ConfirmParent, wrong_device)
            .result,
        Err(RemoteAccessGrantError::WrongDevice)
    );

    grant
        .transition(RemoteAccessGrantTransition::ConfirmParent, context())
        .result
        .expect_value("parent confirmation");
    let mut undisclosed = context();
    undisclosed.child_disclosed = false;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Pair, undisclosed)
            .result,
        Err(RemoteAccessGrantError::ChildDisclosureRequired)
    );
}

#[test]
fn support_role_cannot_create_hidden_standing_access() {
    let mut grant = RemoteAccessGrant::request(
        "grant-support",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::SupportAdmin,
        "audit-support",
    )
    .expect_value("grant request");
    let mut hidden_context = context();
    hidden_context.parent_grant_approved = false;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::ConfirmParent, hidden_context)
            .result,
        Err(RemoteAccessGrantError::SupportAccessRequiresParentGrant)
    );
}

#[test]
fn explicitly_approved_support_access_is_parent_visible_and_audited() {
    let mut grant = RemoteAccessGrant::request(
        "grant-support",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::SupportAdmin,
        "audit-support",
    )
    .expect_value("grant request");
    grant
        .transition(RemoteAccessGrantTransition::ConfirmParent, context())
        .result
        .expect_value("approved support confirmation");
    grant
        .transition(RemoteAccessGrantTransition::Pair, context())
        .result
        .expect_value("approved support pairing");
    assert_eq!(grant.parent_grant(), RemoteAccessGrantParentGrant::Granted);
    assert_eq!(
        grant.disclosure_state(),
        RemoteAccessGrantDisclosureState::Disclosed
    );
}

#[test]
fn pairing_rechecks_current_parent_authority() {
    let mut grant = RemoteAccessGrant::request(
        "grant-alpha",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-alpha",
    )
    .expect_value("grant request");
    grant
        .transition(RemoteAccessGrantTransition::ConfirmParent, context())
        .result
        .expect_value("parent confirmation");
    let mut revoked_authority = context();
    revoked_authority.parent_authorized = false;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Pair, revoked_authority)
            .result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
}

#[test]
fn authorized_household_actor_can_revoke_another_parent_grant() {
    let mut grant = paired_grant();
    let mut other_parent = context();
    other_parent.actor_ref = "parent-other";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Revoke, other_parent)
            .result,
        Ok(RemoteAccessGrantState::Revoked)
    );
}

#[test]
fn authorized_other_actor_cannot_start_or_restore_another_parent_grant() {
    let mut requested = RemoteAccessGrant::request(
        "grant-cross-actor",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-cross-actor",
    )
    .expect_value("grant request");
    let mut other_parent = context_for("attempt-cross-actor-confirm");
    other_parent.actor_ref = "parent-other";
    assert_eq!(
        requested
            .transition(RemoteAccessGrantTransition::ConfirmParent, other_parent)
            .result,
        Err(RemoteAccessGrantError::WrongActor)
    );

    let mut paired = paired_grant();
    let mut other_parent = context_for("attempt-cross-actor-activate");
    other_parent.actor_ref = "parent-other";
    assert_eq!(
        paired
            .transition(RemoteAccessGrantTransition::Activate, other_parent)
            .result,
        Err(RemoteAccessGrantError::WrongActor)
    );
}

#[test]
fn transition_report_exposes_accepted_and_denied_redacted_audit_milestones() {
    let mut grant = RemoteAccessGrant::request(
        "grant-audit",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-report",
    )
    .expect_value("grant request");
    let accepted =
        grant.transition_with_audit(RemoteAccessGrantTransition::ConfirmParent, context());
    assert_eq!(accepted.result, Ok(RemoteAccessGrantState::ParentConfirmed));
    assert_eq!(
        accepted.audit.outcome,
        RemoteAccessGrantAuditOutcome::Accepted
    );
    assert!(accepted.audit.error.is_none());
    assert_eq!(accepted.audit.route, ROUTE);
    assert_eq!(accepted.audit.attempt_ref, "attempt-default");
    accepted.audit.contract().expect_value("audit contract");

    let mut denied_context = context();
    denied_context.parent_authorized = false;
    let denied = grant.transition_with_audit(RemoteAccessGrantTransition::Pair, denied_context);
    assert_eq!(
        denied.result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
    assert_eq!(denied.audit.outcome, RemoteAccessGrantAuditOutcome::Denied);
    assert_eq!(
        denied.audit.error,
        Some(RemoteAccessGrantError::ParentAuthorityRequired)
    );
}

#[test]
fn default_transition_returns_an_audited_report() {
    let mut grant = RemoteAccessGrant::request(
        "grant-default-report",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-default-report",
    )
    .expect_value("grant request");

    let report = grant.transition(
        RemoteAccessGrantTransition::ConfirmParent,
        context_for("attempt-default-report"),
    );
    assert_eq!(report.result, Ok(RemoteAccessGrantState::ParentConfirmed));
    assert_eq!(report.audit.route, ROUTE);
    assert_eq!(report.audit.attempt_ref, "attempt-default-report");
}

#[test]
fn activate_and_reconnect_recheck_current_parent_authority() {
    let mut grant = paired_grant();
    let mut revoked_authority = context_for("attempt-activate-revoked");
    revoked_authority.parent_authorized = false;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Activate, revoked_authority)
            .result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );

    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-activate"),
        )
        .result
        .expect_value("activate");
    grant
        .transition(
            RemoteAccessGrantTransition::Pause,
            context_for("attempt-pause"),
        )
        .result
        .expect_value("pause");
    grant
        .transition(
            RemoteAccessGrantTransition::RequestReconnect,
            context_for("attempt-reconnect-request"),
        )
        .result
        .expect_value("reconnect request");

    let mut revoked_authority = context_for("attempt-reconnect-revoked");
    revoked_authority.parent_authorized = false;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Reconnect, revoked_authority)
            .result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
}

#[test]
fn reconnect_pending_cannot_bypass_the_reconnect_transition() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-bypass-activate"),
        )
        .result
        .expect_value("activate");
    grant
        .transition(
            RemoteAccessGrantTransition::Pause,
            context_for("attempt-bypass-pause"),
        )
        .result
        .expect_value("pause");
    grant
        .transition(
            RemoteAccessGrantTransition::RequestReconnect,
            context_for("attempt-bypass-request"),
        )
        .result
        .expect_value("reconnect request");
    assert_eq!(
        grant
            .transition(
                RemoteAccessGrantTransition::Activate,
                context_for("attempt-bypass-activate-pending"),
            )
            .result,
        Err(RemoteAccessGrantError::InvalidTransition)
    );
}

#[test]
fn route_mismatch_is_denied_before_lifecycle_transition() {
    let mut grant = paired_grant();
    let mut wrong_route = context_for("attempt-wrong-route");
    wrong_route.route = RemoteRoute::CloudRelay;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Activate, wrong_route)
            .result,
        Err(RemoteAccessGrantError::WrongRoute)
    );
}

#[test]
fn early_terminal_states_round_trip_with_lifecycle_evidence() {
    let mut requested = RemoteAccessGrant::request(
        "grant-requested-terminal",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-requested-terminal",
    )
    .expect_value("grant request");
    requested
        .transition(
            RemoteAccessGrantTransition::Revoke,
            context_for("attempt-requested-revoke"),
        )
        .result
        .expect_value("revoke requested grant");
    let restored_requested: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&requested).expect_value("serialize requested terminal"),
    )
    .expect_value("deserialize requested terminal");
    assert_eq!(restored_requested, requested);

    let mut confirmed = RemoteAccessGrant::request(
        "grant-confirmed-terminal",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-confirmed-terminal",
    )
    .expect_value("grant request");
    confirmed
        .transition(
            RemoteAccessGrantTransition::ConfirmParent,
            context_for("attempt-confirm-parent"),
        )
        .result
        .expect_value("confirm parent");
    confirmed
        .transition(
            RemoteAccessGrantTransition::RemoveDevice,
            context_for("attempt-confirmed-remove"),
        )
        .result
        .expect_value("remove confirmed grant");
    let restored_confirmed: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&confirmed).expect_value("serialize confirmed terminal"),
    )
    .expect_value("deserialize confirmed terminal");
    assert_eq!(restored_confirmed, confirmed);
}

#[test]
fn audit_attempt_refs_are_unique_per_attempt_and_stable_on_retry() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-cycle-activate"),
        )
        .result
        .expect_value("activate");
    let pause_one = grant.transition_with_audit(
        RemoteAccessGrantTransition::Pause,
        context_for("attempt-cycle-pause-one"),
    );
    pause_one.result.expect_value("pause one");
    let reconnect_request = grant.transition_with_audit(
        RemoteAccessGrantTransition::RequestReconnect,
        context_for("attempt-cycle-reconnect-request"),
    );
    reconnect_request.result.expect_value("reconnect request");
    grant
        .transition(
            RemoteAccessGrantTransition::Reconnect,
            context_for("attempt-cycle-reconnect"),
        )
        .result
        .expect_value("reconnect");
    let pause_two = grant.transition_with_audit(
        RemoteAccessGrantTransition::Pause,
        context_for("attempt-cycle-pause-two"),
    );
    pause_two.result.expect_value("pause two");
    assert_ne!(
        pause_one.audit.idempotency_key(),
        pause_two.audit.idempotency_key()
    );

    let denied_one = grant.transition_with_audit(
        RemoteAccessGrantTransition::Reconnect,
        context_for("attempt-cycle-denied"),
    );
    let denied_two = grant.transition_with_audit(
        RemoteAccessGrantTransition::Reconnect,
        context_for("attempt-cycle-denied"),
    );
    assert_eq!(
        denied_one.result,
        Err(RemoteAccessGrantError::ReconnectDenied)
    );
    assert_eq!(
        denied_two.result,
        Err(RemoteAccessGrantError::ReconnectDenied)
    );
    assert_eq!(
        denied_one.audit.idempotency_key(),
        denied_two.audit.idempotency_key()
    );
}

#[test]
fn deserialization_rejects_state_without_required_lifecycle_evidence() {
    let grant = paired_grant();
    let mut json = serde_json::to_value(&grant).expect_value("serialize grant");
    json["state"] = serde_json::json!("active");
    json["disclosure_state"] = serde_json::json!("undisclosed");
    let restored = serde_json::from_value::<RemoteAccessGrant>(json);
    let error = restored
        .err()
        .expect_value("invalid lifecycle snapshot is rejected");
    assert_eq!(
        error.to_string().split(" at ").next(),
        Some("serialized grant state violates lifecycle invariants")
    );
}

#[test]
fn grant_round_trips_without_losing_terminal_state() {
    let mut grant = paired_grant();
    grant
        .transition(RemoteAccessGrantTransition::Activate, context())
        .result
        .expect_value("activate");
    grant
        .transition(RemoteAccessGrantTransition::RemoveDevice, context())
        .result
        .expect_value("remove device");
    let json = serde_json::to_value(&grant).expect_value("serialize grant");
    let restored: RemoteAccessGrant =
        serde_json::from_value(json).expect_value("deserialize grant");
    assert_eq!(restored, grant);
    assert_eq!(restored.state(), RemoteAccessGrantState::Removed);
}
