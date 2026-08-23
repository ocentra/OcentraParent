use crate::setup_lifecycle::RecoveryKind;

pub(crate) fn recovery_kind_label(kind: RecoveryKind) -> &'static str {
    match kind {
        RecoveryKind::ForgotLogin => "forgot-login",
        RecoveryKind::LostParentDevice => "lost-parent-device",
        RecoveryKind::CompromisedAccount => "compromised-account",
        RecoveryKind::ChildReinstall => "child-reinstall",
        RecoveryKind::HouseholdTransfer => "household-transfer",
    }
}
