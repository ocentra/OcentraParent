#[cfg(not(windows))]
#[test]
fn account_issuer_p256_custody_requires_the_windows_cng_owner(
) -> Result<(), ocentra_protected_capability_custody_windows_ffi::Error> {
    use ocentra_protected_capability_custody_windows_ffi::{
        Error, OwnedPcpProvider, OwnedTbsContext,
    };

    assert!(matches!(
        OwnedPcpProvider::open_machine(),
        Err(Error::UnsupportedPlatform)
    ));
    assert!(matches!(
        OwnedTbsContext::open(),
        Err(Error::UnsupportedPlatform)
    ));
    Ok(())
}
