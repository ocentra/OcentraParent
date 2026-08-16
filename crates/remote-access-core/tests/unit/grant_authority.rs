use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrantError, RemoteAccessGrantTransition,
};

use super::grant::{context_for, paired_grant};

#[test]
fn pause_and_reconnect_request_recheck_current_parent_authority() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-pause-authority-activate"),
        )
        .result
        .expect_value("activate grant");
    let mut unauthorized_pause = context_for("attempt-pause-authority-denied");
    unauthorized_pause.parent_authorized = false;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Pause, unauthorized_pause)
            .result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
    grant
        .transition(
            RemoteAccessGrantTransition::Pause,
            context_for("attempt-pause-authority-allowed"),
        )
        .result
        .expect_value("pause grant");
    let mut unauthorized_request = context_for("attempt-reconnect-request-authority-denied");
    unauthorized_request.parent_authorized = false;
    assert_eq!(
        grant
            .transition(
                RemoteAccessGrantTransition::RequestReconnect,
                unauthorized_request,
            )
            .result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
}
