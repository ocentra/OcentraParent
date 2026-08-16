use super::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName,
    AgentEventEnvelope, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    BrowserPolicyActionExecutionState, BrowserPolicyAiAuthority, BrowserPolicyBudgets,
    BrowserPolicyDefaultPosture, BrowserPolicyDiscovery, BrowserPolicyEffectivePolicy,
    BrowserPolicyEffectiveRule, BrowserPolicyEvidenceProofLevel, BrowserPolicyEvidenceRequirement,
    BrowserPolicyExecutionMode, BrowserPolicyPatch, BrowserPolicyProofFallback,
    BrowserPolicyRejectionReason, BrowserPolicyRuleAction, BrowserPolicyTargetProofRequirement,
    BrowserPolicyUpdateKind, BrowserPolicyUpdateResponse, BrowserPolicyUpdateStatus,
    BrowserPolicyUrlTargetType, LogFieldValue, LogFields, LogLevel, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use crate::browser_policy::BrowserPolicyPatchRequest;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn browser_policy_patch_command_serializes_to_typescript_contract_shape() {
    let request = BrowserPolicyPatchRequest {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: constants::browser_policy::REQUEST_ID.to_string(),
        kind: BrowserPolicyUpdateKind::Patch,
        policy_id: constants::browser_policy::POLICY_ID.to_string(),
        base_revision_id: constants::browser_policy::REVISION_ID.to_string(),
        patches: vec![BrowserPolicyPatch {
            op: constants::browser_policy::PATCH_OPERATION_REPLACE.to_string(),
            field_id: constants::browser_policy::FIELD_ID_ENABLED.to_string(),
            writes_to: constants::browser_policy::WRITES_TO_ENABLED.to_string(),
            value: serde_json::Value::Bool(true),
        }],
    };
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::BROWSER_POLICY_REQUEST.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&request).expect_value("request serializes: {error:?}"),
        ),
    );
    payload.insert(
        constants::field::BROWSER_POLICY_UPDATE_KIND.to_string(),
        LogFieldValue::String(BrowserPolicyUpdateKind::Patch.as_protocol_str().to_string()),
    );

    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: "cmd-browser-policy".to_string(),
        sent_at: "2026-05-28T17:30:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentBrowserPolicyPatch,
        payload,
    };
    let serialized =
        serde_json::to_value(command).expect_value("browser policy command serializes: {error:?}");
    let request_text = serialized["payload"][constants::field::BROWSER_POLICY_REQUEST]
        .as_str()
        .expect_value("request is encoded as JSON text");
    let request_value: serde_json::Value = serde_json::from_str(request_text)
        .expect_value("request payload decodes as JSON: {error:?}");

    assert_eq!(
        serialized["command"],
        constants::browser_policy::COMMAND_PATCH
    );
    assert_eq!(
        serialized["payload"][constants::field::BROWSER_POLICY_UPDATE_KIND],
        constants::browser_policy::UPDATE_KIND_PATCH
    );
    assert_eq!(
        request_value["kind"],
        constants::browser_policy::UPDATE_KIND_PATCH
    );
    assert_eq!(
        request_value["patches"][0]["writesTo"],
        constants::browser_policy::WRITES_TO_ENABLED
    );
}

