#![cfg(test)]

use super::validate_report_query_custody_schema_version;
use crate::report_query_custody::ReportQueryCustodyDerivationError;
use ocentra_schema::report_query_custody as contracts;

#[test]
fn schema_version_requires_the_canonical_non_empty_contract_identifier() {
    let mut request = contracts::sample_report_query_custody_contract_proof().request;

    assert_eq!(
        validate_report_query_custody_schema_version(&request),
        Ok(())
    );

    for invalid_version in [
        String::new(),
        String::from("0"),
        String::from("unsupported"),
    ] {
        request.schema_version = invalid_version;
        assert_eq!(
            validate_report_query_custody_schema_version(&request),
            Err(ReportQueryCustodyDerivationError::InvalidContractVersion)
        );
    }
}
