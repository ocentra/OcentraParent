use super::*;

pub(super) fn path_segments_for(pathname: &BrowserGameUrlText) -> Vec<BrowserGameUrlText> {
    pathname
        .0
        .split('/')
        .map(|segment| BrowserGameUrlText(segment.trim().to_ascii_lowercase()))
        .filter(|segment| !segment.0.is_empty())
        .collect()
}

pub(super) fn path_depth_for(segments: &[BrowserGameUrlText]) -> BrowserGameShapeCode {
    match segments.len() {
        0 => PATH_ROOT,
        1 => PATH_ONE_SEGMENT,
        2 => PATH_TWO_SEGMENTS,
        _ => PATH_THREE_OR_MORE_SEGMENTS,
    }
}

pub(super) fn route_hints_for(segments: &[BrowserGameUrlText]) -> BrowserGameRouteHints {
    BrowserGameRouteHints {
        has_embed_hint: segments
            .iter()
            .any(|segment| segment.0 == SEGMENT_EMBED || segment.0 == SEGMENT_IFRAME),
        has_play_hint: segments
            .iter()
            .any(|segment| segment.0 == SEGMENT_PLAY || segment.0 == SEGMENT_LAUNCH),
        has_account_hint: segments.iter().any(|segment| {
            segment.0 == SEGMENT_ACCOUNT
                || segment.0 == SEGMENT_LOGIN
                || segment.0 == SEGMENT_SIGNUP
        }),
        has_purchase_hint: segments.iter().any(|segment| {
            segment.0 == SEGMENT_BUY || segment.0 == SEGMENT_STORE || segment.0 == SEGMENT_CHECKOUT
        }),
        has_cloud_session_hint: segments.iter().any(|segment| {
            segment.0 == SEGMENT_CLOUD
                || segment.0 == SEGMENT_STREAM
                || segment.0 == SEGMENT_SESSION
        }),
    }
}

pub(super) fn segment_looks_like_game_id(segment: &BrowserGameUrlText) -> bool {
    segment.0.len() >= 4
        && (segment
            .0
            .chars()
            .any(|character| character.is_ascii_digit())
            || segment.0.contains('-'))
}
