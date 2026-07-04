#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserGameUrlShapeParseTemplate {
    pub input_custody: &'static str,
    pub parse_state: &'static str,
    pub protocol_shape: &'static str,
    pub host_shape: &'static str,
    pub path_depth: &'static str,
    pub route_surface_kind: &'static str,
    pub route_shape_fingerprint: Option<String>,
    pub has_query_shape: bool,
    pub has_fragment_shape: bool,
    pub has_game_id_like_segment: bool,
    pub has_embed_hint: bool,
    pub has_play_hint: bool,
    pub has_account_hint: bool,
    pub has_purchase_hint: bool,
    pub has_cloud_session_hint: bool,
    pub reason_codes: Vec<&'static str>,
    pub confidence: &'static str,
}

struct BrowserGameParsedUrl<'a> {
    hostname: &'a str,
    pathname: &'a str,
    search: &'a str,
    hash: &'a str,
}

struct BrowserGameRouteHints {
    has_embed_hint: bool,
    has_play_hint: bool,
    has_account_hint: bool,
    has_purchase_hint: bool,
    has_cloud_session_hint: bool,
}

struct BrowserGameRouteFingerprintInput<'a> {
    protocol_shape: &'static str,
    host_shape: &'static str,
    path_depth: &'static str,
    route_surface_kind: &'static str,
    has_game_id_like_segment: bool,
    has_query_shape: bool,
    has_fragment_shape: bool,
    route_hints: &'a BrowserGameRouteHints,
}

pub fn evaluate_browser_game_url_shape(input: &str) -> BrowserGameUrlShapeParseTemplate {
    browser_game_url_shape_evaluator_impl::evaluate_browser_game_url_shape(input)
}

pub fn browser_game_url_shape_evaluator_typescript() -> String {
    browser_game_url_shape_evaluator_impl::browser_game_url_shape_evaluator_typescript()
}

#[path = "../../browser-core-generated/browser_game_url_shape_evaluator_impl.rs"]
mod browser_game_url_shape_evaluator_impl;
