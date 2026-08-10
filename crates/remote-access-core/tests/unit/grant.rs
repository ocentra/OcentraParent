use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrant, RemoteAccessGrantActorRole, RemoteAccessGrantContext,
    RemoteAccessGrantDisclosureState, RemoteAccessGrantError, RemoteAccessGrantState,
    RemoteAccessGrantTransition,
};

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
    }
}

fn paired_grant() -> RemoteAccessGrant {
    let mut grant = RemoteAccessGrant::request(
        "grant-alpha",
        HOUSEHOLD,
        CHILD,
        PARENT,
        RemoteAccessGrantActorRole::ParentOwner,
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
    assert_eq!(grant.state, RemoteAccessGrantState::Paired);
    assert_eq!(
        grant.disclosure_state,
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
    assert_eq!(grant.state, RemoteAccessGrantState::Active);

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

    assert_eq!(grant.state, RemoteAccessGrantState::Removed);
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
    assert_eq!(grant.state, RemoteAccessGrantState::Paired);
}

#[test]
fn pairing_rejects_wrong_actor_household_device_and_undisclosed_child() {
    let mut grant = RemoteAccessGrant::request(
        "grant-alpha",
        HOUSEHOLD,
        CHILD,
        PARENT,
        RemoteAccessGrantActorRole::ParentOwner,
        "audit-alpha",
    )
    .expect_value("grant request");

    let mut wrong_actor = context();
    wrong_actor.actor_ref = "parent-other";
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
        RemoteAccessGrantActorRole::SupportAdmin,
        "audit-support",
    )
    .expect_value("grant request");
    assert_eq!(
        grant.transition(RemoteAccessGrantTransition::ConfirmParent, context()),
        Err(RemoteAccessGrantError::SupportAccessRequiresParentGrant)
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
    assert_eq!(restored.state, RemoteAccessGrantState::Removed);
}
