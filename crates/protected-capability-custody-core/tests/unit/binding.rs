use ocentra_protected_capability_custody_core::binding::{
    Action, Binding, BindingError, BindingLocator, GenerationSlot, GenerationSlotName, OperationId,
    TargetEnvelope, TargetKind,
};

#[test]
fn binding_inputs_validate_and_construct_a_complete_locator() -> Result<(), BindingError> {
    assert!(matches!(
        OperationId::try_new(Vec::new()),
        Err(BindingError::EmptyField)
    ));
    assert!(matches!(
        TargetEnvelope::try_new(TargetKind::Device, Vec::new(), vec![5], vec![6]),
        Err(BindingError::EmptyField)
    ));
    assert!(matches!(
        GenerationSlot::try_new(GenerationSlotName::Authority, 0),
        Err(BindingError::ZeroGeneration)
    ));

    let operation = OperationId::try_new(vec![1, 2, 3])?;
    assert_eq!(operation.as_bytes(), &[1, 2, 3]);

    let target = TargetEnvelope::try_new(TargetKind::Device, vec![4], vec![5], vec![6])?;
    assert_eq!(target.kind(), TargetKind::Device);
    assert_eq!(target.household(), &[4]);
    assert_eq!(target.device(), &[5]);
    assert_eq!(target.target(), &[6]);

    let locator = BindingLocator::try_new(operation, Action::Seal, target)?;
    assert_eq!(locator.action(), Action::Seal);
    assert_eq!(locator.operation().as_bytes(), &[1, 2, 3]);
    assert_eq!(locator.target().kind(), TargetKind::Device);

    let generations = [
        GenerationSlot::try_new(GenerationSlotName::Authority, 1)?,
        GenerationSlot::try_new(GenerationSlotName::Target, 2)?,
        GenerationSlot::try_new(GenerationSlotName::Key, 3)?,
        GenerationSlot::try_new(GenerationSlotName::Writer, 4)?,
    ];
    let _binding = Binding::try_new(locator, generations)?;
    Ok(())
}

#[test]
fn binding_rejects_out_of_order_generation_slots() -> Result<(), BindingError> {
    let operation = OperationId::try_new(vec![1])?;
    let target = TargetEnvelope::try_new(TargetKind::Capability, vec![2], vec![3], vec![4])?;
    let locator = BindingLocator::try_new(operation, Action::Rotate, target)?;
    let generations = [
        GenerationSlot::try_new(GenerationSlotName::Target, 1)?,
        GenerationSlot::try_new(GenerationSlotName::Authority, 2)?,
        GenerationSlot::try_new(GenerationSlotName::Key, 3)?,
        GenerationSlot::try_new(GenerationSlotName::Writer, 4)?,
    ];

    assert!(matches!(
        Binding::try_new(locator, generations),
        Err(BindingError::DuplicateGeneration)
    ));
    Ok(())
}
