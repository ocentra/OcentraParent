use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrant, RemoteAccessGrantTransition,
};

use super::grant::{context_for, paired_grant};
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrantState, RemoteAccessGrantTransitionAuthority,
};

#[test]
fn deserialization_rejects_terminal_snapshot_without_prior_lifecycle_evidence() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Revoke,
            context_for("attempt-terminal-lifecycle-revoke"),
        )
        .result
        .expect_value("revoke paired grant");
    let mut json = serde_json::to_value(&grant).expect_value("serialize terminal grant");
    json["disclosure_state"] = serde_json::json!("undisclosed");
    json["parent_grant"] = serde_json::json!("not-granted");
    assert_invalid_snapshot(json, "terminal lifecycle evidence is required");
}

#[test]
fn deserialization_rejects_duplicate_attempt_references() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-duplicate-reference-activate"),
        )
        .result
        .expect_value("activate grant");
    grant
        .transition(
            RemoteAccessGrantTransition::Pause,
            context_for("attempt-duplicate-reference-pause"),
        )
        .result
        .expect_value("pause grant");
    let mut json = serde_json::to_value(&grant).expect_value("serialize grant");
    let duplicate = json["attempts"][0]["attemptRef"].clone();
    json["attempts"][1]["attemptRef"] = duplicate;
    assert_invalid_snapshot(json, "duplicate replay identity is rejected");
}

#[test]
fn deserialization_rejects_blank_or_untrusted_accepted_actors() {
    let grant = paired_grant();
    let encoded = serde_json::to_value(&grant).expect_value("serialize accepted grant history");
    for actor_ref in [" ", "parent-other"] {
        let mut tampered = encoded.clone();
        tampered["attempts"]
            .as_array_mut()
            .and_then(|attempts| attempts.last_mut())
            .expect_value("accepted pair milestone")["actorRef"] = serde_json::json!(actor_ref);
        assert_invalid_snapshot(tampered, "tampered accepted actor is rejected");
    }
}

#[test]
fn deserialization_preserves_component_system_authority_for_accepted_stops() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-component-system-activate"),
        )
        .result
        .expect_value("activate grant before system stop");
    let mut system_stop = context_for("attempt-component-system-stop");
    system_stop.actor_ref = "watchdog";
    system_stop.parent_authorized = false;
    system_stop.transition_authority = RemoteAccessGrantTransitionAuthority::SystemFailure;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Stop, system_stop)
            .result,
        Ok(RemoteAccessGrantState::Stopped)
    );

    let encoded = serde_json::to_value(&grant).expect_value("serialize component system stop");
    let restored: RemoteAccessGrant =
        serde_json::from_value(encoded.clone()).expect_value("restore component system stop");
    assert_eq!(restored, grant);

    let mut tampered = encoded;
    tampered["attempts"]
        .as_array_mut()
        .and_then(|attempts| attempts.last_mut())
        .expect_value("system stop milestone")["transitionAuthority"] = serde_json::json!("parent");
    assert_invalid_snapshot(tampered, "system actor requires persisted system authority");
}

fn assert_invalid_snapshot(json: serde_json::Value, expectation: &str) {
    let restored = serde_json::from_value::<RemoteAccessGrant>(json);
    let error = restored.err().expect_value(expectation);
    assert_eq!(
        error.to_string().split(" at ").next(),
        Some("serialized grant state violates lifecycle invariants")
    );
}
