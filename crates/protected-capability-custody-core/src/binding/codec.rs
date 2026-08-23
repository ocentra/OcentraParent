use super::{
    validation, Action, BindingError, BindingField, GenerationSlot, OperationId, TargetEnvelope,
    BINDING_MAGIC, BINDING_VERSION, GENERATION_SLOT_COUNT,
};

pub(super) fn encode(
    operation: &OperationId,
    action: Action,
    target: &TargetEnvelope,
    generations: &[GenerationSlot; GENERATION_SLOT_COUNT],
) -> Result<Vec<u8>, BindingError> {
    let mut payload = Vec::new();
    append_frame(&mut payload, &operation.0)?;
    payload.push(action as u8);
    payload.push(target.kind as u8);
    append_frame(&mut payload, &target.household)?;
    append_frame(&mut payload, &target.device)?;
    append_frame(&mut payload, &target.target)?;
    payload.push(GENERATION_SLOT_COUNT as u8);
    for slot in generations {
        payload.push(slot.name as u8);
        payload.extend_from_slice(&slot.value.to_be_bytes());
    }

    let mut output = Vec::with_capacity(BINDING_MAGIC.len() + 2 + 4 + payload.len());
    output.extend_from_slice(&BINDING_MAGIC);
    output.extend_from_slice(&BINDING_VERSION.to_be_bytes());
    let payload_length = u32::try_from(payload.len()).map_err(|_| BindingError::FieldTooLarge)?;
    output.extend_from_slice(&payload_length.to_be_bytes());
    output.extend_from_slice(&payload);
    Ok(output)
}

fn append_frame(output: &mut Vec<u8>, value: &[u8]) -> Result<(), BindingError> {
    validation::validate_field(value, BindingField::Operation)?;
    let length = u32::try_from(value.len()).map_err(|_| BindingError::FieldTooLarge)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}
