pub(super) fn first_xml_text_by_local_name(text: &str, local_name: &str) -> Option<String> {
    let mut cursor = 0_usize;
    while let Some(relative_start) = text.get(cursor..)?.find('<') {
        let start = cursor + relative_start + 1;
        let tag_end = start + text.get(start..)?.find('>')?;
        let tag_head = text.get(start..tag_end)?.trim();
        if tag_head.starts_with('/') || tag_head.starts_with('!') || tag_head.starts_with('?') {
            cursor = tag_end + 1;
            continue;
        }
        let tag_name = tag_head
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches('/');
        let local = tag_name.rsplit(':').next().unwrap_or(tag_name);
        if local.eq_ignore_ascii_case(local_name) {
            let content_start = tag_end + 1;
            let closing_relative = text.get(content_start..)?.find("</")?;
            let content = text.get(content_start..content_start + closing_relative)?;
            let sanitized = strip_xml_tags(content);
            if !sanitized.trim().is_empty() {
                return Some(sanitized.trim().to_string());
            }
        }
        cursor = tag_end + 1;
    }
    None
}

fn strip_xml_tags(text: &str) -> String {
    let mut stripped = String::new();
    let mut inside_tag = false;
    for character in text.chars() {
        match character {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => stripped.push(character),
            _ => {}
        }
    }
    stripped
}
