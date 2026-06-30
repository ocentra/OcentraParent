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
    if input.trim().is_empty() {
        return manual_browser_game_url_shape_result("not-text-input", "unknown");
    }

    let parsed_url = match parse_browser_game_url(input) {
        Ok(parsed_url) => parsed_url,
        Err(ParseBrowserGameUrlError::Invalid) => {
            return manual_browser_game_url_shape_result("invalid-url", "unknown")
        }
        Err(ParseBrowserGameUrlError::UnsupportedProtocol(protocol_shape)) => {
            return manual_browser_game_url_shape_result("unsupported-protocol", protocol_shape)
        }
    };

    let segments = path_segments_for(parsed_url.pathname);
    let route_hints = route_hints_for(&segments);
    let path_depth = path_depth_for(&segments);
    let route_surface_kind = route_surface_kind_for(&segments, &route_hints);
    let host_shape = host_shape_for(parsed_url.hostname);
    let has_game_id_like_segment = segments
        .iter()
        .any(|segment| segment_looks_like_game_id(segment));
    let reason_codes = reason_codes_for(route_surface_kind, &route_hints);
    let confidence = confidence_for(route_surface_kind, host_shape, path_depth);

    BrowserGameUrlShapeParseTemplate {
        input_custody: "transient-parse-only",
        parse_state: "parsed",
        protocol_shape: "http-family",
        host_shape,
        path_depth,
        route_surface_kind,
        route_shape_fingerprint: Some(fingerprint_for(&BrowserGameRouteFingerprintInput {
            protocol_shape: "http-family",
            host_shape,
            path_depth,
            route_surface_kind,
            has_game_id_like_segment,
            has_query_shape: !parsed_url.search.is_empty(),
            has_fragment_shape: !parsed_url.hash.is_empty(),
            route_hints: &route_hints,
        })),
        has_query_shape: !parsed_url.search.is_empty(),
        has_fragment_shape: !parsed_url.hash.is_empty(),
        has_game_id_like_segment,
        has_embed_hint: route_hints.has_embed_hint,
        has_play_hint: route_hints.has_play_hint,
        has_account_hint: route_hints.has_account_hint,
        has_purchase_hint: route_hints.has_purchase_hint,
        has_cloud_session_hint: route_hints.has_cloud_session_hint,
        reason_codes,
        confidence,
    }
}

pub fn browser_game_url_shape_evaluator_typescript() -> String {
    BROWSER_GAME_URL_SHAPE_EVALUATOR_TYPESCRIPT.to_string()
}

