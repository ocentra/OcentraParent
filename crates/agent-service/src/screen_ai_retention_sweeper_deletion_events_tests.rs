use crate::screen_ai_retention_sweeper_deletion_events::publish_screen_retention_deletion_events;

#[tokio::test]
async fn screen_retention_deletion_event_publisher_ignores_empty_expiry_set() {
    let store_path = std::env::temp_dir();
    let outcomes = publish_screen_retention_deletion_events(
        &store_path,
        &[],
        ocentra_parent_agent_protocol::constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .await;

    assert!(outcomes.is_empty());
}
