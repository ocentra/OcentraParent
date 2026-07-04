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
    assert_eq!(
        ScreenCaptureWindowTitleQuery::try_from(String::from("  Ocentra Parent  "))
            .as_ref()
            .map(ScreenCaptureWindowTitleQuery::as_str),
        Ok("Ocentra Parent")
    );
}
