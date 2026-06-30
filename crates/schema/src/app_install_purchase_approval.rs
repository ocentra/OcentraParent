use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

macro_rules! approval_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! approval_string_enum {
    ($name:ident { $($variant:ident => $value:expr),+ $(,)? }) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum $name {
            $(
                $variant,
            )+
        }

        impl $name {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }

            pub fn parse(value: &str) -> Option<Self> {
                $(if value == $value {
                    return Some(Self::$variant);
                })+
                None
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::parse(&value).ok_or_else(|| {
                    serde::de::Error::unknown_variant(&value, &[$($value,)+])
                })
            }
        }
    };
}

pub const APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION: &str =
    "app-install-purchase-approval-contract-proof";

mod approval_string_values {
    pub const PARENT_PLATFORM_WINDOWS: &str = "windows";
    pub const PARENT_PLATFORM_LINUX: &str = "linux";
    pub const PARENT_PLATFORM_MACOS: &str = "macos";
    pub const PARENT_PLATFORM_ANDROID: &str = "android";
    pub const PARENT_PLATFORM_IOS: &str = "ios";

    pub const PARENT_ACTOR_ROLE_PARENT: &str = "parent";
    pub const PARENT_ACTOR_ROLE_GUARDIAN: &str = "guardian";
    pub const PARENT_ACTOR_ROLE_SYSTEM: &str = "system";

    pub const PARENT_EVIDENCE_REFERENCE_KIND_JOURNAL_EVENT: &str = "journal-event";
    pub const PARENT_EVIDENCE_REFERENCE_KIND_QUERY_STORE_SUMMARY: &str = "query-store-summary";
    pub const PARENT_EVIDENCE_REFERENCE_KIND_ACTIVITY_EVENT: &str = "activity-event";
    pub const PARENT_EVIDENCE_REFERENCE_KIND_POLICY_DECISION: &str = "policy-decision";
    pub const PARENT_EVIDENCE_REFERENCE_KIND_LOCAL_AI_RESULT: &str = "local-ai-result";

    pub const REQUEST_KIND_INSTALL: &str = "install";
    pub const REQUEST_KIND_PURCHASE: &str = "purchase";
    pub const REQUEST_KIND_SUBSCRIPTION: &str = "subscription";

    pub const STORE_SURFACE_GOOGLE_PLAY: &str = "google-play";
    pub const STORE_SURFACE_APPLE_APP_STORE: &str = "apple-app-store";
    pub const STORE_SURFACE_MAC_APP_STORE: &str = "mac-app-store";
    pub const STORE_SURFACE_MICROSOFT_STORE: &str = "microsoft-store";
    pub const STORE_SURFACE_LINUX_PACKAGE_MANAGER: &str = "linux-package-manager";
    pub const STORE_SURFACE_PARENT_MANUAL_ENTRY: &str = "parent-manual-entry";
    pub const STORE_SURFACE_UNKNOWN_STORE: &str = "unknown-store";

    pub const STORE_METADATA_FRESHNESS_FRESH: &str = "fresh";
    pub const STORE_METADATA_FRESHNESS_STALE: &str = "stale";
    pub const STORE_METADATA_FRESHNESS_UNKNOWN: &str = "unknown";
    pub const STORE_METADATA_FRESHNESS_MANUAL_REQUIRED: &str = "manual-required";
    pub const STORE_METADATA_FRESHNESS_UNAVAILABLE: &str = "unavailable";

    pub const SUPPORT_STATE_SUPPORTED: &str = "supported";
    pub const SUPPORT_STATE_MANUAL_REQUIRED: &str = "manual-required";
    pub const SUPPORT_STATE_UNAVAILABLE: &str = "unavailable";

    pub const DECISION_ACTION_APPROVE: &str = "approve";
    pub const DECISION_ACTION_DENY: &str = "deny";
    pub const DECISION_ACTION_TIME_BOX: &str = "time-box";
    pub const DECISION_ACTION_REVIEW_NEEDED: &str = "review-needed";

    pub const APPROVAL_STATE_PENDING_PARENT_REVIEW: &str = "pending-parent-review";
    pub const APPROVAL_STATE_APPROVED: &str = "approved";
    pub const APPROVAL_STATE_DENIED: &str = "denied";
    pub const APPROVAL_STATE_TIME_BOX_ACTIVE: &str = "time-box-active";
    pub const APPROVAL_STATE_EXPIRED: &str = "expired";
    pub const APPROVAL_STATE_REVIEW_NEEDED: &str = "review-needed";

    pub const EXPIRY_STATE_NOT_EXPIRING: &str = "not-expiring";
    pub const EXPIRY_STATE_TIME_BOX_ACTIVE: &str = "time-box-active";
    pub const EXPIRY_STATE_EXPIRED: &str = "expired";
    pub const EXPIRY_STATE_REVIEW_NEEDED: &str = "review-needed";

    pub const PURCHASE_KIND_ONE_TIME_PURCHASE: &str = "one-time-purchase";
    pub const PURCHASE_KIND_IN_APP_PURCHASE: &str = "in-app-purchase";
    pub const PURCHASE_KIND_SUBSCRIPTION: &str = "subscription";

    pub const SUBSCRIPTION_PERIOD_WEEKLY: &str = "weekly";
    pub const SUBSCRIPTION_PERIOD_MONTHLY: &str = "monthly";
    pub const SUBSCRIPTION_PERIOD_ANNUAL: &str = "annual";
    pub const SUBSCRIPTION_PERIOD_UNKNOWN: &str = "unknown";

