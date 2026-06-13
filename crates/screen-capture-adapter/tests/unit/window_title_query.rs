use ocentra_parent_screen_capture_adapter::{
    ScreenCaptureWindowTitleQuery, ScreenCaptureWindowTitleQueryError,
};

#[test]
fn public_window_title_query_rejects_empty_input_before_platform_capture() {
    assert_eq!(
        ScreenCaptureWindowTitleQuery::try_from(String::from(" \t ")),
        Err(ScreenCaptureWindowTitleQueryError::Empty)
    );
}

#[test]
fn public_window_title_query_trims_without_exposing_raw_env_text() {
    let query = ScreenCaptureWindowTitleQuery::try_from(String::from("  Ocentra Parent  "))
        .expect("query should parse");

    assert_eq!(query.as_str(), "Ocentra Parent");
}
