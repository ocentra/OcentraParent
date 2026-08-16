#[path = "claims/detection.rs"]
mod detection;
#[path = "claims/global.rs"]
mod global;

use super::*;

pub(super) fn reject_global_claims(
    input: &NetworkAiAuditReportInput,
) -> Result<(), NetworkAiAuditReportError> {
    global::reject_global_claims(input)
}

pub(super) fn normalized_detection_refs(
    detections: &[NetworkAiDetectionResult],
) -> Result<Vec<String>, NetworkAiAuditReportError> {
    detection::normalized_detection_refs(detections)
}