    pub const CHILD_FACING_STATUS_PENDING_PARENT_REVIEW_VISIBLE: &str =
        "pending-parent-review-visible";
    pub const CHILD_FACING_STATUS_APPROVED_VISIBLE: &str = "approved-visible";
    pub const CHILD_FACING_STATUS_DENIED_VISIBLE: &str = "denied-visible";
    pub const CHILD_FACING_STATUS_TIME_BOX_VISIBLE: &str = "time-box-visible";
    pub const CHILD_FACING_STATUS_REVIEW_NEEDED_VISIBLE: &str = "review-needed-visible";

    pub const AUDIT_REPORT_SURFACE_REQUEST_AUDIT_HISTORY: &str = "request-audit-history";
    pub const AUDIT_REPORT_SURFACE_PARENT_DECISION_AUDIT_HISTORY: &str =
        "parent-decision-audit-history";
    pub const AUDIT_REPORT_SURFACE_CHILD_FACING_STATE_REPORT: &str = "child-facing-state-report";
    pub const AUDIT_REPORT_SURFACE_PLATFORM_LIMITATION_REPORT: &str = "platform-limitation-report";

    pub const PROOF_INTEGRATION_STATE_CONTRACT_ONLY: &str = "contract-only";
    pub const PROOF_INTEGRATION_STATE_MANUAL_REQUIRED: &str = "manual-required";
    pub const PROOF_INTEGRATION_STATE_UNAVAILABLE: &str = "unavailable";

    pub const AUDIT_EVENT_KIND_REQUEST_RECORDED: &str = "request-recorded";
    pub const AUDIT_EVENT_KIND_METADATA_SOURCE_EVALUATED: &str = "metadata-source-evaluated";
    pub const AUDIT_EVENT_KIND_PARENT_DECISION_RECORDED: &str = "parent-decision-recorded";
    pub const AUDIT_EVENT_KIND_APPROVAL_EXPIRED: &str = "approval-expired";
    pub const AUDIT_EVENT_KIND_PLATFORM_LIMITATION_RECORDED: &str = "platform-limitation-recorded";

    pub const NON_CLAIM_NO_STORE_INTEGRATION: &str = "no-store-integration";
    pub const NON_CLAIM_NO_BILLING_ENTITLEMENT_LOGIC: &str = "no-billing-entitlement-logic";
    pub const NON_CLAIM_NO_PORTAL_UI: &str = "no-portal-ui";
    pub const NON_CLAIM_NO_PLATFORM_ADAPTER: &str = "no-platform-adapter";
    pub const NON_CLAIM_NO_STORE_POLICY_BYPASS: &str = "no-store-policy-bypass";
    pub const NON_CLAIM_NO_REAL_INSTALL_OR_PURCHASE_INTERCEPTION: &str =
        "no-real-install-or-purchase-interception";
    pub const NON_CLAIM_NOT_GENERIC_APP_BLOCKING: &str = "not-generic-app-blocking";

    pub const STORE_INTEGRATION_CLAIM_NOT_CLAIMED: &str = "not-claimed";
    pub const BILLING_ENTITLEMENT_CLAIM_NOT_CLAIMED: &str = "not-claimed";
    pub const PORTAL_UI_CLAIM_NOT_IMPLEMENTED: &str = "not-implemented";
    pub const PLATFORM_ADAPTER_CLAIM_NOT_IMPLEMENTED: &str = "not-implemented";
    pub const INTERCEPTION_CLAIM_NOT_CLAIMED: &str = "not-claimed";
    pub const RUNTIME_BLOCKING_SEPARATION_SEPARATE_FROM_GENERIC_APP_BLOCKING: &str =
        "separate-from-generic-app-blocking";

    pub const PLATFORM_SOURCE_AUTHORITY_GOOGLE_PLAY_LISTING: &str = "google-play-listing";
    pub const PLATFORM_SOURCE_AUTHORITY_APPLE_APP_STORE_LISTING: &str = "apple-app-store-listing";
    pub const PLATFORM_SOURCE_AUTHORITY_MAC_APP_STORE_LISTING: &str = "mac-app-store-listing";
    pub const PLATFORM_SOURCE_AUTHORITY_MICROSOFT_STORE_LISTING: &str = "microsoft-store-listing";
    pub const PLATFORM_SOURCE_AUTHORITY_LINUX_PACKAGE_MANAGER_INDEX: &str =
        "linux-package-manager-index";

    pub const PLATFORM_SOURCE_METADATA_STATE_CONTRACT_ONLY: &str = "contract-only";
    pub const PLATFORM_SOURCE_METADATA_STATE_MANUAL_REQUIRED: &str = "manual-required";
    pub const PLATFORM_SOURCE_METADATA_STATE_UNAVAILABLE: &str = "unavailable";

    pub const PLATFORM_SOURCE_EVIDENCE_STATE_REQUIRES_APPROVED_API_PROOF: &str =
        "requires-approved-api-proof";
    pub const PLATFORM_SOURCE_EVIDENCE_STATE_REQUIRES_STORE_ARTIFACT_PROOF: &str =
        "requires-store-artifact-proof";
    pub const PLATFORM_SOURCE_EVIDENCE_STATE_PLATFORM_UNAVAILABLE: &str = "platform-unavailable";

    pub const PLATFORM_SOURCE_METADATA_FIELD_STORE_LISTING_ID: &str = "store-listing-id";
    pub const PLATFORM_SOURCE_METADATA_FIELD_APP_TITLE: &str = "app-title";
    pub const PLATFORM_SOURCE_METADATA_FIELD_PUBLISHER_NAME: &str = "publisher-name";
    pub const PLATFORM_SOURCE_METADATA_FIELD_CATEGORY: &str = "category";
    pub const PLATFORM_SOURCE_METADATA_FIELD_AGE_RATING: &str = "age-rating";
    pub const PLATFORM_SOURCE_METADATA_FIELD_PRICE_DISPLAY: &str = "price-display";
    pub const PLATFORM_SOURCE_METADATA_FIELD_SUBSCRIPTION_PERIOD: &str = "subscription-period";
    pub const PLATFORM_SOURCE_METADATA_FIELD_SOURCE_URL: &str = "source-url";

