use super::*;

const INPUT_CUSTODY_TRANSIENT_VALUE: &str = "transient-parse-only";
const INPUT_CUSTODY_MANUAL_VALUE: &str = "manual-required";
const PARSE_STATE_PARSED_VALUE: &str = "parsed";
const PARSE_STATE_MANUAL_VALUE: &str = "manual-required";
const PROTOCOL_HTTP_FAMILY_VALUE: &str = "http-family";
const PROTOCOL_UNKNOWN_VALUE: &str = "unknown";
const HOST_LOCALHOST_LIKE_VALUE: &str = "localhost-like";
const HOST_IP_LIKE_VALUE: &str = "ip-like";
const HOST_DOMAIN_LIKE_VALUE: &str = "domain-like";
const HOST_UNKNOWN_VALUE: &str = "unknown";
const PATH_ROOT_CODE_VALUE: &str = "root";
const PATH_ONE_SEGMENT_VALUE: &str = "one-segment";
const PATH_TWO_SEGMENTS_VALUE: &str = "two-segments";
const PATH_THREE_OR_MORE_SEGMENTS_VALUE: &str = "three-or-more-segments";
const PATH_UNKNOWN_VALUE: &str = "unknown";
const ROUTE_CLOUD_SESSION_VALUE: &str = "cloud-session-route";
const ROUTE_EMBED_VALUE: &str = "embed-route";
const ROUTE_PLAY_VALUE: &str = "play-route";
const ROUTE_PURCHASE_VALUE: &str = "purchase-route";
const ROUTE_ACCOUNT_VALUE: &str = "account-route";
const ROUTE_HOME_VALUE: &str = "home-route";
const ROUTE_CATALOG_VALUE: &str = "catalog-route";
const ROUTE_GAME_DETAIL_VALUE: &str = "game-detail-route";
const ROUTE_UNKNOWN_VALUE: &str = "unknown-route";
const REASON_NOT_TEXT_INPUT_VALUE: &str = "not-text-input";
const REASON_INVALID_URL_VALUE: &str = "invalid-url";
const REASON_UNSUPPORTED_PROTOCOL_VALUE: &str = "unsupported-protocol";
const REASON_MANUAL_REQUIRED_VALUE: &str = "manual-required";
const REASON_CLOUD_SESSION_HINT_VALUE: &str = "cloud-session-hint";
const REASON_EMBED_ROUTE_HINT_VALUE: &str = "embed-route-hint";
const REASON_GAME_ROUTE_HINT_VALUE: &str = "game-route-hint";
const REASON_PURCHASE_ROUTE_HINT_VALUE: &str = "purchase-route-hint";
const REASON_ACCOUNT_ROUTE_HINT_VALUE: &str = "account-route-hint";
const REASON_CATALOG_ROUTE_HINT_VALUE: &str = "catalog-route-hint";
const CONFIDENCE_UNKNOWN_VALUE: &str = "unknown";
const CONFIDENCE_LOW_VALUE: &str = "low";
const CONFIDENCE_MEDIUM_VALUE: &str = "medium";
const CONFIDENCE_HIGH_VALUE: &str = "high";
const PROTOCOL_MISSING_VALUE: &str = "missing";
const PROTOCOL_NON_HTTP_VALUE: &str = "non-http";

const INPUT_CUSTODY_TRANSIENT: BrowserGameShapeCode =
    BrowserGameShapeCode(INPUT_CUSTODY_TRANSIENT_VALUE);
