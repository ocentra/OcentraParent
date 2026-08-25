//! Safe, bounded wrappers around the Windows ABI used by protected custody.
//!
//! This crate is intentionally not a custody implementation.  It owns the
//! unsafe ABI calls and handle lifetime mechanics only.  Enrollment, identity
//! comparisons, persistence, transcript construction, and authority remain
//! in the private core adapter.  In particular, no public API accepts caller
//! attestation or returns a raw Windows handle.
//! This package is non-publishable. Its only permitted workspace consumers
//! are the private core adapter and the exact elevated provisioning binary;
//! it is not a general platform utility.

#![deny(unsafe_op_in_unsafe_fn)]

mod error;
mod ffi;
mod input_fault;
mod observations;
mod owned_types;
#[cfg(windows)]
mod security;
#[cfg(windows)]
mod tpm;
#[cfg(not(windows))]
mod unsupported;
#[cfg(windows)]
mod windows;

use core::fmt;

pub type RegistryPath = ffi::text::RegistryPath;
pub type RegistryValueName = ffi::text::RegistryValueName;
pub type ServiceName = ffi::text::ServiceName;
pub type WindowsText = ffi::text::WindowsText;
pub type InputFault = input_fault::InputFault;

/// Maximum size of a value copied from an operating-system API.
pub const MAX_BUFFER_BYTES: usize = 1024 * 1024;
/// Maximum number of UTF-16 code units accepted from a Windows path/value.
pub const MAX_WIDE_CHARS: usize = 32 * 1024;
/// Maximum number of ACEs accepted from a security descriptor.
pub const MAX_ACES: usize = 4096;

/// Errors returned by the ABI/mechanics layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// The current target has no Windows implementation.
    UnsupportedPlatform,
    /// A Windows API returned this error code.
    Win32(u32),
    /// A TPM/TBS API returned this status code.
    Tpm(u32),
    /// An input exceeded a bounded ABI buffer.
    BufferTooLarge,
    /// An input or operating-system response was malformed.
    InvalidInput(InputFault),
    /// A response did not satisfy the strict TPM wire shape.
    MalformedTpm,
    /// A Microsoft CNG/PCP API returned an error status.
    Crypto(u32),
    /// A CNG/PCP key response did not satisfy the required hardware,
    /// non-exportable, sign-only, or TPM material invariants.
    CryptoPropertyViolation,
}

/// Result type used by the FFI boundary.
pub type Result<T> = core::result::Result<T, Error>;

/// A stable identity for the executable file observed through a Windows file
/// handle.  The volume and 128-bit file identifier are OS observations; they
/// are not a product enrollment or authority decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageIdentity {
    volume_serial_number: u64,
    file_id: [u8; 16],
}

/// A pinned executable observation.  The corresponding Windows file handle
/// is retained by [`OwnedProcess`] while this value is consumed by a caller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageObservation {
    path: WindowsText,
    identity: ImageIdentity,
    sha256: [u8; 32],
    security: SecurityDescriptorObservation,
    ancestors: Vec<ImageAncestorObservation>,
    file_attributes: u32,
    reparse_tag: u32,
}

/// A pinned executable ancestor observed through a retained directory handle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageAncestorObservation {
    path: WindowsText,
    identity: ImageIdentity,
    security: SecurityDescriptorObservation,
    file_attributes: u32,
    reparse_tag: u32,
}

/// A process identity snapshot obtained from an owned process and image
/// handle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessObservation {
    process_id: u32,
    creation_time_100ns: u64,
    image: ImageObservation,
    alive: bool,
}

/// A token identity snapshot.  SID bytes are copied while the token handle
/// remains owned by [`OwnedToken`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenObservation {
    sid: Vec<u8>,
    integrity_level: u32,
    session_id: u32,
}

/// One normalized access-control entry from a Windows security descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AceObservation {
    ace_type: u8,
    flags: u8,
    access_mask: u32,
    sid: Vec<u8>,
    raw: Vec<u8>,
}

/// An exact self-relative security-descriptor snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecurityDescriptorObservation {
    descriptor: Vec<u8>,
    owner_sid: Vec<u8>,
    owner_defaulted: bool,
    dacl_present: bool,
    dacl_defaulted: bool,
    dacl: Vec<AceObservation>,
    dacl_protected: bool,
}

/// A registry value copied without interpreting its product meaning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryValue {
    value_type: u32,
    data: Vec<u8>,
}

/// A value copied from the final retained registry key in a custody chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryValueObservation {
    name: RegistryValueName,
    value: RegistryValue,
}

/// A copied, non-authoritative TPM2 `NV_ReadPublic` observation.
///
/// This is raw platform evidence only. It does not represent installer
/// enrollment, select an authorized index, or permit a read/increment. The
/// sole core consumer must compare every field and the TPM Name against its
/// own protected enrollment before constructing private authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TpmNvPublicObservation {
    nv_index: u32,
    name_algorithm: u16,
    attributes: u32,
    auth_policy: Vec<u8>,
    data_size: u16,
    name: Vec<u8>,
}

impl TpmNvPublicObservation {
    pub fn nv_index(&self) -> u32 {
        self.nv_index
    }

    pub fn name_algorithm(&self) -> u16 {
        self.name_algorithm
    }

    pub fn attributes(&self) -> u32 {
        self.attributes
    }

    pub fn auth_policy(&self) -> &[u8] {
        &self.auth_policy
    }

