use ocentra_schema::report_query_custody as contracts;
use serde::Serialize;

/// A report/query proof validated against one authority snapshot.
///
/// The wrapper is serializable but not deserializable and its payload is
/// private, so untrusted wire DTOs cannot mint this type. Validation captures
/// request, source, pagination, redaction, and authority-snapshot consistency
/// at construction time; it does not claim a race-safe repository re-read or
/// live revocation check after construction.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ValidatedReportQueryCustodyProofSnapshot(contracts::ReportQueryCustodyContractProof);

impl ValidatedReportQueryCustodyProofSnapshot {
    pub(super) fn from_contract(contract: contracts::ReportQueryCustodyContractProof) -> Self {
        Self(contract)
    }

    pub fn contract(&self) -> &contracts::ReportQueryCustodyContractProof {
        &self.0
    }
}