    pub const PLATFORM_SOURCE_MANUAL_FALLBACK_CONTRACT_ONLY_PARENT_REVIEW: &str =
        "contract-only-parent-review";

    pub const PACKAGE_SOURCE_ARTIFACT_STATUS_MANUAL_REQUIRED: &str = "manual-required";
    pub const PACKAGE_SOURCE_ARTIFACT_STATUS_DEVICE_PROOF_REQUIRED: &str = "device-proof-required";
    pub const PACKAGE_SOURCE_ARTIFACT_STATUS_UNAVAILABLE: &str = "unavailable";

    pub const PACKAGE_SOURCE_APPROVAL_PATH_STATE_MANUAL_REQUIRED: &str = "manual-required";
    pub const PACKAGE_SOURCE_APPROVAL_PATH_STATE_UNAVAILABLE: &str = "unavailable";

    pub const PACKAGE_SOURCE_ARTIFACT_EVIDENCE_CLAIM_NOT_ATTACHED: &str = "not-attached";

    pub const PACKAGE_SOURCE_FIELD_PACKAGE_IDENTIFIER: &str = "package-identifier";
    pub const PACKAGE_SOURCE_FIELD_INSTALLER_SOURCE: &str = "installer-source";
    pub const PACKAGE_SOURCE_FIELD_PUBLISHER_OR_DEVELOPER: &str = "publisher-or-developer";
    pub const PACKAGE_SOURCE_FIELD_VERSION_OR_BUILD: &str = "version-or-build";
    pub const PACKAGE_SOURCE_FIELD_SIGNATURE_OR_RECEIPT: &str = "signature-or-receipt";
    pub const PACKAGE_SOURCE_FIELD_SOURCE_CAPTURED_AT: &str = "source-captured-at";

    pub const PACKAGE_SOURCE_KIND_WINDOWS_STORE_PACKAGE_IDENTITY: &str =
        "windows-store-package-identity";
    pub const PACKAGE_SOURCE_KIND_MACOS_BUNDLE_RECEIPT: &str = "macos-bundle-receipt";
    pub const PACKAGE_SOURCE_KIND_LINUX_PACKAGE_MANAGER_RECORD: &str =
        "linux-package-manager-record";
    pub const PACKAGE_SOURCE_KIND_ANDROID_PACKAGE_SOURCE_RECORD: &str =
        "android-package-source-record";
    pub const PACKAGE_SOURCE_KIND_IOS_APP_SOURCE_RECORD: &str = "ios-app-source-record";

    pub const PACKAGE_SOURCE_CHILD_DATA_CUSTODY_NO_CHILD_ACTIVITY_DATA: &str =
        "no-child-activity-data";
}

approval_string_enum!(ParentPlatform {
    Windows => approval_string_values::PARENT_PLATFORM_WINDOWS,
    Linux => approval_string_values::PARENT_PLATFORM_LINUX,
    Macos => approval_string_values::PARENT_PLATFORM_MACOS,
    Android => approval_string_values::PARENT_PLATFORM_ANDROID,
    Ios => approval_string_values::PARENT_PLATFORM_IOS,
});

approval_string_enum!(ParentActorRole {
    Parent => approval_string_values::PARENT_ACTOR_ROLE_PARENT,
    Guardian => approval_string_values::PARENT_ACTOR_ROLE_GUARDIAN,
    System => approval_string_values::PARENT_ACTOR_ROLE_SYSTEM,
});

approval_string_enum!(ParentEvidenceReferenceKind {
    JournalEvent => approval_string_values::PARENT_EVIDENCE_REFERENCE_KIND_JOURNAL_EVENT,
    QueryStoreSummary => approval_string_values::PARENT_EVIDENCE_REFERENCE_KIND_QUERY_STORE_SUMMARY,
    ActivityEvent => approval_string_values::PARENT_EVIDENCE_REFERENCE_KIND_ACTIVITY_EVENT,
    PolicyDecision => approval_string_values::PARENT_EVIDENCE_REFERENCE_KIND_POLICY_DECISION,
    LocalAiResult => approval_string_values::PARENT_EVIDENCE_REFERENCE_KIND_LOCAL_AI_RESULT,
});

approval_string_enum!(AppInstallPurchaseApprovalRequestKind {
    Install => approval_string_values::REQUEST_KIND_INSTALL,
    Purchase => approval_string_values::REQUEST_KIND_PURCHASE,
    Subscription => approval_string_values::REQUEST_KIND_SUBSCRIPTION,
});

approval_string_enum!(AppInstallPurchaseApprovalStoreSurface {
    GooglePlay => approval_string_values::STORE_SURFACE_GOOGLE_PLAY,
    AppleAppStore => approval_string_values::STORE_SURFACE_APPLE_APP_STORE,
    MacAppStore => approval_string_values::STORE_SURFACE_MAC_APP_STORE,
    MicrosoftStore => approval_string_values::STORE_SURFACE_MICROSOFT_STORE,
    LinuxPackageManager => approval_string_values::STORE_SURFACE_LINUX_PACKAGE_MANAGER,
    ParentManualEntry => approval_string_values::STORE_SURFACE_PARENT_MANUAL_ENTRY,
    UnknownStore => approval_string_values::STORE_SURFACE_UNKNOWN_STORE,
});

