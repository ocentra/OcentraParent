use crate::family_identity::RecoveryId;
use crate::recovery_lifecycle::{RecoveryCustodyHandoff, RecoveryCustodyHandoffInput};
use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId,
};
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};

use super::authority::timestamp;
use super::support_recovery_kind_from_label::recovery_kind_from_label;
use super::InviteRecoveryRepositoryError;

pub(crate) struct DurableRecoveryHandoffInput<'a> {
    pub(crate) handoff_id: String,
    pub(crate) correlation_id: String,
    pub(crate) recovery_id: String,
    pub(crate) household_id: String,
    pub(crate) account_id: String,
    pub(crate) member_id: String,
    pub(crate) device_id: String,
    pub(crate) kind: &'a str,
    pub(crate) requested_at_epoch_millis: i64,
}

pub(crate) fn durable_handoff(
    input: DurableRecoveryHandoffInput<'_>,
) -> Result<RecoveryCustodyHandoff, InviteRecoveryRepositoryError> {
    let DurableRecoveryHandoffInput {
        handoff_id,
        correlation_id,
        recovery_id,
        household_id,
        account_id,
        member_id,
        device_id,
        kind,
        requested_at_epoch_millis,
    } = input;
    let recovery_id =
        RecoveryId::parse(recovery_id).map_err(InviteRecoveryRepositoryError::InvalidValue)?;
    let household_id =
        FamilyId::parse(household_id).ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
    let account_id =
        ParentAccountId::parse(account_id).ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
    let member_id = AccountIdentityMemberId::parse(member_id)
        .ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
    let device_id = AccountIdentityDeviceId::parse(device_id)
        .ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
    let kind =
        recovery_kind_from_label(kind).ok_or(InviteRecoveryRepositoryError::HandoffConflict)?;
    let requested_at = timestamp(requested_at_epoch_millis)?;
    Ok(RecoveryCustodyHandoff::from_durable(
        RecoveryCustodyHandoffInput {
            handoff_id,
            correlation_id,
            recovery_id,
            household_id,
            account_id,
            member_id,
            device_id,
            kind,
            requested_at,
        },
    ))
}
