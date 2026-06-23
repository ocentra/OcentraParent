#[test]
fn expected_place_window_supports_midnight_crossing_boundaries() {
    let window = ocentra_tracking_core::expected_place::TrackingExpectedPlaceWindow {
        start_minute_of_day: 22 * 60,
        end_minute_of_day: 6 * 60,
    };

    assert!(
        ocentra_tracking_core::expected_place::expected_place_window_contains_minute(
            window,
            23 * 60
        )
    );
    assert!(
        ocentra_tracking_core::expected_place::expected_place_window_contains_minute(
            window,
            5 * 60
        )
    );
    assert!(
        !ocentra_tracking_core::expected_place::expected_place_window_contains_minute(
            window,
            12 * 60
        )
    );
}