approval_string_enum!(AppInstallPurchaseApprovalStoreMetadataFreshness {
    Fresh => approval_string_values::STORE_METADATA_FRESHNESS_FRESH,
    Stale => approval_string_values::STORE_METADATA_FRESHNESS_STALE,
    Unknown => approval_string_values::STORE_METADATA_FRESHNESS_UNKNOWN,
    ManualRequired => approval_string_values::STORE_METADATA_FRESHNESS_MANUAL_REQUIRED,
    Unavailable => approval_string_values::STORE_METADATA_FRESHNESS_UNAVAILABLE,
});

approval_string_enum!(AppInstallPurchaseApprovalSupportState {
    Supported => approval_string_values::SUPPORT_STATE_SUPPORTED,
    ManualRequired => approval_string_values::SUPPORT_STATE_MANUAL_REQUIRED,
    Unavailable => approval_string_values::SUPPORT_STATE_UNAVAILABLE,
});

approval_string_enum!(AppInstallPurchaseApprovalDecisionAction {
    Approve => approval_string_values::DECISION_ACTION_APPROVE,
    Deny => approval_string_values::DECISION_ACTION_DENY,
    TimeBox => approval_string_values::DECISION_ACTION_TIME_BOX,
    ReviewNeeded => approval_string_values::DECISION_ACTION_REVIEW_NEEDED,
});

approval_string_enum!(AppInstallPurchaseApprovalState {
    PendingParentReview => approval_string_values::APPROVAL_STATE_PENDING_PARENT_REVIEW,
    Approved => approval_string_values::APPROVAL_STATE_APPROVED,
    Denied => approval_string_values::APPROVAL_STATE_DENIED,
    TimeBoxActive => approval_string_values::APPROVAL_STATE_TIME_BOX_ACTIVE,
    Expired => approval_string_values::APPROVAL_STATE_EXPIRED,
    ReviewNeeded => approval_string_values::APPROVAL_STATE_REVIEW_NEEDED,
});

approval_string_enum!(AppInstallPurchaseApprovalExpiryState {
    NotExpiring => approval_string_values::EXPIRY_STATE_NOT_EXPIRING,
    TimeBoxActive => approval_string_values::EXPIRY_STATE_TIME_BOX_ACTIVE,
    Expired => approval_string_values::EXPIRY_STATE_EXPIRED,
    ReviewNeeded => approval_string_values::EXPIRY_STATE_REVIEW_NEEDED,
});

approval_string_enum!(AppInstallPurchaseApprovalPurchaseKind {
    OneTimePurchase => approval_string_values::PURCHASE_KIND_ONE_TIME_PURCHASE,
    InAppPurchase => approval_string_values::PURCHASE_KIND_IN_APP_PURCHASE,
    Subscription => approval_string_values::PURCHASE_KIND_SUBSCRIPTION,
});

approval_string_enum!(AppInstallPurchaseApprovalSubscriptionPeriod {
    Weekly => approval_string_values::SUBSCRIPTION_PERIOD_WEEKLY,
    Monthly => approval_string_values::SUBSCRIPTION_PERIOD_MONTHLY,
    Annual => approval_string_values::SUBSCRIPTION_PERIOD_ANNUAL,
    Unknown => approval_string_values::SUBSCRIPTION_PERIOD_UNKNOWN,
});

approval_string_enum!(AppInstallPurchaseApprovalChildFacingStatus {
    PendingParentReviewVisible => approval_string_values::CHILD_FACING_STATUS_PENDING_PARENT_REVIEW_VISIBLE,
    ApprovedVisible => approval_string_values::CHILD_FACING_STATUS_APPROVED_VISIBLE,
    DeniedVisible => approval_string_values::CHILD_FACING_STATUS_DENIED_VISIBLE,
    TimeBoxVisible => approval_string_values::CHILD_FACING_STATUS_TIME_BOX_VISIBLE,
    ReviewNeededVisible => approval_string_values::CHILD_FACING_STATUS_REVIEW_NEEDED_VISIBLE,
});

approval_string_enum!(AppInstallPurchaseApprovalAuditReportSurface {
    RequestAuditHistory => approval_string_values::AUDIT_REPORT_SURFACE_REQUEST_AUDIT_HISTORY,
    ParentDecisionAuditHistory => approval_string_values::AUDIT_REPORT_SURFACE_PARENT_DECISION_AUDIT_HISTORY,
    ChildFacingStateReport => approval_string_values::AUDIT_REPORT_SURFACE_CHILD_FACING_STATE_REPORT,
    PlatformLimitationReport => approval_string_values::AUDIT_REPORT_SURFACE_PLATFORM_LIMITATION_REPORT,
});

approval_string_enum!(AppInstallPurchaseApprovalProofIntegrationState {
    ContractOnly => approval_string_values::PROOF_INTEGRATION_STATE_CONTRACT_ONLY,
    ManualRequired => approval_string_values::PROOF_INTEGRATION_STATE_MANUAL_REQUIRED,
    Unavailable => approval_string_values::PROOF_INTEGRATION_STATE_UNAVAILABLE,
});

approval_string_enum!(AppInstallPurchaseApprovalAuditEventKind {
    RequestRecorded => approval_string_values::AUDIT_EVENT_KIND_REQUEST_RECORDED,
    MetadataSourceEvaluated => approval_string_values::AUDIT_EVENT_KIND_METADATA_SOURCE_EVALUATED,
    ParentDecisionRecorded => approval_string_values::AUDIT_EVENT_KIND_PARENT_DECISION_RECORDED,
    ApprovalExpired => approval_string_values::AUDIT_EVENT_KIND_APPROVAL_EXPIRED,
    PlatformLimitationRecorded => approval_string_values::AUDIT_EVENT_KIND_PLATFORM_LIMITATION_RECORDED,
});

