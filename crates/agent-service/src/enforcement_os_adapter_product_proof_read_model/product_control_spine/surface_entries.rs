use ocentra_parent_agent_protocol::constants::v08_browser_domain_adapter_proof as browser_proof;
use ocentra_parent_agent_protocol::constants::v08_cross_platform_enforcement_capability_proof as cross_proof;
use ocentra_parent_agent_protocol::constants::v08_enforcement_product_control_spine as spine;
use ocentra_parent_agent_protocol::constants::v08_os_adapter_product_proof as os_proof;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofReadModel;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformAdapterExecutionState;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformCapabilityStatus;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityClaimState;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofEntry;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofReadModel;
use ocentra_parent_agent_protocol::enforcement_os_adapter_product_proof::V08OsAdapterProductProofReadModel;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlCapabilityName;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlCapabilityStatus;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlClaimState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlDevicePolicyState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlExecutionState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlParentAction;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSpineEntry;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSurface;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSurfaceKind;

use super::{
    entry_factory::{
        linked_entry, linked_with_manual_entry, manual_entry, product_entry, LinkedEntrySpec,
        LinkedManualEntrySpec, ManualEntrySpec, ProductEntrySpec,
    },
    proof_links::{expect_browser, expect_cross, expect_os},
    GeneratedAtText, ProofEntryId,
};

