use super::*;

pub(super) fn assert_history_state_label(
    state: LanDiscoveryEventHistoryState,
    expected: TestValue,
) {
    let value = projected_subscription_event_json(
        ParentRouteId::Devices,
        lan_status_projection(sample_lan_read_model_with_history_state(state)),
        TestContext("devices subscription event serializes with explicit history state"),
    );

    assert_eq!(
        value["snapshot"]["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]
            ["state"],
        json!(expected.0)
    );
}
