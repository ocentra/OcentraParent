use super::network_control_catalog_text::{
    clean_option_label, matrix_option_labels, split_explicit_options,
};

pub fn question_from_source_text(source_text: &str, explicit_question: Option<&str>) -> String {
    if let Some(question) = explicit_question {
        if !question.is_empty() {
            return question.to_owned();
        }
    }
    let trimmed = source_text.trim_end_matches('.');
    if trimmed.ends_with('?') {
        return trimmed.to_owned();
    }
    if trimmed.starts_with("Capability matrix row |") {
        let capability = trimmed
            .split(" | ")
            .find_map(|part| part.strip_prefix("Capability="))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("network capability");
        return format!("Represent {capability} capability status.");
    }
    if let Some(colon_index) = trimmed.find(':') {
        return format!("Configure {}.", trimmed[..colon_index].to_lowercase());
    }
    format!(
        "Represent {}?",
        super::network_control_catalog_text::lower_first(trimmed)
    )
}

pub fn explicit_option_labels(source_text: &str) -> Vec<String> {
    let matrix_options = matrix_option_labels(source_text);
    if !matrix_options.is_empty() {
        return matrix_options;
    }
    let Some(colon_index) = source_text.find(':') else {
        return Vec::new();
    };
    split_explicit_options(&source_text[colon_index + 1..source_text.len()])
        .into_iter()
        .map(|part| clean_option_label(&part))
        .filter(|part| !part.is_empty())
        .collect()
}
