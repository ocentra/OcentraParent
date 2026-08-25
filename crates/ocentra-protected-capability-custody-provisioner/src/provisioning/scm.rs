use ocentra_protected_capability_custody_windows_ffi::{
    Error as FfiError, OwnedScManager, ServiceName, ServiceObservation, WindowsText,
};
use sha2::{Digest, Sha256};

use super::constants;
use super::enrollment::EnrollmentSnapshot;
use super::error::ProvisioningError;
use super::scm_error;

const SERVICE_WIN32_OWN_PROCESS: u32 = 0x0000_0010;
const SERVICE_AUTO_START: u32 = 2;
const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1;
const LOCAL_SYSTEM_ACCOUNT: &str = "LocalSystem";
const SERVICE_OBSERVATION_DOMAIN: &str = "ocentra.pcc.scm-observation.v1";

pub(super) fn readback(enrollment: &EnrollmentSnapshot) -> Result<(), ProvisioningError> {
    observe(enrollment, scm_error::initial)
}

pub(super) fn revalidate(enrollment: &EnrollmentSnapshot) -> Result<(), ProvisioningError> {
    observe(enrollment, scm_error::revalidation)
}

fn observe(
    enrollment: &EnrollmentSnapshot,
    error_mapper: fn(FfiError) -> ProvisioningError,
) -> Result<(), ProvisioningError> {
    let manager = OwnedScManager::open().map_err(error_mapper)?;
    let name = ServiceName::try_from_str(constants::FIXED_SERVICE_NAME).map_err(error_mapper)?;
    let service = manager.open_service(&name).map_err(error_mapper)?;
    let initial = service.observation().map_err(error_mapper)?;
    validate(&initial, enrollment)?;
    let current = service.reobserve().map_err(scm_error::revalidation)?;
    if current != initial {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    validate(&current, enrollment)
}

fn validate(
    service: &ServiceObservation,
    enrollment: &EnrollmentSnapshot,
) -> Result<(), ProvisioningError> {
    let expected_binary_path = constants::expected_service_binary_path()
        .map_err(|_| ProvisioningError::ExistingStateRejected)?;
    if service.service_name().as_str() != constants::FIXED_SERVICE_NAME
        || service.service_type() != SERVICE_WIN32_OWN_PROCESS
        || service.start_type() != SERVICE_AUTO_START
        || service.binary_path() != Some(&expected_binary_path)
        || service.start_name().map(WindowsText::as_str) != Some(LOCAL_SYSTEM_ACCOUNT)
        || service.service_sid_type() != SERVICE_SID_TYPE_UNRESTRICTED
        || service.security().descriptor().is_empty()
        || service.security().owner_was_defaulted()
        || !service.security().dacl_is_present()
        || service.security().dacl_was_defaulted()
        || !service.security().dacl_is_protected()
        || service_digest(service) != enrollment.service_digest
    {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    Ok(())
}

fn service_digest(service: &ServiceObservation) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update((SERVICE_OBSERVATION_DOMAIN.len() as u32).to_be_bytes());
    digest.update(SERVICE_OBSERVATION_DOMAIN.as_bytes());
    text(&mut digest, service.service_name());
    u32_value(&mut digest, service.service_type());
    u32_value(&mut digest, service.start_type());
    u32_value(&mut digest, service.error_control());
    optional_text(&mut digest, service.binary_path());
    optional_text(&mut digest, service.load_order_group());
    u32_value(&mut digest, service.tag_id());
    digest_field(
        &mut digest,
        &(service.dependencies().len() as u32).to_be_bytes(),
    );
    for dependency in service.dependencies() {
        text(&mut digest, dependency);
    }
    optional_text(&mut digest, service.start_name());
    optional_text(&mut digest, service.display_name());
    u32_value(&mut digest, service.service_sid_type());
    digest_field(
        &mut digest,
        &(service.required_privileges().len() as u32).to_be_bytes(),
    );
    for privilege in service.required_privileges() {
        text(&mut digest, privilege);
    }
    boolean(&mut digest, service.delayed_auto_start());
    u32_value(&mut digest, service.launch_protected());
    u32_value(&mut digest, service.failure_actions_reset_period());
    optional_text(&mut digest, service.failure_actions_reboot_message());
    optional_text(&mut digest, service.failure_actions_command());
    digest_field(
        &mut digest,
        &(service.failure_actions().len() as u32).to_be_bytes(),
    );
    for action in service.failure_actions() {
        digest_field(&mut digest, &action.action_type().to_be_bytes());
        u32_value(&mut digest, action.delay_ms());
    }
    boolean(&mut digest, service.failure_actions_on_non_crash_failures());
    digest_field(&mut digest, service.security().descriptor());
    digest.finalize().into()
}

fn optional_text(digest: &mut Sha256, value: Option<&WindowsText>) {
    boolean(digest, value.is_some());
    if let Some(value) = value {
        text(digest, value);
    }
}

fn text(digest: &mut Sha256, value: &WindowsText) {
    digest_field(digest, value.as_str().as_bytes());
}

fn u32_value(digest: &mut Sha256, value: u32) {
    digest_field(digest, &value.to_be_bytes());
}

fn boolean(digest: &mut Sha256, value: bool) {
    digest_field(digest, &[u8::from(value)]);
}

fn digest_field(digest: &mut Sha256, value: &[u8]) {
    digest.update((value.len() as u32).to_be_bytes());
    digest.update(value);
}
