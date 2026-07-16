use super::*;

const SOURCE_MANAGED_EXACT_URL: &str = "managed-browser-exact-url";
const SOURCE_MANAGED_TARGET_LIST: &str = "managed-browser-target-list";
const SOURCE_UNMANAGED_PROCESS: &str = "unmanaged-browser-process";
const SOURCE_NETWORK_DOMAIN: &str = "network-domain";
const TARGET_UNKNOWN: &str = "unknown";
const PLATFORM_UNKNOWN: &str = "unknown";
const CONFIDENCE_LOW: &str = "low";
const REASON_CONTENT_NOT_INFERRED: &str = "content-not-inferred";
const REASON_UNSUPPORTED_SCHEME: &str = "unsupported-scheme";
const REASON_UNMANAGED_PROCESS_ONLY: &str = "unmanaged-process-only";
const REASON_NETWORK_DOMAIN_ONLY: &str = "network-domain-only";
const REASON_NO_EXACT_EVIDENCE: &str = "no-exact-evidence";

pub(super) fn evaluate_browser_url_shape(
    input: &BrowserUrlShapeEvaluationInput<'_>,
) -> BrowserUrlShapeClassificationTemplate {
    if input.source_kind != SOURCE_MANAGED_EXACT_URL {
        return browser_url_shape_classification_for_non_exact(input);
    }

    let Some(parsed) = input.url.and_then(parse_url) else {
        return browser_url_shape_classification_for_unsupported_scheme(input);
    };

    browser_url_shape_classification_for_parsed(input, parsed)
}

fn browser_url_shape_classification_for_non_exact(
    input: &BrowserUrlShapeEvaluationInput<'_>,
) -> BrowserUrlShapeClassificationTemplate {
    let source_kind = match input.source_kind {
        SOURCE_MANAGED_EXACT_URL => SOURCE_MANAGED_EXACT_URL,
        SOURCE_MANAGED_TARGET_LIST => SOURCE_MANAGED_TARGET_LIST,
        SOURCE_UNMANAGED_PROCESS => SOURCE_UNMANAGED_PROCESS,
        SOURCE_NETWORK_DOMAIN => SOURCE_NETWORK_DOMAIN,
        _ => SOURCE_MANAGED_TARGET_LIST,
    };
    let evidence_reason = match input.source_kind {
        SOURCE_UNMANAGED_PROCESS => REASON_UNMANAGED_PROCESS_ONLY,
        SOURCE_NETWORK_DOMAIN => REASON_NETWORK_DOMAIN_ONLY,
        _ => REASON_NO_EXACT_EVIDENCE,
    };

    BrowserUrlShapeClassificationTemplate {
        schema_version: 1,
        classification_id: input.classification_id.to_string(),
        classified_at: input.classified_at.to_string(),
        source_evidence_ids: input
            .source_evidence_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        source_kind,
        url: None,
        domain: None,
        title: None,
        target_kind: TARGET_UNKNOWN,
        platform: PLATFORM_UNKNOWN,
        platform_ids: empty_platform_ids(),
        confidence: CONFIDENCE_LOW,
        reason_codes: vec![evidence_reason, REASON_CONTENT_NOT_INFERRED],
        exact_url_evidence: false,
        content_semantics_claimed: false,
        ai_decision_claimed: false,
        policy_decision_claimed: false,
    }
}

fn browser_url_shape_classification_for_unsupported_scheme(
    input: &BrowserUrlShapeEvaluationInput<'_>,
) -> BrowserUrlShapeClassificationTemplate {
    BrowserUrlShapeClassificationTemplate {
        schema_version: 1,
        classification_id: input.classification_id.to_string(),
        classified_at: input.classified_at.to_string(),
        source_evidence_ids: input
            .source_evidence_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        source_kind: SOURCE_MANAGED_EXACT_URL,
        url: None,
        domain: None,
        title: input.title.map(str::to_string),
        target_kind: TARGET_UNKNOWN,
        platform: PLATFORM_UNKNOWN,
        platform_ids: empty_platform_ids(),
        confidence: CONFIDENCE_LOW,
        reason_codes: vec![REASON_UNSUPPORTED_SCHEME, REASON_CONTENT_NOT_INFERRED],
        exact_url_evidence: false,
        content_semantics_claimed: false,
        ai_decision_claimed: false,
        policy_decision_claimed: false,
    }
}

fn browser_url_shape_classification_for_parsed(
    input: &BrowserUrlShapeEvaluationInput<'_>,
    parsed: ParsedBrowserUrl,
) -> BrowserUrlShapeClassificationTemplate {
    let mut shape =
        super::browser_url_intelligence_shape_dispatch_impl::shape_for_parsed_url(&parsed);
    shape.reason_codes.push(REASON_CONTENT_NOT_INFERRED);

    BrowserUrlShapeClassificationTemplate {
        schema_version: 1,
        classification_id: input.classification_id.to_string(),
        classified_at: input.classified_at.to_string(),
        source_evidence_ids: input
            .source_evidence_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        source_kind: SOURCE_MANAGED_EXACT_URL,
        url: Some(parsed.normalized_url),
        domain: Some(parsed.domain),
        title: input.title.map(str::to_string),
        target_kind: shape.target_kind,
        platform: shape.platform,
        platform_ids: shape.platform_ids,
        confidence: shape.confidence,
        reason_codes: shape.reason_codes,
        exact_url_evidence: true,
        content_semantics_claimed: false,
        ai_decision_claimed: false,
        policy_decision_claimed: false,
    }
}
