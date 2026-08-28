use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::parent_child_runtime_input::ParentChildRuntimeInput;

use crate::test_text::TestText;
use ocentra_parent_agent_core::parent_child_event_runtime::publish_parent_child_runtime_for_validated_intent;
mod parent_child_event_runtime_decode;
use parent_child_event_runtime_decode::expect_no_subscriber;

type TestResult = Result<(), TestText>;

#[tokio::test]
async fn parent_child_runtime_rejects_validated_intent_without_child_receiver() -> TestResult {
    expect_no_subscriber(
        publish_parent_child_runtime_for_validated_intent(
            ParentChildRuntimeInput::validated_review_fixture(),
        )
        .await,
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES,
    )
}

#[tokio::test]
async fn parent_child_transport_handoff_rejects_without_child_receiver() -> TestResult {
    expect_no_subscriber(
        publish_parent_child_runtime_for_validated_intent(
            ParentChildRuntimeInput::validated_review_fixture(),
        )
        .await,
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES,
    )
}

#[tokio::test]
async fn child_agent_receive_rejects_without_child_receiver() -> TestResult {
    expect_no_subscriber(
        publish_parent_child_runtime_for_validated_intent(
            ParentChildRuntimeInput::validated_review_fixture(),
        )
        .await,
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES,
    )
}

#[tokio::test]
async fn browser_action_intent_handoff_rejects_without_child_receiver() -> TestResult {
    expect_no_subscriber(
        publish_parent_child_runtime_for_validated_intent(
            ParentChildRuntimeInput::browser_action_intent_handoff_fixture(),
        )
        .await,
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES,
    )
}
