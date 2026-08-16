use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::schema_domain_ai_wire::{
    LocalAiAdapterBoundary, LocalAiEvidenceContextBuildRequestWire,
    LocalAiEvidenceContextBuildResultWire, LocalAiExecutionState, LocalAiProviderPrivacyMode,
    LocalAiProviderSource, LocalAiRequestedEvaluationKind, LocalModelRuntimeStatusWire,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::ai::{
    LocalAiCapabilityFlag, LocalAiContextBuildState, LocalAiDegradedState, LocalAiEvidenceCustody,
    LocalAiModelId, LocalAiModelLoadState, LocalAiModelReference, LocalAiPromptVersion,
    LocalAiProviderId, LocalAiResourceClass, LocalAiRuntimeReferenceId, LocalAiTimestamp,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::{
    ChildProfileReference, ParentDevicePlatform, ParentDeviceReference,
};

fn to_json<T: serde::Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect_value("schema domain AI wire fixture must serialize")
}

#[test]
fn schema_domain_ai_wire_runtime_status_serializes_to_typescript_shape() {
    let value = to_json(LocalModelRuntimeStatusWire {
        runtime_reference_id: LocalAiRuntimeReferenceId::from("runtime-ref-1"),
        provider_id: LocalAiProviderId::from("provider-1"),
        model_id: LocalAiModelId::from("model-1"),
        model_reference: LocalAiModelReference::from("model-ref-1"),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::LocalAdapterReady,
        execution_state: LocalAiExecutionState::DryRunReady,
        provider_source: LocalAiProviderSource::LocalConfig,
        load_state: LocalAiModelLoadState::Loaded,
        capability_flags: vec![LocalAiCapabilityFlag::Classification],
        resource_class: LocalAiResourceClass::Cpu,
        degraded_state: LocalAiDegradedState::None,
        last_checked_at: LocalAiTimestamp::from("2026-06-20T18:10:00.000Z"),
        unavailable_reason: None,
    });

    assert_eq!(value["runtimeReferenceId"], "runtime-ref-1");
    assert_eq!(value["privacyMode"], "local-only");
    assert_eq!(value["adapterBoundary"], "local-adapter-ready");
    assert_eq!(value["executionState"], "dry-run-ready");
    assert_eq!(value["providerSource"], "local-config");
    assert_eq!(value["loadState"], "loaded");
    assert_eq!(value["capabilityFlags"][0], "classification");
    assert_eq!(value["resourceClass"], "cpu");
    assert_eq!(value["degradedState"], "none");
}
#[test]
fn schema_domain_ai_wire_build_request_and_result_keep_nested_parent_shapes() {
    let request = to_json(LocalAiEvidenceContextBuildRequestWire {
        schema_version: "v0.6".to_string().into(),
        request_id: "request-1".to_string().into(),
        requested_at: "2026-06-20T18:11:00.000Z".to_string().into(),
        child_profile: ChildProfileReference {
            child_profile_id: "child-1".to_string(),
            display_name: "Child One".to_string(),
        },
        device: ParentDeviceReference {
            device_id: "device-1".to_string().into(),
            child_profile_id: Some("child-1".to_string().into()),
            label: "Parent Laptop".to_string(),
            platform: ParentDevicePlatform::Windows,
        },
        requested_evaluation_kind: LocalAiRequestedEvaluationKind::MixedContext,
        required_evidence_kinds: vec![],
        parent_rule_context_references: vec![],
        model_task_requirements: vec![LocalAiCapabilityFlag::Classification],
        allowed_custody: vec![LocalAiEvidenceCustody::LiveLocalChildAgent],
        prompt_version: LocalAiPromptVersion::from("prompt-v1"),
    });

    let result = to_json(LocalAiEvidenceContextBuildResultWire {
        schema_version: "v0.6".to_string().into(),
        request_id: "request-1".to_string().into(),
        state: LocalAiContextBuildState::Ready,
        context: None,
        rejected_fields: vec![],
        missing_evidence_kinds: vec![],
        degraded_source_refs: vec![],
        custody_boundary_summary: "local-only".to_string().into(),
        validation_gate_summary: "ready".to_string().into(),
        audit_evidence_references: vec![],
    });

    assert_eq!(request["requestedEvaluationKind"], "mixed-context");
    assert_eq!(request["childProfile"]["childProfileId"], "child-1");
    assert_eq!(request["childProfile"]["displayName"], "Child One");
    assert!(request["childProfile"].get("familyId").is_none());
    assert_eq!(request["device"]["platform"], "windows");
    assert_eq!(request["allowedCustody"][0], "live-local-child-agent");
    assert_eq!(request["promptVersion"], "prompt-v1");

    assert_eq!(result["state"], "ready");
    assert_eq!(result["custodyBoundarySummary"], "local-only");
    assert_eq!(result["validationGateSummary"], "ready");
    assert_eq!(result["context"], serde_json::Value::Null);
}
