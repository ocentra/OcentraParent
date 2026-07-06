use std::fmt::Write as _;

use serde::Serialize;

use crate::schema_result_or_unreachable;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct V3NotificationRuleProviderRetryContractEntry {
    schema_version: String,
    contract_entry_id: String,
    reason_code: String,
    provider_channel: String,
    delivery_attempt_state: String,
    delivery_result_state: String,
    retry_policy_state: String,
    quiet_hours_decision: String,
    escalation_decision: String,
    parent_preference_state: String,
    notification_rule_ref: String,
    notification_intent_ref: String,
    delivery_attempt_ref: String,
    delivery_result_ref: String,
    retry_policy_ref: String,
    quiet_hours_policy_ref: String,
    escalation_policy_ref: String,
    parent_preference_ref: String,
    audit_refs: Vec<String>,
    evidence_refs: Vec<String>,
    provider_receipt_refs: Vec<String>,
    manual_proof_requirements: Vec<String>,
    minimal_provider_payload_boundary: String,
    provider_adapter_implemented: bool,
    delivery_attempt_executed: bool,
    provider_receipt_observed: bool,
    raw_evidence_in_provider_payload: bool,
    provider_stores_child_evidence_claimed: bool,
    last_checked_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct V3NotificationRuleProviderRetryContractReadModel {
    schema_version: String,
    read_model_id: String,
    generated_at: String,
    source_read_model_ids: Vec<String>,
    entries: Vec<V3NotificationRuleProviderRetryContractEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct V3NotificationRuleProviderRetryContractRowInput {
    contract_entry_id: String,
    reason_code: String,
    provider_channel: String,
    delivery_attempt_state: String,
    delivery_result_state: String,
    retry_policy_state: String,
    quiet_hours_decision: String,
    escalation_decision: String,
    parent_preference_state: String,
    notification_rule_ref: String,
    notification_intent_ref: String,
    delivery_attempt_ref: String,
    delivery_result_ref: String,
    retry_policy_ref: String,
    quiet_hours_policy_ref: String,
    escalation_policy_ref: String,
    parent_preference_ref: String,
    audit_refs: Vec<String>,
    evidence_refs: Vec<String>,
    provider_receipt_refs: Vec<String>,
    manual_proof_requirements: Vec<String>,
    minimal_provider_payload_boundary: String,
}

const V3_NOTIFICATION_RULE_PROVIDER_RETRY_CONTRACT_HELPERS: &str = r#"
type GeneratedV3NotificationRuleProviderRetryContractEntry = {
  readonly schemaVersion: string;
  readonly contractEntryId: string;
  readonly reasonCode: string;
  readonly providerChannel: string;
  readonly deliveryAttemptState: string;
  readonly deliveryResultState: string;
  readonly retryPolicyState: string;
  readonly quietHoursDecision: string;
  readonly escalationDecision: string;
  readonly parentPreferenceState: string;
  readonly notificationRuleRef: string;
  readonly notificationIntentRef: string;
  readonly deliveryAttemptRef: string;
  readonly deliveryResultRef: string;
  readonly retryPolicyRef: string;
  readonly quietHoursPolicyRef: string;
  readonly escalationPolicyRef: string;
  readonly parentPreferenceRef: string;
  readonly auditRefs: readonly string[];
  readonly evidenceRefs: readonly string[];
  readonly providerReceiptRefs: readonly string[];
  readonly manualProofRequirements: readonly string[];
  readonly minimalProviderPayloadBoundary: string;
  readonly providerAdapterImplemented: boolean;
  readonly deliveryAttemptExecuted: boolean;
  readonly providerReceiptObserved: boolean;
  readonly rawEvidenceInProviderPayload: boolean;
  readonly providerStoresChildEvidenceClaimed: boolean;
  readonly lastCheckedAt: string;
};
type GeneratedV3NotificationRuleProviderRetryContractReadModel = {
  readonly schemaVersion: string;
  readonly readModelId: string;
  readonly generatedAt: string;
  readonly sourceReadModelIds: readonly string[];
  readonly entries: readonly GeneratedV3NotificationRuleProviderRetryContractEntry[];
};

export function generatedV3NotificationRuleProviderRetryContractEntryIsHonest(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return (
    !generatedV3NotificationRuleProviderRetryContractHasRuntimeClaim(entry) &&
    generatedV3NotificationRuleProviderRetryContractHasRequiredRefs(entry) &&
    generatedV3NotificationRuleProviderRetryContractDeliveryStateIsCoherent(entry)
  );
}

export function generatedV3NotificationRuleProviderRetryContractReadModelIsHonest(
  readModel: GeneratedV3NotificationRuleProviderRetryContractReadModel
): boolean {
  return (
    new Set(readModel.entries.map((entry) => entry.contractEntryId)).size === readModel.entries.length &&
    generatedV3NotificationRuleProviderRetryContractCoversReasonCodes(readModel.entries) &&
    generatedV3NotificationRuleProviderRetryContractCoversProviderChannels(readModel.entries) &&
    generatedV3NotificationRuleProviderRetryContractCoversDeliveryAndRetry(readModel.entries)
  );
}

function generatedV3NotificationRuleProviderRetryContractHasRuntimeClaim(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return [
    entry.providerAdapterImplemented,
    entry.deliveryAttemptExecuted,
    entry.providerReceiptObserved,
    entry.rawEvidenceInProviderPayload,
    entry.providerStoresChildEvidenceClaimed,
  ].some(Boolean);
}

function generatedV3NotificationRuleProviderRetryContractHasRequiredRefs(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return (
    entry.auditRefs.length > 0 &&
    entry.evidenceRefs.length > 0 &&
    entry.notificationRuleRef.trim().length > 0 &&
    entry.notificationIntentRef.trim().length > 0 &&
    entry.deliveryAttemptRef.trim().length > 0 &&
    entry.deliveryResultRef.trim().length > 0 &&
    entry.retryPolicyRef.trim().length > 0 &&
    entry.quietHoursPolicyRef.trim().length > 0 &&
    entry.escalationPolicyRef.trim().length > 0 &&
    entry.parentPreferenceRef.trim().length > 0 &&
    entry.minimalProviderPayloadBoundary.trim().length > 0
  );
}

function generatedV3NotificationRuleProviderRetryContractDeliveryStateIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return (
    generatedV3NotificationRuleProviderRetryContractQuietHoursIsCoherent(entry) &&
    generatedV3NotificationRuleProviderRetryContractParentPreferenceIsCoherent(entry) &&
    generatedV3NotificationRuleProviderRetryContractRetryableFailureIsCoherent(entry) &&
    generatedV3NotificationRuleProviderRetryContractReceiptRequiredIsCoherent(entry) &&
    generatedV3NotificationRuleProviderRetryContractPermanentFailureIsCoherent(entry) &&
    generatedV3NotificationRuleProviderRetryContractNonReceiptRowsAreCoherent(entry)
  );
}

function generatedV3NotificationRuleProviderRetryContractQuietHoursIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return (
    entry.quietHoursDecision !== 'defer-noncritical' ||
    (entry.deliveryAttemptState === 'suppressed-quiet-hours' &&
      entry.retryPolicyState === 'quiet-hours-deferred' &&
      entry.parentPreferenceState === 'quiet-hours-active')
  );
}

