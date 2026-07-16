use super::*;

const URL_SEPARATOR: &str = "://";
const HTTP_SCHEME: &str = "http";
const HTTPS_SCHEME: &str = "https";
const EMPTY_TEXT: &str = "";
const ROOT_PATH: &str = "/";
const SPACE_TEXT: &str = " ";
const DOMAIN_SEPARATOR: &str = ".";

pub(super) fn parse_url(value: impl std::fmt::Display) -> Option<ParsedBrowserUrl> {
    let value = value.to_string();
    let separator_index = value.find(URL_SEPARATOR)?;
    if separator_index == 0 {
        return None;
    }
    let scheme = value[..separator_index].to_ascii_lowercase();
    if scheme != HTTP_SCHEME && scheme != HTTPS_SCHEME {
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
    let suffix = authority_end
        .map(|index| &remainder[index..])
        .unwrap_or(EMPTY_TEXT);
    let path = path_from_suffix(suffix);
    Some(ParsedBrowserUrl {
        normalized_url: format!("{scheme}{URL_SEPARATOR}{}{suffix}", normalized.0),
        domain: normalized.1.to_string(),
        path: path.to_string(),
        query: query_from_suffix(suffix).map(|value| value.to_string()),
    })
}

fn first_suffix_index(value: impl std::fmt::Display) -> Option<usize> {
    let value = value.to_string();
    ['/', '?', '#']
        .iter()
        .filter_map(|separator| value.find(*separator))
        .min()
}

fn normalized_authority(value: impl std::fmt::Display) -> Option<(BrowserUrlText, BrowserUrlText)> {
    let (host, port) = split_host_and_port(value);
    let domain = normalized_host(host)?;
    let authority = match port {
        Some(port) => format!("{domain}:{port}"),
        None => domain.to_string(),
    };
    Some((BrowserUrlText::from_display(authority), domain))
}

fn split_host_and_port(value: impl std::fmt::Display) -> (BrowserUrlText, Option<BrowserUrlText>) {
    let value = value.to_string();
    if value.matches(':').count() == 1 {
        let separator_index = value.rfind(':').unwrap_or_default();
        let host = &value[..separator_index];
        let port = &value[separator_index + 1..];
        if !host.is_empty()
            && !port.is_empty()
            && port.chars().all(|character| character.is_ascii_digit())
        {
            return (
                BrowserUrlText::from_display(host),
                Some(BrowserUrlText::from_display(port)),
            );
        }
    }
    (BrowserUrlText::from_display(value), None)
}

fn normalized_host(value: impl std::fmt::Display) -> Option<BrowserUrlText> {
    let value = value.to_string();
    let normalized = value.trim_end_matches('.').to_ascii_lowercase();
    (!normalized.is_empty() && !normalized.contains('/'))
        .then(|| BrowserUrlText::from_display(normalized))
}

fn path_from_suffix(value: impl std::fmt::Display) -> BrowserUrlText {
    let value = value.to_string();
    if !value.starts_with('/') {
        return BrowserUrlText::from_display(ROOT_PATH);
    }
    let end_index = [value.find('?'), value.find('#')]
        .into_iter()
        .flatten()
        .min()
        .unwrap_or(value.len());
    BrowserUrlText::from_display(&value[..end_index])
}

fn query_from_suffix(value: impl std::fmt::Display) -> Option<BrowserUrlText> {
    let value = value.to_string();
    let query_start = value.find('?')?;
    let hash_index = value[query_start..]
        .find('#')
        .map(|index| query_start + index)
        .unwrap_or(value.len());
    Some(BrowserUrlText::from_display(
        &value[query_start + 1..hash_index],
    ))
}

pub(super) fn path_segments(parsed: &ParsedBrowserUrl) -> Vec<BrowserUrlText> {
    parsed
        .path
        .split('/')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(BrowserUrlText::from_display)
        .collect()
}

pub(super) fn first_path_segment(parsed: &ParsedBrowserUrl) -> Option<BrowserUrlText> {
    path_segments(parsed).into_iter().next()
}

pub(super) fn query_param(
    parsed: &ParsedBrowserUrl,
    key: impl std::fmt::Display,
) -> Option<BrowserUrlText> {
    let key = key.to_string();
    let query = parsed.query.as_ref()?;
    for part in query.split('&') {
        let separator_index = part.find('=');
        let raw_key = separator_index.map(|index| &part[..index]).unwrap_or(part);
        if raw_key == key {
            let value = separator_index
                .map(|index| &part[index + 1..])
                .unwrap_or(EMPTY_TEXT);
            if value.is_empty() {
                return None;
            }
            return Some(BrowserUrlText::from_display(value.replace('+', SPACE_TEXT)));
        }
    }
    None
}

pub(super) fn domain_matches_any(
    domain: impl std::fmt::Display,
    bases: impl IntoIterator<Item = impl std::fmt::Display>,
) -> bool {
    let domain = domain.to_string();
    bases.into_iter().any(|base| {
        let base = base.to_string();
        domain == base || domain.ends_with(&format!("{DOMAIN_SEPARATOR}{base}"))
    })
}
