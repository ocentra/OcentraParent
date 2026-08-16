use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;

const DOMAIN_MAX_LEN: usize = 253;
const LABEL_MAX_LEN: usize = 63;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedDomainEvidence {
    pub normalized_domain: String,
    pub labels: Vec<String>,
    pub public_suffix: Option<String>,
    pub registrable_domain: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicSuffixModel {
    suffixes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainNormalizationError {
    EmptyDomain,
    DomainTooLong,
    EmptyLabel,
    LabelTooLong,
    LabelStartsOrEndsWithHyphen,
    InvalidLabelCharacter { label: String, character: char },
    InvalidPublicSuffixRule,
}

pub fn normalize_domain_with_public_suffix(
    input: &str,
    model: &PublicSuffixModel,
) -> Result<NormalizedDomainEvidence, DomainNormalizationError> {
    let normalized_domain = normalize_domain(input)?;
    let labels = domain_labels(&normalized_domain);
    let public_suffix = model.public_suffix_for(&normalized_domain);
    let registrable_domain = public_suffix
        .as_ref()
        .and_then(|suffix| registrable_domain(&labels, suffix));

    Ok(NormalizedDomainEvidence {
        normalized_domain,
        labels,
        public_suffix,
        registrable_domain,
        evidence_grade: NetworkEvidenceGrade::C,
        exact_url_available: false,
        decrypted_payload_available: false,
    })
}

impl PublicSuffixModel {
    pub fn from_suffixes(rules: &[&str]) -> Result<Self, DomainNormalizationError> {
        let mut suffixes = Vec::new();
        for rule in rules {
            let suffix = normalize_domain(rule)?;
            if suffix.contains("..") {
                return Err(DomainNormalizationError::InvalidPublicSuffixRule);
            }
            suffixes.push(suffix);
        }
        suffixes.sort_by_key(|suffix| std::cmp::Reverse(suffix.split('.').count()));
        suffixes.dedup();

        Ok(Self { suffixes })
    }

    pub fn ocentra_fixture() -> Self {
        Self {
            suffixes: vec![
                "github.io".to_owned(),
                "co.uk".to_owned(),
                "test".to_owned(),
                "com".to_owned(),
                "uk".to_owned(),
            ],
        }
    }

    pub fn public_suffix_for(&self, domain: &str) -> Option<String> {
        self.suffixes
            .iter()
            .find(|suffix| domain == suffix.as_str() || domain.ends_with(&format!(".{suffix}")))
            .cloned()
    }
}

fn normalize_domain(input: &str) -> Result<String, DomainNormalizationError> {
    let trimmed = input.trim().trim_end_matches('.');
    if trimmed.is_empty() {
        return Err(DomainNormalizationError::EmptyDomain);
    }
    if trimmed.len() > DOMAIN_MAX_LEN {
        return Err(DomainNormalizationError::DomainTooLong);
    }

    let normalized = trimmed.to_ascii_lowercase();
    for label in normalized.split('.') {
        validate_label(label)?;
    }

    Ok(normalized)
}

fn validate_label(label: &str) -> Result<(), DomainNormalizationError> {
    if label.is_empty() {
        return Err(DomainNormalizationError::EmptyLabel);
    }
    if label.len() > LABEL_MAX_LEN {
        return Err(DomainNormalizationError::LabelTooLong);
    }
    if label.starts_with('-') || label.ends_with('-') {
        return Err(DomainNormalizationError::LabelStartsOrEndsWithHyphen);
    }
    for character in label.chars() {
        if !character.is_ascii_alphanumeric() && character != '-' {
            return Err(DomainNormalizationError::InvalidLabelCharacter {
                label: label.to_owned(),
                character,
            });
        }
    }

    Ok(())
}

fn domain_labels(domain: &str) -> Vec<String> {
    domain.split('.').map(str::to_owned).collect()
}

fn registrable_domain(labels: &[String], suffix: &str) -> Option<String> {
    let suffix_label_count = suffix.split('.').count();
    if labels.len() <= suffix_label_count {
        return None;
    }

    Some(labels[labels.len() - suffix_label_count - 1..].join("."))
}
