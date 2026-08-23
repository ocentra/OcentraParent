use super::cursor::Cursor;
use super::{
    Binding, BindingError, GenerationSlot, GenerationSlotName, OperationId, TargetEnvelope,
    BINDING_MAGIC, BINDING_VERSION, GENERATION_SLOT_COUNT,
};

pub(super) fn decode(canonical: &[u8]) -> Result<Binding, BindingError> {
    let mut cursor = Cursor::new(canonical);
    let magic = cursor.take_exact(BINDING_MAGIC.len())?;
    if magic != BINDING_MAGIC {
        return Err(BindingError::InvalidEncoding);
    }
    if cursor.take_u16()? != BINDING_VERSION {
        return Err(BindingError::UnsupportedVersion);
    }
    let payload_length = cursor.take_u32()? as usize;
    if payload_length != cursor.remaining() {
        return Err(BindingError::InvalidEncoding);
    }
    let operation = OperationId(cursor.take_frame(super::BindingField::Operation)?);
    let action = super::validation::decode_action(cursor.take_u8()?)?;
    let kind = super::validation::decode_target(cursor.take_u8()?)?;
    let household = cursor.take_frame(super::BindingField::Household)?;
    let device = cursor.take_frame(super::BindingField::Device)?;
    let target = cursor.take_frame(super::BindingField::Target)?;
    let target = TargetEnvelope::try_new(kind, household, device, target)?;
    if cursor.take_u8()? as usize != GENERATION_SLOT_COUNT {
        return Err(BindingError::MissingGeneration);
    }
    let mut generations = [
        GenerationSlot::try_new(GenerationSlotName::Authority, 1)?,
        GenerationSlot::try_new(GenerationSlotName::Target, 1)?,
        GenerationSlot::try_new(GenerationSlotName::Key, 1)?,
        GenerationSlot::try_new(GenerationSlotName::Writer, 1)?,
    ];
    for slot in &mut generations {
        let name = super::validation::decode_generation(cursor.take_u8()?)?;
        let value = cursor.take_u64()?;
        *slot = GenerationSlot::try_new(name, value)?;
    }
    if cursor.remaining() != 0 {
        return Err(BindingError::TrailingBytes);
    }
    let binding = Binding::try_new(operation, action, target, generations)?;
    if binding.canonical != canonical {
        return Err(BindingError::NonCanonicalEncoding);
    }
    Ok(binding)
}
