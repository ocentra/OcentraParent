#[test]
fn time_helpers_are_linked() {
    let _ = crate::time::timestamp_from_epoch_seconds;
    let _ = crate::time::timestamp_after_epoch_seconds;
}