#[test]
fn browser_policy_rejected_event_serializes_typed_reason() {
    let response = BrowserPolicyUpdateResponse {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: constants::browser_policy::REQUEST_ID.to_string(),
        kind: BrowserPolicyUpdateKind::Patch,
        status: BrowserPolicyUpdateStatus::Rejected,
        policy: None,
        effective_policy: None,
        capability_registry: None,
        rejection_reason: Some(BrowserPolicyRejectionReason::ScaffoldUnavailable),
        audit_event_id: None,
        message: Some(constants::browser_policy::SCAFFOLD_UNAVAILABLE_MESSAGE.to_string()),
    };
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::BROWSER_POLICY_RESPONSE.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&response).expect_value("response serializes: {error:?}"),
        ),
    );
    payload.insert(
        constants::field::BROWSER_POLICY_REJECTION_REASON.to_string(),
        LogFieldValue::String(
            constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE.to_string(),
        ),
    );

    let event = AgentEventEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        event_id: constants::event_id::BROWSER_POLICY_PATCH_REJECTED.to_string(),
        correlation_id: "cmd-browser-policy".to_string(),
        sent_at: "2026-05-28T17:30:01Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentBrowserPolicyPatchRejected,
        severity: LogLevel::Warn,
        payload,
        snapshot: None,
    };
    let serialized =
        serde_json::to_value(event).expect_value("browser policy event serializes: {error:?}");
    let response_text = serialized["payload"][constants::field::BROWSER_POLICY_RESPONSE]
        .as_str()
        .expect_value("response is encoded as JSON text");
    let response_value: serde_json::Value = serde_json::from_str(response_text)
        .expect_value("response payload decodes as JSON: {error:?}");

    assert_eq!(
        serialized["event"],
        constants::browser_policy::EVENT_PATCH_REJECTED
    );
    assert_eq!(
        serialized["payload"][constants::field::BROWSER_POLICY_REJECTION_REASON],
        constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE
    );
    assert_eq!(
        response_value["rejectionReason"],
        constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE
    );
}

#[test]
fn browser_policy_effective_rule_serializes_compiler_result_fields() {
    let effective_policy = BrowserPolicyEffectivePolicy {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        policy_id: constants::browser_policy::POLICY_ID.to_string(),
        revision_id: constants::browser_policy::REVISION_ID.to_string(),
        compiled_hash: constants::browser_policy::COMPILED_HASH_PREFIX.to_string(),
        compiled_at: constants::browser_policy::TEST_SENT_AT.to_string(),
        execution_mode: BrowserPolicyExecutionMode::DryRun,
        default_posture: BrowserPolicyDefaultPosture::Warn,
        fallback_posture: None,
        discovery: BrowserPolicyDiscovery::default(),
        budgets: BrowserPolicyBudgets {
            enabled: true,
            default_daily_minutes: Some(30),
            counting_mode: Default::default(),
        },
        rules: vec![BrowserPolicyEffectiveRule {
            rule_id: constants::browser_policy::DEFAULT_RULE_ID.to_string(),
            target_type: BrowserPolicyUrlTargetType::CloudGaming,
            target_value: constants::browser_policy::DEFAULT_TARGET_VALUE.to_string(),
            default_posture: BrowserPolicyDefaultPosture::Warn,
            evidence: BrowserPolicyEvidenceRequirement {
                url_scope: Default::default(),
                required_proof: BrowserPolicyEvidenceProofLevel::BrowserGameRuntimeSignal,
                proof_fallback: Some(BrowserPolicyProofFallback::AskParent),
                when_proof_unavailable: BrowserPolicyProofFallback::Ask,
                never_collect: Vec::new(),
            },
            action: BrowserPolicyRuleAction::Ask,
            target_proof_requirement: BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
            capability_state: super::BrowserPolicyCapabilityState::ManualRequired,
            action_execution: BrowserPolicyActionExecutionState::DryRunNoExecution,
            ai_authority: BrowserPolicyAiAuthority::AiCandidateOnly,
            compile_note: constants::browser_policy::COMPILE_NOTE_GAME_REQUIRED.to_string(),
        }],
    };

    let serialized = serde_json::to_value(effective_policy)
        .expect_value("effective policy serializes: {error:?}");
    let rule = &serialized["rules"][0];

    assert_eq!(rule["targetType"], "cloud-gaming");
    assert_eq!(
        rule["targetProofRequirement"],
        "browser-game-runtime-signal"
    );
    assert_eq!(rule["actionExecution"], "dry-run-no-execution");
    assert_eq!(rule["aiAuthority"], "ai-candidate-only");
}