approval_string_enum!(AppInstallPurchaseApprovalNonClaim {
    NoStoreIntegration => approval_string_values::NON_CLAIM_NO_STORE_INTEGRATION,
    NoBillingEntitlementLogic => approval_string_values::NON_CLAIM_NO_BILLING_ENTITLEMENT_LOGIC,
    NoPortalUi => approval_string_values::NON_CLAIM_NO_PORTAL_UI,
    NoPlatformAdapter => approval_string_values::NON_CLAIM_NO_PLATFORM_ADAPTER,
    NoStorePolicyBypass => approval_string_values::NON_CLAIM_NO_STORE_POLICY_BYPASS,
    NoRealInstallOrPurchaseInterception => approval_string_values::NON_CLAIM_NO_REAL_INSTALL_OR_PURCHASE_INTERCEPTION,
    NotGenericAppBlocking => approval_string_values::NON_CLAIM_NOT_GENERIC_APP_BLOCKING,
});

approval_string_enum!(AppInstallPurchaseApprovalStoreIntegrationClaim {
    NotClaimed => approval_string_values::STORE_INTEGRATION_CLAIM_NOT_CLAIMED,
});

approval_string_enum!(AppInstallPurchaseApprovalBillingEntitlementClaim {
    NotClaimed => approval_string_values::BILLING_ENTITLEMENT_CLAIM_NOT_CLAIMED,
});

approval_string_enum!(AppInstallPurchaseApprovalPortalUiClaim {
    NotImplemented => approval_string_values::PORTAL_UI_CLAIM_NOT_IMPLEMENTED,
});

approval_string_enum!(AppInstallPurchaseApprovalPlatformAdapterClaim {
    NotImplemented => approval_string_values::PLATFORM_ADAPTER_CLAIM_NOT_IMPLEMENTED,
});

approval_string_enum!(AppInstallPurchaseApprovalInterceptionClaim {
    NotClaimed => approval_string_values::INTERCEPTION_CLAIM_NOT_CLAIMED,
});

approval_string_enum!(AppInstallPurchaseApprovalRuntimeBlockingSeparation {
    SeparateFromGenericAppBlocking => approval_string_values::RUNTIME_BLOCKING_SEPARATION_SEPARATE_FROM_GENERIC_APP_BLOCKING,
});

approval_string_enum!(AppInstallPurchaseApprovalPlatformSourceAuthority {
    GooglePlayListing => approval_string_values::PLATFORM_SOURCE_AUTHORITY_GOOGLE_PLAY_LISTING,
    AppleAppStoreListing => approval_string_values::PLATFORM_SOURCE_AUTHORITY_APPLE_APP_STORE_LISTING,
    MacAppStoreListing => approval_string_values::PLATFORM_SOURCE_AUTHORITY_MAC_APP_STORE_LISTING,
    MicrosoftStoreListing => approval_string_values::PLATFORM_SOURCE_AUTHORITY_MICROSOFT_STORE_LISTING,
    LinuxPackageManagerIndex => approval_string_values::PLATFORM_SOURCE_AUTHORITY_LINUX_PACKAGE_MANAGER_INDEX,
});

approval_string_enum!(AppInstallPurchaseApprovalPlatformSourceMetadataState {
    ContractOnly => approval_string_values::PLATFORM_SOURCE_METADATA_STATE_CONTRACT_ONLY,
    ManualRequired => approval_string_values::PLATFORM_SOURCE_METADATA_STATE_MANUAL_REQUIRED,
    Unavailable => approval_string_values::PLATFORM_SOURCE_METADATA_STATE_UNAVAILABLE,
});

approval_string_enum!(AppInstallPurchaseApprovalPlatformSourceEvidenceState {
    RequiresApprovedApiProof => approval_string_values::PLATFORM_SOURCE_EVIDENCE_STATE_REQUIRES_APPROVED_API_PROOF,
    RequiresStoreArtifactProof => approval_string_values::PLATFORM_SOURCE_EVIDENCE_STATE_REQUIRES_STORE_ARTIFACT_PROOF,
    PlatformUnavailable => approval_string_values::PLATFORM_SOURCE_EVIDENCE_STATE_PLATFORM_UNAVAILABLE,
});

approval_string_enum!(AppInstallPurchaseApprovalPlatformSourceMetadataField {
    StoreListingId => approval_string_values::PLATFORM_SOURCE_METADATA_FIELD_STORE_LISTING_ID,
    AppTitle => approval_string_values::PLATFORM_SOURCE_METADATA_FIELD_APP_TITLE,
    PublisherName => approval_string_values::PLATFORM_SOURCE_METADATA_FIELD_PUBLISHER_NAME,
    Category => approval_string_values::PLATFORM_SOURCE_METADATA_FIELD_CATEGORY,
    AgeRating => approval_string_values::PLATFORM_SOURCE_METADATA_FIELD_AGE_RATING,
    PriceDisplay => approval_string_values::PLATFORM_SOURCE_METADATA_FIELD_PRICE_DISPLAY,
    SubscriptionPeriod => approval_string_values::PLATFORM_SOURCE_METADATA_FIELD_SUBSCRIPTION_PERIOD,
    SourceUrl => approval_string_values::PLATFORM_SOURCE_METADATA_FIELD_SOURCE_URL,
});

approval_string_enum!(AppInstallPurchaseApprovalPlatformSourceManualFallback {
    ContractOnlyParentReview => approval_string_values::PLATFORM_SOURCE_MANUAL_FALLBACK_CONTRACT_ONLY_PARENT_REVIEW,
});

approval_string_enum!(AppInstallPurchaseApprovalPackageSourceArtifactStatus {
    ManualRequired => approval_string_values::PACKAGE_SOURCE_ARTIFACT_STATUS_MANUAL_REQUIRED,
    DeviceProofRequired => approval_string_values::PACKAGE_SOURCE_ARTIFACT_STATUS_DEVICE_PROOF_REQUIRED,
    Unavailable => approval_string_values::PACKAGE_SOURCE_ARTIFACT_STATUS_UNAVAILABLE,
});

