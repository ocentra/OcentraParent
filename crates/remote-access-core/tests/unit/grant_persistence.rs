use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrant, RemoteAccessGrantTransition,
};

use super::grant::{context_for, paired_grant};

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

fn assert_invalid_snapshot(json: serde_json::Value, expectation: &str) {
    let restored = serde_json::from_value::<RemoteAccessGrant>(json);
    let error = restored.err().expect_value(expectation);
    assert_eq!(
        error.to_string().split(" at ").next(),
        Some("serialized grant state violates lifecycle invariants")
    );
}