const BROWSER_GAME_URL_SHAPE_EVALUATOR_TYPESCRIPT: &str = [
    [
        "/* generated from crates/browser-core/src/browser_game_url_shape_evaluator.rs */",
        "",
        "type BrowserGameParsedUrl = {",
        "  readonly protocol: string;",
        "  readonly hostname: string;",
        "  readonly pathname: string;",
        "  readonly search: string;",
        "  readonly hash: string;",
        "};",
        "",
        "type BrowserGameUrlConstructor = new (value: string) => BrowserGameParsedUrl;",
        "",
        "export function browserGameUrlShapeParseResultTemplate(input: unknown) {",
        "  if (typeof input !== 'string' || input.trim().length === 0) {",
        "    return manualBrowserGameUrlShapeResult('not-text-input');",
        "  }",
        "",
        "  const UrlConstructor = (globalThis as unknown as { readonly URL?: BrowserGameUrlConstructor }).URL;",
        "  if (!UrlConstructor) {",
        "    return manualBrowserGameUrlShapeResult('unavailable');",
        "  }",
        "",
        "  let parsedUrl: BrowserGameParsedUrl;",
        "  try {",
        "    parsedUrl = new UrlConstructor(input);",
        "  } catch {",
        "    return manualBrowserGameUrlShapeResult('invalid-url');",
        "  }",
        "",
        "  const protocolShape = protocolShapeFor(parsedUrl);",
        "  if (protocolShape !== 'http-family') {",
        "    return manualBrowserGameUrlShapeResult('unsupported-protocol', protocolShape);",
        "  }",
        "",
        "  const segments = pathSegmentsFor(parsedUrl);",
        "  const routeHints = routeHintsFor(segments);",
        "  const pathDepth = pathDepthFor(segments);",
        "  const routeSurfaceKind = routeSurfaceKindFor(segments, routeHints);",
        "  const hostShape = hostShapeFor(parsedUrl);",
        "  const hasGameIdLikeSegment = segments.some(segmentLooksLikeGameId);",
        "  const reasonCodes = reasonCodesFor(routeSurfaceKind, routeHints);",
        "  const confidence = confidenceFor(routeSurfaceKind, hostShape, pathDepth);",
        "",
        "  return {",
        "    schemaVersion: 'browser-game-url-shape-parser-contract',",
        "    parserResultId: 'browser-game-url-shape-parser-result',",
        "    inputCustody: 'transient-parse-only',",
        "    parseState: 'parsed',",
        "    protocolShape,",
        "    hostShape,",
        "    pathDepth,",
        "    routeSurfaceKind,",
        "    routeShapeFingerprint: fingerprintFor({",
        "      protocolShape,",
        "      hostShape,",
        "      pathDepth,",
        "      routeSurfaceKind,",
        "      hasGameIdLikeSegment,",
        "      hasQueryShape: parsedUrl.search.length > 0,",
        "      hasFragmentShape: parsedUrl.hash.length > 0,",
        "      ...routeHints,",
        "    }),",
        "    hasQueryShape: parsedUrl.search.length > 0,",
        "    hasFragmentShape: parsedUrl.hash.length > 0,",
        "    hasGameIdLikeSegment,",
        "    ...routeHints,",
        "    reasonCodes,",
        "    confidence,",
        "    rawUrlStored: false,",
        "    rawDomainStored: false,",
        "    rawPathStored: false,",
        "    rawQueryStored: false,",
        "    browserNavigationClaimed: false,",
        "    runtimeDetectionClaimed: false,",
        "    aiClassificationClaimed: false,",
        "    policyDecisionClaimed: false,",
        "    cloudFrameAnalysisClaimed: false,",
        "    nativeGameControlClaimed: false,",
        "    enforcementClaimed: false,",
        "  } as const;",
        "}",
        "",
        "function manualBrowserGameUrlShapeResult(",
        "  reasonCode: string,",
        "  protocolShape: string = 'unknown'",
        ") {",
        "  return {",
        "    schemaVersion: 'browser-game-url-shape-parser-contract',",
        "    parserResultId: 'browser-game-url-shape-parser-result',",
        "    inputCustody: 'manual-required',",
        "    parseState: 'manual-required',",
        "    protocolShape,",
        "    hostShape: 'unknown',",
        "    pathDepth: 'unknown',",
        "    routeSurfaceKind: 'unknown-route',",
        "    routeShapeFingerprint: null,",
        "    hasQueryShape: false,",
        "    hasFragmentShape: false,",
        "    hasGameIdLikeSegment: false,",
        "    hasEmbedHint: false,",
        "    hasPlayHint: false,",
        "    hasAccountHint: false,",
        "    hasPurchaseHint: false,",
        "    hasCloudSessionHint: false,",
        "    reasonCodes: [reasonCode, 'manual-required'],",
        "    confidence: 'low',",
        "    rawUrlStored: false,",
        "    rawDomainStored: false,",
        "    rawPathStored: false,",
        "    rawQueryStored: false,",
        "    browserNavigationClaimed: false,",
        "    runtimeDetectionClaimed: false,",
        "    aiClassificationClaimed: false,",
        "    policyDecisionClaimed: false,",
        "    cloudFrameAnalysisClaimed: false,",
        "    nativeGameControlClaimed: false,",
        "    enforcementClaimed: false,",
        "  } as const;",
        "}",
        "",
        "function protocolShapeFor(parsedUrl: BrowserGameParsedUrl) {",
        "  if (parsedUrl.protocol === 'http:' || parsedUrl.protocol === 'https:') {",
        "    return 'http-family';",
        "  }",
        "  return parsedUrl.protocol.length > 0 ? 'non-http' : 'missing';",
        "}",
        "",
        "function hostShapeFor(parsedUrl: BrowserGameParsedUrl) {",
        "  const hostname = parsedUrl.hostname.toLowerCase();",
        "  if (hostname === 'localhost') {",
        "    return 'localhost-like';",
        "  }",
        "  if (/^\\d{1,3}(?:\\.\\d{1,3}){3}$/.test(hostname)) {",
        "    return 'ip-like';",
        "  }",
        "  return hostname.includes('.') ? 'domain-like' : 'unknown';",
        "}",
        "",
        "function pathSegmentsFor(parsedUrl: BrowserGameParsedUrl) {",
        "  return parsedUrl.pathname",
        "    .split('/')",
        "    .map((segment) => segment.trim().toLowerCase())",
        "    .filter((segment) => segment.length > 0);",
        "}",
        "",
        "function pathDepthFor(segments: ReadonlyArray<unknown>) {",
        "  if (segments.length === 0) {",
        "    return 'root';",
        "  }",
        "  if (segments.length === 1) {",
        "    return 'one-segment';",
        "  }",
        "  if (segments.length === 2) {",
        "    return 'two-segments';",
        "  }",
        "  return 'three-or-more-segments';",
        "}",
        "",
        "function routeHintsFor(segments: ReadonlyArray<unknown>) {",
        "  const normalizedSegments = segments.filter((segment): segment is string => typeof segment === 'string');",
        "  return {",
        "    hasEmbedHint: normalizedSegments.some((segment) => segment === 'embed' || segment === 'iframe'),",
        "    hasPlayHint: normalizedSegments.some((segment) => segment === 'play' || segment === 'launch'),",
        "    hasAccountHint: normalizedSegments.some(",
        "      (segment) => segment === 'account' || segment === 'login' || segment === 'signup'",
        "    ),",
        "    hasPurchaseHint: normalizedSegments.some(",
        "      (segment) => segment === 'buy' || segment === 'store' || segment === 'checkout'",
        "    ),",
        "    hasCloudSessionHint: normalizedSegments.some(",
        "      (segment) => segment === 'cloud' || segment === 'stream' || segment === 'session'",
        "    ),",
        "  };",
        "}",
        "",
        "function routeSurfaceKindFor(",
        "  segments: ReadonlyArray<unknown>,",
        "  routeHints: ReturnType<typeof routeHintsFor>",
        ") {",
        "  if (routeHints.hasCloudSessionHint) {",
        "    return 'cloud-session-route';",
        "  }",
        "  if (routeHints.hasEmbedHint) {",
        "    return 'embed-route';",
        "  }",
        "  if (routeHints.hasPlayHint) {",
        "    return 'play-route';",
        "  }",
        "  if (routeHints.hasPurchaseHint) {",
        "    return 'purchase-route';",
        "  }",
        "  if (routeHints.hasAccountHint) {",
        "    return 'account-route';",
        "  }",
        "  if (segments.length === 0) {",
        "    return 'home-route';",
        "  }",
        "  if (segments.length <= 2) {",
        "    return 'catalog-route';",
        "  }",
        "  return 'game-detail-route';",
        "}",
        "",
        "function reasonCodesFor(",
        "  routeSurfaceKind: string,",
        "  routeHints: ReturnType<typeof routeHintsFor>",
        ") {",
        "  if (routeHints.hasCloudSessionHint || routeSurfaceKind === 'cloud-session-route') {",
        "    return ['cloud-session-hint'] as const;",
        "  }",
        "  if (routeHints.hasEmbedHint || routeSurfaceKind === 'embed-route') {",
        "    return ['embed-route-hint'] as const;",
        "  }",
        "  if (routeHints.hasPlayHint || routeSurfaceKind === 'play-route') {",
        "    return ['game-route-hint'] as const;",
        "  }",
        "  if (routeHints.hasPurchaseHint || routeSurfaceKind === 'purchase-route') {",
        "    return ['purchase-route-hint'] as const;",
        "  }",
        "  if (routeHints.hasAccountHint || routeSurfaceKind === 'account-route') {",
        "    return ['account-route-hint'] as const;",
        "  }",
        "  return ['catalog-route-hint'] as const;",
        "}",
        "",
        "function confidenceFor(",
        "  routeSurfaceKind: string,",
        "  hostShape: string,",
        "  pathDepth: string",
        ") {",
        "  if (hostShape === 'unknown' || pathDepth === 'unknown' || routeSurfaceKind === 'unknown-route') {",
        "    return 'unknown';",
        "  }",
        "  if (routeSurfaceKind === 'home-route' || routeSurfaceKind === 'catalog-route') {",
        "    return 'medium';",
        "  }",
        "  return 'high';",
        "}",
        "",
        "function segmentLooksLikeGameId(segment: unknown): boolean {",
        "  return typeof segment === 'string' && segment.length >= 4 && (/\\d/.test(segment) || segment.includes('-'));",
        "}",
        "",
        "function fingerprintFor(parts: {",
        "  protocolShape: string;",
        "  hostShape: string;",
        "  pathDepth: string;",
        "  routeSurfaceKind: string;",
        "  hasGameIdLikeSegment: boolean;",
        "  hasQueryShape: boolean;",
        "  hasFragmentShape: boolean;",
        "  hasEmbedHint: boolean;",
        "  hasPlayHint: boolean;",
        "  hasAccountHint: boolean;",
        "  hasPurchaseHint: boolean;",
        "  hasCloudSessionHint: boolean;",
        "}) {",
        "  return [",
        "    'url-shape',",
        "    parts.protocolShape,",
        "    parts.hostShape,",
        "    parts.pathDepth,",
        "    parts.routeSurfaceKind,",
        "    parts.hasGameIdLikeSegment ? 'game-id-like' : 'no-game-id',",
        "    parts.hasQueryShape ? 'query' : 'no-query',",
        "    parts.hasFragmentShape ? 'fragment' : 'no-fragment',",
        "    parts.hasEmbedHint ? 'embed' : 'no-embed',",
        "    parts.hasPlayHint ? 'play' : 'no-play',",
        "    parts.hasAccountHint ? 'account' : 'no-account',",
        "    parts.hasPurchaseHint ? 'purchase' : 'no-purchase',",
        "    parts.hasCloudSessionHint ? 'cloud-session' : 'no-cloud-session',",
        "  ].join(':');",
        "}",
        "",
    ]
    .join("\n")
].join("\n");