const INPUT_CUSTODY_MANUAL: BrowserGameShapeCode = BrowserGameShapeCode(INPUT_CUSTODY_MANUAL_VALUE);
const PARSE_STATE_PARSED: BrowserGameShapeCode = BrowserGameShapeCode(PARSE_STATE_PARSED_VALUE);
const PARSE_STATE_MANUAL: BrowserGameShapeCode = BrowserGameShapeCode(PARSE_STATE_MANUAL_VALUE);
const PROTOCOL_HTTP_FAMILY: BrowserGameShapeCode = BrowserGameShapeCode(PROTOCOL_HTTP_FAMILY_VALUE);
const PROTOCOL_UNKNOWN: BrowserGameShapeCode = BrowserGameShapeCode(PROTOCOL_UNKNOWN_VALUE);
const HOST_LOCALHOST_LIKE: BrowserGameShapeCode = BrowserGameShapeCode(HOST_LOCALHOST_LIKE_VALUE);
const HOST_IP_LIKE: BrowserGameShapeCode = BrowserGameShapeCode(HOST_IP_LIKE_VALUE);
const HOST_DOMAIN_LIKE: BrowserGameShapeCode = BrowserGameShapeCode(HOST_DOMAIN_LIKE_VALUE);
const HOST_UNKNOWN: BrowserGameShapeCode = BrowserGameShapeCode(HOST_UNKNOWN_VALUE);
const PATH_ROOT: BrowserGameShapeCode = BrowserGameShapeCode(PATH_ROOT_CODE_VALUE);
const PATH_ONE_SEGMENT: BrowserGameShapeCode = BrowserGameShapeCode(PATH_ONE_SEGMENT_VALUE);
const PATH_TWO_SEGMENTS: BrowserGameShapeCode = BrowserGameShapeCode(PATH_TWO_SEGMENTS_VALUE);
const PATH_THREE_OR_MORE_SEGMENTS: BrowserGameShapeCode =
    BrowserGameShapeCode(PATH_THREE_OR_MORE_SEGMENTS_VALUE);
const PATH_UNKNOWN: BrowserGameShapeCode = BrowserGameShapeCode(PATH_UNKNOWN_VALUE);
const ROUTE_CLOUD_SESSION: BrowserGameShapeCode = BrowserGameShapeCode(ROUTE_CLOUD_SESSION_VALUE);
const ROUTE_EMBED: BrowserGameShapeCode = BrowserGameShapeCode(ROUTE_EMBED_VALUE);
const ROUTE_PLAY: BrowserGameShapeCode = BrowserGameShapeCode(ROUTE_PLAY_VALUE);
const ROUTE_PURCHASE: BrowserGameShapeCode = BrowserGameShapeCode(ROUTE_PURCHASE_VALUE);
const ROUTE_ACCOUNT: BrowserGameShapeCode = BrowserGameShapeCode(ROUTE_ACCOUNT_VALUE);
const ROUTE_HOME: BrowserGameShapeCode = BrowserGameShapeCode(ROUTE_HOME_VALUE);
const ROUTE_CATALOG: BrowserGameShapeCode = BrowserGameShapeCode(ROUTE_CATALOG_VALUE);
const ROUTE_GAME_DETAIL: BrowserGameShapeCode = BrowserGameShapeCode(ROUTE_GAME_DETAIL_VALUE);
const ROUTE_UNKNOWN: BrowserGameShapeCode = BrowserGameShapeCode(ROUTE_UNKNOWN_VALUE);
const REASON_NOT_TEXT_INPUT: BrowserGameShapeCode =
    BrowserGameShapeCode(REASON_NOT_TEXT_INPUT_VALUE);
const REASON_INVALID_URL: BrowserGameShapeCode = BrowserGameShapeCode(REASON_INVALID_URL_VALUE);
const REASON_UNSUPPORTED_PROTOCOL: BrowserGameShapeCode =
    BrowserGameShapeCode(REASON_UNSUPPORTED_PROTOCOL_VALUE);
const REASON_MANUAL_REQUIRED: BrowserGameShapeCode =
    BrowserGameShapeCode(REASON_MANUAL_REQUIRED_VALUE);
const REASON_CLOUD_SESSION_HINT: BrowserGameShapeCode =
    BrowserGameShapeCode(REASON_CLOUD_SESSION_HINT_VALUE);
const REASON_EMBED_ROUTE_HINT: BrowserGameShapeCode =
    BrowserGameShapeCode(REASON_EMBED_ROUTE_HINT_VALUE);
const REASON_GAME_ROUTE_HINT: BrowserGameShapeCode =
    BrowserGameShapeCode(REASON_GAME_ROUTE_HINT_VALUE);
const REASON_PURCHASE_ROUTE_HINT: BrowserGameShapeCode =
    BrowserGameShapeCode(REASON_PURCHASE_ROUTE_HINT_VALUE);
