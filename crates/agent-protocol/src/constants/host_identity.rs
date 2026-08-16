pub const READ_MODEL_ID_V0_8: &str = "v0-8-host-identity-read-model-proof";

pub const ENTRY_ID_INSTALLED_APP_INVENTORY: &str = "host-identity-installed-app-inventory";
pub const ENTRY_ID_PROCESS_LINEAGE: &str = "host-identity-process-lineage";
pub const ENTRY_ID_EXECUTABLE_IDENTITY: &str = "host-identity-executable-identity";
pub const ENTRY_ID_PACKAGE_IDENTITY: &str = "host-identity-package-identity";
pub const ENTRY_ID_PUBLISHER_SIGNATURE: &str = "host-identity-publisher-signature";
pub const ENTRY_ID_INVENTORY_PROCESS_LINK: &str = "host-identity-inventory-process-link";
pub const ENTRY_ID_UNSUPPORTED_IDENTITY: &str = "host-identity-unsupported-identity";
pub const ENTRY_ID_ROLLBACK_READINESS: &str = "host-identity-rollback-readiness";
pub const ENTRY_ID_AUDIT_CUSTODY: &str = "host-identity-audit-custody";

pub const KIND_INSTALLED_APP_INVENTORY: &str = "installed-app-inventory";
pub const KIND_PROCESS_LINEAGE: &str = "process-lineage";
pub const KIND_EXECUTABLE_IDENTITY: &str = "executable-identity";
pub const KIND_PACKAGE_IDENTITY: &str = "package-identity";
pub const KIND_PUBLISHER_SIGNATURE: &str = "publisher-signature";
pub const KIND_INVENTORY_PROCESS_LINK: &str = "inventory-process-link";
pub const KIND_UNSUPPORTED_IDENTITY: &str = "unsupported-identity";
pub const KIND_ROLLBACK_READINESS: &str = "rollback-readiness";
pub const KIND_AUDIT_CUSTODY: &str = "audit-custody";

pub const CLASS_INVENTORY: &str = "inventory";
pub const CLASS_PROCESS: &str = "process";
pub const CLASS_EXECUTABLE: &str = "executable";
pub const CLASS_PACKAGE: &str = "package";
pub const CLASS_PUBLISHER_SIGNATURE: &str = "publisher-signature";
pub const CLASS_ROLLBACK: &str = "rollback";
pub const CLASS_AUDIT: &str = "audit";

pub const REQUIREMENT_INSTALLED_APP_INVENTORY: &str = "Installed app inventory must come from a real Windows host source before broad app blocking can target it.";
pub const REQUIREMENT_PROCESS_LINEAGE: &str = "Process lineage must identify pid, parent pid when available, executable path, start time, and observation source.";
pub const REQUIREMENT_EXECUTABLE_IDENTITY: &str = "Executable identity must include canonical path plus a host-derived fingerprint before it can support app identity.";
pub const REQUIREMENT_PACKAGE_IDENTITY: &str = "Package identity must distinguish packaged apps from unpackaged Win32 executables without silently upgrading unknown apps.";
pub const REQUIREMENT_PUBLISHER_SIGNATURE: &str = "Publisher and signature evidence must be captured or explicitly unavailable before trust-sensitive app identity claims upgrade.";
pub const REQUIREMENT_INVENTORY_PROCESS_LINK: &str = "Inventory and running-process evidence must agree before the runtime can claim a target is the same app.";
pub const REQUIREMENT_UNSUPPORTED_IDENTITY: &str = "Unsupported, permission-limited, or unknown host identity must remain unavailable instead of becoming app proof.";
pub const REQUIREMENT_ROLLBACK_READINESS: &str = "Rollback readiness for broad app blocking is not claimed until the same app identity has apply and rollback artifacts.";
pub const REQUIREMENT_AUDIT_CUSTODY: &str = "Audit custody must tie identity evidence, parent rule, adapter outcome, fallback, and evidence refs together.";

