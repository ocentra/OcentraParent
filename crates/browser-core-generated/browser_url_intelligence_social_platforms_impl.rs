use super::*;

pub(super) fn facebook_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(segments.first().map(String::as_str), Some("watch")) {
        if let Some(video_id) = query_param(parsed, "v") {
            return video_shape("facebook", &video_id, vec!["parsed-url"]);
        }
    }
    if matches!(
        segments.first().map(String::as_str),
        Some("reel" | "videos")
    ) {
        if let Some(post_id) = segments.get(1) {
            let target_kind = if segments[0] == "reel" {
                "short-video"
            } else {
                "video"
            };
            return social_post_shape(target_kind, "facebook", post_id, "high");
        }
    }
    if segments.is_empty()
        || matches!(
            segments.first().map(String::as_str),
            Some("home" | "feed" | "watch")
        )
    {
        return social_route_shape(
            "social-feed",
            "facebook",
            "medium",
            vec!["parsed-url", "dynamic-feed", "parsed-social-route"],
        );
    }
    if matches!(segments.first().map(String::as_str), Some("live")) {
        return social_route_shape(
            "social-livestream",
            "facebook",
            "medium",
            vec!["parsed-url", "parsed-social-route"],
        );
    }
    channel_shape(
        "facebook",
        segments
            .first()
            .map(String::as_str)
            .unwrap_or(parsed.domain.as_str()),
        "medium",
    )
}

pub(super) fn twitch_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(segments.first().map(String::as_str), Some("videos")) {
        if let Some(video_id) = segments.get(1) {
            return video_shape("twitch", video_id, vec!["parsed-url"]);
        }
    }
    if matches!(
        segments.first().map(String::as_str),
        Some("directory" | "following")
    ) {
        return social_route_shape(
            "social-feed",
            "twitch",
            "medium",
            vec!["parsed-url", "dynamic-feed", "parsed-social-route"],
        );
    }
    if let Some(channel_id) = segments.first() {
        return channel_shape("twitch", channel_id, "medium");
    }
    social_route_shape(
        "social-feed",
        "twitch",
        "medium",
        vec!["parsed-url", "dynamic-feed", "parsed-social-route"],
    )
}

pub(super) fn x_twitter_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(segments.first().map(String::as_str), Some("search")) {
        if let Some(query) = query_param(parsed, "q") {
            return search_shape("x-twitter", &query);
        }
    }
    if matches!(segments.get(1).map(String::as_str), Some("status")) {
        if let Some(post_id) = segments.get(2) {
            return social_post_shape("social-post", "x-twitter", post_id, "high");
        }
    }
    if segments.is_empty()
        || matches!(
            segments.first().map(String::as_str),
            Some("home" | "explore")
        )
    {
        return social_route_shape(
            "social-feed",
            "x-twitter",
            "medium",
            vec!["parsed-url", "dynamic-feed", "parsed-social-route"],
        );
    }
    channel_shape("x-twitter", segments[0].as_str(), "medium")
}

pub(super) fn reddit_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(segments.first().map(String::as_str), Some("r"))
        && segments.len() >= 4
        && segments[2] == "comments"
    {
        return social_post_shape("social-post", "reddit", &segments[3], "medium");
    }
    if matches!(segments.first().map(String::as_str), Some("r")) && segments.get(1).is_some() {
        return ParsedUrlShape {
            target_kind: "forum",
            platform: "reddit",
            platform_ids: BrowserUrlShapePlatformIdsTemplate {
                channel_id: Some(segments[1].to_string()),
                ..empty_platform_ids()
            },
            confidence: "medium",
            reason_codes: vec!["parsed-url", "parsed-channel-id", "parsed-social-route"],
        };
    }
    if segments.is_empty() {
        return social_route_shape(
            "social-feed",
            "reddit",
            "medium",
            vec!["parsed-url", "dynamic-feed", "parsed-social-route"],
        );
    }
    simple_shape(
        "unknown",
        "reddit",
        "low",
        vec!["parsed-url", "manual-required"],
    )
}

pub(super) fn discord_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(segments.first().map(String::as_str), Some("channels")) {
        return social_route_shape(
            "social-messaging",
            "discord",
            "medium",
            vec!["parsed-url", "parsed-social-route"],
        );
    }
    simple_shape(
        "unknown",
        "discord",
        "low",
        vec!["parsed-url", "manual-required"],
    )
}