    pub fn data_size(&self) -> u16 {
        self.data_size
    }

    pub fn name(&self) -> &[u8] {
        &self.name
    }
}

/// The service configuration and protected security snapshot observed from
/// the Service Control Manager.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceObservation {
    service_name: WindowsText,
    service_type: u32,
    start_type: u32,
    error_control: u32,
    binary_path: Option<WindowsText>,
    load_order_group: Option<WindowsText>,
    tag_id: u32,
    dependencies: Vec<WindowsText>,
    start_name: Option<WindowsText>,
    display_name: Option<WindowsText>,
    service_sid_type: u32,
    required_privileges: Vec<WindowsText>,
    delayed_auto_start: bool,
    launch_protected: u32,
    failure_actions_reset_period: u32,
    failure_actions_reboot_message: Option<WindowsText>,
    failure_actions_command: Option<WindowsText>,
    failure_actions: Vec<ServiceFailureAction>,
    failure_actions_on_non_crash_failures: bool,
    security: SecurityDescriptorObservation,
}

/// One bounded SCM failure action observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceFailureAction {
    action_type: i32,
    delay_ms: u32,
}

/// An installer/SCM-owned path and its exact ancestor security observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryAncestorObservation {
    path: WindowsText,
    security: SecurityDescriptorObservation,
}

/// A retained chain of registry keys opened relative to one another.
pub type OwnedRegistryChain = owned_types::OwnedRegistryChain;

/// A process handle retained for the observation lifetime.
pub type OwnedProcess = owned_types::OwnedProcess;

/// A token handle retained for the observation lifetime.
pub type OwnedToken = owned_types::OwnedToken;

/// A Service Control Manager database handle.
pub type OwnedScManager = owned_types::OwnedScManager;

/// A service handle retained for configuration and security observation.
pub type OwnedService = owned_types::OwnedService;

/// A TBS context retained for bounded TPM2 NV mechanics.
pub type OwnedTbsContext = owned_types::OwnedTbsContext;

/// Strict RSA-3072/PSS-SHA256 public material for the fixed TPM policy signer.
#[cfg(windows)]
pub type TpmPolicySignerPublic = tpm::codec_types::signer::TpmPolicySignerPublic;

/// One fixed-width PCP signature accepted by the prepared TPM policy flow.
#[cfg(windows)]
pub type TpmPolicySignature = tpm::codec_types::signer::TpmPolicySignature;

/// A one-use TPM counter read awaiting the enrolled PCP signature.
#[cfg(windows)]
pub type PreparedTpmCounterRead<'a> = tpm::session::facade::PreparedTpmCounterRead<'a>;

/// A one-use TPM counter increment awaiting the enrolled PCP signature.
#[cfg(windows)]
pub type PreparedTpmCounterIncrement<'a> = tpm::session::facade::PreparedTpmCounterIncrement<'a>;

/// Result of a fixed TPM NV counter increment after the command crossed TBS.
///
/// `Uncertain` is not a retry signal. The caller must start a fresh signed
/// fixed-counter read and reconcile it against its previously observed value;
/// blindly repeating the increment could advance the counter twice.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TpmCounterIncrementOutcome {
    /// A strict success response proved that the TPM accepted the increment.
    Committed,
    /// TBS did not return a response whose success or failure could be proved.
    Uncertain(TpmCounterIncrementUncertainty),
}

/// Why an increment result needs reconciliation through a new signed read.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TpmCounterIncrementUncertainty {
    /// The TBS submission boundary did not return a TPM response.
    Transport,
    /// A response arrived but its success framing or authorization was invalid.
    MalformedResponse,
}

/// A machine-scoped handle to the fixed Microsoft Platform Crypto Provider.
///
/// The provider identity is compiled into the implementation; callers cannot
/// select a provider or pass an MSI-controlled name through this boundary.
pub type OwnedPcpProvider = owned_types::OwnedPcpProvider;

/// A retained non-exportable PCP signing key and its provider handle.
///
/// This type exposes only signing and mechanical observations. It does not
/// mint enrollment, custody, or caller authority.
pub type OwnedPcpSigningKey = owned_types::OwnedPcpSigningKey;

/// Mechanical properties observed from the fixed PCP signing key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcpKeyObservation {
    key_name: WindowsText,
    implementation_type: u32,
    export_policy: u32,
    key_usage: u32,
    pcp_key_usage_policy: u32,
    platform_type: WindowsText,
    security_descriptor: Vec<u8>,
    ek_public: Vec<u8>,
    tpm2b_name: Vec<u8>,
}

impl PcpKeyObservation {
    pub fn key_name(&self) -> &WindowsText {
        &self.key_name
    }

    pub fn implementation_type(&self) -> u32 {
        self.implementation_type
    }

    pub fn export_policy(&self) -> u32 {
        self.export_policy
    }

    pub fn key_usage(&self) -> u32 {
        self.key_usage
    }

    pub fn pcp_key_usage_policy(&self) -> u32 {
        self.pcp_key_usage_policy
    }

    pub fn platform_type(&self) -> &WindowsText {
        &self.platform_type
    }

    pub fn security_descriptor(&self) -> &[u8] {
        &self.security_descriptor
    }

    pub fn ek_public(&self) -> &[u8] {
        &self.ek_public
    }

    pub fn tpm2b_name(&self) -> &[u8] {
        &self.tpm2b_name
    }
}