pub const ARTIFACTS_INSTALLED_APP_INVENTORY: &[&str] = &[
    "Windows installed app inventory source and timestamp",
    "Package or executable identity for each inventory row",
    "Source adapter id and permission state for the inventory read",
];
pub const ARTIFACTS_PROCESS_LINEAGE: &[&str] = &[
    "Process id and parent process id when available",
    "Executable path and process start timestamp",
    "Observation adapter id, freshness, and custody evidence",
];
pub const ARTIFACTS_EXECUTABLE_IDENTITY: &[&str] = &[
    "Canonical executable path",
    "File fingerprint or version metadata from the host",
    "Path normalization and custody evidence",
];
pub const ARTIFACTS_PACKAGE_IDENTITY: &[&str] = &[
    "Package family name, product id, or explicit unpackaged identity",
    "Inventory source that produced the package or executable identity",
    "Unknown or unpackaged status when package metadata is unavailable",
];
pub const ARTIFACTS_PUBLISHER_SIGNATURE: &[&str] = &[
    "Publisher name or certificate chain when available",
    "Signature verification result or unsigned state",
    "Verification source, timestamp, and custody evidence",
];
pub const ARTIFACTS_INVENTORY_PROCESS_LINK: &[&str] = &[
    "Joined inventory evidence id and process evidence id",
    "Matching package id or executable identity",
    "Freshness window and mismatch reason when the join fails",
];
pub const ARTIFACTS_UNSUPPORTED_IDENTITY: &[&str] = &[
    "Unsupported or permission-limited host state",
    "Missing package identity reason",
    "Manual remediation or alternate source requirement",
];
pub const ARTIFACTS_ROLLBACK_READINESS: &[&str] = &[
    "Block apply artifact for the same app identity",
    "Rollback token and rollback result for the same app identity",
    "Failure, unavailable, and audit evidence for rollback attempts",
];
pub const ARTIFACTS_AUDIT_CUSTODY: &[&str] = &[
    "Parent rule or policy decision id",
    "Identity evidence refs used by the adapter decision",
    "Adapter outcome, fallback state, and audit event ids",
];

pub const SIGNALS_INSTALLED_APP_INVENTORY: &[&str] = &[
    "Inventory row has a stable package id or explicit unpackaged executable identity.",
    "Inventory evidence timestamp and adapter id are present.",
];
pub const SIGNALS_PROCESS_LINEAGE: &[&str] = &[
    "Process lineage is tied to a current evidence ref.",
    "Missing parent process data is represented as unavailable rather than invented.",
];
pub const SIGNALS_EXECUTABLE_IDENTITY: &[&str] = &[
    "Executable identity is stable across one inventory/process join.",
    "Fingerprint unavailable states remain typed and do not become proof.",
];
pub const SIGNALS_PACKAGE_IDENTITY: &[&str] = &[
    "Packaged and unpackaged identities are represented separately.",
    "Unknown app identity remains unknown until a supported source proves it.",
];
pub const SIGNALS_PUBLISHER_SIGNATURE: &[&str] = &[
    "Unsigned, invalid, unavailable, and valid signatures are distinct states.",
    "Publisher evidence is tied to the same executable or package identity.",
];
pub const SIGNALS_INVENTORY_PROCESS_LINK: &[&str] = &[
    "The join uses package id or executable identity, not display text.",
    "Mismatches produce typed unavailable/manual-required output.",
];
pub const SIGNALS_UNSUPPORTED_IDENTITY: &[&str] = &[
    "Unsupported identity is visible as unavailable.",
    "The fallback tells runtime and Portal not to treat unknown apps as known targets.",
];
pub const SIGNALS_ROLLBACK_READINESS: &[&str] = &[
    "Rollback evidence references the same package or executable identity.",
    "Unavailable rollback remains visible and blocks product-ready claims.",
];
pub const SIGNALS_AUDIT_CUSTODY: &[&str] = &[
    "Audit events include the identity evidence refs used for the decision.",
    "Manual-required, unavailable, and not-claimed outcomes are auditable.",
];

pub const FALLBACK_INSTALLED_APP_INVENTORY: &str =
    "Keep broad app targets manual-required when installed inventory is missing or stale.";
pub const FALLBACK_PROCESS_LINEAGE: &str =
    "Treat unknown or stale lineage as unavailable for broad app identity matching.";
pub const FALLBACK_EXECUTABLE_IDENTITY: &str = "Use unavailable or manual-required when canonical path or fingerprint evidence cannot be collected.";
pub const FALLBACK_PACKAGE_IDENTITY: &str = "Keep the app target manual-required when package identity is unknown, ambiguous, or unpackaged without proof.";
pub const FALLBACK_PUBLISHER_SIGNATURE: &str = "Represent missing signature evidence as manual-required or unavailable; do not invent trust state.";
pub const FALLBACK_INVENTORY_PROCESS_LINK: &str =
    "Reject broad app targeting when inventory and process evidence cannot be joined.";
pub const FALLBACK_UNSUPPORTED_IDENTITY: &str = "Return unavailable and require manual proof when host identity is unsupported or permission-limited.";
pub const FALLBACK_ROLLBACK_READINESS: &str = "Keep broad app rollback not-claimed until apply and rollback artifacts exist for the same identity.";
pub const FALLBACK_AUDIT_CUSTODY: &str =
    "Require real service audit custody before any process/package identity claim can upgrade.";
