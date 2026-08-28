use super::{TestResult, TestText};
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_core::browser_event_runtime::action_handoff_child_status::prove_browser_runtime_action_intent_child_status;
use ocentra_parent_agent_core::browser_event_runtime::action_handoff_child_status_types::BrowserRuntimeActionIntentChildStatusError;
use ocentra_parent_agent_protocol::constants;

#[tokio::test]
async fn browser_runtime_action_intent_child_status_rejects_without_child_receiver() -> TestResult {
    match prove_browser_runtime_action_intent_child_status().await {
        Err(BrowserRuntimeActionIntentChildStatusError::ParentChildRuntime(
            EventingError::NoSubscriber { event_type },
        )) if event_type.as_str() == constants::child_agent::EVENT_COMMAND_RECEIVED => Ok(()),
        Err(error) => Err(TestText::from_display(format!(
            "expected ParentChildRuntime(NoSubscriber({})), got {error:?}",
            constants::child_agent::EVENT_COMMAND_RECEIVED
        ))),
        Ok(_) => Err(TestText::from_display(format!(
            "expected ParentChildRuntime(NoSubscriber({})), got success",
            constants::child_agent::EVENT_COMMAND_RECEIVED
        ))),
    }
}