pub(super) fn entries(
    cross_platform: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    browser_domain: &V08BrowserDomainAdapterProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> Vec<V08EnforcementProductControlSpineEntry> {
    vec![
        owned_process_entry(cross_platform, os_product, generated_at),
        app_time_limit_entry(cross_platform, os_product, generated_at),
        managed_browser_entry(cross_platform, browser_domain, generated_at),
        unmanaged_browser_entry(cross_platform, browser_domain, os_product, generated_at),
        policy_dry_run_entry(generated_at),
        approval_override_entry(generated_at),
        restart_recovery_entry(browser_domain, os_product, generated_at),
        rollback_audit_entry(browser_domain, os_product, generated_at),
        child_explanation_entry(generated_at),
        broad_app_entry(cross_platform, os_product, generated_at),
        network_domain_entry(cross_platform, browser_domain, os_product, generated_at),
        managed_exact_url_entry(browser_domain, os_product, generated_at),
        unmanaged_exact_url_entry(browser_domain, os_product, generated_at),
        permission_loss_entry(generated_at),
        tamper_uninstall_entry(generated_at),
    ]
}

fn owned_process_entry(
    cross_platform: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    expect_cross(
        cross_platform,
        ProofEntryId(cross_proof::ENTRY_ID_WINDOWS_OWNED_PROCESS),
    );
    expect_os(
        os_product,
        ProofEntryId(os_proof::ENTRY_ID_OWNED_PROCESS_TERMINATE),
    );
    linked_entry(LinkedEntrySpec {
        entry_id: spine::ENTRY_ID_OWNED_PROCESS,
        surface: V08EnforcementProductControlSurface::WindowsOwnedProcessTimeLimit,
        surface_kind: V08EnforcementProductControlSurfaceKind::Process,
        capability: V08EnforcementProductControlCapabilityName::OwnedProcessTerminate,
        product_claim_state: V08EnforcementProductControlClaimState::ImplementedBoundary,
        adapter_execution_state: V08EnforcementProductControlExecutionState::ExecutesRealService,
        device_policy_state: V08EnforcementProductControlDevicePolicyState::ControlCapable,
        parent_visible_actions: &[
            V08EnforcementProductControlParentAction::Observe,
            V08EnforcementProductControlParentAction::TimeLimit,
            V08EnforcementProductControlParentAction::BlockScopedProcess,
        ],
        linked_proof_commands: &[spine::COMMAND_WINDOWS_UNMANAGED_PROOF],
        linked_proof_artifacts: &[spine::ARTIFACT_WINDOWS_UNMANAGED_PROOF],
        claim_boundary: os_proof::CLAIM_OWNED_PROCESS,
        fallback_behavior: os_proof::FALLBACK_OWNED_PROCESS,
        generated_at: generated_at.0.as_str(),
    })
}

fn app_time_limit_entry(
    cross_platform: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    let cross_entry = expect_cross(
        cross_platform,
        ProofEntryId(cross_proof::ENTRY_ID_WINDOWS_APP_TIME_LIMIT),
    );
    expect_os(
        os_product,
        ProofEntryId(os_proof::ENTRY_ID_APP_TIME_LIMIT_LIFECYCLE),
    );
    let (capability_status, product_claim_state, adapter_execution_state, device_policy_state) =
        product_states_from_cross_entry(cross_entry);
    let parent_visible_actions = if product_claim_state
        == V08EnforcementProductControlClaimState::ImplementedBoundary
        && adapter_execution_state
            == V08EnforcementProductControlExecutionState::ExecutesRealService
    {
        vec![
            V08EnforcementProductControlParentAction::Observe,
            V08EnforcementProductControlParentAction::TimeLimit,
            V08EnforcementProductControlParentAction::AskParent,
        ]
    } else {
        vec![V08EnforcementProductControlParentAction::ReportOnly]
    };

    V08EnforcementProductControlSpineEntry {
        schema_version: cross_entry.schema_version.clone(),
        entry_id: spine::ENTRY_ID_APP_TIME_LIMIT.to_string(),
        surface: V08EnforcementProductControlSurface::WindowsAppTimeLimitLifecycle,
        surface_kind: V08EnforcementProductControlSurfaceKind::AppGame,
        platform: cross_entry.platform,
        capability: V08EnforcementProductControlCapabilityName::AppTimeLimit,
        capability_status,
        product_claim_state,
        adapter_execution_state,
        device_policy_state,
        parent_visible_actions,
        linked_proof_commands: cross_entry.linked_proof_commands.clone(),
        linked_proof_artifacts: cross_entry.linked_proof_artifacts.clone(),
        manual_proof_requirements: cross_entry.manual_proof_requirements.clone(),
        claim_boundary: cross_entry.claim_boundary.clone(),
        fallback_behavior: cross_entry.fallback_behavior.clone(),
        broad_app_blocking_claimed: false,
        network_domain_blocking_claimed: false,
        managed_exact_url_blocking_claimed: false,
        unmanaged_exact_url_claimed: false,
        tamper_resistance_claimed: false,
        notification_delivery_claimed: false,
        last_checked_at: generated_at.0.to_string(),
    }
}

fn product_states_from_cross_entry(
    cross_entry: &V08CrossPlatformEnforcementCapabilityProofEntry,
) -> (
    V08EnforcementProductControlCapabilityStatus,
    V08EnforcementProductControlClaimState,
    V08EnforcementProductControlExecutionState,
    V08EnforcementProductControlDevicePolicyState,
) {
    (
        product_capability_status(cross_entry.capability_status),
        product_claim_state(cross_entry.product_claim_state),
        product_execution_state(cross_entry.adapter_execution_state),
        product_device_policy_state(cross_entry.product_claim_state),
    )
}

fn product_capability_status(
    status: V08CrossPlatformCapabilityStatus,
) -> V08EnforcementProductControlCapabilityStatus {
    match status {
        V08CrossPlatformCapabilityStatus::Implemented =>
            V08EnforcementProductControlCapabilityStatus::Implemented,
        V08CrossPlatformCapabilityStatus::ManualRequired =>
            V08EnforcementProductControlCapabilityStatus::ManualRequired,
        V08CrossPlatformCapabilityStatus::Supported
        | V08CrossPlatformCapabilityStatus::PreviewScaffold
        | V08CrossPlatformCapabilityStatus::Scaffold
        | V08CrossPlatformCapabilityStatus::Unavailable
        | V08CrossPlatformCapabilityStatus::Planned
        | V08CrossPlatformCapabilityStatus::NotImplemented =>
            V08EnforcementProductControlCapabilityStatus::NotImplemented,
    }
}

fn product_claim_state(
    state: V08CrossPlatformEnforcementCapabilityClaimState,
) -> V08EnforcementProductControlClaimState {
    match state {
        V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary =>
            V08EnforcementProductControlClaimState::ImplementedBoundary,
        V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired =>
            V08EnforcementProductControlClaimState::ManualRequired,
        V08CrossPlatformEnforcementCapabilityClaimState::Scaffold
        | V08CrossPlatformEnforcementCapabilityClaimState::Unavailable =>
            V08EnforcementProductControlClaimState::Unavailable,
        V08CrossPlatformEnforcementCapabilityClaimState::Planned
        | V08CrossPlatformEnforcementCapabilityClaimState::NotClaimed =>
            V08EnforcementProductControlClaimState::NotClaimed,
    }
}

fn product_execution_state(
    state: V08CrossPlatformAdapterExecutionState,
) -> V08EnforcementProductControlExecutionState {
    match state {
        V08CrossPlatformAdapterExecutionState::ExecutesRealService =>
            V08EnforcementProductControlExecutionState::ExecutesRealService,
        V08CrossPlatformAdapterExecutionState::ReturnsManualRequired =>
            V08EnforcementProductControlExecutionState::ReturnsManualRequired,
        V08CrossPlatformAdapterExecutionState::ReturnsUnavailable =>
            V08EnforcementProductControlExecutionState::ReturnsUnavailable,
        V08CrossPlatformAdapterExecutionState::ScaffoldOnly
        | V08CrossPlatformAdapterExecutionState::NotInvoked =>
            V08EnforcementProductControlExecutionState::NotInvoked,
    }
}

fn product_device_policy_state(
    state: V08CrossPlatformEnforcementCapabilityClaimState,
) -> V08EnforcementProductControlDevicePolicyState {
    match state {
        V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary =>
            V08EnforcementProductControlDevicePolicyState::ControlCapable,
        V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired =>
            V08EnforcementProductControlDevicePolicyState::ManualRequired,
        V08CrossPlatformEnforcementCapabilityClaimState::Scaffold
        | V08CrossPlatformEnforcementCapabilityClaimState::Unavailable =>
            V08EnforcementProductControlDevicePolicyState::Unavailable,
        V08CrossPlatformEnforcementCapabilityClaimState::Planned
        | V08CrossPlatformEnforcementCapabilityClaimState::NotClaimed =>
            V08EnforcementProductControlDevicePolicyState::NotClaimed,
    }
}

fn managed_browser_entry(
    cross_platform: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    browser_domain: &V08BrowserDomainAdapterProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    expect_cross(
        cross_platform,
        ProofEntryId(cross_proof::ENTRY_ID_WINDOWS_MANAGED_BROWSER),
    );
    expect_browser(
        browser_domain,
        ProofEntryId(browser_proof::ENTRY_ID_MANAGED_INTERVENTION),
    );
    manual_entry(ManualEntrySpec {
        entry_id: spine::ENTRY_ID_MANAGED_BROWSER_SESSION,
        surface: V08EnforcementProductControlSurface::WindowsManagedBrowserSessionIntervention,
        surface_kind: V08EnforcementProductControlSurfaceKind::ManagedBrowser,
        capability: V08EnforcementProductControlCapabilityName::ManagedBrowserControl,
        manual_proof_requirements: &[
            browser_proof::REQUIREMENT_MANAGED_PROFILE,
            browser_proof::REQUIREMENT_ACTIVE_TAB,
            browser_proof::REQUIREMENT_ROLLBACK,
            browser_proof::REQUIREMENT_AUDIT_CUSTODY,
        ],
        claim_boundary: browser_proof::CLAIM_MANAGED_INTERVENTION,
        fallback_behavior: browser_proof::FALLBACK_MANAGED_INTERVENTION,
        generated_at: generated_at.0.as_str(),
    })
}

fn unmanaged_browser_entry(
    cross_platform: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    browser_domain: &V08BrowserDomainAdapterProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    expect_cross(
        cross_platform,
        ProofEntryId(cross_proof::ENTRY_ID_WINDOWS_UNMANAGED_BROWSER),
    );
    expect_browser(
        browser_domain,
        ProofEntryId(browser_proof::ENTRY_ID_UNMANAGED_WARN),
    );
    expect_os(
        os_product,
        ProofEntryId(os_proof::ENTRY_ID_UNMANAGED_BROWSER_PROCESS_ONLY),
    );
    linked_with_manual_entry(LinkedManualEntrySpec {
        entry_id: spine::ENTRY_ID_UNMANAGED_BROWSER_PROCESS,
        surface: V08EnforcementProductControlSurface::WindowsUnmanagedBrowserProcessFallback,
        surface_kind: V08EnforcementProductControlSurfaceKind::UnmanagedBrowser,
        capability: V08EnforcementProductControlCapabilityName::UnmanagedBrowserDetection,
        capability_status: V08EnforcementProductControlCapabilityStatus::Implemented,
        product_claim_state: V08EnforcementProductControlClaimState::DegradedBoundary,
        adapter_execution_state: V08EnforcementProductControlExecutionState::ReturnsDegradedNoop,
        device_policy_state: V08EnforcementProductControlDevicePolicyState::ReportOnly,
        parent_visible_actions: &[
            V08EnforcementProductControlParentAction::Observe,
            V08EnforcementProductControlParentAction::Warn,
            V08EnforcementProductControlParentAction::ReportOnly,
        ],
        linked_proof_commands: &[spine::COMMAND_WINDOWS_UNMANAGED_PROOF],
        linked_proof_artifacts: &[spine::ARTIFACT_WINDOWS_UNMANAGED_PROOF],
        manual_proof_requirements: &[
            browser_proof::REQUIREMENT_WARNING_DELIVERY,
            browser_proof::REQUIREMENT_BROWSER_INTEGRATION,
        ],
        claim_boundary: browser_proof::CLAIM_UNMANAGED_WARN,
        fallback_behavior: browser_proof::FALLBACK_UNMANAGED_WARN,
        generated_at: generated_at.0.as_str(),
    })
}

fn policy_dry_run_entry(generated_at: &GeneratedAtText) -> V08EnforcementProductControlSpineEntry {
    linked_entry(LinkedEntrySpec {
        entry_id: spine::ENTRY_ID_POLICY_DRY_RUN,
        surface: V08EnforcementProductControlSurface::WindowsPolicyDryRunPreview,
        surface_kind: V08EnforcementProductControlSurfaceKind::Policy,
        capability: V08EnforcementProductControlCapabilityName::TypedProtocolBridge,
        product_claim_state: V08EnforcementProductControlClaimState::DryRunOnly,
        adapter_execution_state: V08EnforcementProductControlExecutionState::ReturnsDryRunPreview,
        device_policy_state: V08EnforcementProductControlDevicePolicyState::PreviewOnly,
        parent_visible_actions: &[
            V08EnforcementProductControlParentAction::DryRunPreview,
            V08EnforcementProductControlParentAction::AskParent,
        ],
        linked_proof_commands: &[spine::COMMAND_POLICY_PREVIEW],
        linked_proof_artifacts: &[spine::ARTIFACT_POLICY_PREVIEW],
        claim_boundary: spine::CLAIM_POLICY_DRY_RUN,
        fallback_behavior: spine::FALLBACK_POLICY_DRY_RUN,
        generated_at: generated_at.0.as_str(),
    })
}

fn approval_override_entry(
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    linked_entry(LinkedEntrySpec {
        entry_id: spine::ENTRY_ID_APPROVAL_OVERRIDE,
        surface: V08EnforcementProductControlSurface::WindowsApprovalOverrideAudit,
        surface_kind: V08EnforcementProductControlSurfaceKind::Policy,
        capability: V08EnforcementProductControlCapabilityName::TypedProtocolBridge,
        product_claim_state: V08EnforcementProductControlClaimState::ImplementedBoundary,
        adapter_execution_state: V08EnforcementProductControlExecutionState::ExecutesRealService,
        device_policy_state: V08EnforcementProductControlDevicePolicyState::ControlCapable,
        parent_visible_actions: &[
            V08EnforcementProductControlParentAction::AskParent,
            V08EnforcementProductControlParentAction::ReportOnly,
        ],
        linked_proof_commands: &[spine::COMMAND_PROTOCOL_ENFORCEMENT],
        linked_proof_artifacts: &[spine::ARTIFACT_PRODUCT_PROOF],
        claim_boundary: spine::CLAIM_APPROVAL_OVERRIDE,
        fallback_behavior: spine::FALLBACK_APPROVAL_OVERRIDE,
        generated_at: generated_at.0.as_str(),
    })
}

fn restart_recovery_entry(
    browser_domain: &V08BrowserDomainAdapterProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    expect_browser(
        browser_domain,
        ProofEntryId(browser_proof::ENTRY_ID_RESTART_RECOVERY),
    );
    expect_os(
        os_product,
        ProofEntryId(os_proof::ENTRY_ID_RESTART_RECOVERY),
    );
    linked_entry(LinkedEntrySpec {
        entry_id: spine::ENTRY_ID_RESTART_RECOVERY,
        surface: V08EnforcementProductControlSurface::WindowsRestartRecoveryTimer,
        surface_kind: V08EnforcementProductControlSurfaceKind::Recovery,
        capability: V08EnforcementProductControlCapabilityName::AppTimeLimit,
        product_claim_state: V08EnforcementProductControlClaimState::ImplementedBoundary,
        adapter_execution_state: V08EnforcementProductControlExecutionState::ExecutesRealService,
        device_policy_state: V08EnforcementProductControlDevicePolicyState::ControlCapable,
        parent_visible_actions: &[
            V08EnforcementProductControlParentAction::TimeLimit,
            V08EnforcementProductControlParentAction::ReportOnly,
        ],
        linked_proof_commands: &[spine::COMMAND_WINDOWS_TIMER_PROOF],
        linked_proof_artifacts: &[spine::ARTIFACT_WINDOWS_TIMER_PROOF],
        claim_boundary: os_proof::CLAIM_RESTART_RECOVERY,
        fallback_behavior: os_proof::FALLBACK_RESTART_RECOVERY,
        generated_at: generated_at.0.as_str(),
    })
}

fn rollback_audit_entry(
    browser_domain: &V08BrowserDomainAdapterProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    expect_browser(
        browser_domain,
        ProofEntryId(browser_proof::ENTRY_ID_BROWSER_POLICY_ROLLBACK),
    );
    expect_os(os_product, ProofEntryId(os_proof::ENTRY_ID_AUDIT_CUSTODY));
    linked_entry(LinkedEntrySpec {
        entry_id: spine::ENTRY_ID_ROLLBACK_AUDIT,
        surface: V08EnforcementProductControlSurface::WindowsRollbackAuditBoundary,
        surface_kind: V08EnforcementProductControlSurfaceKind::Audit,
        capability: V08EnforcementProductControlCapabilityName::TypedProtocolBridge,
        product_claim_state: V08EnforcementProductControlClaimState::ImplementedBoundary,
        adapter_execution_state: V08EnforcementProductControlExecutionState::ExecutesRealService,
        device_policy_state: V08EnforcementProductControlDevicePolicyState::ReportOnly,
        parent_visible_actions: &[V08EnforcementProductControlParentAction::ReportOnly],
        linked_proof_commands: &[spine::COMMAND_BROWSER_POLICY_ROLLBACK],
        linked_proof_artifacts: &[spine::ARTIFACT_BROWSER_DOMAIN_PROOF],
        claim_boundary: browser_proof::CLAIM_BROWSER_POLICY_ROLLBACK,
        fallback_behavior: browser_proof::FALLBACK_BROWSER_POLICY_ROLLBACK,
        generated_at: generated_at.0.as_str(),
    })
}

fn child_explanation_entry(
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    manual_entry(ManualEntrySpec {
        entry_id: spine::ENTRY_ID_CHILD_EXPLANATION,
        surface: V08EnforcementProductControlSurface::WindowsChildFacingExplanation,
        surface_kind: V08EnforcementProductControlSurfaceKind::ChildExplanation,
        capability: V08EnforcementProductControlCapabilityName::TypedProtocolBridge,
        manual_proof_requirements: &[
            spine::REQUIREMENT_CHILD_EXPLANATION_COPY,
            spine::REQUIREMENT_CHILD_EXPLANATION_DELIVERY,
            spine::REQUIREMENT_CHILD_EXPLANATION_AUDIT,
        ],
        claim_boundary: spine::CLAIM_CHILD_EXPLANATION,
        fallback_behavior: spine::FALLBACK_CHILD_EXPLANATION,
        generated_at: generated_at.0.as_str(),
    })
}

fn broad_app_entry(
    cross_platform: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    expect_cross(
        cross_platform,
        ProofEntryId(cross_proof::ENTRY_ID_WINDOWS_BROAD_APP),
    );
    expect_os(
        os_product,
        ProofEntryId(os_proof::ENTRY_ID_BROAD_APP_BLOCKING),
    );
    manual_entry(ManualEntrySpec {
        entry_id: spine::ENTRY_ID_BROAD_APP,
        surface: V08EnforcementProductControlSurface::WindowsBroadAppBlocking,
        surface_kind: V08EnforcementProductControlSurfaceKind::AppGame,
        capability: V08EnforcementProductControlCapabilityName::AppBlocking,
        manual_proof_requirements: &[
            spine::REQUIREMENT_OS_APP_IDENTITY,
            spine::REQUIREMENT_BLOCK_APPLY,
            spine::REQUIREMENT_ROLLBACK,
            spine::REQUIREMENT_AUDIT_CUSTODY,
        ],
        claim_boundary: os_proof::CLAIM_BROAD_APP,
        fallback_behavior: os_proof::FALLBACK_BROAD_APP,
        generated_at: generated_at.0.as_str(),
    })
}

fn network_domain_entry(
    cross_platform: &V08CrossPlatformEnforcementCapabilityProofReadModel,
    browser_domain: &V08BrowserDomainAdapterProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    expect_cross(
        cross_platform,
        ProofEntryId(cross_proof::ENTRY_ID_WINDOWS_NETWORK_DOMAIN),
    );
    expect_browser(
        browser_domain,
        ProofEntryId(browser_proof::ENTRY_ID_NETWORK_FILTER_MANUAL),
    );
    expect_os(
        os_product,
        ProofEntryId(os_proof::ENTRY_ID_NETWORK_DOMAIN_BLOCKING),
    );
    manual_entry(ManualEntrySpec {
        entry_id: spine::ENTRY_ID_NETWORK_DOMAIN,
        surface: V08EnforcementProductControlSurface::WindowsNetworkDomainBlocking,
        surface_kind: V08EnforcementProductControlSurfaceKind::NetworkDomain,
        capability: V08EnforcementProductControlCapabilityName::NetworkDomainBlocking,
        manual_proof_requirements: &[
            spine::REQUIREMENT_NETWORK_FILTER,
            spine::REQUIREMENT_DOMAIN_APPLY,
            spine::REQUIREMENT_ROLLBACK,
            spine::REQUIREMENT_AUDIT_CUSTODY,
        ],
        claim_boundary: os_proof::CLAIM_NETWORK_DOMAIN,
        fallback_behavior: os_proof::FALLBACK_NETWORK_DOMAIN,
        generated_at: generated_at.0.as_str(),
    })
}

fn managed_exact_url_entry(
    browser_domain: &V08BrowserDomainAdapterProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    expect_browser(
        browser_domain,
        ProofEntryId(browser_proof::ENTRY_ID_MANAGED_EXACT_URL),
    );
    expect_os(
        os_product,
        ProofEntryId(os_proof::ENTRY_ID_MANAGED_BROWSER_EXACT_URL),
    );
    manual_entry(ManualEntrySpec {
        entry_id: spine::ENTRY_ID_MANAGED_EXACT_URL,
        surface: V08EnforcementProductControlSurface::WindowsManagedExactUrlControl,
        surface_kind: V08EnforcementProductControlSurfaceKind::ManagedBrowser,
        capability: V08EnforcementProductControlCapabilityName::ManagedBrowserControl,
        manual_proof_requirements: &[
            spine::REQUIREMENT_ACTIVE_TAB,
            spine::REQUIREMENT_EXACT_URL_APPLY,
            spine::REQUIREMENT_ROLLBACK,
            spine::REQUIREMENT_AUDIT_CUSTODY,
        ],
        claim_boundary: browser_proof::CLAIM_MANAGED_EXACT_URL,
        fallback_behavior: browser_proof::FALLBACK_MANAGED_EXACT_URL,
        generated_at: generated_at.0.as_str(),
    })
}

fn unmanaged_exact_url_entry(
    browser_domain: &V08BrowserDomainAdapterProofReadModel,
    os_product: &V08OsAdapterProductProofReadModel,
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    expect_browser(
        browser_domain,
        ProofEntryId(browser_proof::ENTRY_ID_UNMANAGED_EXACT_EVIDENCE),
    );
    expect_os(
        os_product,
        ProofEntryId(os_proof::ENTRY_ID_UNMANAGED_BROWSER_EXACT_EVIDENCE),
    );
    product_entry(ProductEntrySpec {
        entry_id: spine::ENTRY_ID_UNMANAGED_EXACT_URL,
        surface: V08EnforcementProductControlSurface::WindowsUnmanagedExactUrlNotClaimed,
        surface_kind: V08EnforcementProductControlSurfaceKind::UnmanagedBrowser,
        capability: V08EnforcementProductControlCapabilityName::UnmanagedBrowserDetection,
        capability_status: V08EnforcementProductControlCapabilityStatus::NotImplemented,
        product_claim_state: V08EnforcementProductControlClaimState::NotClaimed,
        adapter_execution_state: V08EnforcementProductControlExecutionState::NotInvoked,
        device_policy_state: V08EnforcementProductControlDevicePolicyState::NotClaimed,
        parent_visible_actions: &[V08EnforcementProductControlParentAction::ReportOnly],
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements: &[spine::REQUIREMENT_UNMANAGED_INTEGRATION],
        claim_boundary: browser_proof::CLAIM_UNMANAGED_EXACT_EVIDENCE,
        fallback_behavior: browser_proof::FALLBACK_UNMANAGED_EXACT_EVIDENCE,
        generated_at: generated_at.0.as_str(),
    })
}

fn permission_loss_entry(generated_at: &GeneratedAtText) -> V08EnforcementProductControlSpineEntry {
    manual_entry(ManualEntrySpec {
        entry_id: spine::ENTRY_ID_PERMISSION_LOSS,
        surface: V08EnforcementProductControlSurface::WindowsPermissionLossAlerts,
        surface_kind: V08EnforcementProductControlSurfaceKind::Integrity,
        capability: V08EnforcementProductControlCapabilityName::Notifications,
        manual_proof_requirements: &[
            spine::REQUIREMENT_NOTIFICATION_PROVIDER,
            spine::REQUIREMENT_PERMISSION_DETECTOR,
            spine::REQUIREMENT_DELIVERY_RECEIPT,
        ],
        claim_boundary: spine::CLAIM_PERMISSION_LOSS,
        fallback_behavior: spine::FALLBACK_PERMISSION_LOSS,
        generated_at: generated_at.0.as_str(),
    })
}

fn tamper_uninstall_entry(
    generated_at: &GeneratedAtText,
) -> V08EnforcementProductControlSpineEntry {
    manual_entry(ManualEntrySpec {
        entry_id: spine::ENTRY_ID_TAMPER_UNINSTALL,
        surface: V08EnforcementProductControlSurface::WindowsTamperUninstallAlerts,
        surface_kind: V08EnforcementProductControlSurfaceKind::Integrity,
        capability: V08EnforcementProductControlCapabilityName::PackageLifecycle,
        manual_proof_requirements: &[
            spine::REQUIREMENT_TAMPER_DESIGN,
            spine::REQUIREMENT_REMOVAL_DETECTOR,
            spine::REQUIREMENT_NON_STEALTH_ALERT,
        ],
        claim_boundary: spine::CLAIM_TAMPER_UNINSTALL,
        fallback_behavior: spine::FALLBACK_TAMPER_UNINSTALL,
        generated_at: generated_at.0.as_str(),
    })
}