enum ParseBrowserGameUrlError {
    Invalid,
    UnsupportedProtocol(&'static str),
}

fn manual_browser_game_url_shape_result(
    reason_code: &'static str,
    protocol_shape: &'static str,
) -> BrowserGameUrlShapeParseTemplate {
    BrowserGameUrlShapeParseTemplate {
        input_custody: "manual-required",
        parse_state: "manual-required",
        protocol_shape,
        host_shape: "unknown",
        path_depth: "unknown",
        route_surface_kind: "unknown-route",
        route_shape_fingerprint: None,
        has_query_shape: false,
        has_fragment_shape: false,
        has_game_id_like_segment: false,
        has_embed_hint: false,
        has_play_hint: false,
        has_account_hint: false,
        has_purchase_hint: false,
        has_cloud_session_hint: false,
        reason_codes: vec![reason_code, "manual-required"],
        confidence: "low",
    }
}

fn parse_browser_game_url(
    input: &str,
) -> Result<BrowserGameParsedUrl<'_>, ParseBrowserGameUrlError> {
    let colon_index = input.find(':').ok_or(ParseBrowserGameUrlError::Invalid)?;
    if colon_index == 0 {
        return Err(ParseBrowserGameUrlError::Invalid);
    }

    let protocol = &input[..=colon_index];
    if protocol != "http:" && protocol != "https:" {
        let protocol_shape = if protocol.is_empty() {
            "missing"
        } else {
            "non-http"
        };
        return Err(ParseBrowserGameUrlError::UnsupportedProtocol(
            protocol_shape,
        ));
    }

