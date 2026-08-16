use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrantError, RemoteAccessGrantState, RemoteAccessGrantTransition,
};

use super::grant::{context_for, paired_grant};

#[test]
fn corrected_device_retry_replaces_its_wrong_device_denial_at_replay_capacity() {
    let mut grant = paired_grant();
    for index in 0..30 {
        let activate_ref =
            Box::leak(format!("attempt-saturated-correction-activate-{index}").into_boxed_str());
        grant
            .transition(
                RemoteAccessGrantTransition::Activate,
                context_for(activate_ref),
            )
            .result
            .expect_value("activate saturated grant");
        let pause_ref =
            Box::leak(format!("attempt-saturated-correction-pause-{index}").into_boxed_str());
        grant
            .transition(RemoteAccessGrantTransition::Pause, context_for(pause_ref))
            .result
            .expect_value("pause saturated grant");
    }
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-saturated-correction-activate-final"),
        )
        .result
        .expect_value("activate before corrected-device denial");
    let mut wrong_device = context_for("attempt-saturated-corrected-device");
    wrong_device.child_device_ref = "child-other";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Pause, wrong_device)
            .result,
        Err(RemoteAccessGrantError::WrongDevice)
    );
    assert_eq!(
        serde_json::to_value(&grant).expect_value("serialize saturated corrected-device grant")
            ["attempts"]
            .as_array()
            .map(Vec::len),
        Some(64)
    );

    let corrected = grant.transition(
        RemoteAccessGrantTransition::Pause,
        context_for("attempt-saturated-corrected-device"),
    );
    assert_eq!(corrected.result, Ok(RemoteAccessGrantState::Paused));
    let encoded = serde_json::to_value(&grant).expect_value("serialize corrected retry");
    let corrected_attempts = encoded["attempts"]
        .as_array()
        .expect_value("replay attempts");
    assert_eq!(corrected_attempts.len(), 64);
    assert_eq!(
        corrected_attempts
            .iter()
            .filter(|attempt| attempt["attemptRef"] == "attempt-saturated-corrected-device")
            .count(),
        1
    );
    assert_eq!(
        corrected_attempts
            .iter()
            .find(|attempt| attempt["attemptRef"] == "attempt-saturated-corrected-device")
            .expect_value("corrected retry milestone")["outcome"],
        serde_json::json!("accepted")
    );
    let replay = grant.transition(
        RemoteAccessGrantTransition::Pause,
        context_for("attempt-saturated-corrected-device"),
    );
    assert_eq!(replay.result, corrected.result);
    assert_eq!(replay.audit, corrected.audit);
}
