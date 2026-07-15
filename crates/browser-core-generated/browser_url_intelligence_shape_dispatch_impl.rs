use super::*;

const DOMAIN_YOUTUBE: &str = "youtube.com";
const DOMAIN_SHORT_YOUTUBE: &str = "youtu.be";
const DOMAIN_VIMEO: &str = "vimeo.com";
const DOMAIN_TIKTOK: &str = "tiktok.com";
const DOMAIN_INSTAGRAM: &str = "instagram.com";
const DOMAIN_FACEBOOK: &str = "facebook.com";
const DOMAIN_TWITCH: &str = "twitch.tv";
const DOMAIN_X: &str = "x.com";
const DOMAIN_TWITTER: &str = "twitter.com";
const DOMAIN_REDDIT: &str = "reddit.com";
const DOMAIN_DISCORD: &str = "discord.com";
const PLATFORM_YOUTUBE: &str = "youtube";
const TARGET_UNKNOWN: &str = "unknown";
const PLATFORM_GENERIC_WEB: &str = "generic-web";
const CONFIDENCE_LOW: &str = "low";
const REASON_PARSED_URL: &str = "parsed-url";
const REASON_YOUTUBE_VIDEO_ID: &str = "parsed-youtube-video-id";

pub(super) fn shape_for_parsed_url(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    if domain_matches_any(&parsed.domain, [DOMAIN_YOUTUBE]) {
        return youtube_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, [DOMAIN_SHORT_YOUTUBE]) {
        if let Some(video_id) = first_path_segment(parsed) {
            return video_shape(
                PLATFORM_YOUTUBE,
                video_id.0.as_str(),
                vec![REASON_PARSED_URL, REASON_YOUTUBE_VIDEO_ID],
            );
        }
        return simple_shape(
            TARGET_UNKNOWN,
            PLATFORM_GENERIC_WEB,
            CONFIDENCE_LOW,
            vec![REASON_PARSED_URL],
        );
    }
    if domain_matches_any(&parsed.domain, [DOMAIN_VIMEO]) {
        if let Some(shape) = vimeo_shape(parsed) {
            return shape;
        }
        return simple_shape(
            TARGET_UNKNOWN,
            PLATFORM_GENERIC_WEB,
            CONFIDENCE_LOW,
            vec![REASON_PARSED_URL],
        );
    }
    if domain_matches_any(&parsed.domain, [DOMAIN_TIKTOK]) {
        return tiktok_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, [DOMAIN_INSTAGRAM]) {
        return instagram_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, [DOMAIN_FACEBOOK]) {
        return facebook_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, [DOMAIN_TWITCH]) {
        return twitch_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, [DOMAIN_X, DOMAIN_TWITTER]) {
        return x_twitter_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, [DOMAIN_REDDIT]) {
        return reddit_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, [DOMAIN_DISCORD]) {
        return discord_shape(parsed);
    }
    simple_shape(
        TARGET_UNKNOWN,
        PLATFORM_GENERIC_WEB,
        CONFIDENCE_LOW,
        vec![REASON_PARSED_URL],
    )
}
