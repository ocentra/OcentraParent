use super::{
    validation, Action, BindingError, BindingField, BindingLocator, GenerationSlot, OperationId,
    TargetEnvelope, BINDING_MAGIC, BINDING_VERSION, GENERATION_SLOT_COUNT, LOCATOR_MAGIC,
};

pub(super) fn encode_locator(
    operation: &OperationId,
    action: Action,
    target: &TargetEnvelope,
) -> Result<Vec<u8>, BindingError> {
    let mut payload = Vec::new();
    append_frame(&mut payload, &operation.0, BindingField::Operation)?;
    payload.push(action as u8);
    payload.push(target.kind as u8);
    append_frame(&mut payload, &target.household, BindingField::Household)?;
    append_frame(&mut payload, &target.device, BindingField::Device)?;
    append_frame(&mut payload, &target.target, BindingField::Target)?;
    wrap(LOCATOR_MAGIC, payload)
}

pub(super) fn encode_binding(
    locator: &BindingLocator,
    generations: &[GenerationSlot; GENERATION_SLOT_COUNT],
) -> Result<Vec<u8>, BindingError> {
    let mut payload = Vec::new();
    append_frame(
        &mut payload,
        locator.canonical_bytes(),
        BindingField::Locator,
    )?;
    payload.push(GENERATION_SLOT_COUNT as u8);
    for slot in generations {
        payload.push(slot.name as u8);
        payload.extend_from_slice(&slot.value.to_be_bytes());
    }
    wrap(BINDING_MAGIC, payload)
}

fn wrap(magic: [u8; 4], payload: Vec<u8>) -> Result<Vec<u8>, BindingError> {
    let mut output = Vec::with_capacity(magic.len() + 2 + 4 + payload.len());
    output.extend_from_slice(&magic);
    output.extend_from_slice(&BINDING_VERSION.to_be_bytes());
    let payload_length = u32::try_from(payload.len()).map_err(|_| BindingError::FieldTooLarge)?;
    output.extend_from_slice(&payload_length.to_be_bytes());
    output.extend_from_slice(&payload);
    Ok(output)
}

fn append_frame(
    output: &mut Vec<u8>,
    value: &[u8],
    field: BindingField,
) -> Result<(), BindingError> {
    validation::validate_field(value, field)?;
    let length = u32::try_from(value.len()).map_err(|_| BindingError::FieldTooLarge)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}
