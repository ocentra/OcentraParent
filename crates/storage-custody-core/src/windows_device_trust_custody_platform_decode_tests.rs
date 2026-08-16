use super::{decode_hex, Error};

#[test]
fn corrupt_multibyte_registry_epoch_is_rejected_without_panic() {
    assert_eq!(decode_hex("00Ã©"), Err(Error::Missing));
}
