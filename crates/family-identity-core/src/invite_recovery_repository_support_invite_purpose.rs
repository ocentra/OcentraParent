use crate::setup_lifecycle::SetupInvitePurpose;

pub(crate) fn purpose_label(purpose: SetupInvitePurpose) -> &'static str {
    match purpose {
        SetupInvitePurpose::CoParentInvite => "co-parent-invite",
        SetupInvitePurpose::ObserverInvite => "observer-invite",
        SetupInvitePurpose::ChildDevicePairing => "child-device-pairing",
        SetupInvitePurpose::HouseholdTransfer => "household-transfer",
    }
}