approval_string_enum!(AppInstallPurchaseApprovalPackageSourceApprovalPathState {
    ManualRequired => approval_string_values::PACKAGE_SOURCE_APPROVAL_PATH_STATE_MANUAL_REQUIRED,
    Unavailable => approval_string_values::PACKAGE_SOURCE_APPROVAL_PATH_STATE_UNAVAILABLE,
});

approval_string_enum!(AppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaim {
    NotAttached => approval_string_values::PACKAGE_SOURCE_ARTIFACT_EVIDENCE_CLAIM_NOT_ATTACHED,
});

approval_string_enum!(AppInstallPurchaseApprovalPackageSourceField {
    PackageIdentifier => approval_string_values::PACKAGE_SOURCE_FIELD_PACKAGE_IDENTIFIER,
    InstallerSource => approval_string_values::PACKAGE_SOURCE_FIELD_INSTALLER_SOURCE,
    PublisherOrDeveloper => approval_string_values::PACKAGE_SOURCE_FIELD_PUBLISHER_OR_DEVELOPER,
    VersionOrBuild => approval_string_values::PACKAGE_SOURCE_FIELD_VERSION_OR_BUILD,
    SignatureOrReceipt => approval_string_values::PACKAGE_SOURCE_FIELD_SIGNATURE_OR_RECEIPT,
    SourceCapturedAt => approval_string_values::PACKAGE_SOURCE_FIELD_SOURCE_CAPTURED_AT,
});

approval_string_enum!(AppInstallPurchaseApprovalPackageSourceKind {
    WindowsStorePackageIdentity => approval_string_values::PACKAGE_SOURCE_KIND_WINDOWS_STORE_PACKAGE_IDENTITY,
    MacosBundleReceipt => approval_string_values::PACKAGE_SOURCE_KIND_MACOS_BUNDLE_RECEIPT,
    LinuxPackageManagerRecord => approval_string_values::PACKAGE_SOURCE_KIND_LINUX_PACKAGE_MANAGER_RECORD,
    AndroidPackageSourceRecord => approval_string_values::PACKAGE_SOURCE_KIND_ANDROID_PACKAGE_SOURCE_RECORD,
    IosAppSourceRecord => approval_string_values::PACKAGE_SOURCE_KIND_IOS_APP_SOURCE_RECORD,
});

approval_string_enum!(AppInstallPurchaseApprovalPackageSourceChildDataCustody {
    NoChildActivityData => approval_string_values::PACKAGE_SOURCE_CHILD_DATA_CUSTODY_NO_CHILD_ACTIVITY_DATA,
});

