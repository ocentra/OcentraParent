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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BrowserGameUrlText(pub(super) String);

impl BrowserGameUrlText {
    pub(super) fn from_display(value: impl std::fmt::Display) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BrowserGameShapeCode(pub(super) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BrowserGameUrlFingerprint(pub(super) String);

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
    protocol_shape: BrowserGameShapeCode,
    host_shape: BrowserGameShapeCode,
    path_depth: BrowserGameShapeCode,
    route_surface_kind: BrowserGameShapeCode,
    has_game_id_like_segment: bool,
    has_query_shape: bool,
    has_fragment_shape: bool,
    route_hints: &'a BrowserGameRouteHints,
}

pub fn evaluate_browser_game_url_shape(
    input: impl std::fmt::Display,
) -> BrowserGameUrlShapeParseTemplate {
    let input = BrowserGameUrlText::from_display(input);
    browser_game_url_shape_evaluator_impl::evaluate_browser_game_url_shape(&input)
}

pub fn browser_game_url_shape_evaluator_typescript(
) -> crate::social_schema_generated_values::GeneratedTypescript {
    crate::social_schema_generated_values::GeneratedTypescript::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/browser_game_url_shape_evaluator.ts"
    )))
}

#[path = "../../browser-core-generated/browser_game_url_shape_evaluator_impl.rs"]
mod browser_game_url_shape_evaluator_impl;
