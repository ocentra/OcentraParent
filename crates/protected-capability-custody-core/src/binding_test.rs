use super::{
    Action, Binding, BindingError, BindingLocator, GenerationSlot, GenerationSlotName, OperationId,
    TargetEnvelope, TargetKind, MAX_FIELD_BYTES,
};

fn binding_fixture() -> Result<Binding, BindingError> {
    let operation = OperationId::try_new(vec![1, 2, 3])?;
    let target = TargetEnvelope::try_new(TargetKind::Capability, vec![4], vec![5], vec![6])?;
    let locator = BindingLocator::try_new(operation, Action::Seal, target)?;
    Binding::try_new(
        locator,
        [
            GenerationSlot::try_new(GenerationSlotName::Authority, 7)?,
            GenerationSlot::try_new(GenerationSlotName::Target, 8)?,
            GenerationSlot::try_new(GenerationSlotName::Key, 9)?,
            GenerationSlot::try_new(GenerationSlotName::Writer, 10)?,
        ],
    )
}

#[test]
fn canonical_binding_round_trips_and_preserves_domain_digest() -> Result<(), BindingError> {
    let binding = binding_fixture()?;
    let decoded = Binding::decode(binding.canonical_bytes())?;

    assert_eq!(decoded.canonical_bytes(), binding.canonical_bytes());
    assert_eq!(decoded.digest(), binding.digest());
    assert_eq!(decoded.locator().target().household(), &[4]);
    assert_eq!(decoded.locator().target().device(), &[5]);
    assert_eq!(decoded.locator().target().target(), &[6]);

    let other_target = TargetEnvelope::try_new(TargetKind::Capability, vec![8], vec![5], vec![6])?;
    let other_locator = BindingLocator::try_new(
        OperationId::try_new(vec![1, 2, 3])?,
        Action::Seal,
        other_target,
    )?;
    assert_ne!(
        binding.locator().lookup_digest(),
        other_locator.lookup_digest()
    );
    Ok(())
}

#[test]
fn binding_rejects_empty_oversized_and_zero_generation_inputs() {
    assert!(matches!(
        OperationId::try_new(Vec::new()),
        Err(BindingError::EmptyField)
    ));
    assert!(matches!(
        OperationId::try_new(vec![0; MAX_FIELD_BYTES + 1]),
        Err(BindingError::FieldTooLarge)
    ));
    assert!(matches!(
        TargetEnvelope::try_new(TargetKind::Device, vec![1], vec![2], Vec::new()),
        Err(BindingError::EmptyField)
    ));
    assert!(matches!(
        GenerationSlot::try_new(GenerationSlotName::Authority, 0),
        Err(BindingError::ZeroGeneration)
    ));
}

#[test]
fn binding_decoder_rejects_version_action_target_and_generation_drift() -> Result<(), BindingError>
{
    let binding = binding_fixture()?;
    let mut unsupported_version = binding.canonical_bytes().to_vec();
    unsupported_version[4..6].copy_from_slice(&u16::MAX.to_be_bytes());
    assert!(matches!(
        Binding::decode(&unsupported_version),
        Err(BindingError::UnsupportedVersion)
    ));

    let locator_start = 10 + 4;
    let operation_action_offset = locator_start + 10 + 4 + 3;
    let mut unsupported_action = binding.canonical_bytes().to_vec();
    unsupported_action[operation_action_offset] = u8::MAX;
    assert!(matches!(
        Binding::decode(&unsupported_action),
        Err(BindingError::UnsupportedAction)
    ));

    let mut unsupported_target = binding.canonical_bytes().to_vec();
    unsupported_target[operation_action_offset + 1] = u8::MAX;
    assert!(matches!(
        Binding::decode(&unsupported_target),
        Err(BindingError::UnsupportedTarget)
    ));

    let generation_start = 10 + 4 + binding.locator().canonical_bytes().len();
    let mut duplicate_generation = binding.canonical_bytes().to_vec();
    duplicate_generation[generation_start + 1] = GenerationSlotName::Target as u8;
    assert!(matches!(
        Binding::decode(&duplicate_generation),
        Err(BindingError::DuplicateGeneration)
    ));

    let mut zero_generation = binding.canonical_bytes().to_vec();
    zero_generation[generation_start + 2..generation_start + 10].fill(0);
    assert!(matches!(
        Binding::decode(&zero_generation),
        Err(BindingError::ZeroGeneration)
    ));
    Ok(())
}

#[test]
fn binding_decoder_rejects_truncated_and_trailing_payloads() -> Result<(), BindingError> {
    let binding = binding_fixture()?;
    let canonical = binding.canonical_bytes();

    assert!(matches!(
        Binding::decode(&canonical[..canonical.len() - 1]),
        Err(BindingError::InvalidEncoding)
    ));

    let mut trailing = canonical.to_vec();
    let declared = u32::from_be_bytes(
        trailing[6..10]
            .try_into()
            .map_err(|_| BindingError::InvalidEncoding)?,
    );
    trailing[6..10].copy_from_slice(&declared.saturating_add(1).to_be_bytes());
    trailing.push(0);
    assert!(matches!(
        Binding::decode(&trailing),
        Err(BindingError::TrailingBytes)
    ));
    Ok(())
}
