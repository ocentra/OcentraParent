use super::*;

macro_rules! retention_delete_identifier {
    ($function_name:ident, $type_name:ty, $expect_message:ident) => {
        pub(super) fn $function_name(value: impl Into<String>) -> $type_name {
            crate::schema_option_or_unreachable(<$type_name>::parse(value), $expect_message)
        }
    };
}

retention_delete_identifier!(
    contract_version,
    RetentionDeleteContractVersion,
    RETENTION_DELETE_EXPECT_CONTRACT_VERSION
);
retention_delete_identifier!(
    request_id,
    RetentionDeleteRequestId,
    RETENTION_DELETE_EXPECT_REQUEST_ID
);
retention_delete_identifier!(row_id, RetentionDeleteRowId, RETENTION_DELETE_EXPECT_ROW_ID);
retention_delete_identifier!(
    family_id,
    RetentionDeleteFamilyId,
    RETENTION_DELETE_EXPECT_FAMILY_ID
);
retention_delete_identifier!(
    actor_id,
    RetentionDeleteActorId,
    RETENTION_DELETE_EXPECT_ACTOR_ID
);
retention_delete_identifier!(
    action_ref,
    RetentionDeleteActionRef,
    RETENTION_DELETE_EXPECT_ACTION_REF
);
retention_delete_identifier!(
    tombstone_ref,
    RetentionDeleteTombstoneRef,
    RETENTION_DELETE_EXPECT_TOMBSTONE_REF
);
retention_delete_identifier!(
    replay_ref,
    RetentionDeleteReplayRef,
    RETENTION_DELETE_EXPECT_REPLAY_REF
);
retention_delete_identifier!(
    proof_ref,
    RetentionDeleteProofRef,
    RETENTION_DELETE_EXPECT_PROOF_REF
);
retention_delete_identifier!(
    timestamp,
    RetentionDeleteTimestamp,
    RETENTION_DELETE_EXPECT_TIMESTAMP
);
