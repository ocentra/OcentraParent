use ocentra_protected_capability_custody_windows_ffi::{
    Error, InputFault, RegistryPath, RegistryValueName, ServiceName, WindowsText, MAX_WIDE_CHARS,
};

#[test]
fn windows_text_and_registry_paths_enforce_abi_boundaries() {
    assert_eq!(
        WindowsText::try_from_str("OcentraParent").map(|text| text.as_str().to_owned()),
        Ok(String::from("OcentraParent"))
    );
    assert_eq!(
        WindowsText::try_from_str("contains\0nul"),
        Err(Error::InvalidInput(InputFault::WindowsTextInvalid))
    );
    assert_eq!(
        WindowsText::try_from_str(&"x".repeat(MAX_WIDE_CHARS)),
        Err(Error::BufferTooLarge)
    );

    assert_eq!(
        RegistryPath::try_from_str("Software\\Ocentra\\ProtectedCustody")
            .map(|path| path.as_str().to_owned()),
        Ok(String::from("Software\\Ocentra\\ProtectedCustody"))
    );
    assert_eq!(
        RegistryPath::try_from_str("Software\\..\\Ocentra"),
        Err(Error::InvalidInput(InputFault::RegistryPathTraversal))
    );
    assert_eq!(
        RegistryValueName::try_from_str(""),
        Err(Error::InvalidInput(InputFault::WindowsTextInvalid))
    );
    assert_eq!(
        ServiceName::try_from_str(""),
        Err(Error::InvalidInput(InputFault::WindowsTextInvalid))
    );
}

#[cfg(not(windows))]
#[test]
fn platform_handles_fail_closed_outside_windows() -> Result<(), Error> {
    use ocentra_protected_capability_custody_windows_ffi::{
        OwnedPcpProvider, OwnedProcess, OwnedScManager, OwnedTbsContext,
    };

    assert!(matches!(
        OwnedPcpProvider::open_machine(),
        Err(Error::UnsupportedPlatform)
    ));
    assert!(matches!(
        OwnedProcess::open_for_peer_observation(42),
        Err(Error::UnsupportedPlatform)
    ));
    assert!(matches!(
        OwnedScManager::open(),
        Err(Error::UnsupportedPlatform)
    ));
    assert!(matches!(
        OwnedTbsContext::open(),
        Err(Error::UnsupportedPlatform)
    ));
    Ok(())
}
