use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

fn parse_text_identifier(value: impl Into<String>) -> Option<String> {
    let value = value.into();
    (!value.trim().is_empty()).then_some(value)
}

export_import_text_identifiers!(
    ExportImportContractVersion,
    ExportImportBundleId,
    ExportImportHouseholdId,
    ExportImportDeviceId,
    ExportImportKeyRef,
    ExportImportPayloadRef,
    ExportImportIntegrityRef,
    ExportImportTombstoneCursor,
    ExportImportTimestamp,
    ExportImportProductVersion,
    ExportImportMigrationRef,
    ExportImportScheduleRef,
    ExportImportJobRef,
    ExportImportOperationRef,
    ExportImportIdempotencyRef,
    ExportImportExecutionRef,
    ExportImportProviderOperationRef,
    ExportImportMigrationPlanRef,
);
