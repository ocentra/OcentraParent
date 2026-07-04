#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserUrlShapeEvaluationInput<'a> {
    pub classification_id: &'a str,
    pub classified_at: &'a str,
    pub source_evidence_ids: &'a [&'a str],
    pub source_kind: &'a str,
    pub url: Option<&'a str>,
    pub title: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserUrlShapePlatformIdsTemplate {
    pub video_id: Option<String>,
    pub channel_id: Option<String>,
    pub playlist_id: Option<String>,
    pub post_id: Option<String>,
    pub query: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserUrlShapeClassificationTemplate {
    pub schema_version: u8,
    pub classification_id: String,
    pub classified_at: String,
    pub source_evidence_ids: Vec<String>,
    pub source_kind: &'static str,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub target_kind: &'static str,
    pub platform: &'static str,
    pub platform_ids: BrowserUrlShapePlatformIdsTemplate,
    pub confidence: &'static str,
    pub reason_codes: Vec<&'static str>,
    pub exact_url_evidence: bool,
    pub content_semantics_claimed: bool,
    pub ai_decision_claimed: bool,
    pub policy_decision_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedUrlShape {
    target_kind: &'static str,
    platform: &'static str,
    platform_ids: BrowserUrlShapePlatformIdsTemplate,
    confidence: &'static str,
    reason_codes: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedBrowserUrl {
    normalized_url: String,
    domain: String,
    path: String,
    query: Option<String>,
}

#[path = "browser_url_intelligence_common.rs"]
mod browser_url_intelligence_common;
#[path = "browser_url_intelligence_shapes.rs"]
mod browser_url_intelligence_shapes;
#[path = "browser_url_intelligence_social_platforms.rs"]
mod browser_url_intelligence_social_platforms;
#[path = "browser_url_intelligence_social_video.rs"]
mod browser_url_intelligence_social_video;
#[path = "browser_url_intelligence_youtube.rs"]
mod browser_url_intelligence_youtube;

use self::browser_url_intelligence_common::*;
use self::browser_url_intelligence_shapes::*;
use self::browser_url_intelligence_social_platforms::*;
use self::browser_url_intelligence_social_video::*;
use self::browser_url_intelligence_youtube::*;

pub fn evaluate_browser_url_shape(
    input: &BrowserUrlShapeEvaluationInput<'_>,
) -> BrowserUrlShapeClassificationTemplate {
    browser_url_intelligence_impl::evaluate_browser_url_shape(input)
}

pub fn browser_url_intelligence_typescript() -> String {
    browser_url_intelligence_impl::browser_url_intelligence_typescript()
}

#[path = "../../browser-core-generated/browser_url_intelligence_impl.rs"]
mod browser_url_intelligence_impl;
