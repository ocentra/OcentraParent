use ocentra_protected_capability_custody_windows_ffi::{
    OwnedScManager, OwnedService, ServiceName, ServiceObservation, WindowsText,
};

use crate::platform::PlatformError;

use super::enrollment::VerifiedEnrollment;
use super::enrollment_security::append_security;
use super::{map_ffi_error, ObservationDigest};

const SERVICE_OBSERVATION_DOMAIN: &[u8] = b"ocentra.pcc.scm-observation.v1";
const SERVICE_WIN32_OWN_PROCESS: u32 = 0x0000_0010;
const SERVICE_AUTO_START: u32 = 2;
const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1;
const LOCAL_SYSTEM_ACCOUNT: &str = "LocalSystem";

pub(super) struct VerifiedBrokerService {
    _manager: OwnedScManager,
    service: OwnedService,
    initial: ServiceObservation,
}

impl VerifiedBrokerService {
    pub(super) fn open(enrollment: &VerifiedEnrollment) -> Result<Self, PlatformError> {
        let manager = OwnedScManager::open().map_err(map_ffi_error)?;
        let service_name = ServiceName::try_from_str(
            ocentra_protected_capability_custody_protocol::constants::BROKER_SERVICE_NAME,
        )
        .map_err(map_ffi_error)?;
        let service = manager.open_service(&service_name).map_err(map_ffi_error)?;
        let initial = service.observation().map_err(map_ffi_error)?;
        let owner = Self {
            _manager: manager,
            service,
            initial,
        };
        owner.revalidate(enrollment)?;
        Ok(owner)
    }

    pub(super) fn revalidate(&self, enrollment: &VerifiedEnrollment) -> Result<(), PlatformError> {
        let current = self.service.reobserve().map_err(map_ffi_error)?;
        let binary_path = expected_binary_path();
        if current != self.initial
            || current.service_name().as_str()
                != ocentra_protected_capability_custody_protocol::constants::BROKER_SERVICE_NAME
            || current.service_type() != SERVICE_WIN32_OWN_PROCESS
            || current.start_type() != SERVICE_AUTO_START
            || current.binary_path().map(WindowsText::as_str) != Some(binary_path.as_str())
            || current.start_name().map(WindowsText::as_str) != Some(LOCAL_SYSTEM_ACCOUNT)
            || current.service_sid_type() != SERVICE_SID_TYPE_UNRESTRICTED
            || current.security().owner_was_defaulted()
            || !current.security().dacl_is_present()
            || current.security().dacl_was_defaulted()
            || !current.security().dacl_is_protected()
            || service_digest(&current) != *enrollment.service_digest()
        {
            return Err(PlatformError::Tampered);
        }
        Ok(())
    }

    pub(super) fn service(&self) -> &OwnedService {
        &self.service
    }
}

fn expected_binary_path() -> String {
    let root = String::from_utf16_lossy(
        ocentra_protected_capability_custody_protocol::constants::BROKER_INSTALL_ROOT_UTF16,
    );
    format!(
        "\"{}\\{}\"",
        root,
        ocentra_protected_capability_custody_protocol::constants::BROKER_EXECUTABLE_NAME
    )
}

fn service_digest(service: &ServiceObservation) -> [u8; 32] {
    let mut digest = ObservationDigest::new(SERVICE_OBSERVATION_DOMAIN);
    digest.text(service.service_name().as_str());
    digest.u32(service.service_type());
    digest.u32(service.start_type());
    digest.u32(service.error_control());
    append_optional_text(&mut digest, service.binary_path());
    append_optional_text(&mut digest, service.load_order_group());
    digest.u32(service.tag_id());
    append_texts(&mut digest, service.dependencies());
    append_optional_text(&mut digest, service.start_name());
    append_optional_text(&mut digest, service.display_name());
    digest.u32(service.service_sid_type());
    append_texts(&mut digest, service.required_privileges());
    digest.boolean(service.delayed_auto_start());
    digest.u32(service.launch_protected());
    digest.u32(service.failure_actions_reset_period());
    append_optional_text(&mut digest, service.failure_actions_reboot_message());
    append_optional_text(&mut digest, service.failure_actions_command());
    digest.u32(service.failure_actions().len() as u32);
    for action in service.failure_actions() {
        digest.field(&action.action_type().to_be_bytes());
        digest.u32(action.delay_ms());
    }
    digest.boolean(service.failure_actions_on_non_crash_failures());
    append_security(&mut digest, service.security());
    digest.finish()
}

fn append_optional_text(digest: &mut ObservationDigest, value: Option<&WindowsText>) {
    digest.boolean(value.is_some());
    if let Some(value) = value {
        digest.text(value.as_str());
    }
}

fn append_texts(digest: &mut ObservationDigest, values: &[WindowsText]) {
    digest.u32(values.len() as u32);
    for value in values {
        digest.text(value.as_str());
    }
}
