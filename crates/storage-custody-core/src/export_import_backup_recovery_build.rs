use ocentra_schema::export_import_backup_recovery as contracts;

use super::{
    ExportBundleBuildError, ExportBundleBuildRequest, ExportHumanSummaryInput,
    ExportPayloadSectionInput,
};

pub(super) fn derive_export_bundle(
    _request: ExportBundleBuildRequest,
    _sections: Vec<ExportPayloadSectionInput>,
    _summary: ExportHumanSummaryInput,
) -> Result<contracts::ExportImportRecoveryBundle, ExportBundleBuildError> {
    // The wire model describes encrypted payloads, but this crate has no
    // production key-custody/envelope owner yet.  Refusing to construct a
    // bundle keeps metadata from being mistaken for encryption.
    Err(ExportBundleBuildError::EncryptionCustodyUnavailable)
}
