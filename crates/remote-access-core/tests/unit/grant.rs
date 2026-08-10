use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantContext,
    RemoteAccessGrantDisclosureState, RemoteAccessGrantError, RemoteAccessGrantParentGrant,
    RemoteAccessGrantState, RemoteAccessGrantTransition,
};
use ocentra_schema::remote_capability_fabric::RemoteActorRole;

const HOUSEHOLD: &str = "household-alpha";
const PARENT: &str = "parent-alpha";
const CHILD: &str = "child-alpha";

fn context() -> RemoteAccessGrantContext<'static> {
    RemoteAccessGrantContext {
        household_ref: HOUSEHOLD,
        actor_ref: PARENT,
        child_device_ref: CHILD,
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
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-alpha",
    )
    .expect_value("grant request");
    grant
        .transition(RemoteAccessGrantTransition::ConfirmParent, context())
        .expect_value("parent confirmation");
    grant
        .transition(RemoteAccessGrantTransition::Pair, context())
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
        .expect_value("activate");
    grant
        .transition(RemoteAccessGrantTransition::Pause, context())
        .expect_value("pause");
    grant
        .transition(RemoteAccessGrantTransition::RequestReconnect, context())
        .expect_value("reconnect request");
    grant
        .transition(RemoteAccessGrantTransition::Reconnect, context())
        .expect_value("reconnect");
    assert_eq!(grant.state(), RemoteAccessGrantState::Active);

    grant
        .transition(RemoteAccessGrantTransition::Revoke, context())
        .expect_value("revoke");
    assert!(!grant.can_reconnect());
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::Reconnect, context()),
        Err(RemoteAccessGrantError::ReconnectDenied)
    );
}

#[test]
fn removed_device_cannot_reconnect_or_be_reactivated() {
    let mut grant = paired_grant();
    grant
        .transition(RemoteAccessGrantTransition::Activate, context())
        .expect_value("activate");
    grant
        .transition(RemoteAccessGrantTransition::RemoveDevice, context())
        .expect_value("remove device");

    assert_eq!(grant.state(), RemoteAccessGrantState::Removed);
    assert!(!grant.can_reconnect());
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::Activate, context()),
        Err(RemoteAccessGrantError::InvalidTransition)
    );
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::Reconnect, context()),
        Err(RemoteAccessGrantError::ReconnectDenied)
    );
}

#[test]
fn revoke_and_remove_require_parent_authority() {
    let mut grant = paired_grant();
    let mut unauthorized = context();
    unauthorized.parent_authorized = false;

    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::Revoke, unauthorized),
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::RemoveDevice, unauthorized),
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
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-alpha",
    )
    .expect_value("grant request");

    let mut wrong_actor = context();
    wrong_actor.actor_ref = "parent-other";
    wrong_actor.parent_authorized = false;
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::ConfirmParent, wrong_actor),
        Err(RemoteAccessGrantError::WrongActor)
    );

    let mut wrong_household = context();
    wrong_household.household_ref = "household-other";
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::ConfirmParent, wrong_household),
        Err(RemoteAccessGrantError::WrongHousehold)
    );

    let mut wrong_device = context();
    wrong_device.child_device_ref = "child-other";
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::ConfirmParent, wrong_device),
        Err(RemoteAccessGrantError::WrongDevice)
    );

    grant
        .transition(RemoteAccessGrantTransition::ConfirmParent, context())
        .expect_value("parent confirmation");
    let mut undisclosed = context();
    undisclosed.child_disclosed = false;
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::Pair, undisclosed),
        Err(RemoteAccessGrantError::ChildDisclosureRequired)
    );
}

#[test]
fn support_role_cannot_create_hidden_standing_access() {
    let mut grant = RemoteAccessGrant::request(
        "grant-support",
        HOUSEHOLD,
        CHILD,
        PARENT,
        RemoteActorRole::SupportAdmin,
        "audit-support",
    )
    .expect_value("grant request");
    let mut hidden_context = context();
    hidden_context.parent_grant_approved = false;
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::ConfirmParent, hidden_context),
        Err(RemoteAccessGrantError::SupportAccessRequiresParentGrant)
    );
}

#[test]
fn explicitly_approved_support_access_is_parent_visible_and_audited() {
    let mut grant = RemoteAccessGrant::request(
        "grant-support",
        HOUSEHOLD,
        CHILD,
        PARENT,
        RemoteActorRole::SupportAdmin,
        "audit-support",
    )
    .expect_value("grant request");
    grant
        .transition(RemoteAccessGrantTransition::ConfirmParent, context())
        .expect_value("approved support confirmation");
    grant
        .transition(RemoteAccessGrantTransition::Pair, context())
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
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-alpha",
    )
    .expect_value("grant request");
    grant
        .transition(RemoteAccessGrantTransition::ConfirmParent, context())
        .expect_value("parent confirmation");
    let mut revoked_authority = context();
    revoked_authority.parent_authorized = false;
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::Pair, revoked_authority),
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
}

#[test]
fn authorized_household_actor_can_revoke_another_parent_grant() {
    let mut grant = paired_grant();
    let mut other_parent = context();
    other_parent.actor_ref = "parent-other";
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::Revoke, other_parent),
        Ok(RemoteAccessGrantState::Revoked)
    );
}

#[test]
fn transition_report_exposes_accepted_and_denied_redacted_audit_milestones() {
    let mut grant = RemoteAccessGrant::request(
        "grant-audit",
        HOUSEHOLD,
        CHILD,
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
        .expect_value("activate");
    grant
        .transition(RemoteAccessGrantTransition::RemoveDevice, context())
        .expect_value("remove device");
    let json = serde_json::to_value(&grant).expect_value("serialize grant");
    let restored: RemoteAccessGrant =
        serde_json::from_value(json).expect_value("deserialize grant");
    assert_eq!(restored, grant);
    assert_eq!(restored.state(), RemoteAccessGrantState::Removed);
}
