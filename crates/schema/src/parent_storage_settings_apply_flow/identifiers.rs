use super::constants::{
    PARENT_STORAGE_EXPECT_ACTION_ID, PARENT_STORAGE_EXPECT_APPLY_ID,
    PARENT_STORAGE_EXPECT_CONTRACT_VERSION, PARENT_STORAGE_EXPECT_PREVIEW_ID,
    PARENT_STORAGE_EXPECT_ROW_ID, PARENT_STORAGE_EXPECT_TIMESTAMP,
};
use super::text_types::{
    ParentStorageActionId, ParentStorageApplyId, ParentStorageContractVersion,
    ParentStoragePreviewId, ParentStorageSettingsRowId, ParentStorageTimestamp,
};

pub(super) fn owned_text(value: &str) -> String {
    value.to_owned()
}

macro_rules! parent_storage_identifier {
    ($function_name:ident, $type_name:ty, $expect_message:ident) => {
        pub(super) fn $function_name(value: impl Into<String>) -> $type_name {
            crate::schema_option_or_unreachable(<$type_name>::parse(value), $expect_message)
        }
    };
}

parent_storage_identifier!(
    contract_version,
    ParentStorageContractVersion,
    PARENT_STORAGE_EXPECT_CONTRACT_VERSION
);
parent_storage_identifier!(
    row_id,
    ParentStorageSettingsRowId,
    PARENT_STORAGE_EXPECT_ROW_ID
);
parent_storage_identifier!(
    preview_id,
    ParentStoragePreviewId,
    PARENT_STORAGE_EXPECT_PREVIEW_ID
);
parent_storage_identifier!(
    apply_id,
    ParentStorageApplyId,
    PARENT_STORAGE_EXPECT_APPLY_ID
);
parent_storage_identifier!(
    action_id,
    ParentStorageActionId,
    PARENT_STORAGE_EXPECT_ACTION_ID
);
parent_storage_identifier!(
    timestamp,
    ParentStorageTimestamp,
    PARENT_STORAGE_EXPECT_TIMESTAMP
);
