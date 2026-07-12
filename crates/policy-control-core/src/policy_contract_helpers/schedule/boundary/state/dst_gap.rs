#![forbid(unsafe_code)]

use super::super::super::{
    PolicyContractScheduleBoundary, PolicyContractScheduleDstResolution,
    PolicyContractScheduleDstTransition, PolicyContractValidationResult,
};

pub(super) fn validate(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    let Some(dst_boundary) = &boundary.dst_boundary else {
        return Err("dst-gap boundaries require dstBoundary details".into());
    };
    if dst_boundary.transition != PolicyContractScheduleDstTransition::SpringForward {
        return Err("dst-gap boundaries must use the spring-forward transition".into());
    }
    if matches!(
        dst_boundary.resolution,
        PolicyContractScheduleDstResolution::FirstOccurrence
            | PolicyContractScheduleDstResolution::SecondOccurrence
    ) {
        return Err("dst-gap boundaries cannot use overlap-only resolutions".into());
    }
    Ok(())
}
