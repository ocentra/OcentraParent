use crate::setup_lifecycle::RecoveryKind;

pub(crate) fn recovery_kind_from_label(value: &str) -> Option<RecoveryKind> {
    Some(match value {
        "forgot-login" => RecoveryKind::ForgotLogin,
        "lost-parent-device" => RecoveryKind::LostParentDevice,
        "compromised-account" => RecoveryKind::CompromisedAccount,
        "child-reinstall" => RecoveryKind::ChildReinstall,
        "household-transfer" => RecoveryKind::HouseholdTransfer,
        _ => return None,
    })
}
