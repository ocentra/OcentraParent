const SEGMENT_WATCH: &str = "watch";
const SEGMENT_REEL: &str = "reel";
const SEGMENT_VIDEOS: &str = "videos";
const SEGMENT_HOME: &str = "home";
const SEGMENT_FEED: &str = "feed";
const SEGMENT_LIVE: &str = "live";
const SEGMENT_DIRECTORY: &str = "directory";
const SEGMENT_FOLLOWING: &str = "following";
const SEGMENT_SEARCH: &str = "search";
const SEGMENT_EXPLORE: &str = "explore";
const SEGMENT_STATUS: &str = "status";
const SEGMENT_REDDIT: &str = "r";
const SEGMENT_COMMENTS: &str = "comments";
const SEGMENT_CHANNELS: &str = "channels";
const QUERY_VIDEO: &str = "v";
const QUERY_SEARCH: &str = "q";
const PLATFORM_FACEBOOK: &str = "facebook";
const PLATFORM_TWITCH: &str = "twitch";
const PLATFORM_X_TWITTER: &str = "x-twitter";
const PLATFORM_REDDIT: &str = "reddit";
const PLATFORM_DISCORD: &str = "discord";
const TARGET_VIDEO: &str = "video";
const TARGET_SHORT_VIDEO: &str = "short-video";
const TARGET_SOCIAL_POST: &str = "social-post";
const TARGET_FORUM: &str = "forum";
const TARGET_UNKNOWN: &str = "unknown";
const ROUTE_SOCIAL_FEED: &str = "social-feed";
const ROUTE_SOCIAL_LIVESTREAM: &str = "social-livestream";
const ROUTE_SOCIAL_MESSAGING: &str = "social-messaging";
const CONFIDENCE_HIGH: &str = "high";
const CONFIDENCE_MEDIUM: &str = "medium";
const CONFIDENCE_LOW: &str = "low";
const REASON_PARSED_URL: &str = "parsed-url";
const REASON_DYNAMIC_FEED: &str = "dynamic-feed";
const REASON_PARSED_SOCIAL_ROUTE: &str = "parsed-social-route";
const REASON_PARSED_CHANNEL_ID: &str = "parsed-channel-id";
const REASON_MANUAL_REQUIRED: &str = "manual-required";

#[path = "../../browser-core-generated/browser_url_intelligence_discord_impl.rs"]
mod discord;
#[path = "../../browser-core-generated/browser_url_intelligence_facebook_impl.rs"]
mod facebook;
#[path = "../../browser-core-generated/browser_url_intelligence_reddit_impl.rs"]
mod reddit;
#[path = "../../browser-core-generated/browser_url_intelligence_twitch_impl.rs"]
mod twitch;
#[path = "../../browser-core-generated/browser_url_intelligence_x_twitter_impl.rs"]
mod x_twitter;

pub(super) fn facebook_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    facebook::facebook_shape(parsed)
}

pub(super) fn twitch_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    twitch::twitch_shape(parsed)
}

pub(super) fn x_twitter_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    x_twitter::x_twitter_shape(parsed)
}

pub(super) fn reddit_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    reddit::reddit_shape(parsed)
}

pub(super) fn discord_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    discord::discord_shape(parsed)
}
use super::*;