approval_text_identifier!(FamilyId);
approval_text_identifier!(ChildProfileId);
approval_text_identifier!(ChildProfileDisplayName);
approval_text_identifier!(ParentDeviceId);
approval_text_identifier!(ParentDeviceLabel);
approval_text_identifier!(ParentActorId);
approval_text_identifier!(ParentPolicyVersion);
approval_text_identifier!(ParentEvidenceReferenceId);
approval_text_identifier!(ParentActionReferenceId);
approval_text_identifier!(ParentTimestamp);
approval_text_identifier!(AppInstallPurchaseApprovalRequestId);
approval_text_identifier!(AppInstallPurchaseApprovalDecisionId);
approval_text_identifier!(AppInstallPurchaseApprovalAuditEventId);
approval_text_identifier!(AppInstallPurchaseApprovalStoreListingId);
approval_text_identifier!(AppInstallPurchaseApprovalAppTitle);
approval_text_identifier!(AppInstallPurchaseApprovalPublisherName);
approval_text_identifier!(AppInstallPurchaseApprovalCategory);
approval_text_identifier!(AppInstallPurchaseApprovalAgeRating);
approval_text_identifier!(AppInstallPurchaseApprovalReviewReason);
approval_text_identifier!(AppInstallPurchaseApprovalProofRequirement);
approval_text_identifier!(AppInstallPurchaseApprovalUnavailableReason);
approval_text_identifier!(AppInstallPurchaseApprovalManualRequirement);
approval_text_identifier!(AppInstallPurchaseApprovalClaimBoundary);
approval_text_identifier!(AppInstallPurchaseApprovalPriceDisplay);
approval_text_identifier!(AppInstallPurchaseApprovalChildStateId);
approval_text_identifier!(AppInstallPurchaseApprovalReportRef);
approval_text_identifier!(AppInstallPurchaseApprovalPlatformSourceRowId);
approval_text_identifier!(AppInstallPurchaseApprovalPlatformSourceArtifactRequirement);
approval_text_identifier!(AppInstallPurchaseApprovalPlatformSourceLimitationReason);
approval_text_identifier!(AppInstallPurchaseApprovalPlatformSourceReportRef);
approval_text_identifier!(AppInstallPurchaseApprovalPlatformSourceClaimBoundary);
approval_text_identifier!(AppInstallPurchaseApprovalPackageSourceArtifactRowId);
approval_text_identifier!(AppInstallPurchaseApprovalPackageSourceMetadataRowId);
approval_text_identifier!(AppInstallPurchaseApprovalPackageSourceArtifactRequirement);
approval_text_identifier!(AppInstallPurchaseApprovalPackageSourceLimitationReason);
approval_text_identifier!(AppInstallPurchaseApprovalPackageSourceReportRef);
approval_text_identifier!(AppInstallPurchaseApprovalPackageSourceClaimBoundary);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActorReference {
    pub actor_id: ParentActorId,
    pub role: ParentActorRole,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyReference {
    pub family_id: FamilyId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildProfileReference {
    pub child_profile_id: ChildProfileId,
    pub display_name: ChildProfileDisplayName,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentDeviceReference {
    pub device_id: ParentDeviceId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_profile_id: Option<ChildProfileId>,
    pub label: ParentDeviceLabel,
    pub platform: ParentPlatform,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentEvidenceReference {
    pub evidence_reference_id: ParentEvidenceReferenceId,
    pub kind: ParentEvidenceReferenceKind,
    pub observed_at: ParentTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActionReference {
    pub action_reference_id: ParentActionReferenceId,
    pub actor: ParentActorReference,
    pub policy_version: ParentPolicyVersion,
    pub created_at: ParentTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalAuditEventRef {
    pub audit_event_id: AppInstallPurchaseApprovalAuditEventId,
    pub event_kind: AppInstallPurchaseApprovalAuditEventKind,
    pub recorded_at: ParentTimestamp,
    pub evidence_references: Vec<ParentEvidenceReference>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalStoreMetadata {
    pub store_surface: AppInstallPurchaseApprovalStoreSurface,
    pub source_state: AppInstallPurchaseApprovalSupportState,
    pub freshness: AppInstallPurchaseApprovalStoreMetadataFreshness,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_id: Option<AppInstallPurchaseApprovalStoreListingId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_title: Option<AppInstallPurchaseApprovalAppTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<AppInstallPurchaseApprovalPublisherName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<AppInstallPurchaseApprovalCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_rating: Option<AppInstallPurchaseApprovalAgeRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_at: Option<ParentTimestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_at: Option<ParentTimestamp>,
    pub proof_requirement: AppInstallPurchaseApprovalProofRequirement,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalStateSnapshot {
    pub state: AppInstallPurchaseApprovalState,
    pub expiry_state: AppInstallPurchaseApprovalExpiryState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<ParentTimestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_reason: Option<AppInstallPurchaseApprovalReviewReason>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallRequest {
    pub schema_version: String,
    pub request_id: AppInstallPurchaseApprovalRequestId,
    pub request_kind: AppInstallPurchaseApprovalRequestKind,
    pub family: FamilyReference,
    pub child: ChildProfileReference,
    pub device: ParentDeviceReference,
    pub platform: ParentPlatform,
    pub store_metadata: AppInstallPurchaseApprovalStoreMetadata,
    pub approval_state: AppInstallPurchaseApprovalStateSnapshot,
    pub requested_at: ParentTimestamp,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub audit_event_refs: Vec<AppInstallPurchaseApprovalAuditEventRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseRequest {
    pub schema_version: String,
    pub request_id: AppInstallPurchaseApprovalRequestId,
    pub request_kind: AppInstallPurchaseApprovalRequestKind,
    pub family: FamilyReference,
    pub child: ChildProfileReference,
    pub device: ParentDeviceReference,
    pub platform: ParentPlatform,
    pub store_metadata: AppInstallPurchaseApprovalStoreMetadata,
    pub approval_state: AppInstallPurchaseApprovalStateSnapshot,
    pub requested_at: ParentTimestamp,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub audit_event_refs: Vec<AppInstallPurchaseApprovalAuditEventRef>,
    pub purchase_kind: AppInstallPurchaseApprovalPurchaseKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<AppInstallPurchaseApprovalSubscriptionPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_display: Option<AppInstallPurchaseApprovalPriceDisplay>,
    pub billing_entitlement_claim: AppInstallPurchaseApprovalBillingEntitlementClaim,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalDecision {
    pub schema_version: String,
    pub decision_id: AppInstallPurchaseApprovalDecisionId,
    pub request_id: AppInstallPurchaseApprovalRequestId,
    pub request_kind: AppInstallPurchaseApprovalRequestKind,
    pub decision_action: AppInstallPurchaseApprovalDecisionAction,
    pub resulting_state: AppInstallPurchaseApprovalStateSnapshot,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_action: Option<ParentActionReference>,
    pub decided_at: ParentTimestamp,
    pub audit_event_refs: Vec<AppInstallPurchaseApprovalAuditEventRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalChildFacingState {
    pub schema_version: String,
    pub child_state_id: AppInstallPurchaseApprovalChildStateId,
    pub request_id: AppInstallPurchaseApprovalRequestId,
    pub request_kind: AppInstallPurchaseApprovalRequestKind,
    pub platform: ParentPlatform,
    pub child_visible_status: AppInstallPurchaseApprovalChildFacingStatus,
    pub source_approval_state: AppInstallPurchaseApprovalStateSnapshot,
    pub delivery_state: AppInstallPurchaseApprovalSupportState,
    pub delivery_requirement: AppInstallPurchaseApprovalProofRequirement,
    pub audit_event_refs: Vec<AppInstallPurchaseApprovalAuditEventRef>,
    pub report_refs: Vec<AppInstallPurchaseApprovalReportRef>,
    pub claim_boundary: AppInstallPurchaseApprovalClaimBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalAuditReportIntegration {
    pub schema_version: String,
    pub surface: AppInstallPurchaseApprovalAuditReportSurface,
    pub integration_state: AppInstallPurchaseApprovalProofIntegrationState,
    pub audit_event_refs: Vec<AppInstallPurchaseApprovalAuditEventRef>,
    pub report_refs: Vec<AppInstallPurchaseApprovalReportRef>,
    pub proof_requirement: AppInstallPurchaseApprovalProofRequirement,
    pub claim_boundary: AppInstallPurchaseApprovalClaimBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalPlatformSupportRow {
    pub platform: ParentPlatform,
    pub store_surface: AppInstallPurchaseApprovalStoreSurface,
    pub contract_request_state: AppInstallPurchaseApprovalSupportState,
    pub store_metadata_state: AppInstallPurchaseApprovalSupportState,
    pub install_interception_state: AppInstallPurchaseApprovalSupportState,
    pub purchase_interception_state: AppInstallPurchaseApprovalSupportState,
    pub subscription_interception_state: AppInstallPurchaseApprovalSupportState,
    pub child_pending_state: AppInstallPurchaseApprovalSupportState,
    pub approval_delivery_state: AppInstallPurchaseApprovalSupportState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_requirement: Option<AppInstallPurchaseApprovalManualRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<AppInstallPurchaseApprovalUnavailableReason>,
    pub proof_requirement: AppInstallPurchaseApprovalProofRequirement,
    pub claim_boundary: AppInstallPurchaseApprovalClaimBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalPlatformSourceMetadataRow {
    pub schema_version: String,
    pub source_row_id: AppInstallPurchaseApprovalPlatformSourceRowId,
    pub platform: ParentPlatform,
    pub store_surface: AppInstallPurchaseApprovalStoreSurface,
    pub source_authority: AppInstallPurchaseApprovalPlatformSourceAuthority,
    pub metadata_state: AppInstallPurchaseApprovalPlatformSourceMetadataState,
    pub source_evidence_state: AppInstallPurchaseApprovalPlatformSourceEvidenceState,
    pub fields_available_from_contract: Vec<AppInstallPurchaseApprovalPlatformSourceMetadataField>,
    pub fields_requiring_platform_proof: Vec<AppInstallPurchaseApprovalPlatformSourceMetadataField>,
    pub request_kind_coverage: Vec<AppInstallPurchaseApprovalRequestKind>,
    pub required_artifacts: Vec<AppInstallPurchaseApprovalPlatformSourceArtifactRequirement>,
    pub limitation_reason: AppInstallPurchaseApprovalPlatformSourceLimitationReason,
    pub limitation_report_ref: AppInstallPurchaseApprovalPlatformSourceReportRef,
    pub parent_manual_fallback: AppInstallPurchaseApprovalPlatformSourceManualFallback,
    pub store_integration_claim: AppInstallPurchaseApprovalStoreIntegrationClaim,
    pub platform_adapter_claim: AppInstallPurchaseApprovalPlatformAdapterClaim,
    pub interception_claim: AppInstallPurchaseApprovalInterceptionClaim,
    pub claim_boundary: AppInstallPurchaseApprovalPlatformSourceClaimBoundary,
    pub last_checked_at: ParentTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalPackageSourceArtifactRow {
    pub schema_version: String,
    pub artifact_row_id: AppInstallPurchaseApprovalPackageSourceArtifactRowId,
    pub platform: ParentPlatform,
    pub store_surface: AppInstallPurchaseApprovalStoreSurface,
    pub platform_source_row_id: AppInstallPurchaseApprovalPackageSourceMetadataRowId,
    pub package_source_kind: AppInstallPurchaseApprovalPackageSourceKind,
    pub artifact_status: AppInstallPurchaseApprovalPackageSourceArtifactStatus,
    pub approval_path_state: AppInstallPurchaseApprovalPackageSourceApprovalPathState,
    pub package_source_fields_required: Vec<AppInstallPurchaseApprovalPackageSourceField>,
    pub package_source_fields_attached: Vec<AppInstallPurchaseApprovalPackageSourceField>,
    pub request_kind_coverage: Vec<AppInstallPurchaseApprovalRequestKind>,
    pub required_artifacts: Vec<AppInstallPurchaseApprovalPackageSourceArtifactRequirement>,
    pub artifact_evidence_claim: AppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaim,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_evidence_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_captured_at: Option<ParentTimestamp>,
    pub limitation_reason: AppInstallPurchaseApprovalPackageSourceLimitationReason,
    pub limitation_report_ref: AppInstallPurchaseApprovalPackageSourceReportRef,
    pub store_integration_claim: AppInstallPurchaseApprovalStoreIntegrationClaim,
    pub platform_adapter_claim: AppInstallPurchaseApprovalPlatformAdapterClaim,
    pub interception_claim: AppInstallPurchaseApprovalInterceptionClaim,
    pub child_data_custody: AppInstallPurchaseApprovalPackageSourceChildDataCustody,
    pub claim_boundary: AppInstallPurchaseApprovalPackageSourceClaimBoundary,
    pub last_checked_at: ParentTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallPurchaseApprovalContractProof {
    pub schema_version: String,
    pub install_request: AppInstallRequest,
    pub purchase_request: PurchaseRequest,
    pub subscription_request: PurchaseRequest,
    pub approval_decisions: Vec<AppInstallPurchaseApprovalDecision>,
    pub platform_support_matrix: Vec<AppInstallPurchaseApprovalPlatformSupportRow>,
    pub platform_source_metadata: Vec<AppInstallPurchaseApprovalPlatformSourceMetadataRow>,
    pub package_source_artifacts: Vec<AppInstallPurchaseApprovalPackageSourceArtifactRow>,
    pub child_facing_states: Vec<AppInstallPurchaseApprovalChildFacingState>,
    pub audit_report_integration: Vec<AppInstallPurchaseApprovalAuditReportIntegration>,
    pub non_claims: Vec<AppInstallPurchaseApprovalNonClaim>,
    pub store_integration_claim: AppInstallPurchaseApprovalStoreIntegrationClaim,
    pub billing_entitlement_claim: AppInstallPurchaseApprovalBillingEntitlementClaim,
    pub portal_ui_claim: AppInstallPurchaseApprovalPortalUiClaim,
    pub platform_adapter_claim: AppInstallPurchaseApprovalPlatformAdapterClaim,
    pub interception_claim: AppInstallPurchaseApprovalInterceptionClaim,
    pub runtime_blocking_separation: AppInstallPurchaseApprovalRuntimeBlockingSeparation,
    pub updated_at: ParentTimestamp,
}
