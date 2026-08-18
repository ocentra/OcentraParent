use crate::setup_lifecycle::RecoverySupportChannel;

pub(crate) fn support_channel_label(channel: RecoverySupportChannel) -> &'static str {
    match channel {
        RecoverySupportChannel::SelfServe => "self-serve",
        RecoverySupportChannel::HouseholdOwnerAssisted => "household-owner-assisted",
        RecoverySupportChannel::SupportAssisted => "support-assisted",
    }
}
