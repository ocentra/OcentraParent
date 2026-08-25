//! Windows security-descriptor copying and normalization.

use crate::{Error, Result, SecurityDescriptorObservation, MAX_BUFFER_BYTES};
use windows_sys::core::BOOL;
use windows_sys::Win32::Security::{
    GetSecurityDescriptorControl, GetSecurityDescriptorDacl, GetSecurityDescriptorLength,
    GetSecurityDescriptorOwner, IsValidSecurityDescriptor, ACL, PSECURITY_DESCRIPTOR, PSID,
    SECURITY_DESCRIPTOR_CONTROL, SE_DACL_PROTECTED,
};

#[path = "security_ace.rs"]
mod ace;
#[path = "security_ace_shape.rs"]
mod ace_shape;
#[path = "security_acl.rs"]
mod acl;
#[path = "security_sid.rs"]
mod sid;

pub(crate) fn copy_descriptor(descriptor: Vec<u8>) -> Result<SecurityDescriptorObservation> {
    if descriptor.is_empty() || descriptor.len() > MAX_BUFFER_BYTES {
        return Err(Error::InvalidInput(
            "security descriptor is empty or too large",
        ));
    }
    if descriptor.len() < 20 {
        return Err(Error::InvalidInput(
            "security descriptor is shorter than its fixed header",
        ));
    }
    let descriptor_ptr = descriptor.as_ptr() as PSECURITY_DESCRIPTOR;
    if unsafe { IsValidSecurityDescriptor(descriptor_ptr) } == 0 {
        return Err(Error::InvalidInput(
            "Windows returned an invalid security descriptor",
        ));
    }
    let descriptor_length = unsafe { GetSecurityDescriptorLength(descriptor_ptr) } as usize;
    if descriptor_length == 0 || descriptor_length > descriptor.len() {
        return Err(Error::InvalidInput(
            "security descriptor length exceeds its returned buffer",
        ));
    }
    let mut owner: PSID = core::ptr::null_mut();
    let mut owner_defaulted: BOOL = 0;
    let owner_ok =
        unsafe { GetSecurityDescriptorOwner(descriptor_ptr, &mut owner, &mut owner_defaulted) };
    if owner_ok == 0 || owner.is_null() {
        return Err(Error::Win32(last_error()));
    }
    let owner_sid = sid::copy_sid_in_buffer(owner, descriptor.as_ptr(), descriptor_length)?;

    let mut dacl_present: BOOL = 0;
    let mut dacl: *mut ACL = core::ptr::null_mut();
    let mut dacl_defaulted: BOOL = 0;
    let dacl_ok = unsafe {
        GetSecurityDescriptorDacl(
            descriptor_ptr,
            &mut dacl_present,
            &mut dacl,
            &mut dacl_defaulted,
        )
    };
    if dacl_ok == 0 {
        return Err(Error::Win32(last_error()));
    }
    let dacl = if dacl_present != 0 {
        if dacl.is_null() {
            return Err(Error::InvalidInput(
                "security descriptor has a null present DACL",
            ));
        }
        acl::copy_aces(dacl, descriptor.as_ptr(), descriptor_length)?
    } else {
        Vec::new()
    };
    let mut control: SECURITY_DESCRIPTOR_CONTROL = 0;
    let mut revision = 0;
    let control_ok =
        unsafe { GetSecurityDescriptorControl(descriptor_ptr, &mut control, &mut revision) };
    if control_ok == 0 || revision == 0 {
        return Err(Error::Win32(last_error()));
    }
    Ok(SecurityDescriptorObservation {
        descriptor,
        owner_sid,
        owner_defaulted: owner_defaulted != 0,
        dacl_present: dacl_present != 0,
        dacl_defaulted: dacl_defaulted != 0,
        dacl,
        dacl_protected: control & SE_DACL_PROTECTED != 0,
    })
}

fn last_error() -> u32 {
    unsafe { windows_sys::Win32::Foundation::GetLastError() }
}
