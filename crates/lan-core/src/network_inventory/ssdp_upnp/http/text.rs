pub fn extract_xml_text(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let start = find_xml_open_tag_start(xml, &open)?;
    let open_tag_tail = xml[(start + open.len())..].chars().next()?;
    if !matches!(open_tag_tail, '>' | '/' | ' ' | '\t' | '\r' | '\n') {
        return None;
    }
    let start = xml[(start + open.len())..].find('>')? + start + open.len() + 1;
    let end = xml[start..].find(&close)? + start;
    let text = &xml[start..end];
    if text.contains('<') || text.contains('>') {
        return None;
    }
    let text = unescape_xml_text(text);
    let text = text.trim();
    (!text.is_empty()).then_some(text.to_string())
}

pub fn find_xml_open_tag_start(xml: &str, open: &str) -> Option<usize> {
    let mut search_start = 0_usize;
    while let Some(index) = xml[search_start..].find(open) {
        let start = search_start + index;
        if start == 0 {
            return Some(start);
        }
        let prev = xml[..start].chars().last()?;
        if !prev.is_alphanumeric() && prev != ':' && prev != '_' && prev != '-' {
            return Some(start);
        }
        search_start = start + open.len();
    }
    None
}

pub fn unescape_xml_text(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

pub fn parse_udn(value: &str) -> Option<String> {
    let value = value.trim();
    let value = value
        .strip_prefix("urn:uuid:")
        .or_else(|| value.strip_prefix("uuid:"))
        .unwrap_or(value);
    let value = value.split("::").next().unwrap_or(value);
    let value = value.trim();
    (!value.is_empty()).then_some(value.to_string())
}

pub fn short_ssdp_label(value: Option<&str>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim();
        if value.is_empty() {
            return None;
        }
        let leaf = short_ssdp_leaf_name(value).trim();
        (!leaf.is_empty()).then_some(leaf.to_string())
    })
}

pub fn short_ssdp_leaf_name(value: &str) -> &str {
    let value = value.split(['#', '/']).next_back().unwrap_or(value);
    let mut parts = value.split(':').collect::<Vec<_>>();
    if parts.last().is_some_and(|part| {
        !part.is_empty() && part.chars().all(|character| character.is_ascii_digit())
    }) {
        let _ = parts.pop();
    }
    parts
        .into_iter()
        .rev()
        .find(|part| !part.is_empty() && *part != "device" && *part != "service")
        .unwrap_or(value)
}
