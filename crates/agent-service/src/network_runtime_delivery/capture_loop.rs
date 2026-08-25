use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    activity_capture::{record_activity_capture_once_with_network, ActivityCaptureError},
    fields::fields_from_pairs,
};

pub(super) struct ActivityCaptureFailureReason(pub(super) &'static str);

pub(super) fn spawn() {
    if !crate::activity_capture::startup_activity_capture_enabled() {
        return;
    }
    tokio::task::spawn(async {
        loop {
            if super::reconcile_retained_network_runtime().await.is_err() {
                log_activity_capture_failure(ActivityCaptureFailureReason(
                    constants::network_flow::NETWORK_RUNTIME_STARTUP_RECONCILIATION_FAILURE,
                ));
            }
            run_activity_capture_once_blocking().await;
            tokio::time::sleep(Duration::from_millis(
                constants::activity_capture::RECURRING_CAPTURE_INTERVAL_MS,
            ))
            .await;
        }
    });
}

async fn run_activity_capture_once_blocking() {
    let captured = tokio::task::spawn_blocking(record_activity_capture_once_with_network).await;
    match captured {
        Ok(Ok(captured)) => {
            if super::publish_captured_network_observations(&captured.network_observations)
                .await
                .is_err()
            {
                log_activity_capture_failure(ActivityCaptureFailureReason(
                    constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES,
                ));
            }
        }
        Ok(Err(error)) => log_activity_capture_error(&error),
        Err(_) => log_activity_capture_failure(ActivityCaptureFailureReason(
            constants::value::ACTIVITY_CAPTURE_IO_ERROR,
        )),
    }
}

fn log_activity_capture_error(error: &ActivityCaptureError) {
    log_activity_capture_failure(ActivityCaptureFailureReason(error.reason().0));
}

fn log_activity_capture_failure(reason: ActivityCaptureFailureReason) {
    let _ = crate::dev_log::write_agent_info(
        constants::dev_log_message::ACTIVITY_CAPTURE_FAILED,
        fields_from_pairs(vec![(
            constants::field::REASON,
            LogFieldValue::String(reason.0.to_string()),
        )]),
    );
}
