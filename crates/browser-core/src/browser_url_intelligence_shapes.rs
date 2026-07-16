use super::*;

pub(super) fn video_shape(
    platform: &'static str,
    video_id: &str,
    reason_codes: Vec<&'static str>,
) -> ParsedUrlShape {
    ParsedUrlShape {
        target_kind: "video",
        platform,
        platform_ids: BrowserUrlShapePlatformIdsTemplate {
            video_id: Some(video_id.to_string()),
            ..empty_platform_ids()
        },
        confidence: "high",
        reason_codes,
    }
}

pub(super) fn social_post_shape(
    target_kind: &'static str,
    platform: &'static str,
    post_id: &str,
    confidence: &'static str,
) -> ParsedUrlShape {
    ParsedUrlShape {
        target_kind,
        platform,
        platform_ids: BrowserUrlShapePlatformIdsTemplate {
            video_id: is_video_kind(target_kind).then(|| post_id.to_string()),
            post_id: Some(post_id.to_string()),
            ..empty_platform_ids()
        },
        confidence,
        reason_codes: vec!["parsed-url", "parsed-post-id", "parsed-social-route"],
    }
}

pub(super) fn channel_shape(
    platform: &'static str,
    channel_id: &str,
    confidence: &'static str,
) -> ParsedUrlShape {
    ParsedUrlShape {
        target_kind: "channel",
        platform,
        platform_ids: BrowserUrlShapePlatformIdsTemplate {
            channel_id: Some(channel_id.to_string()),
            ..empty_platform_ids()
        },
        confidence,
        reason_codes: vec!["parsed-url", "parsed-channel-id", "parsed-social-route"],
    }
}

pub(super) fn search_shape(platform: &'static str, query: &str) -> ParsedUrlShape {
    ParsedUrlShape {
        target_kind: "search",
        platform,
        platform_ids: BrowserUrlShapePlatformIdsTemplate {
            query: Some(query.to_string()),
            ..empty_platform_ids()
        },
        confidence: "medium",
        reason_codes: vec!["parsed-url", "parsed-search-query", "parsed-social-route"],
    }
}

pub(super) fn social_route_shape(
    target_kind: &'static str,
    platform: &'static str,
    confidence: &'static str,
    reason_codes: Vec<&'static str>,
) -> ParsedUrlShape {
    ParsedUrlShape {
        target_kind,
        platform,
        platform_ids: empty_platform_ids(),
        confidence,
        reason_codes,
    }
}

pub(super) fn simple_shape(
    target_kind: &'static str,
    platform: &'static str,
    confidence: &'static str,
    reason_codes: Vec<&'static str>,
) -> ParsedUrlShape {
    ParsedUrlShape {
        target_kind,
        platform,
        platform_ids: empty_platform_ids(),
        confidence,
        reason_codes,
    }
}

fn is_video_kind(value: &str) -> bool {
    matches!(value, "video" | "short-video")
}

pub(super) fn empty_platform_ids() -> BrowserUrlShapePlatformIdsTemplate {
    BrowserUrlShapePlatformIdsTemplate {
        video_id: None,
        channel_id: None,
        playlist_id: None,
        post_id: None,
        query: None,
    }
}