function generatedV3NotificationRuleProviderRetryContractParentPreferenceIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return (
    entry.parentPreferenceState !== 'channel-disabled' ||
    (entry.deliveryAttemptState === 'provider-disabled' &&
      entry.deliveryResultState === 'not-sent' &&
      entry.retryPolicyState === 'provider-disabled')
  );
}

function generatedV3NotificationRuleProviderRetryContractRetryableFailureIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return (
    entry.deliveryResultState !== 'retryable-failure' ||
    (entry.retryPolicyState === 'exponential-backoff' && entry.manualProofRequirements.length > 0)
  );
}

function generatedV3NotificationRuleProviderRetryContractReceiptRequiredIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return (
    entry.deliveryResultState !== 'receipt-required' ||
    (entry.providerReceiptRefs.length > 0 &&
      entry.manualProofRequirements.length > 0 &&
      entry.deliveryAttemptState === 'queued')
  );
}

function generatedV3NotificationRuleProviderRetryContractPermanentFailureIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return (
    entry.deliveryResultState !== 'permanent-failure' ||
    (entry.retryPolicyState === 'manual-review' && entry.manualProofRequirements.length > 0)
  );
}

function generatedV3NotificationRuleProviderRetryContractNonReceiptRowsAreCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return entry.deliveryResultState === 'receipt-required' || entry.providerReceiptRefs.length === 0;
}

