use crate::binding::{Action, BindingLocator, OperationId, TargetEnvelope, TargetKind};
use ocentra_protected_capability_custody_protocol::request::{
    ExpectedGenerations, UntrustedRequest,
};
use ocentra_protected_capability_custody_protocol::response::ObservedGenerations;

use super::{error_status, BrokerRuntimeError};

pub(super) fn validated_locator(
    request: &UntrustedRequest,
) -> Result<BindingLocator, BrokerRuntimeError> {
    let operation = OperationId::try_new(request.operation().to_vec())?;
    let target = TargetEnvelope::try_new(
        target_kind(request.target().kind()),
        request.target().household().to_vec(),
        request.target().device().to_vec(),
        request.target().target().to_vec(),
    )?;
    BindingLocator::try_new(operation, action(request.action()), target)
        .map_err(BrokerRuntimeError::from)
}

pub(super) fn observed(
    generations: ExpectedGenerations,
) -> Result<ObservedGenerations, BrokerRuntimeError> {
    ObservedGenerations::try_new(
        generations.authority(),
        generations.target(),
        generations.key(),
        generations.writer(),
    )
    .map_err(error_status::protocol)
}

fn action(value: ocentra_protected_capability_custody_protocol::target::Action) -> Action {
    match value {
        ocentra_protected_capability_custody_protocol::target::Action::Seal => Action::Seal,
        ocentra_protected_capability_custody_protocol::target::Action::Rotate => Action::Rotate,
        ocentra_protected_capability_custody_protocol::target::Action::Revoke => Action::Revoke,
        ocentra_protected_capability_custody_protocol::target::Action::Recover => Action::Recover,
    }
}

fn target_kind(
    value: ocentra_protected_capability_custody_protocol::target::TargetKind,
) -> TargetKind {
    match value {
        ocentra_protected_capability_custody_protocol::target::TargetKind::Device => {
            TargetKind::Device
        }
        ocentra_protected_capability_custody_protocol::target::TargetKind::Household => {
            TargetKind::Household
        }
        ocentra_protected_capability_custody_protocol::target::TargetKind::Capability => {
            TargetKind::Capability
        }
    }
}
