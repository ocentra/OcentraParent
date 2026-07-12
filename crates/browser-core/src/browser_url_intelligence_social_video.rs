use super::*;

pub(super) fn tiktok_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some("upload")
    ) {
        return social_route_shape(
            "social-upload-post",
            "tiktok",
            "medium",
            vec!["parsed-url", "parsed-social-route"],
        );
    }

    if let Some(video_index) = segments.iter().position(|segment| segment.0 == "video") {
        if let Some(video_id) = segments.get(video_index + 1) {
            return video_shape("tiktok", video_id.0.as_str(), vec!["parsed-url"]);
        }
    }

    if matches!(segments.first(), Some(first) if first.0.starts_with('@'))
        && matches!(
            segments.get(1).map(|segment| segment.0.as_str()),
            Some("live")
        )
    {
        return social_route_shape(
            "social-livestream",
            "tiktok",
            "medium",
            vec!["parsed-url", "parsed-social-route"],
        );
    }

    if matches!(segments.first(), Some(first) if first.0.starts_with('@')) && segments.len() == 1 {
        return channel_shape("tiktok", segments[0].0.as_str(), "medium");
    }

    social_route_shape(
        "social-feed",
        "tiktok",
        "medium",
        vec!["parsed-url", "dynamic-feed", "parsed-social-route"],
    )
}

pub(super) fn instagram_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    instagram_feed_shape(&segments)
        .or_else(|| instagram_create_shape(&segments))
        .or_else(|| instagram_live_shape(&segments))
        .or_else(|| instagram_reel_shape(&segments))
        .or_else(|| instagram_post_shape(&segments))
        .or_else(|| instagram_stories_shape(&segments))
        .or_else(|| instagram_direct_shape(&segments))
        .unwrap_or_else(|| {
            channel_shape(
                "instagram",
                segments
                    .first()
                    .map(|segment| segment.0.as_str())
                    .unwrap_or(parsed.domain.as_str()),
                "medium",
            )
        })
}

fn instagram_feed_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    if segments.is_empty()
        || matches!(
            segments.first().map(|segment| segment.0.as_str()),
            Some("explore" | "reels")
        )
    {
        Some(instagram_dynamic_feed_shape())
    } else {
        None
    }
}

fn instagram_create_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some("create")
    )
    .then(|| {
        social_route_shape(
            "social-upload-post",
            "instagram",
            "medium",
            vec!["parsed-url", "parsed-social-route"],
        )
    })
}

fn instagram_live_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some("live")
    )
    .then(|| {
        social_route_shape(
            "social-livestream",
            "instagram",
            "medium",
            vec!["parsed-url", "parsed-social-route"],
        )
    })
}

fn instagram_reel_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    match segments {
        [first, second, ..] if first.0 == "reel" || first.0 == "reels" => Some(social_post_shape(
            "short-video",
            "instagram",
            second.0.as_str(),
            "high",
        )),
        _ => None,
    }
}

fn instagram_post_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    match segments {
        [first, second, ..] if first.0 == "p" || first.0 == "tv" => Some(social_post_shape(
            "social-post",
            "instagram",
            second.0.as_str(),
            "medium",
        )),
        _ => None,
    }
}

fn instagram_stories_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some("stories")
    )
    .then(instagram_dynamic_feed_shape)
}

fn instagram_direct_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some("direct")
    )
    .then(|| {
        social_route_shape(
            "social-messaging",
            "instagram",
            "medium",
            vec!["parsed-url", "parsed-social-route"],
        )
    })
}

fn instagram_dynamic_feed_shape() -> ParsedUrlShape {
    social_route_shape(
        "social-feed",
        "instagram",
        "medium",
        vec!["parsed-url", "dynamic-feed", "parsed-social-route"],
    )
}