function generatedV3NotificationRuleProviderRetryContractCoversReasonCodes(
  entries: readonly GeneratedV3NotificationRuleProviderRetryContractEntry[]
): boolean {
  const reasonCodes = new Set(entries.map((entry) => entry.reasonCode));
  return GeneratedV3NotificationRuleReasonCodes.every((reasonCode) => reasonCodes.has(reasonCode));
}

function generatedV3NotificationRuleProviderRetryContractCoversProviderChannels(
  entries: readonly GeneratedV3NotificationRuleProviderRetryContractEntry[]
): boolean {
  const channels = new Set(entries.map((entry) => entry.providerChannel));
  return GeneratedV3NotificationProviderChannels.every((channel) => channels.has(channel));
}

function generatedV3NotificationRuleProviderRetryContractCoversDeliveryAndRetry(
  entries: readonly GeneratedV3NotificationRuleProviderRetryContractEntry[]
): boolean {
  const deliveryResults = new Set(entries.map((entry) => entry.deliveryResultState));
  const retryPolicies = new Set(entries.map((entry) => entry.retryPolicyState));
  const quietHours = new Set(entries.map((entry) => entry.quietHoursDecision));
  const escalation = new Set(entries.map((entry) => entry.escalationDecision));
  const preferences = new Set(entries.map((entry) => entry.parentPreferenceState));

  return (
    GeneratedV3NotificationDeliveryResultStates.every((state) => deliveryResults.has(state)) &&
    GeneratedV3NotificationRetryPolicyStates.every((state) => retryPolicies.has(state)) &&
    GeneratedV3NotificationQuietHoursDecisions.every((state) => quietHours.has(state)) &&
    GeneratedV3NotificationEscalationDecisions.every((state) => escalation.has(state)) &&
    GeneratedV3NotificationParentPreferenceStates.every((state) => preferences.has(state))
  );
}
"#;

fn push_export<T: Serialize>(output: &mut String, name: &str, value: &T) {
    let json = schema_result_or_unreachable(serde_json::to_string_pretty(value), name);
    writeln!(output, "export const {name} = {json} as const;\n")
        .expect("write notification-v3 provider retry ts");
}

pub fn notification_v3_provider_retry_typescript() -> String {
    let mut output = String::from(
        "/* generated from crates/schema/src/notification_v3_provider_retry_ts.rs */\n\n",
    );

    push_export(
        &mut output,
        "GeneratedV3NotificationRuleReasonCodes",
        &[
            "policy-violation",
            "parent-request",
            "suspicious-unknown",
            "device-offline",
            "sync-failure",
            "provider-failure",
        ],
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationProviderChannels",
        &["push", "email", "sms", "whatsapp", "in-app"],
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationDeliveryAttemptStates",
        &[
            "eligible",
            "queued",
            "suppressed-quiet-hours",
            "retry-scheduled",
            "failed-final",
            "provider-disabled",
        ],
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationDeliveryResultStates",
        &[
            "not-sent",
            "queued",
            "retryable-failure",
            "permanent-failure",
            "receipt-required",
            "manual-required",
        ],
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationRetryPolicyStates",
        &[
            "no-retry",
            "exponential-backoff",
            "quiet-hours-deferred",
            "manual-review",
            "provider-disabled",
        ],
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationQuietHoursDecisions",
        &["allow", "defer-noncritical", "emergency-override", "manual-required"],
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationEscalationDecisions",
        &["none", "wait-window", "escalate-parent", "manual-review"],
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationParentPreferenceStates",
        &["enabled", "quiet-hours-active", "channel-disabled", "manual-setup-required"],
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationRuleProviderRetryContractGeneratedAt",
        &"2026-06-02T15:18:13.000Z",
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationRuleProviderRetryContractSourceReadModelIds",
        &[
            "reports-notifications-sync-provider-status",
            "v0-8-integrity-alert-status-bridge",
            "data-custody-provider-boundary",
            "notification-feature-expectations-contract-boundary",
        ],
    );
    push_export(
        &mut output,
        "GeneratedV3NotificationRuleProviderRetryContractReadModel",
        &V3NotificationRuleProviderRetryContractReadModel {
            schema_version: "v0.6".to_string(),
            read_model_id: "v3-notification-rule-provider-retry-contract".to_string(),
            generated_at: "2026-06-02T15:18:13.000Z".to_string(),
            source_read_model_ids: vec![
                "reports-notifications-sync-provider-status".to_string(),
                "v0-8-integrity-alert-status-bridge".to_string(),
                "data-custody-provider-boundary".to_string(),
                "notification-feature-expectations-contract-boundary".to_string(),
            ],
            entries: v3_entries(),
        },
    );
    output.push_str(V3_NOTIFICATION_RULE_PROVIDER_RETRY_CONTRACT_HELPERS);

    output
}

