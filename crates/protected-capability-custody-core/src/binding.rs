use std::fmt;

use sha2::{Digest, Sha256};
use thiserror::Error;

mod codec;
mod cursor;
mod decode;
mod validation;

const BINDING_MAGIC: [u8; 4] = *b"OCPC";
const BINDING_VERSION: u16 = 1;
const MAX_FIELD_BYTES: usize = 1024;
const GENERATION_SLOT_COUNT: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Action {
    Seal = 1,
    Rotate = 2,
    Revoke = 3,
    Recover = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TargetKind {
    Device = 1,
    Household = 2,
    Capability = 3,
}

#[derive(Clone, Eq, PartialEq)]
pub struct OperationId(Vec<u8>);

impl OperationId {
    pub fn try_new(value: Vec<u8>) -> Result<Self, BindingError> {
        validation::validate_field(&value, BindingField::Operation).map(|_| Self(value))
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

#[derive(Clone, Eq, PartialEq)]
pub struct TargetEnvelope {
    kind: TargetKind,
    household: Vec<u8>,
    device: Vec<u8>,
    target: Vec<u8>,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum GenerationSlotName {
    Authority = 1,
    Target = 2,
    Key = 3,
    Writer = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GenerationSlot {
    name: GenerationSlotName,
    value: u64,
}

impl GenerationSlot {
    pub fn try_new(name: GenerationSlotName, value: u64) -> Result<Self, BindingError> {
        if value == 0 {
            return Err(BindingError::ZeroGeneration);
        }
        Ok(Self { name, value })
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Binding {
    operation: OperationId,
    action: Action,
    target: TargetEnvelope,
    generations: [GenerationSlot; GENERATION_SLOT_COUNT],
    canonical: Vec<u8>,
}

impl Binding {
    pub fn try_new(
        operation: OperationId,
        action: Action,
        target: TargetEnvelope,
        generations: [GenerationSlot; GENERATION_SLOT_COUNT],
    ) -> Result<Self, BindingError> {
        validation::validate_generation_order(&generations)?;
        let canonical = codec::encode(&operation, action, &target, &generations)?;
        Ok(Self {
            operation,
            action,
            target,
            generations,
            canonical,
        })
    }

    pub(crate) fn decode(canonical: &[u8]) -> Result<Self, BindingError> {
        decode::decode(canonical)
    }

    pub(crate) fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }

    pub(crate) fn digest(&self) -> [u8; 32] {
        let digest = Sha256::digest(&self.canonical);
        let mut output = [0_u8; 32];
        output.copy_from_slice(&digest);
        output
    }
}

impl fmt::Debug for Binding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Binding")
            .field("operation", &self.operation)
            .field("action", &self.action)
            .field("target", &self.target)
            .field("generation_slot_count", &self.generations.len())
            .field("canonical_length", &self.canonical.len())
            .finish()
    }
}

#[derive(Debug, Error)]
pub enum BindingError {
    #[error("binding field is empty")]
    EmptyField,
    #[error("binding field is too large")]
    FieldTooLarge,
    #[error("binding field is not canonical")]
    NonCanonicalEncoding,
    #[error("binding encoding is invalid")]
    InvalidEncoding,
    #[error("binding version is unsupported")]
    UnsupportedVersion,
    #[error("binding encoding has trailing bytes")]
    TrailingBytes,
    #[error("binding action is unsupported")]
    UnsupportedAction,
    #[error("binding target is unsupported")]
    UnsupportedTarget,
    #[error("generation slot is unknown")]
    UnknownGenerationSlot,
    #[error("generation slot is duplicated or out of order")]
    DuplicateGeneration,
    #[error("generation slot is missing")]
    MissingGeneration,
    #[error("generation value is zero")]
    ZeroGeneration,
}

#[derive(Clone, Copy)]
enum BindingField {
    Operation,
    Household,
    Device,
    Target,
}
