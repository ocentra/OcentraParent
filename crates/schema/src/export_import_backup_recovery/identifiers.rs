use super::*;

macro_rules! export_import_identifier {
    ($function_name:ident, $type_name:ty, $expect_message:ident) => {
        pub(super) fn $function_name(value: impl Into<String>) -> $type_name {
            crate::schema_option_or_unreachable(<$type_name>::parse(value), $expect_message)
        }
    };
}

export_import_identifier!(
    contract_version,
    ExportImportContractVersion,
    EXPORT_IMPORT_EXPECT_CONTRACT_VERSION
);
export_import_identifier!(
    bundle_id,
    ExportImportBundleId,
    EXPORT_IMPORT_EXPECT_BUNDLE_ID
);
export_import_identifier!(
    household_id,
    ExportImportHouseholdId,
    EXPORT_IMPORT_EXPECT_HOUSEHOLD_ID
);
export_import_identifier!(
    device_id,
    ExportImportDeviceId,
    EXPORT_IMPORT_EXPECT_DEVICE_ID
);
export_import_identifier!(key_ref, ExportImportKeyRef, EXPORT_IMPORT_EXPECT_KEY_REF);
export_import_identifier!(
    payload_ref,
    ExportImportPayloadRef,
    EXPORT_IMPORT_EXPECT_PAYLOAD_REF
);
export_import_identifier!(
    integrity_ref,
    ExportImportIntegrityRef,
    EXPORT_IMPORT_EXPECT_INTEGRITY_REF
);
export_import_identifier!(
    tombstone_cursor,
    ExportImportTombstoneCursor,
    EXPORT_IMPORT_EXPECT_TOMBSTONE_CURSOR
);
export_import_identifier!(
    timestamp,
    ExportImportTimestamp,
    EXPORT_IMPORT_EXPECT_TIMESTAMP
);
export_import_identifier!(
    product_version,
    ExportImportProductVersion,
    EXPORT_IMPORT_EXPECT_PRODUCT_VERSION
);
export_import_identifier!(
    migration_ref,
    ExportImportMigrationRef,
    EXPORT_IMPORT_EXPECT_MIGRATION_REF
);
