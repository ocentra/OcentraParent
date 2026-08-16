use crate::screen_ai_retention_sweeper_deletion_events::publish_screen_retention_deletion_events;

#[tokio::test]
async fn screen_retention_deletion_event_publisher_ignores_empty_expiry_set() {
    let store_path = std::env::temp_dir();
    let runtime = crate::test_invariants::require_ok(
        crate::screen_ai_service_event_subscription::ScreenAiServiceEventRuntime::start().await,
        ocentra_parent_agent_protocol::constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let outcomes = publish_screen_retention_deletion_events(
        &runtime,
        &store_path,
        &[],
        crate::screen_ai_service_event_subscription::ObservedAtText(
            ocentra_parent_agent_protocol::constants::activity_store::TEST_SECOND_OBSERVED_AT
                .to_string(),
        ),
    )
    .await;

    assert!(outcomes.is_empty());
}
