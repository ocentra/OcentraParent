use super::network_control_catalog_text::capitalize;

pub fn slug_token(value: &str) -> String {
    let mut slugged = String::new();
    let mut previous_dash = false;
    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            slugged.push(ch);
            previous_dash = false;
        } else if !previous_dash {
            slugged.push('-');
            previous_dash = true;
        }
    }
    let slugged = slugged.trim_matches('-').to_owned();
    if slugged.is_empty() {
        "item".to_owned()
    } else {
        slugged
    }
}

pub fn title_from_token(value: &str) -> String {
    value
        .split('-')
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}
