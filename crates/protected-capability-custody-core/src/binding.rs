use std::fmt;

use sha2::{Digest, Sha256};
use thiserror::Error;

mod codec;
mod cursor;
mod decode;
mod validation;
mod values;

#[cfg(test)]
mod binding_test;

const LOCATOR_MAGIC: [u8; 4] = *b"OCPL";
const BINDING_MAGIC: [u8; 4] = *b"OCPC";
pub(crate) const BINDING_VERSION: u16 = 2;
const MAX_FIELD_BYTES: usize = 1024;
const MAX_LOCATOR_BYTES: usize = 4 * MAX_FIELD_BYTES + 64;
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

#[derive(Clone, Eq, PartialEq)]
pub struct TargetEnvelope {
    kind: TargetKind,
    household: Vec<u8>,
    device: Vec<u8>,
    target: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct BindingLocator {
    operation: OperationId,
    action: Action,
    target: TargetEnvelope,
    canonical: Vec<u8>,
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

    pub fn name(&self) -> GenerationSlotName {
        self.name
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Binding {
    locator: BindingLocator,
    generations: [GenerationSlot; GENERATION_SLOT_COUNT],
    canonical: Vec<u8>,
}

impl Binding {
    pub fn try_new(
        locator: BindingLocator,
        generations: [GenerationSlot; GENERATION_SLOT_COUNT],
    ) -> Result<Self, BindingError> {
        validation::validate_generation_order(&generations)?;
        let canonical = codec::encode_binding(&locator, &generations)?;
        Ok(Self {
            locator,
            generations,
            canonical,
        })
    }

    pub(crate) fn decode(canonical: &[u8]) -> Result<Self, BindingError> {
        decode::decode(canonical)
    }

    pub(crate) fn locator(&self) -> &BindingLocator {
        &self.locator
    }

    pub(crate) fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }

    pub(crate) fn digest(&self) -> [u8; 32] {
        domain_digest(b"ocentra.binding-digest.v2", &self.canonical)
    }
}

impl fmt::Debug for Binding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Binding")
            .field("locator", &self.locator)
            .field("generation_slot_count", &self.generations.len())
            .field("canonical_length", &self.canonical.len())
            .finish()
    }
}

fn domain_digest(domain: &[u8], canonical: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update((domain.len() as u32).to_be_bytes());
    hasher.update(domain);
    hasher.update((crate::RECORD_NAMESPACE.len() as u32).to_be_bytes());
    hasher.update(crate::RECORD_NAMESPACE);
    hasher.update(crate::STORAGE_SCHEMA_VERSION.to_be_bytes());
    hasher.update(BINDING_VERSION.to_be_bytes());
    hasher.update((canonical.len() as u32).to_be_bytes());
    hasher.update(canonical);
    hasher.finalize().into()
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
    Locator,
    Operation,
    Household,
    Device,
    Target,
}
