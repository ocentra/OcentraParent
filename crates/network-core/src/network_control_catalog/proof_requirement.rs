use super::network_control_catalog_text::contains_any;

pub fn proof_requirement_for(
    section_title: &str,
    group_title: &str,
    source_text: &str,
) -> Option<&'static str> {
    let searchable = super::network_control_catalog_text::searchable_text(&[
        section_title,
        group_title,
        source_text,
    ]);
    proof_requirement_for_exact_url(&searchable)
        .or_else(|| proof_requirement_for_sensitive_payload(&searchable))
        .or_else(|| proof_requirement_for_strict_enforcement(&searchable))
        .or_else(|| proof_requirement_for_metadata_claims(&searchable))
}

fn proof_requirement_for_exact_url(searchable: &str) -> Option<&'static str> {
    if contains_any(
        searchable,
        &[
            "exact url",
            "path/query",
            "active tab",
            "page title",
            "download source",
        ],
    ) {
        return Some(
            "Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.",
        );
    }
    None
}

fn proof_requirement_for_sensitive_payload(searchable: &str) -> Option<&'static str> {
    if contains_any(
        searchable,
        &[
            "decrypted",
            "payload",
            "page body",
            "chat content",
            "search terms",
            "form values",
            "cookies",
            "tokens",
            "credentials",
            "packet payload",
        ],
    ) {
        return Some(
            "Network controls must not collect decrypted content or payload fields; use metadata evidence only.",
        );
    }
    None
}

fn proof_requirement_for_strict_enforcement(searchable: &str) -> Option<&'static str> {
    if contains_any(
        searchable,
        &[
            "block",
            "firewall",
            "wfp",
            "packet filter",
            "vpn",
            "router",
            "always-on",
            "lockdown",
            "force all traffic",
            "strict",
        ],
    ) {
        return Some(
            "Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.",
        );
    }
    None
}

fn proof_requirement_for_metadata_claims(searchable: &str) -> Option<&'static str> {
    if contains_any(
        searchable,
        &[
            "domain",
            "dns",
            "ip",
            "port",
            "protocol",
            "flow",
            "process",
            "indicator",
            "attribution",
            "confidence",
            "evidence id",
        ],
    ) {
        return Some(
            "Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.",
        );
    }
    None
}
