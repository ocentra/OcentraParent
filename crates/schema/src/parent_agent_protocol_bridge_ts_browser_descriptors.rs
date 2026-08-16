fn replace_tokens(mut template: String, tokens: &[(&str, &str)]) -> String {
    for (token, value) in tokens {
        template = template.replace(token, value);
    }
    template
}

fn browser_runtime_event_type_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "EvidenceObserved",
            value: BrowserRuntimePhase::EvidenceObserved.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "EvidenceJournaled",
            value: BrowserRuntimePhase::EvidenceJournaled.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisRequested",
            value: BrowserRuntimePhase::AiAnalysisRequested.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisCompleted",
            value: BrowserRuntimePhase::AiAnalysisCompleted.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "PolicyEvaluationRequested",
            value: BrowserRuntimePhase::PolicyEvaluationRequested.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "PolicyDecisionCompleted",
            value: BrowserRuntimePhase::PolicyDecisionCompleted.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "InterventionCommandIssued",
            value: BrowserRuntimePhase::InterventionCommandIssued.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "InterventionResultObserved",
            value: BrowserRuntimePhase::InterventionResultObserved.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "AuditEntryCommitted",
            value: BrowserRuntimePhase::AuditEntryCommitted.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "ReadModelProjected",
            value: BrowserRuntimePhase::ReadModelProjected.event_type(),
        },
    ]
}

fn browser_runtime_phase_descriptors() -> Vec<ProtocolLiteralDescriptor<BrowserRuntimePhase>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "EvidenceObserved",
            value: BrowserRuntimePhase::EvidenceObserved,
        },
        ProtocolLiteralDescriptor {
            key: "EvidenceJournaled",
            value: BrowserRuntimePhase::EvidenceJournaled,
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisRequested",
            value: BrowserRuntimePhase::AiAnalysisRequested,
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisCompleted",
            value: BrowserRuntimePhase::AiAnalysisCompleted,
        },
        ProtocolLiteralDescriptor {
            key: "PolicyEvaluationRequested",
            value: BrowserRuntimePhase::PolicyEvaluationRequested,
        },
        ProtocolLiteralDescriptor {
            key: "PolicyDecisionCompleted",
            value: BrowserRuntimePhase::PolicyDecisionCompleted,
        },
        ProtocolLiteralDescriptor {
            key: "InterventionCommandIssued",
            value: BrowserRuntimePhase::InterventionCommandIssued,
        },
        ProtocolLiteralDescriptor {
            key: "InterventionResultObserved",
            value: BrowserRuntimePhase::InterventionResultObserved,
        },
        ProtocolLiteralDescriptor {
            key: "AuditEntryCommitted",
            value: BrowserRuntimePhase::AuditEntryCommitted,
        },
        ProtocolLiteralDescriptor {
            key: "ReadModelProjected",
            value: BrowserRuntimePhase::ReadModelProjected,
        },
    ]
}

fn browser_capability_status_descriptors() -> Vec<ProtocolLiteralDescriptor<BrowserCapabilityStatus>>
{
    vec![
        ProtocolLiteralDescriptor {
            key: "Available",
            value: BrowserCapabilityStatus::Available,
        },
        ProtocolLiteralDescriptor {
            key: "TabListOnly",
            value: BrowserCapabilityStatus::TabListOnly,
        },
        ProtocolLiteralDescriptor {
            key: "UnsupportedBrowser",
            value: BrowserCapabilityStatus::UnsupportedBrowser,
        },
        ProtocolLiteralDescriptor {
            key: "UnmanagedBrowser",
            value: BrowserCapabilityStatus::UnmanagedBrowser,
        },
        ProtocolLiteralDescriptor {
            key: "ManagedProfileMissing",
            value: BrowserCapabilityStatus::ManagedProfileMissing,
        },
        ProtocolLiteralDescriptor {
            key: "BridgeMissing",
            value: BrowserCapabilityStatus::BridgeMissing,
        },
        ProtocolLiteralDescriptor {
            key: "PermissionLimited",
            value: BrowserCapabilityStatus::PermissionLimited,
        },
        ProtocolLiteralDescriptor {
            key: "Stale",
            value: BrowserCapabilityStatus::Stale,
        },
        ProtocolLiteralDescriptor {
            key: "AdapterError",
            value: BrowserCapabilityStatus::AdapterError,
        },
        ProtocolLiteralDescriptor {
            key: "DisabledByParent",
            value: BrowserCapabilityStatus::DisabledByParent,
        },
    ]
}

fn browser_custody_label_descriptors() -> Vec<ProtocolLiteralDescriptor<BrowserCustodyLabel>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ChildDeviceLocal",
            value: BrowserCustodyLabel::ChildDeviceLocal,
        },
        ProtocolLiteralDescriptor {
            key: "LocalNetworkChildAgent",
            value: BrowserCustodyLabel::LocalNetworkChildAgent,
        },
        ProtocolLiteralDescriptor {
            key: "ParentCache",
            value: BrowserCustodyLabel::ParentCache,
        },
        ProtocolLiteralDescriptor {
            key: "ParentOwnedExport",
            value: BrowserCustodyLabel::ParentOwnedExport,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: BrowserCustodyLabel::Unavailable,
        },
    ]
}

fn browser_query_visibility_descriptors(
) -> Vec<ProtocolLiteralDescriptor<BrowserQueryVisibilityLabel>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "LiveLocal",
            value: BrowserQueryVisibilityLabel::LiveLocal,
        },
        ProtocolLiteralDescriptor {
            key: "LiveLan",
            value: BrowserQueryVisibilityLabel::LiveLan,
        },
        ProtocolLiteralDescriptor {
            key: "ParentCache",
            value: BrowserQueryVisibilityLabel::ParentCache,
        },
        ProtocolLiteralDescriptor {
            key: "ParentOwnedExport",
            value: BrowserQueryVisibilityLabel::ParentOwnedExport,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: BrowserQueryVisibilityLabel::Unavailable,
        },
    ]
}
