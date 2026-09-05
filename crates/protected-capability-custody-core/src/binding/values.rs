use std::fmt;

use super::{
    codec, domain_digest, validation, Action, BindingError, BindingField, BindingLocator,
    OperationId, TargetEnvelope, TargetKind,
};

impl OperationId {
    pub fn try_new(value: Vec<u8>) -> Result<Self, BindingError> {
        validation::validate_field(&value, BindingField::Operation).map(|_| Self(value))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for OperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperationId")
            .field("length", &self.0.len())
            .finish()
    }
}

impl TargetEnvelope {
    pub fn try_new(
        kind: TargetKind,
        household: Vec<u8>,
        device: Vec<u8>,
        target: Vec<u8>,
    ) -> Result<Self, BindingError> {
        validation::validate_field(&household, BindingField::Household)?;
        validation::validate_field(&device, BindingField::Device)?;
        validation::validate_field(&target, BindingField::Target)?;
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

impl fmt::Debug for TargetEnvelope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TargetEnvelope")
            .field("kind", &self.kind)
            .field("household_length", &self.household.len())
            .field("device_length", &self.device.len())
            .field("target_length", &self.target.len())
            .finish()
    }
}

impl BindingLocator {
    pub fn try_new(
        operation: OperationId,
        action: Action,
        target: TargetEnvelope,
    ) -> Result<Self, BindingError> {
        let canonical = codec::encode_locator(&operation, action, &target)?;
        Ok(Self {
            operation,
            action,
            target,
            canonical,
        })
    }

    pub fn operation(&self) -> &OperationId {
        &self.operation
    }

    pub fn action(&self) -> Action {
        self.action
    }

    pub fn target(&self) -> &TargetEnvelope {
        &self.target
    }

    pub(crate) fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }

    pub(crate) fn lookup_digest(&self) -> [u8; 32] {
        domain_digest(b"ocentra.lookup-digest.v2", &self.canonical)
    }
}

impl fmt::Debug for BindingLocator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BindingLocator")
            .field("operation", &self.operation)
            .field("action", &self.action)
            .field("target", &self.target)
            .finish()
    }
}
