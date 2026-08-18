use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use ocentra_schema::report_query_custody as contracts;

use super::{
    ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput,
    report_query_custody_request_validate, report_query_custody_row_validate,
};

/// A source result that has crossed the custody boundary.
///
/// The fields deliberately remain private. Request/query transport values may
/// describe a source, but they cannot mint this result or assert source
/// currentness. The only constructor is crate-private and requires the
/// already-verified account authority plus the custody request checks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportQueryCustodySourceResolution {
    input: ReportQueryCustodyDerivationInput,
    authority_generation: u64,
}

impl ReportQueryCustodySourceResolution {
    pub(crate) fn from_owned_source(
        request: &contracts::ReportQueryCustodyRequest,
        input: ReportQueryCustodyDerivationInput,
        authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<Self, ReportQueryCustodyDerivationError> {
        report_query_custody_request_validate::validate_report_query_custody_request(
            request, authority,
        )?;
        report_query_custody_row_validate::validate_report_query_custody_input(
            request, &input, authority,
        )?;
        Ok(Self {
            input,
            authority_generation: authority.authority_generation(),
        })
    }

    pub(super) fn into_input(self) -> ReportQueryCustodyDerivationInput {
        self.input
    }

    pub(super) fn authority_generation(&self) -> u64 {
        self.authority_generation
    }
}

mod sealed {
    pub trait Sealed {}
}

/// Producer-owned query sources must be handed into custody through this port.
///
/// This port is sealed until the owning producer can provide a real current
/// query-store resolution. In particular, `ActivityStore` and saved report
/// JSON remain agent-service-owned; storage custody must not import them or
/// treat a path, row id, cursor, or report document as authority. The exact
/// handoff owner is `crates/agent-service`'s activity/report surface, pending
/// an approved dependency edge into this boundary.
pub trait ReportQueryCustodySourcePort: sealed::Sealed {
    type Error;

    fn resolve(
        &self,
        request: &contracts::ReportQueryCustodyRequest,
        authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<Vec<ReportQueryCustodySourceResolution>, Self::Error>;
}
