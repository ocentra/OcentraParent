use super::cursor::Cursor;
use super::{
    Binding, BindingError, BindingLocator, GenerationSlot, GenerationSlotName, OperationId,
    TargetEnvelope, BINDING_MAGIC, BINDING_VERSION, GENERATION_SLOT_COUNT, LOCATOR_MAGIC,
};

pub(super) fn decode(canonical: &[u8]) -> Result<Binding, BindingError> {
    let mut cursor = payload_cursor(canonical, BINDING_MAGIC)?;
    let locator_bytes = cursor.take_frame(super::BindingField::Locator)?;
    let locator = decode_locator(&locator_bytes)?;
    if cursor.take_u8()? as usize != GENERATION_SLOT_COUNT {
        return Err(BindingError::MissingGeneration);
    }
    let mut generations = initial_generations()?;
    for slot in &mut generations {
        let name = super::validation::decode_generation(cursor.take_u8()?)?;
        *slot = GenerationSlot::try_new(name, cursor.take_u64()?)?;
    }
    if cursor.remaining() != 0 {
        return Err(BindingError::TrailingBytes);
    }
    let binding = Binding::try_new(locator, generations)?;
    if binding.canonical != canonical {
        return Err(BindingError::NonCanonicalEncoding);
    }
    Ok(binding)
}

fn decode_locator(canonical: &[u8]) -> Result<BindingLocator, BindingError> {
    let mut cursor = payload_cursor(canonical, LOCATOR_MAGIC)?;
    let operation = OperationId::try_new(cursor.take_frame(super::BindingField::Operation)?)?;
    let action = super::validation::decode_action(cursor.take_u8()?)?;
    let kind = super::validation::decode_target(cursor.take_u8()?)?;
    let household = cursor.take_frame(super::BindingField::Household)?;
    let device = cursor.take_frame(super::BindingField::Device)?;
    let target = cursor.take_frame(super::BindingField::Target)?;
    if cursor.remaining() != 0 {
        return Err(BindingError::TrailingBytes);
    }
    let envelope = TargetEnvelope::try_new(kind, household, device, target)?;
    let locator = BindingLocator::try_new(operation, action, envelope)?;
    if locator.canonical != canonical {
        return Err(BindingError::NonCanonicalEncoding);
    }
    Ok(locator)
}

fn payload_cursor<'a>(canonical: &'a [u8], magic: [u8; 4]) -> Result<Cursor<'a>, BindingError> {
    let mut cursor = Cursor::new(canonical);
    if cursor.take_exact(magic.len())? != magic {
        return Err(BindingError::InvalidEncoding);
    }
    if cursor.take_u16()? != BINDING_VERSION {
        return Err(BindingError::UnsupportedVersion);
    }
    if cursor.take_u32()? as usize != cursor.remaining() {
        return Err(BindingError::InvalidEncoding);
    }
    Ok(cursor)
}

fn initial_generations() -> Result<[GenerationSlot; GENERATION_SLOT_COUNT], BindingError> {
    Ok([
        GenerationSlot::try_new(GenerationSlotName::Authority, 1)?,
        GenerationSlot::try_new(GenerationSlotName::Target, 1)?,
        GenerationSlot::try_new(GenerationSlotName::Key, 1)?,
        GenerationSlot::try_new(GenerationSlotName::Writer, 1)?,
    ])
}
