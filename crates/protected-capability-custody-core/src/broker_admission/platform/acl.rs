#[cfg(windows)]
use std::fs::File;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
use windows_acl::acl::{ACLEntry, AceType, ACL};
#[cfg(windows)]
use windows_sys::Win32::Foundation::{GENERIC_ALL, GENERIC_WRITE};
#[cfg(windows)]
use windows_sys::Win32::Storage::FileSystem::{
    DELETE, FILE_APPEND_DATA, FILE_DELETE_CHILD, FILE_WRITE_ATTRIBUTES, FILE_WRITE_DATA,
    FILE_WRITE_EA, WRITE_DAC, WRITE_OWNER,
};
#[cfg(windows)]
use winreg::enums::{KEY_READ, KEY_WRITE};

#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
const SYSTEM_SID: &str = "S-1-5-18";
#[cfg(windows)]
const ADMINISTRATORS_SID: &str = "S-1-5-32-544";
#[cfg(windows)]
const CREATOR_OWNER_SID: &str = "S-1-3-0";
#[cfg(windows)]
const TRUSTED_INSTALLER_SID: &str = concat!(
    "S-1-5-80-956008885-3418522649-",
    "1831038044-1853292631-2271478464"
);
#[cfg(windows)]
const INHERIT_ONLY_ACE: u8 = 0x08;

#[cfg(windows)]
pub(super) fn registry_custody_adapter_available() -> Result<(), PlatformError> {
    // The current dependency set cannot prove registry owner, protected-DACL,
    // deny ordering, and pinned parent-chain custody. This is a static
    // capability check: it deliberately does not open a key or create state.
    Err(PlatformError::DeploymentRequired)
}

#[cfg(windows)]
pub(super) fn validate_path(path: &Path) -> Result<(), PlatformError> {
    let text = path.to_str().ok_or(PlatformError::InvalidAttestation)?;
    let acl = ACL::from_file_path(text, false).map_err(map_acl_error)?;
    validate_entries(acl.all().map_err(map_acl_error)?)
}

#[cfg(windows)]
pub(super) fn validate_file(file: &File) -> Result<(), PlatformError> {
    let acl = ACL::from_file_handle(file.as_raw_handle(), false).map_err(map_acl_error)?;
    validate_entries(acl.all().map_err(map_acl_error)?)
}

#[cfg(windows)]
pub(super) fn validate_secret_store(key: &winreg::RegKey) -> Result<(), PlatformError> {
    validate_registry_custody(key, runtime_registry_requirements())
}

#[cfg(windows)]
pub(super) fn validate_enrollment_store(key: &winreg::RegKey) -> Result<(), PlatformError> {
    validate_registry_custody(key, enrollment_registry_requirements())
}

#[cfg(windows)]
#[derive(Clone, Copy)]
struct RegistryAclRequirements {
    owner_sid: &'static str,
    system_mask: u32,
    trusted_installer_mask: Option<u32>,
    dacl_protected: bool,
    reject_inherited_aces: bool,
    validate_parent_chain: bool,
}

#[cfg(windows)]
fn runtime_registry_requirements() -> RegistryAclRequirements {
    RegistryAclRequirements {
        owner_sid: SYSTEM_SID,
        system_mask: KEY_READ | KEY_WRITE,
        trusted_installer_mask: None,
        dacl_protected: true,
        reject_inherited_aces: true,
        validate_parent_chain: true,
    }
}

#[cfg(windows)]
fn enrollment_registry_requirements() -> RegistryAclRequirements {
    RegistryAclRequirements {
        owner_sid: TRUSTED_INSTALLER_SID,
        system_mask: KEY_READ,
        trusted_installer_mask: Some(KEY_READ | KEY_WRITE),
        dacl_protected: true,
        reject_inherited_aces: true,
        validate_parent_chain: true,
    }
}

#[cfg(windows)]
fn validate_registry_custody(
    key: &winreg::RegKey,
    requirements: RegistryAclRequirements,
) -> Result<(), PlatformError> {
    // `windows-acl` exposes flattened ACEs but not a safe, pinned registry
    // security-descriptor observation that proves owner, SE_DACL_PROTECTED,
    // canonical deny/allow ordering, or the custody of every parent key. Those
    // properties are required together; accepting a partial ACE list would
    // turn inheritance or an attacker-owned parent into registry authority.
    //
    // A future adapter must validate the exact registry KEY_* masks above,
    // reject inherited/unknown/extra allows and ambiguous denies, prove the
    // owner and protected DACL, and retain pinned handles for HKLM, the Ocentra
    // root, registry-id parent, and selected Runtime/Enrollment child.
    let _ = (
        key.raw_handle(),
        requirements.owner_sid,
        requirements.system_mask,
        requirements.trusted_installer_mask,
        requirements.dacl_protected,
        requirements.reject_inherited_aces,
        requirements.validate_parent_chain,
    );
    Err(PlatformError::DeploymentRequired)
}

#[cfg(windows)]
fn validate_entries(entries: Vec<ACLEntry>) -> Result<(), PlatformError> {
    if entries.is_empty() {
        return Err(PlatformError::Tampered);
    }
    let current_user = current_user_sid()?;
    for entry in entries {
        if entry.entry_type == AceType::Unknown {
            return Err(PlatformError::Tampered);
        }
        if is_allow(&entry) && grants_write(entry.mask) && !trusted_writer(&entry, &current_user) {
            return Err(PlatformError::Tampered);
        }
    }
    Ok(())
}

#[cfg(windows)]
fn current_user_sid() -> Result<String, PlatformError> {
    let user = windows_acl::helper::current_user().ok_or(PlatformError::Unavailable)?;
    let mut sid = windows_acl::helper::name_to_sid(&user, None).map_err(map_acl_error)?;
    windows_acl::helper::sid_to_string(sid.as_mut_ptr().cast()).map_err(map_acl_error)
}

#[cfg(windows)]
fn is_allow(entry: &ACLEntry) -> bool {
    matches!(
        entry.entry_type,
        AceType::AccessAllow
            | AceType::AccessAllowCallback
            | AceType::AccessAllowObject
            | AceType::AccessAllowCallbackObject
    )
}

#[cfg(windows)]
fn grants_write(mask: u32) -> bool {
    const MAXIMUM_ALLOWED_MASK: u32 = 0x0200_0000;
    const WRITE_MASK: u32 = FILE_WRITE_DATA
        | FILE_APPEND_DATA
        | FILE_WRITE_EA
        | FILE_DELETE_CHILD
        | FILE_WRITE_ATTRIBUTES
        | DELETE
        | WRITE_DAC
        | WRITE_OWNER
        | GENERIC_WRITE
        | GENERIC_ALL
        | MAXIMUM_ALLOWED_MASK;
    mask & WRITE_MASK != 0
}

#[cfg(windows)]
fn trusted_writer(entry: &ACLEntry, current_user: &str) -> bool {
    entry.string_sid == current_user
        || entry.string_sid == SYSTEM_SID
        || entry.string_sid == ADMINISTRATORS_SID
        || entry.string_sid == TRUSTED_INSTALLER_SID
        || (entry.string_sid == CREATOR_OWNER_SID && entry.flags & INHERIT_ONLY_ACE != 0)
}

#[cfg(windows)]
fn map_acl_error(_error: u32) -> PlatformError {
    PlatformError::Unavailable
}
