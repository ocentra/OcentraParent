use ocentra_entitlement_core::entitlement_snapshot::{
    checked_effective_child_device_limit, EntitlementSnapshotDerivationError,
};

#[test]
fn derives_starter_plus_paid_and_referral_seats() {
    assert_eq!(checked_effective_child_device_limit(1, 2, 3), Ok(6));
}

#[test]
fn rejects_zero_base_limit() {
    assert_eq!(
        checked_effective_child_device_limit(0, 1, 1),
        Err(EntitlementSnapshotDerivationError::ZeroBaseChildDeviceLimit)
    );
}

#[test]
fn rejects_arithmetic_overflow() {
    assert_eq!(
        checked_effective_child_device_limit(u32::MAX, 1, 0),
        Err(EntitlementSnapshotDerivationError::SeatLimitOverflow)
    );
}