    let after_protocol = &input[colon_index + 1..];
    if !after_protocol.starts_with("//") {
        return Err(ParseBrowserGameUrlError::Invalid);
    }

    let remainder = &after_protocol[2..];
    let host_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let hostname = &remainder[..host_end];
    if hostname.trim().is_empty() {
        return Err(ParseBrowserGameUrlError::Invalid);
    }

    let path_and_more = &remainder[host_end..];
    let hash_index = path_and_more.find('#');
    let (before_hash, hash) = match hash_index {
        Some(index) => (&path_and_more[..index], &path_and_more[index..]),
        None => (path_and_more, ""),
    };
    let search_index = before_hash.find('?');
    let (pathname, search) = match search_index {
        Some(index) => (&before_hash[..index], &before_hash[index..]),
        None => (before_hash, ""),
    };

    Ok(BrowserGameParsedUrl {
        hostname,
        pathname: if pathname.is_empty() { "/" } else { pathname },
        search,
        hash,
    })
}

fn host_shape_for(hostname: &str) -> &'static str {
    let lowercase = hostname.to_ascii_lowercase();
    if lowercase == "localhost" {
        return "localhost-like";
    }
    if lowercase.split('.').all(|segment| {
        !segment.is_empty()
            && segment.len() <= 3
            && segment.chars().all(|character| character.is_ascii_digit())
    }) && lowercase.matches('.').count() == 3
    {
        return "ip-like";
    }
    if lowercase.contains('.') {
        return "domain-like";
    }
    "unknown"
}

