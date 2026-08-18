use super::ActivityScreenChildDisclosureState;

pub(super) fn copy_for_state(
    state: ActivityScreenChildDisclosureState,
) -> (&'static str, &'static str) {
    match state {
        ActivityScreenChildDisclosureState::Enabled => (
            "Screen disclosure is proposed",
            "Current typed settings and capability permit screen analysis; no child disclosure surface is delivered.",
        ),
        ActivityScreenChildDisclosureState::Paused => (
            "Screen disclosure is proposed and paused",
            "An authoritative paused state is not currently supplied; no child disclosure surface is delivered.",
        ),
        ActivityScreenChildDisclosureState::Disabled => (
            "Screen disclosure is proposed and disabled",
            "Current parent settings keep screen analysis disabled; no child disclosure surface is delivered.",
        ),
        ActivityScreenChildDisclosureState::Unavailable => (
            "Screen disclosure is unavailable",
            "No typed current capability proves a child disclosure surface. Hidden capture and raw pixels are not represented.",
        ),
        ActivityScreenChildDisclosureState::ManualRequired => (
            "Screen disclosure needs an authoritative manual state",
            "The current typed owner has not supplied a manual-required disclosure state; no child surface is delivered.",
        ),
        ActivityScreenChildDisclosureState::CaptureActive => (
            "Screen capture disclosure is proposed",
            "An authoritative capture lifecycle is not currently supplied; no child disclosure surface is delivered.",
        ),
        ActivityScreenChildDisclosureState::ProtectedSurface => (
            "Protected-screen disclosure is proposed",
            "An authoritative protected-surface state is not currently supplied; raw pixels are not represented.",
        ),
        ActivityScreenChildDisclosureState::SummaryReady => (
            "Screen summary disclosure is diagnostic",
            "An authoritative analysis lifecycle is not currently supplied; only a proposed safe summary boundary exists.",
        ),
    }
}