fn v3_entries() -> Vec<V3NotificationRuleProviderRetryContractEntry> {
    vec![
        v3_entry(V3NotificationRuleProviderRetryContractRowInput {
            contract_entry_id: "notification-rule-policy-violation-push-queued".to_string(),
            reason_code: "policy-violation".to_string(),
            provider_channel: "push".to_string(),
            delivery_attempt_state: "queued".to_string(),
            delivery_result_state: "queued".to_string(),
            retry_policy_state: "no-retry".to_string(),
            quiet_hours_decision: "emergency-override".to_string(),
            escalation_decision: "escalate-parent".to_string(),
            parent_preference_state: "enabled".to_string(),
            notification_rule_ref: "notification-rule-policy-violation-ref".to_string(),
            notification_intent_ref: "notification-intent-policy-violation-ref".to_string(),
            delivery_attempt_ref: "delivery-attempt-policy-violation-push-ref".to_string(),
            delivery_result_ref: "delivery-result-policy-violation-queued-ref".to_string(),
            retry_policy_ref: "retry-policy-no-retry-critical-ref".to_string(),
            quiet_hours_policy_ref: "quiet-hours-emergency-override-ref".to_string(),
            escalation_policy_ref: "escalation-policy-escalate-parent-ref".to_string(),
            parent_preference_ref: "parent-preference-push-enabled-ref".to_string(),
            audit_refs: vec!["notification-audit-policy-violation-ref".to_string()],
            evidence_refs: vec![
                "policy-decision-evidence-ref".to_string(),
                "authenticated-drill-in-ref".to_string(),
            ],
            provider_receipt_refs: vec![],
            manual_proof_requirements: vec![],
            minimal_provider_payload_boundary:
                "Critical policy-violation payload carries only alert id, severity, reason code, evidence ref, policy ref, and parent action link.".to_string(),
        }),
        v3_entry(V3NotificationRuleProviderRetryContractRowInput {
            contract_entry_id: "notification-rule-parent-request-in-app-receipt-required".to_string(),
            reason_code: "parent-request".to_string(),
            provider_channel: "in-app".to_string(),
            delivery_attempt_state: "queued".to_string(),
            delivery_result_state: "receipt-required".to_string(),
            retry_policy_state: "no-retry".to_string(),
            quiet_hours_decision: "allow".to_string(),
            escalation_decision: "wait-window".to_string(),
            parent_preference_state: "enabled".to_string(),
            notification_rule_ref: "notification-rule-parent-request-ref".to_string(),
            notification_intent_ref: "notification-intent-parent-request-ref".to_string(),
            delivery_attempt_ref: "delivery-attempt-parent-request-in-app-ref".to_string(),
            delivery_result_ref: "delivery-result-parent-request-receipt-required-ref".to_string(),
            retry_policy_ref: "retry-policy-no-retry-parent-action-ref".to_string(),
            quiet_hours_policy_ref: "quiet-hours-allow-parent-action-ref".to_string(),
            escalation_policy_ref: "escalation-policy-wait-window-ref".to_string(),
            parent_preference_ref: "parent-preference-in-app-enabled-ref".to_string(),
            audit_refs: vec!["notification-audit-parent-request-ref".to_string()],
            evidence_refs: vec![
                "parent-request-ref".to_string(),
                "authenticated-parent-action-ref".to_string(),
            ],
            provider_receipt_refs: vec!["provider-receipt-parent-action-required-ref".to_string()],
            manual_proof_requirements: vec![
                "real in-app receipt artifact before parent notification delivery can be claimed".to_string(),
            ],
            minimal_provider_payload_boundary:
                "Ask-parent payload carries intent ref and parent action link; sensitive child detail remains behind authenticated parent surfaces.".to_string(),
        }),
        v3_entry(V3NotificationRuleProviderRetryContractRowInput {
            contract_entry_id: "notification-rule-suspicious-unknown-email-retryable-failure".to_string(),
            reason_code: "suspicious-unknown".to_string(),
            provider_channel: "email".to_string(),
            delivery_attempt_state: "retry-scheduled".to_string(),
            delivery_result_state: "retryable-failure".to_string(),
            retry_policy_state: "exponential-backoff".to_string(),
            quiet_hours_decision: "allow".to_string(),
            escalation_decision: "manual-review".to_string(),
            parent_preference_state: "manual-setup-required".to_string(),
            notification_rule_ref: "notification-rule-suspicious-unknown-ref".to_string(),
            notification_intent_ref: "notification-intent-suspicious-unknown-ref".to_string(),
            delivery_attempt_ref: "delivery-attempt-suspicious-unknown-email-ref".to_string(),
            delivery_result_ref: "delivery-result-suspicious-unknown-retryable-ref".to_string(),
            retry_policy_ref: "retry-policy-exponential-backoff-ref".to_string(),
            quiet_hours_policy_ref: "quiet-hours-allow-suspicious-unknown-ref".to_string(),
            escalation_policy_ref: "escalation-policy-manual-review-ref".to_string(),
            parent_preference_ref: "parent-preference-email-setup-required-ref".to_string(),
            audit_refs: vec!["notification-audit-suspicious-unknown-ref".to_string()],
            evidence_refs: vec![
                "classified-evidence-ref".to_string(),
                "notification-intent-audit-ref".to_string(),
            ],
            provider_receipt_refs: vec![],
            manual_proof_requirements: vec![
                "provider error artifact before retry execution can be claimed".to_string(),
            ],
            minimal_provider_payload_boundary:
                "Suspicious-unknown payload avoids raw observation details and carries only reason, severity, evidence ref, and authenticated drill-in.".to_string(),
        }),
        v3_entry(V3NotificationRuleProviderRetryContractRowInput {
            contract_entry_id: "notification-rule-device-offline-sms-permanent-failure".to_string(),
            reason_code: "device-offline".to_string(),
            provider_channel: "sms".to_string(),
            delivery_attempt_state: "failed-final".to_string(),
            delivery_result_state: "permanent-failure".to_string(),
            retry_policy_state: "manual-review".to_string(),
            quiet_hours_decision: "manual-required".to_string(),
            escalation_decision: "manual-review".to_string(),
            parent_preference_state: "manual-setup-required".to_string(),
            notification_rule_ref: "notification-rule-device-offline-ref".to_string(),
            notification_intent_ref: "notification-intent-device-offline-ref".to_string(),
            delivery_attempt_ref: "delivery-attempt-device-offline-sms-ref".to_string(),
            delivery_result_ref: "delivery-result-device-offline-permanent-failure-ref".to_string(),
            retry_policy_ref: "retry-policy-manual-review-ref".to_string(),
            quiet_hours_policy_ref: "quiet-hours-manual-required-ref".to_string(),
            escalation_policy_ref: "escalation-policy-manual-review-ref".to_string(),
            parent_preference_ref: "parent-preference-sms-setup-required-ref".to_string(),
            audit_refs: vec!["notification-audit-device-offline-ref".to_string()],
            evidence_refs: vec![
                "device-health-status-ref".to_string(),
                "offline-window-evidence-ref".to_string(),
            ],
            provider_receipt_refs: vec![],
            manual_proof_requirements: vec![
                "provider failure artifact and parent preference setup before SMS retry can be claimed".to_string(),
            ],
            minimal_provider_payload_boundary:
                "Device-offline SMS payload carries device scope, reason code, and action link only; raw child activity is excluded.".to_string(),
        }),
        v3_entry(V3NotificationRuleProviderRetryContractRowInput {
            contract_entry_id: "notification-rule-sync-failure-whatsapp-quiet-hours-deferred".to_string(),
            reason_code: "sync-failure".to_string(),
            provider_channel: "whatsapp".to_string(),
            delivery_attempt_state: "suppressed-quiet-hours".to_string(),
            delivery_result_state: "manual-required".to_string(),
            retry_policy_state: "quiet-hours-deferred".to_string(),
            quiet_hours_decision: "defer-noncritical".to_string(),
            escalation_decision: "wait-window".to_string(),
            parent_preference_state: "quiet-hours-active".to_string(),
            notification_rule_ref: "notification-rule-sync-failure-ref".to_string(),
            notification_intent_ref: "notification-intent-sync-failure-ref".to_string(),
            delivery_attempt_ref: "delivery-attempt-sync-failure-whatsapp-ref".to_string(),
            delivery_result_ref: "delivery-result-sync-failure-quiet-hours-deferred-ref".to_string(),
            retry_policy_ref: "retry-policy-quiet-hours-deferred-ref".to_string(),
            quiet_hours_policy_ref: "quiet-hours-defer-noncritical-ref".to_string(),
            escalation_policy_ref: "escalation-policy-wait-window-sync-ref".to_string(),
            parent_preference_ref: "parent-preference-whatsapp-quiet-hours-ref".to_string(),
            audit_refs: vec!["notification-audit-sync-failure-ref".to_string()],
            evidence_refs: vec![
                "sync-failure-state-ref".to_string(),
                "parent-owned-storage-ref".to_string(),
            ],
            provider_receipt_refs: vec![],
            manual_proof_requirements: vec![
                "quiet-hours preference artifact before deferred provider send can be claimed".to_string(),
            ],
            minimal_provider_payload_boundary:
                "Sync-failure payload is deferable and references parent-owned storage state without embedding report or raw evidence content.".to_string(),
        }),
        v3_entry(V3NotificationRuleProviderRetryContractRowInput {
            contract_entry_id: "notification-rule-provider-failure-in-app-channel-disabled".to_string(),
            reason_code: "provider-failure".to_string(),
            provider_channel: "in-app".to_string(),
            delivery_attempt_state: "provider-disabled".to_string(),
            delivery_result_state: "not-sent".to_string(),
            retry_policy_state: "provider-disabled".to_string(),
            quiet_hours_decision: "allow".to_string(),
            escalation_decision: "none".to_string(),
            parent_preference_state: "channel-disabled".to_string(),
            notification_rule_ref: "notification-rule-provider-failure-ref".to_string(),
            notification_intent_ref: "notification-intent-provider-failure-ref".to_string(),
            delivery_attempt_ref: "delivery-attempt-provider-disabled-ref".to_string(),
            delivery_result_ref: "delivery-result-not-sent-provider-disabled-ref".to_string(),
            retry_policy_ref: "retry-policy-provider-disabled-ref".to_string(),
            quiet_hours_policy_ref: "quiet-hours-allow-provider-failure-ref".to_string(),
            escalation_policy_ref: "escalation-policy-none-provider-disabled-ref".to_string(),
            parent_preference_ref: "parent-preference-channel-disabled-ref".to_string(),
            audit_refs: vec!["notification-audit-provider-disabled-ref".to_string()],
            evidence_refs: vec![
                "provider-configuration-state-ref".to_string(),
                "notification-routing-status-ref".to_string(),
            ],
            provider_receipt_refs: vec![],
            manual_proof_requirements: vec![
                "provider enablement and credential review before send or retry can be claimed".to_string(),
            ],
            minimal_provider_payload_boundary:
                "Provider-failure row is an audit and preference state only; no provider payload is sent while the channel is disabled.".to_string(),
        }),
    ]
}

