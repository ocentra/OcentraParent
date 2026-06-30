use super::*;

fn parse_url(value: &str) -> Option<ParsedBrowserUrl> {
    let separator_index = value.find("://")?;
    if separator_index == 0 {
        return None;
    }
    let scheme = value[..separator_index].to_ascii_lowercase();
    if scheme != "http" && scheme != "https" {
        return None;
    }
    let remainder = &value[separator_index + 3..];
    let authority_end = first_suffix_index(remainder);
    let authority = authority_end
        .map(|index| &remainder[..index])
        .unwrap_or(remainder);
    if authority.is_empty() || authority.contains('@') {
        return None;
    }
    let normalized = normalized_authority(authority)?;
    let suffix = authority_end.map(|index| &remainder[index..]).unwrap_or("");
    let path = path_from_suffix(suffix);
    Some(ParsedBrowserUrl {
        normalized_url: format!("{scheme}://{}{suffix}", normalized.0),
        domain: normalized.1,
        path,
        query: query_from_suffix(suffix),
    })
}

fn first_suffix_index(value: &str) -> Option<usize> {
    ['/', '?', '#']
        .iter()
        .filter_map(|separator| value.find(*separator))
        .min()
}

fn normalized_authority(value: &str) -> Option<(String, String)> {
    let (host, port) = split_host_and_port(value);
    let domain = normalized_host(host)?;
    let authority = match port {
        Some(port) => format!("{domain}:{port}"),
        None => domain.clone(),
    };
    Some((authority, domain))
}

fn split_host_and_port(value: &str) -> (&str, Option<&str>) {
    if value.matches(':').count() == 1 {
        let separator_index = value.rfind(':').unwrap_or_default();
        let host = &value[..separator_index];
        let port = &value[separator_index + 1..];
        if !host.is_empty()
            && !port.is_empty()
            && port.chars().all(|character| character.is_ascii_digit())
        {
            return (host, Some(port));
        }
    }
    (value, None)
}

fn normalized_host(value: &str) -> Option<String> {
    let normalized = value.trim_end_matches('.').to_ascii_lowercase();
    if normalized.is_empty() || normalized.contains('/') {
        None
    } else {
        Some(normalized)
    }
}

fn path_from_suffix(value: &str) -> String {
    if !value.starts_with('/') {
        return "/".to_string();
    }
    let end_index = [value.find('?'), value.find('#')]
        .into_iter()
        .flatten()
        .min()
        .unwrap_or(value.len());
    value[..end_index].to_string()
}

fn query_from_suffix(value: &str) -> Option<String> {
    let query_start = value.find('?')?;
    let hash_index = value[query_start..]
        .find('#')
        .map(|index| query_start + index)
        .unwrap_or(value.len());
    Some(value[query_start + 1..hash_index].to_string())
}

fn path_segments(parsed: &ParsedBrowserUrl) -> Vec<String> {
    parsed
        .path
        .split('/')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(str::to_string)
        .collect()
}

fn first_path_segment(parsed: &ParsedBrowserUrl) -> Option<String> {
    path_segments(parsed).into_iter().next()
}

fn query_param(parsed: &ParsedBrowserUrl, key: &str) -> Option<String> {
    let query = parsed.query.as_ref()?;
    for part in query.split('&') {
        let separator_index = part.find('=');
        let raw_key = separator_index.map(|index| &part[..index]).unwrap_or(part);
        if raw_key == key {
            let value = separator_index
                .map(|index| &part[index + 1..])
                .unwrap_or("");
            if value.is_empty() {
                return None;
            }
            return Some(value.replace('+', " "));
        }
    }
    None
}

fn domain_matches_any(domain: &str, bases: &[&str]) -> bool {
    bases
        .iter()
        .any(|base| domain == *base || domain.ends_with(&format!(".{base}")))
}

fn non_exact_evidence_reason(value: &str) -> &'static str {
    match value {
        "unmanaged-browser-process" => "unmanaged-process-only",
        "network-domain" => "network-domain-only",
        _ => "no-exact-evidence",
    }
}


