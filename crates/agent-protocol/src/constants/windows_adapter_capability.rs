pub const READ_MODEL_ID_V0_8: &str = "v0-8-windows-adapter-capability-proof";

pub const ENTRY_ID_APP_TARGET: &str = "windows-adapter-app-target-capability";
pub const ENTRY_ID_DOMAIN_NETWORK_TARGET: &str = "windows-adapter-domain-network-capability";
pub const ENTRY_ID_MANAGED_BROWSER_TARGET: &str = "windows-adapter-managed-browser-capability";
pub const ENTRY_ID_UNMANAGED_BROWSER_TARGET: &str = "windows-adapter-unmanaged-browser-capability";
pub const ENTRY_ID_UNSUPPORTED_OS_TARGET: &str = "windows-adapter-unsupported-os-capability";
pub const ENTRY_ID_ROLLBACK_AUDIT_TARGET: &str = "windows-adapter-rollback-audit-capability";

pub const SURFACE_APP_TARGET: &str = "app-target";
pub const SURFACE_DOMAIN_NETWORK_TARGET: &str = "domain-network-target";
pub const SURFACE_MANAGED_BROWSER_TARGET: &str = "managed-browser-target";
pub const SURFACE_UNMANAGED_BROWSER_TARGET: &str = "unmanaged-browser-target";
pub const SURFACE_UNSUPPORTED_OS_TARGET: &str = "unsupported-os-target";
pub const SURFACE_ROLLBACK_AUDIT_TARGET: &str = "rollback-audit-target";

pub const OUTCOME_MANUAL_REQUIRED: &str = "manual-required";
pub const OUTCOME_UNAVAILABLE: &str = "unavailable";
pub const OUTCOME_PROCESS_ONLY_IMPLEMENTED: &str = "process-only-implemented";
pub const OUTCOME_NOT_CLAIMED: &str = "not-claimed";

pub const CLAIM_BOUNDARY_APP_TARGET: &str =
    "Windows app targets require host identity evidence before broad app blocking can upgrade.";
pub const CLAIM_BOUNDARY_DOMAIN_NETWORK_TARGET: &str = "Domain and network targets require a host network control adapter before blocking can upgrade.";
pub const CLAIM_BOUNDARY_MANAGED_BROWSER_TARGET: &str =
    "Managed browser service-command readiness is not exact URL control proof.";
pub const CLAIM_BOUNDARY_UNMANAGED_BROWSER_TARGET: &str =
    "Unmanaged browser capability is process-only and cannot become exact URL evidence.";
pub const CLAIM_BOUNDARY_UNSUPPORTED_OS_TARGET: &str =
    "Windows adapter capability proof must stay unavailable on unsupported OS targets.";
pub const CLAIM_BOUNDARY_ROLLBACK_AUDIT_TARGET: &str =
    "Rollback and audit claims require same-identity apply, rollback, and custody artifacts.";

pub const FALLBACK_APP_TARGET: &str = "Keep app target enforcement manual-required until inventory, process, executable, package, signature, and join evidence are present.";
pub const FALLBACK_DOMAIN_NETWORK_TARGET: &str =
    "Return manual-required or unavailable until network filter apply and rollback proof exists.";
pub const FALLBACK_MANAGED_BROWSER_TARGET: &str = "Return manual-required for service-command browser targets and do not claim exact URL control.";
pub const FALLBACK_UNMANAGED_BROWSER_TARGET: &str = "Allow only process pid/name guardrails and warn/no-op behavior; exact unmanaged URL evidence remains not claimed.";
pub const FALLBACK_UNSUPPORTED_OS_TARGET: &str = "Return unavailable on non-Windows or unsupported host targets instead of using a Windows adapter claim.";
pub const FALLBACK_ROLLBACK_AUDIT_TARGET: &str = "Keep rollback and audit readiness manual-required until the same package or executable identity has apply, rollback, and custody evidence.";

pub const ARTIFACT_WINDOWS_APP_IDENTITY: &str = "Windows app identity evidence chain";
pub const ARTIFACT_WINDOWS_DOMAIN_FILTER: &str = "Windows domain/network filter evidence";
pub const ARTIFACT_WINDOWS_MANAGED_BROWSER: &str = "Windows managed browser command evidence";
pub const ARTIFACT_WINDOWS_UNMANAGED_BROWSER: &str = "Windows unmanaged browser process evidence";
pub const ARTIFACT_UNSUPPORTED_OS: &str = "Unsupported OS unavailable-state evidence";
pub const ARTIFACT_ROLLBACK_AUDIT: &str = "Windows rollback and audit custody evidence";