const REASON_ACCOUNT_ROUTE_HINT: BrowserGameShapeCode =
    BrowserGameShapeCode(REASON_ACCOUNT_ROUTE_HINT_VALUE);
const REASON_CATALOG_ROUTE_HINT: BrowserGameShapeCode =
    BrowserGameShapeCode(REASON_CATALOG_ROUTE_HINT_VALUE);
const CONFIDENCE_UNKNOWN: BrowserGameShapeCode = BrowserGameShapeCode(CONFIDENCE_UNKNOWN_VALUE);
const CONFIDENCE_LOW: BrowserGameShapeCode = BrowserGameShapeCode(CONFIDENCE_LOW_VALUE);
const CONFIDENCE_MEDIUM: BrowserGameShapeCode = BrowserGameShapeCode(CONFIDENCE_MEDIUM_VALUE);
const CONFIDENCE_HIGH: BrowserGameShapeCode = BrowserGameShapeCode(CONFIDENCE_HIGH_VALUE);
const PROTOCOL_MISSING: BrowserGameShapeCode = BrowserGameShapeCode(PROTOCOL_MISSING_VALUE);
const PROTOCOL_NON_HTTP: BrowserGameShapeCode = BrowserGameShapeCode(PROTOCOL_NON_HTTP_VALUE);

const FINGERPRINT_URL_SHAPE: &str = "url-shape";
const FINGERPRINT_GAME_ID_LIKE: &str = "game-id-like";
const FINGERPRINT_NO_GAME_ID: &str = "no-game-id";
const FINGERPRINT_QUERY: &str = "query";
const FINGERPRINT_NO_QUERY: &str = "no-query";
const FINGERPRINT_FRAGMENT: &str = "fragment";
const FINGERPRINT_NO_FRAGMENT: &str = "no-fragment";
const FINGERPRINT_EMBED: &str = "embed";
const FINGERPRINT_NO_EMBED: &str = "no-embed";
const FINGERPRINT_PLAY: &str = "play";
const FINGERPRINT_NO_PLAY: &str = "no-play";
const FINGERPRINT_ACCOUNT: &str = "account";
const FINGERPRINT_NO_ACCOUNT: &str = "no-account";
const FINGERPRINT_PURCHASE: &str = "purchase";
const FINGERPRINT_NO_PURCHASE: &str = "no-purchase";
const FINGERPRINT_CLOUD_SESSION: &str = "cloud-session";
const FINGERPRINT_NO_CLOUD_SESSION: &str = "no-cloud-session";
const FINGERPRINT_SEPARATOR: &str = ":";
const SEGMENT_EMBED: &str = "embed";
const SEGMENT_IFRAME: &str = "iframe";
const SEGMENT_PLAY: &str = "play";
const SEGMENT_LAUNCH: &str = "launch";
const SEGMENT_ACCOUNT: &str = "account";
const SEGMENT_LOGIN: &str = "login";
const SEGMENT_SIGNUP: &str = "signup";
const SEGMENT_BUY: &str = "buy";
const SEGMENT_STORE: &str = "store";
const SEGMENT_CHECKOUT: &str = "checkout";
const SEGMENT_CLOUD: &str = "cloud";
const SEGMENT_STREAM: &str = "stream";
const SEGMENT_SESSION: &str = "session";
const PATH_ROOT_VALUE: &str = "/";
const HOST_LOCALHOST: &str = "localhost";
const PROTOCOL_HTTP: &str = "http:";
const PROTOCOL_HTTPS: &str = "https:";
const URL_AUTHORITY_PREFIX: &str = "//";
const EMPTY_TEXT: &str = "";

#[path = "browser_game_url_shape_evaluator_confidence_impl.rs"]
mod confidence;
#[path = "browser_game_url_shape_evaluator_fingerprint_impl.rs"]
mod fingerprint;
#[path = "browser_game_url_shape_evaluator_host_impl.rs"]
mod host;
#[path = "browser_game_url_shape_evaluator_parse_impl.rs"]
mod parse;
#[path = "browser_game_url_shape_evaluator_reason_codes_impl.rs"]
mod reason_codes;
#[path = "browser_game_url_shape_evaluator_route_hints_impl.rs"]
mod route_hints;
#[path = "browser_game_url_shape_evaluator_route_surface_impl.rs"]
mod route_surface;

