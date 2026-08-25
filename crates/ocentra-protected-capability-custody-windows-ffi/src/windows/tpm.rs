//! Windows TBS context and TPM command submission.

use super::handles::TbsContextInner;
use crate::tpm;
use crate::{
    Error, InputFault, OwnedTbsContext, OwnedTpmNvCapability, Result, TpmNvEnrollment, TpmNvPublic,
    MAX_BUFFER_BYTES,
};
use std::ptr;
use windows_sys::Win32::System::TpmBaseServices::{
    Tbsi_Context_Create, Tbsi_Is_Tpm_Present, Tbsip_Submit_Command, TBS_COMMAND_LOCALITY_ZERO,
    TBS_COMMAND_PRIORITY_NORMAL, TBS_CONTEXT_PARAMS, TBS_CONTEXT_PARAMS2, TBS_CONTEXT_PARAMS2_0,
    TBS_SUCCESS, TPM_VERSION_20,
};

impl OwnedTbsContext {
    pub fn open() -> Result<Self> {
        // TBS_CONTEXT_PARAMS2 flags: bit 0=requestRaw, bit 1=include TPM 1.2,
        // bit 2=include TPM 2.0.  Only TPM 2.0 is permitted for these codecs.
        const INCLUDE_TPM20_ONLY: u32 = 0b100;
        let params = TBS_CONTEXT_PARAMS2 {
            version: TPM_VERSION_20,
            Anonymous: TBS_CONTEXT_PARAMS2_0 {
                asUINT32: INCLUDE_TPM20_ONLY,
            },
        };
        let mut context = ptr::null_mut();
        let status = unsafe {
            Tbsi_Context_Create(
                &params as *const TBS_CONTEXT_PARAMS2 as *const TBS_CONTEXT_PARAMS,
                &mut context,
            )
        };
        if status != TBS_SUCCESS || context.is_null() {
            return Err(Error::Tpm(status));
        }
        Ok(Self {
            inner: TbsContextInner { context },
        })
    }

    pub fn is_tpm_present() -> Result<bool> {
        Ok(unsafe { Tbsi_Is_Tpm_Present() != 0 })
    }

    pub(crate) fn submit(&self, command: &[u8]) -> Result<Vec<u8>> {
        if command.len() < 10 || command.len() > MAX_BUFFER_BYTES {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        validate_tpm_command(command)?;
        let mut output = vec![0u8; MAX_BUFFER_BYTES];
        let mut output_length = u32::try_from(output.len())?;
        let status = unsafe {
            Tbsip_Submit_Command(
                self.inner.context,
                TBS_COMMAND_LOCALITY_ZERO,
                TBS_COMMAND_PRIORITY_NORMAL,
                command.as_ptr(),
                u32::try_from(command.len())?,
                output.as_mut_ptr(),
                &mut output_length,
            )
        };
        if status != TBS_SUCCESS {
            return Err(Error::Tpm(status));
        }
        let output_length = usize::try_from(output_length)?;
        if output_length > output.len() {
            return Err(Error::BufferTooLarge);
        }
        output.truncate(output_length);
        Ok(output)
    }

    pub fn bind_enrolled_nv(self, enrollment: TpmNvEnrollment) -> Result<OwnedTpmNvCapability> {
        let public = observe_nv_public(&self, enrollment.nv_index)?;
        enrollment.verify_public(&public)?;
        Ok(OwnedTpmNvCapability {
            context: self,
            enrollment,
        })
    }
}

impl OwnedTpmNvCapability {
    pub fn read(&self, authorization: &[u8], size: u16, offset: u16) -> Result<Vec<u8>> {
        self.enrollment.validate_read_range(size, offset)?;
        self.verify_live_public()?;
        let index = self.enrollment.nv_index;
        let command = tpm::command::encode_nv_read(index, index, authorization, size, offset)?;
        let response = self.context.submit(&command)?;
        let data = tpm::response::decode_nv_read(&response)?;
        self.verify_live_public()?;
        Ok(data)
    }

    pub fn increment(&self, authorization: &[u8]) -> Result<()> {
        self.enrollment.validate_increment_shape()?;
        self.verify_live_public()?;
        let index = self.enrollment.nv_index;
        let command = tpm::command::encode_nv_increment(index, index, authorization)?;
        let response = self.context.submit(&command)?;
        tpm::response::decode_nv_increment(&response)?;
        // A failure here is fail-closed evidence of post-command drift. The
        // TPM may already have applied the increment; this layer never claims
        // rollback of an issued hardware mutation.
        self.verify_live_public()
    }

    fn verify_live_public(&self) -> Result<()> {
        let public = observe_nv_public(&self.context, self.enrollment.nv_index)?;
        self.enrollment.verify_public(&public)
    }
}

fn observe_nv_public(context: &OwnedTbsContext, index: u32) -> Result<TpmNvPublic> {
    let response = context.submit(&tpm::command::encode_nv_read_public(index)?)?;
    tpm::response::decode_nv_read_public(&response, index)
}

fn validate_tpm_command(command: &[u8]) -> Result<()> {
    let declared_size =
        u32::from_be_bytes([command[2], command[3], command[4], command[5]]) as usize;
    let tag = u16::from_be_bytes([command[0], command[1]]);
    if declared_size != command.len() || (tag != 0x8001 && tag != 0x8002) {
        return Err(Error::MalformedTpm);
    }
    Ok(())
}
