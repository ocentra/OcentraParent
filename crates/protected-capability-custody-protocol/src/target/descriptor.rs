use std::fmt;

use crate::constants;
use crate::types::ProtocolError;

use super::{validation, TargetDescriptor, TargetKind};

impl TargetDescriptor {
    pub fn try_new(
        kind: TargetKind,
        household: Vec<u8>,
        device: Vec<u8>,
        target: Vec<u8>,
    ) -> Result<Self, ProtocolError> {
        validation::validate_field(&household)?;
        validation::validate_field(&device)?;
        validation::validate_field(&target)?;
        Ok(Self {
            kind,
            household,
            device,
            target,
        })
    }

    pub fn kind(&self) -> TargetKind {
        self.kind
    }

    pub fn household(&self) -> &[u8] {
        &self.household
    }

    pub fn device(&self) -> &[u8] {
        &self.device
    }

    pub fn target(&self) -> &[u8] {
        &self.target
    }
}

impl fmt::Debug for TargetDescriptor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(constants::DEBUG_TARGET_DESCRIPTOR)
            .field(constants::DEBUG_FIELD_KIND, &self.kind)
            .field(
                constants::DEBUG_FIELD_HOUSEHOLD_LENGTH,
                &self.household.len(),
            )
            .field(constants::DEBUG_FIELD_DEVICE_LENGTH, &self.device.len())
            .field(constants::DEBUG_FIELD_TARGET_LENGTH, &self.target.len())
            .finish()
    }
}