pub(super) fn evaluate_browser_game_url_shape(
    input: &BrowserGameUrlText,
) -> BrowserGameUrlShapeParseTemplate {
    if input.0.trim().is_empty() {
        return manual_browser_game_url_shape_result(REASON_NOT_TEXT_INPUT, PROTOCOL_UNKNOWN);
    }

    let parsed_url = match parse::parse_browser_game_url(input) {
        Ok(parsed_url) => parsed_url,
        Err(parse::ParseBrowserGameUrlError::Invalid) => {
            return manual_browser_game_url_shape_result(REASON_INVALID_URL, PROTOCOL_UNKNOWN);
        }
        Err(parse::ParseBrowserGameUrlError::UnsupportedProtocol(protocol_shape)) => {
            return manual_browser_game_url_shape_result(
                REASON_UNSUPPORTED_PROTOCOL,
                protocol_shape,
            );
        }
    };

    let pathname = BrowserGameUrlText::from_display(parsed_url.pathname);
    let hostname = BrowserGameUrlText::from_display(parsed_url.hostname);
    let segments = route_hints::path_segments_for(&pathname);
    let route_hints = route_hints::route_hints_for(&segments);
    let path_depth = route_hints::path_depth_for(&segments);
    let route_surface_kind = route_surface::route_surface_kind_for(&segments, &route_hints);
    let host_shape = host::host_shape_for(&hostname);
    let has_game_id_like_segment = segments.iter().any(route_hints::segment_looks_like_game_id);
    let reason_codes = reason_codes::reason_codes_for(route_surface_kind, &route_hints);
    let confidence = confidence::confidence_for(route_surface_kind, host_shape, path_depth);

    BrowserGameUrlShapeParseTemplate {
        input_custody: INPUT_CUSTODY_TRANSIENT.0,
        parse_state: PARSE_STATE_PARSED.0,
        protocol_shape: PROTOCOL_HTTP_FAMILY.0,
        host_shape: host_shape.0,
        path_depth: path_depth.0,
        route_surface_kind: route_surface_kind.0,
        route_shape_fingerprint: Some(
            fingerprint::fingerprint_for(&BrowserGameRouteFingerprintInput {
                protocol_shape: PROTOCOL_HTTP_FAMILY,
                host_shape,
                path_depth,
                route_surface_kind,
                has_game_id_like_segment,
                has_query_shape: !parsed_url.search.is_empty(),
                has_fragment_shape: !parsed_url.hash.is_empty(),
                route_hints: &route_hints,
            })
            .0,
        ),
        has_query_shape: !parsed_url.search.is_empty(),
        has_fragment_shape: !parsed_url.hash.is_empty(),
        has_game_id_like_segment,
        has_embed_hint: route_hints.has_embed_hint,
        has_play_hint: route_hints.has_play_hint,
        has_account_hint: route_hints.has_account_hint,
        has_purchase_hint: route_hints.has_purchase_hint,
        has_cloud_session_hint: route_hints.has_cloud_session_hint,
        reason_codes: reason_codes.into_iter().map(|code| code.0).collect(),
        confidence: confidence.0,
    }
}

fn manual_browser_game_url_shape_result(
    reason_code: BrowserGameShapeCode,
    protocol_shape: BrowserGameShapeCode,
) -> BrowserGameUrlShapeParseTemplate {
    BrowserGameUrlShapeParseTemplate {
        input_custody: INPUT_CUSTODY_MANUAL.0,
        parse_state: PARSE_STATE_MANUAL.0,
        protocol_shape: protocol_shape.0,
        host_shape: HOST_UNKNOWN.0,
        path_depth: PATH_UNKNOWN.0,
        route_surface_kind: ROUTE_UNKNOWN.0,
        route_shape_fingerprint: None,
        has_query_shape: false,
        has_fragment_shape: false,
        has_game_id_like_segment: false,
        has_embed_hint: false,
        has_play_hint: false,
        has_account_hint: false,
        has_purchase_hint: false,
        has_cloud_session_hint: false,
        reason_codes: vec![reason_code.0, REASON_MANUAL_REQUIRED.0],
        confidence: CONFIDENCE_LOW.0,
    }
}