fn path_segments_for(pathname: &str) -> Vec<String> {
    pathname
        .split('/')
        .map(|segment| segment.trim().to_ascii_lowercase())
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn path_depth_for(segments: &[String]) -> &'static str {
    match segments.len() {
        0 => "root",
        1 => "one-segment",
        2 => "two-segments",
        _ => "three-or-more-segments",
    }
}

fn route_hints_for(segments: &[String]) -> BrowserGameRouteHints {
    BrowserGameRouteHints {
        has_embed_hint: segments
            .iter()
            .any(|segment| segment == "embed" || segment == "iframe"),
        has_play_hint: segments
            .iter()
            .any(|segment| segment == "play" || segment == "launch"),
        has_account_hint: segments
            .iter()
            .any(|segment| segment == "account" || segment == "login" || segment == "signup"),
        has_purchase_hint: segments
            .iter()
            .any(|segment| segment == "buy" || segment == "store" || segment == "checkout"),
        has_cloud_session_hint: segments
            .iter()
            .any(|segment| segment == "cloud" || segment == "stream" || segment == "session"),
    }
}

fn route_surface_kind_for(
    segments: &[String],
    route_hints: &BrowserGameRouteHints,
) -> &'static str {
    if route_hints.has_cloud_session_hint {
        return "cloud-session-route";
    }
    if route_hints.has_embed_hint {
        return "embed-route";
    }
    if route_hints.has_play_hint {
        return "play-route";
    }
    if route_hints.has_purchase_hint {
        return "purchase-route";
    }
    if route_hints.has_account_hint {
        return "account-route";
    }
    if segments.is_empty() {
        return "home-route";
    }
    if segments.len() <= 2 {
        return "catalog-route";
    }
    "game-detail-route"
}

fn reason_codes_for(
    route_surface_kind: &'static str,
    route_hints: &BrowserGameRouteHints,
) -> Vec<&'static str> {
    if route_hints.has_cloud_session_hint || route_surface_kind == "cloud-session-route" {
        return vec!["cloud-session-hint"];
    }
    if route_hints.has_embed_hint || route_surface_kind == "embed-route" {
        return vec!["embed-route-hint"];
    }
    if route_hints.has_play_hint || route_surface_kind == "play-route" {
        return vec!["game-route-hint"];
    }
    if route_hints.has_purchase_hint || route_surface_kind == "purchase-route" {
        return vec!["purchase-route-hint"];
    }
    if route_hints.has_account_hint || route_surface_kind == "account-route" {
        return vec!["account-route-hint"];
    }
    vec!["catalog-route-hint"]
}

fn confidence_for(
    route_surface_kind: &'static str,
    host_shape: &'static str,
    path_depth: &'static str,
) -> &'static str {
    if host_shape == "unknown" || path_depth == "unknown" || route_surface_kind == "unknown-route" {
        return "unknown";
    }
    if route_surface_kind == "home-route" || route_surface_kind == "catalog-route" {
        return "medium";
    }
    "high"
}

fn segment_looks_like_game_id(segment: &str) -> bool {
    segment.len() >= 4
        && (segment.chars().any(|character| character.is_ascii_digit()) || segment.contains('-'))
}

fn fingerprint_for(input: &BrowserGameRouteFingerprintInput<'_>) -> String {
    [
        "url-shape".to_string(),
        input.protocol_shape.to_string(),
        input.host_shape.to_string(),
        input.path_depth.to_string(),
        input.route_surface_kind.to_string(),
        if input.has_game_id_like_segment {
            "game-id-like".to_string()
        } else {
            "no-game-id".to_string()
        },
        if input.has_query_shape {
            "query".to_string()
        } else {
            "no-query".to_string()
        },
        if input.has_fragment_shape {
            "fragment".to_string()
        } else {
            "no-fragment".to_string()
        },
        if input.route_hints.has_embed_hint {
            "embed".to_string()
        } else {
            "no-embed".to_string()
        },
        if input.route_hints.has_play_hint {
            "play".to_string()
        } else {
            "no-play".to_string()
        },
        if input.route_hints.has_account_hint {
            "account".to_string()
        } else {
            "no-account".to_string()
        },
        if input.route_hints.has_purchase_hint {
            "purchase".to_string()
        } else {
            "no-purchase".to_string()
        },
        if input.route_hints.has_cloud_session_hint {
            "cloud-session".to_string()
        } else {
            "no-cloud-session".to_string()
        },
    ]
    .join(":")
}
