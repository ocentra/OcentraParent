use super::ActivityScreenChildDisclosureState;

pub(super) fn copy_for_state(
    state: ActivityScreenChildDisclosureState,
) -> (&'static str, &'static str) {
    match state {
        ActivityScreenChildDisclosureState::Enabled => (
            "Screen activity checks are on",
            "A parent-enabled local screen check is ready and will show its status here.",
        ),
        ActivityScreenChildDisclosureState::Paused => (
            "Screen activity checks are paused",
            "Screen checks are paused by the parent. This status will remain visible until they resume.",
        ),
        ActivityScreenChildDisclosureState::Disabled => (
            "Screen activity checks are off",
            "Screen checks are disabled by the parent, so this device is not capturing screen evidence.",
        ),
        ActivityScreenChildDisclosureState::Unavailable => (
            "Screen activity status is unavailable",
            "The device could not provide a current screen status. No hidden capture is represented.",
        ),
        ActivityScreenChildDisclosureState::ManualRequired => (
            "Screen activity needs device permission",
            "A device permission or manual step is required before a screen check can run.",
        ),
        ActivityScreenChildDisclosureState::CaptureActive => (
            "A screen check is active",
            "This notice is visible while the local device processes an allowed screen check.",
        ),
        ActivityScreenChildDisclosureState::ProtectedSurface => (
            "This screen is protected",
            "The platform blocked this screen, so Ocentra records the limitation instead of capturing pixels.",
        ),
        ActivityScreenChildDisclosureState::SummaryReady => (
            "A screen activity summary is ready",
            "The local image is deleted; only the safe summary and evidence references remain.",
        ),
    }
}
