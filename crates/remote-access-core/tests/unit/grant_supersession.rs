use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrant, RemoteAccessGrantError, RemoteAccessGrantState, RemoteAccessGrantTransition,
    RemoteAccessGrantTransitionAuthority,
};
use ocentra_schema::remote_capability_fabric::RemoteActorRole;

use super::grant::{context_for, paired_grant, CHILD, HOUSEHOLD, PARENT, ROUTE};

#[test]
fn system_failure_can_stop_revoke_or_remove_without_parent_authority() {
    let mut revoked = paired_grant();
    let mut system_revoke = context_for("attempt-system-revoke");
    system_revoke.actor_ref = "system-failure";
    system_revoke.parent_authorized = false;
    system_revoke.transition_authority = RemoteAccessGrantTransitionAuthority::SystemFailure;
    revoked
        .transition(RemoteAccessGrantTransition::Revoke, system_revoke)
        .result
        .expect_value("system revoke");
    assert_eq!(revoked.state(), RemoteAccessGrantState::Revoked);

    let mut removed = paired_grant();
    let mut system_remove = context_for("attempt-system-remove");
    system_remove.actor_ref = "system-failure";
    system_remove.parent_authorized = false;
    system_remove.transition_authority = RemoteAccessGrantTransitionAuthority::SystemFailure;
    removed
        .transition(RemoteAccessGrantTransition::RemoveDevice, system_remove)
        .result
        .expect_value("system remove");
    assert_eq!(removed.state(), RemoteAccessGrantState::Removed);

    let mut stopped = paired_grant();
    stopped
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-system-stop-activate"),
        )
        .result
        .expect_value("activate before system stop");
    let mut system_stop = context_for("attempt-system-stop");
    system_stop.actor_ref = "system-failure";
    system_stop.parent_authorized = false;
    system_stop.transition_authority = RemoteAccessGrantTransitionAuthority::SystemFailure;
    stopped
        .transition(RemoteAccessGrantTransition::Stop, system_stop)
        .result
        .expect_value("system stop");
    assert_eq!(stopped.state(), RemoteAccessGrantState::Stopped);
}

#[test]
fn same_scope_rotation_supersedes_the_old_grant_and_survives_restore() {
    let mut old_grant = paired_grant();
    old_grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-supersede-activate"),
        )
        .result
        .expect_value("activate old grant");

    let replacement = RemoteAccessGrant::request(
        "grant-rotated",
        HOUSEHOLD,
        CHILD,
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-rotated",
    )
    .expect_value("replacement grant");
    let report = old_grant.supersede_with(&replacement, context_for("attempt-supersede-rotated"));
    report.result.expect_value("supersede old grant");
    assert_eq!(old_grant.state(), RemoteAccessGrantState::Superseded);
    assert_eq!(old_grant.superseded_by(), Some("grant-rotated"));
    assert!(!old_grant.can_reconnect());
    assert_eq!(
        old_grant
            .transition(
                RemoteAccessGrantTransition::Reconnect,
                context_for("attempt-supersede-reconnect"),
            )
            .result,
        Err(RemoteAccessGrantError::ReconnectDenied)
    );

    let restored: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&old_grant).expect_value("serialize superseded grant"),
    )
    .expect_value("deserialize superseded grant");
    assert_eq!(restored.state(), RemoteAccessGrantState::Superseded);
    assert_eq!(restored.superseded_by(), Some("grant-rotated"));
}

#[test]
fn supersession_rejects_a_different_scope_or_missing_replacement() {
    let mut grant = paired_grant();
    let replacement = RemoteAccessGrant::request(
        "grant-other-device",
        HOUSEHOLD,
        "child-other",
        ROUTE,
        PARENT,
        RemoteActorRole::ParentOwner,
        "audit-other-device",
    )
    .expect_value("replacement grant");
    assert_eq!(
        grant
            .supersede_with(&replacement, context_for("attempt-supersede-mismatch"))
            .result,
        Err(RemoteAccessGrantError::SupersedingGrantMismatch)
    );
    assert_eq!(grant.state(), RemoteAccessGrantState::Paired);
    assert_eq!(
        grant
            .transition(
                RemoteAccessGrantTransition::Supersede,
                context_for("attempt-supersede-missing"),
            )
            .result,
        Err(RemoteAccessGrantError::SupersedingGrantRequired)
    );
}
