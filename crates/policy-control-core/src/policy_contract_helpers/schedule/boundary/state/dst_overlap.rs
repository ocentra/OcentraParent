#![forbid(unsafe_code)]

use super::super::super::{
    PolicyContractScheduleBoundary, PolicyContractScheduleDstResolution,
    PolicyContractScheduleDstTransition, PolicyContractValidationResult,
};

pub(super) fn validate(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    let Some(dst_boundary) = &boundary.dst_boundary else {
        return Err("dst-overlap boundaries require dstBoundary details".into());
    };
    if dst_boundary.transition != PolicyContractScheduleDstTransition::FallBack {
        return Err("dst-overlap boundaries must use the fall-back transition".into());
    }
    if dst_boundary.resolution == PolicyContractScheduleDstResolution::SkipForward {
        return Err("dst-overlap boundaries cannot skip the repeated hour".into());
    }
    Ok(())
}
