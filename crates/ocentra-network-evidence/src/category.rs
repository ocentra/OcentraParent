use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;
use crate::domain::NormalizedDomainEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkCategory {
    Social,
    Video,
    Game,
    CloudGaming,
    Education,
    Productivity,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategorySourceCustody {
    BuiltInFixture,
    SignedLocalSnapshot,
    ParentProvidedImport,
    Unverified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainCategorySource {
    pub source_id: String,
    pub retrieved_at_epoch_seconds: u64,
    pub max_age_seconds: u64,
    pub custody: CategorySourceCustody,
    pub signature_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainCategoryRecord {
    pub domain: String,
    pub category: NetworkCategory,
    pub source: DomainCategorySource,
    pub confidence_percent: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainCategoryDatabase {
    pub records: Vec<DomainCategoryRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategoryMatchKind {
    ExactDomain,
    RegistrableDomain,
    NoMatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategoryFreshnessState {
    Fresh {
        age_seconds: u64,
        max_age_seconds: u64,
    },
    Stale {
        age_seconds: u64,
        max_age_seconds: u64,
    },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainCategoryLookup {
    pub normalized_domain: String,
    pub matched_domain: Option<String>,
    pub match_kind: CategoryMatchKind,
    pub category: NetworkCategory,
    pub source_id: Option<String>,
    pub source_custody: Option<CategorySourceCustody>,
    pub freshness: CategoryFreshnessState,
    pub confidence_percent: Option<u8>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainCategoryError {
    InvalidRecordDomain(String),
    DuplicateRecordDomain(String),
    InvalidConfidencePercent {
        domain: String,
        confidence_percent: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategoryUpdateDecision {
    Accept,
    RejectMissingSignature,
    RejectOlderSnapshot,
}

pub fn lookup_domain_category(
    database: &DomainCategoryDatabase,
    evidence: &NormalizedDomainEvidence,
    now_epoch_seconds: u64,
) -> DomainCategoryLookup {
    let matched = database
        .record_for_domain(&evidence.normalized_domain)
        .map(|record| (record, CategoryMatchKind::ExactDomain))
        .or_else(|| {
            evidence.registrable_domain.as_ref().and_then(|domain| {
                database
                    .record_for_domain(domain)
                    .map(|record| (record, CategoryMatchKind::RegistrableDomain))
            })
        });

    match matched {
        Some((record, match_kind)) => DomainCategoryLookup {
            normalized_domain: evidence.normalized_domain.clone(),
            matched_domain: Some(record.domain.clone()),
            match_kind,
            category: record.category,
            source_id: Some(record.source.source_id.clone()),
            source_custody: Some(record.source.custody),
            freshness: source_freshness(&record.source, now_epoch_seconds),
            confidence_percent: Some(record.confidence_percent),
            evidence_grade: NetworkEvidenceGrade::C,
            exact_url_available: false,
            decrypted_payload_available: false,
        },
        None => DomainCategoryLookup {
            normalized_domain: evidence.normalized_domain.clone(),
            matched_domain: None,
            match_kind: CategoryMatchKind::NoMatch,
            category: NetworkCategory::Unknown,
            source_id: None,
            source_custody: None,
            freshness: CategoryFreshnessState::Unknown,
            confidence_percent: None,
            evidence_grade: evidence.evidence_grade,
            exact_url_available: false,
            decrypted_payload_available: false,
        },
    }
}

pub fn evaluate_category_source_update(
    current: Option<&DomainCategorySource>,
    candidate: &DomainCategorySource,
) -> CategoryUpdateDecision {
    if candidate.signature_required && candidate.custody == CategorySourceCustody::Unverified {
        return CategoryUpdateDecision::RejectMissingSignature;
    }

    if let Some(current) = current {
        if candidate.retrieved_at_epoch_seconds <= current.retrieved_at_epoch_seconds {
            return CategoryUpdateDecision::RejectOlderSnapshot;
        }
    }

    CategoryUpdateDecision::Accept
}

impl DomainCategoryDatabase {
    pub fn from_records(records: Vec<DomainCategoryRecord>) -> Result<Self, DomainCategoryError> {
        let mut domains = Vec::new();
        for record in &records {
            validate_record(record)?;
            if domains.contains(&record.domain) {
                return Err(DomainCategoryError::DuplicateRecordDomain(
                    record.domain.clone(),
                ));
            }
            domains.push(record.domain.clone());
        }

        Ok(Self { records })
    }

    fn record_for_domain(&self, domain: &str) -> Option<&DomainCategoryRecord> {
        self.records.iter().find(|record| record.domain == domain)
    }
}

fn source_freshness(
    source: &DomainCategorySource,
    now_epoch_seconds: u64,
) -> CategoryFreshnessState {
    let age_seconds = now_epoch_seconds.saturating_sub(source.retrieved_at_epoch_seconds);
    if age_seconds > source.max_age_seconds {
        CategoryFreshnessState::Stale {
            age_seconds,
            max_age_seconds: source.max_age_seconds,
        }
    } else {
        CategoryFreshnessState::Fresh {
            age_seconds,
            max_age_seconds: source.max_age_seconds,
        }
    }
}

fn validate_record(record: &DomainCategoryRecord) -> Result<(), DomainCategoryError> {
    if !is_normalized_record_domain(&record.domain) {
        return Err(DomainCategoryError::InvalidRecordDomain(
            record.domain.clone(),
        ));
    }
    if record.confidence_percent > 100 {
        return Err(DomainCategoryError::InvalidConfidencePercent {
            domain: record.domain.clone(),
            confidence_percent: record.confidence_percent,
        });
    }

    Ok(())
}

fn is_normalized_record_domain(domain: &str) -> bool {
    !domain.is_empty()
        && domain == domain.trim()
        && !domain.ends_with('.')
        && !domain.contains("..")
        && domain.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '-'
                || character == '.'
        })
}
