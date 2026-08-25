mod descriptor;
pub(crate) mod validation;

use crate::types::ProtocolError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Action {
    Seal = 1,
    Rotate = 2,
    Revoke = 3,
    Recover = 4,
}

impl Action {
    pub(crate) fn decode(value: u8) -> Result<Self, ProtocolError> {
        match value {
            1 => Ok(Self::Seal),
            2 => Ok(Self::Rotate),
            3 => Ok(Self::Revoke),
            4 => Ok(Self::Recover),
            other => Err(ProtocolError::UnsupportedAction(other)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TargetKind {
    Device = 1,
    Household = 2,
    Capability = 3,
}

impl TargetKind {
    pub(crate) fn decode(value: u8) -> Result<Self, ProtocolError> {
        match value {
            1 => Ok(Self::Device),
            2 => Ok(Self::Household),
            3 => Ok(Self::Capability),
            other => Err(ProtocolError::UnsupportedTarget(other)),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TargetDescriptor {
    pub(crate) kind: TargetKind,
    pub(crate) household: Vec<u8>,
    pub(crate) device: Vec<u8>,
    pub(crate) target: Vec<u8>,
}