fn v3_entry(input: V3NotificationRuleProviderRetryContractRowInput) -> V3NotificationRuleProviderRetryContractEntry {
    V3NotificationRuleProviderRetryContractEntry {
        schema_version: "v0.6".to_string(),
        contract_entry_id: input.contract_entry_id,
        reason_code: input.reason_code,
        provider_channel: input.provider_channel,
        delivery_attempt_state: input.delivery_attempt_state,
        delivery_result_state: input.delivery_result_state,
        retry_policy_state: input.retry_policy_state,
        quiet_hours_decision: input.quiet_hours_decision,
        escalation_decision: input.escalation_decision,
        parent_preference_state: input.parent_preference_state,
        notification_rule_ref: input.notification_rule_ref,
        notification_intent_ref: input.notification_intent_ref,
        delivery_attempt_ref: input.delivery_attempt_ref,
        delivery_result_ref: input.delivery_result_ref,
        retry_policy_ref: input.retry_policy_ref,
        quiet_hours_policy_ref: input.quiet_hours_policy_ref,
        escalation_policy_ref: input.escalation_policy_ref,
        parent_preference_ref: input.parent_preference_ref,
        audit_refs: input.audit_refs,
        evidence_refs: input.evidence_refs,
        provider_receipt_refs: input.provider_receipt_refs,
        manual_proof_requirements: input.manual_proof_requirements,
        minimal_provider_payload_boundary: input.minimal_provider_payload_boundary,
        provider_adapter_implemented: false,
        delivery_attempt_executed: false,
        provider_receipt_observed: false,
        raw_evidence_in_provider_payload: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: "2026-06-02T15:18:13.000Z".to_string(),
    }
}
