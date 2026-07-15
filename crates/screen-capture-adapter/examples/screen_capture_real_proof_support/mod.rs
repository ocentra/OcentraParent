mod queue;

#[path = "../../../screen-capture-adapter-generated/screen_capture_real_proof_support_impl.rs"]
mod screen_capture_real_proof_support_impl;

use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_screen_capture_adapter::{
    CapturedScreenImage, ScreenCaptureScope, ScreenCaptureWindowTitleQuery,
};
use std::fmt;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenCaptureProofRunId(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenCaptureProofText(pub(crate) String);

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScreenCaptureProofScopeLabel {
    ActiveWindow,
    SelectedWindow,
    PrimaryDisplay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ScreenCaptureProofPath<'a>(pub(crate) &'a Path);

#[derive(Debug)]
pub(crate) struct ScreenCaptureProofError(pub(crate) String);

impl fmt::Display for ScreenCaptureProofError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ScreenCaptureProofError {}

pub(crate) type ProofResult<T = ()> = Result<T, ScreenCaptureProofError>;

pub(crate) const DEFAULT_OUTPUT_DIR_PATH: &str =
    screen_capture_real_proof_support_impl::DEFAULT_DIR;

const SCREEN_CAPTURE_PROOF_SCOPE_WIRE_LABELS: [&str; 3] = [
    screen_capture_real_proof_support_impl::SCREEN_CAPTURE_PROOF_SCOPE_ACTIVE_WINDOW,
    screen_capture_real_proof_support_impl::SCREEN_CAPTURE_PROOF_SCOPE_SELECTED_WINDOW,
    screen_capture_real_proof_support_impl::SCREEN_CAPTURE_PROOF_SCOPE_PRIMARY_DISPLAY,
];

pub(crate) fn write_run_metadata(
    output_dir: ScreenCaptureProofPath<'_>,
    run_id: &ScreenCaptureProofRunId,
    status: &ActivityCaptureCapabilityStatus,
    target_title: Option<&ScreenCaptureWindowTitleQuery>,
    requested_scope: ScreenCaptureProofScopeLabel,
    keep_raw_until_analysis: bool,
) -> ProofResult {
    screen_capture_real_proof_support_impl::write_run_metadata(
        output_dir,
        run_id,
        status,
        target_title,
        requested_scope,
        keep_raw_until_analysis,
    )
}

pub(crate) fn write_trigger_input(
    output_dir: ScreenCaptureProofPath<'_>,
    requested_scope: ScreenCaptureProofScopeLabel,
) -> ProofResult {
    screen_capture_real_proof_support_impl::write_trigger_input(output_dir, requested_scope)
}

pub(crate) fn write_captured_artifacts(
    output_dir: ScreenCaptureProofPath<'_>,
    run_id: &ScreenCaptureProofRunId,
    image: &CapturedScreenImage,
    requested_scope: ScreenCaptureProofScopeLabel,
    keep_raw_until_analysis: bool,
) -> ProofResult {
    screen_capture_real_proof_support_impl::write_captured_artifacts(
        output_dir,
        run_id,
        image,
        requested_scope,
        keep_raw_until_analysis,
    )
}

pub(crate) fn write_degraded_artifacts(
    output_dir: ScreenCaptureProofPath<'_>,
    status: &ActivityCaptureCapabilityStatus,
) -> ProofResult {
    screen_capture_real_proof_support_impl::write_degraded_artifacts(output_dir, status)
}

pub(crate) fn proof_scope_label(scope: ScreenCaptureScope) -> ScreenCaptureProofScopeLabel {
    match scope {
        ScreenCaptureScope::ActiveWindow => ScreenCaptureProofScopeLabel::ActiveWindow,
        ScreenCaptureScope::SelectedWindow => ScreenCaptureProofScopeLabel::SelectedWindow,
        ScreenCaptureScope::PrimaryDisplay => ScreenCaptureProofScopeLabel::PrimaryDisplay,
    }
}

pub(crate) fn run_id() -> ProofResult<ScreenCaptureProofRunId> {
    screen_capture_real_proof_support_impl::run_id()
}

pub(crate) fn scope_wire_label(scope: ScreenCaptureProofScopeLabel) -> ScreenCaptureProofText {
    ScreenCaptureProofText(
        SCREEN_CAPTURE_PROOF_SCOPE_WIRE_LABELS[scope_wire_label_index(scope)].to_owned(),
    )
}

pub(crate) fn degraded_reason(status: &ActivityCaptureCapabilityStatus) -> ScreenCaptureProofText {
    const REASONS: [&str; 6] = [
        screen_capture_real_proof_support_impl::DEGRADED_REASON_NOT_DEGRADED,
        screen_capture_real_proof_support_impl::DEGRADED_REASON_PLATFORM_ADAPTER,
        screen_capture_real_proof_support_impl::DEGRADED_REASON_ACCESS_DENIED,
        screen_capture_real_proof_support_impl::DEGRADED_REASON_NO_ACTIVE_WINDOW,
        screen_capture_real_proof_support_impl::DEGRADED_REASON_NOT_SCREEN_CAPTURE,
        screen_capture_real_proof_support_impl::DEGRADED_REASON_ADAPTER_ERROR,
    ];
    ScreenCaptureProofText(REASONS[*status as usize].to_owned())
}

const fn scope_wire_label_index(scope: ScreenCaptureProofScopeLabel) -> usize {
    scope as usize
}
