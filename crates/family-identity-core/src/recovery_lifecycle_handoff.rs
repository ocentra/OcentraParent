use super::*;

impl RecoveryCustodyHandoff {
    pub fn handoff_id(&self) -> &str {
        &self.handoff_id
    }

    pub fn correlation_id(&self) -> &str {
        &self.correlation_id
    }

    pub fn recovery_id(&self) -> &RecoveryId {
        &self.recovery_id
    }

    pub fn household_id(&self) -> &FamilyId {
        &self.household_id
    }

    pub fn account_id(&self) -> &ParentAccountId {
        &self.account_id
    }

    pub fn member_id(&self) -> &AccountIdentityMemberId {
        &self.member_id
    }

    pub fn device_id(&self) -> &AccountIdentityDeviceId {
        &self.device_id
    }

    pub fn kind(&self) -> RecoveryKind {
        self.kind
    }

    pub fn requested_at(&self) -> &str {
        &self.requested_at
    }

    pub(crate) fn from_durable(input: RecoveryCustodyHandoffInput) -> Self {
        let RecoveryCustodyHandoffInput {
            handoff_id,
            correlation_id,
            recovery_id,
            household_id,
            account_id,
            member_id,
            device_id,
            kind,
            requested_at,
        } = input;
        Self {
            handoff_id,
            correlation_id,
            recovery_id,
            household_id,
            account_id,
            member_id,
            device_id,
            kind,
            requested_at,
        }
    }
}
