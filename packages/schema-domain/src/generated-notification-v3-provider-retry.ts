/* generated from crates/schema/src/notification_v3_provider_retry_ts.rs */

export const GeneratedV3NotificationRuleReasonCodes = [
  'policy-violation',
  'parent-request',
  'suspicious-unknown',
  'device-offline',
  'sync-failure',
  'provider-failure',
] as const;

export const GeneratedV3NotificationProviderChannels = ['push', 'email', 'sms', 'whatsapp', 'in-app'] as const;

export const GeneratedV3NotificationDeliveryAttemptStates = [
  'eligible',
  'queued',
  'suppressed-quiet-hours',
  'retry-scheduled',
  'failed-final',
  'provider-disabled',
] as const;

export const GeneratedV3NotificationDeliveryResultStates = [
  'not-sent',
  'queued',
  'retryable-failure',
  'permanent-failure',
  'receipt-required',
  'manual-required',
] as const;

export const GeneratedV3NotificationRetryPolicyStates = [
  'no-retry',
  'exponential-backoff',
  'quiet-hours-deferred',
  'manual-review',
  'provider-disabled',
] as const;

export const GeneratedV3NotificationQuietHoursDecisions = [
  'allow',
  'defer-noncritical',
  'emergency-override',
  'manual-required',
] as const;

export const GeneratedV3NotificationEscalationDecisions = [
  'none',
  'wait-window',
  'escalate-parent',
  'manual-review',
] as const;

export const GeneratedV3NotificationParentPreferenceStates = [
  'enabled',
  'quiet-hours-active',
  'channel-disabled',
  'manual-setup-required',
] as const;

export const GeneratedV3NotificationRuleProviderRetryContractGeneratedAt = '2026-06-02T15:18:13.000Z' as const;

export const GeneratedV3NotificationRuleProviderRetryContractSourceReadModelIds = [
  'reports-notifications-sync-provider-status',
  'v0-8-integrity-alert-status-bridge',
  'data-custody-provider-boundary',
  'notification-feature-expectations-contract-boundary',
] as const;

export const GeneratedV3NotificationRuleProviderRetryContractReadModel = {
  schemaVersion: 'v0.6',
  readModelId: 'v3-notification-rule-provider-retry-contract',
  generatedAt: '2026-06-02T15:18:13.000Z',
  sourceReadModelIds: [
    'reports-notifications-sync-provider-status',
    'v0-8-integrity-alert-status-bridge',
    'data-custody-provider-boundary',
    'notification-feature-expectations-contract-boundary',
  ],
  entries: [
    {
      schemaVersion: 'v0.6',
      contractEntryId: 'notification-rule-policy-violation-push-queued',
      reasonCode: 'policy-violation',
      providerChannel: 'push',
      deliveryAttemptState: 'queued',
      deliveryResultState: 'queued',
      retryPolicyState: 'no-retry',
      quietHoursDecision: 'emergency-override',
      escalationDecision: 'escalate-parent',
      parentPreferenceState: 'enabled',
      notificationRuleRef: 'notification-rule-policy-violation-ref',
      notificationIntentRef: 'notification-intent-policy-violation-ref',
      deliveryAttemptRef: 'delivery-attempt-policy-violation-push-ref',
      deliveryResultRef: 'delivery-result-policy-violation-queued-ref',
      retryPolicyRef: 'retry-policy-no-retry-critical-ref',
      quietHoursPolicyRef: 'quiet-hours-emergency-override-ref',
      escalationPolicyRef: 'escalation-policy-escalate-parent-ref',
      parentPreferenceRef: 'parent-preference-push-enabled-ref',
      auditRefs: ['notification-audit-policy-violation-ref'],
      evidenceRefs: ['policy-decision-evidence-ref', 'authenticated-drill-in-ref'],
      providerReceiptRefs: [],
      manualProofRequirements: [],
      minimalProviderPayloadBoundary:
        'Critical policy-violation payload carries only alert id, severity, reason code, evidence ref, policy ref, and parent action link.',
      providerAdapterImplemented: false,
      deliveryAttemptExecuted: false,
      providerReceiptObserved: false,
      rawEvidenceInProviderPayload: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: '2026-06-02T15:18:13.000Z',
    },
    {
      schemaVersion: 'v0.6',
      contractEntryId: 'notification-rule-parent-request-in-app-receipt-required',
      reasonCode: 'parent-request',
      providerChannel: 'in-app',
      deliveryAttemptState: 'queued',
      deliveryResultState: 'receipt-required',
      retryPolicyState: 'no-retry',
      quietHoursDecision: 'allow',
      escalationDecision: 'wait-window',
      parentPreferenceState: 'enabled',
      notificationRuleRef: 'notification-rule-parent-request-ref',
      notificationIntentRef: 'notification-intent-parent-request-ref',
      deliveryAttemptRef: 'delivery-attempt-parent-request-in-app-ref',
      deliveryResultRef: 'delivery-result-parent-request-receipt-required-ref',
      retryPolicyRef: 'retry-policy-no-retry-parent-action-ref',
      quietHoursPolicyRef: 'quiet-hours-allow-parent-action-ref',
      escalationPolicyRef: 'escalation-policy-wait-window-ref',
      parentPreferenceRef: 'parent-preference-in-app-enabled-ref',
      auditRefs: ['notification-audit-parent-request-ref'],
      evidenceRefs: ['parent-request-ref', 'authenticated-parent-action-ref'],
      providerReceiptRefs: ['provider-receipt-parent-action-required-ref'],
      manualProofRequirements: ['real in-app receipt artifact before parent notification delivery can be claimed'],
      minimalProviderPayloadBoundary:
        'Ask-parent payload carries intent ref and parent action link; sensitive child detail remains behind authenticated parent surfaces.',
      providerAdapterImplemented: false,
      deliveryAttemptExecuted: false,
      providerReceiptObserved: false,
      rawEvidenceInProviderPayload: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: '2026-06-02T15:18:13.000Z',
    },
    {
      schemaVersion: 'v0.6',
      contractEntryId: 'notification-rule-suspicious-unknown-email-retryable-failure',
      reasonCode: 'suspicious-unknown',
      providerChannel: 'email',
      deliveryAttemptState: 'retry-scheduled',
      deliveryResultState: 'retryable-failure',
      retryPolicyState: 'exponential-backoff',
      quietHoursDecision: 'allow',
      escalationDecision: 'manual-review',
      parentPreferenceState: 'manual-setup-required',
      notificationRuleRef: 'notification-rule-suspicious-unknown-ref',
      notificationIntentRef: 'notification-intent-suspicious-unknown-ref',
      deliveryAttemptRef: 'delivery-attempt-suspicious-unknown-email-ref',
      deliveryResultRef: 'delivery-result-suspicious-unknown-retryable-ref',
      retryPolicyRef: 'retry-policy-exponential-backoff-ref',
      quietHoursPolicyRef: 'quiet-hours-allow-suspicious-unknown-ref',
      escalationPolicyRef: 'escalation-policy-manual-review-ref',
      parentPreferenceRef: 'parent-preference-email-setup-required-ref',
      auditRefs: ['notification-audit-suspicious-unknown-ref'],
      evidenceRefs: ['classified-evidence-ref', 'notification-intent-audit-ref'],
      providerReceiptRefs: [],
      manualProofRequirements: ['provider error artifact before retry execution can be claimed'],
      minimalProviderPayloadBoundary:
        'Suspicious-unknown payload avoids raw observation details and carries only reason, severity, evidence ref, and authenticated drill-in.',
      providerAdapterImplemented: false,
      deliveryAttemptExecuted: false,
      providerReceiptObserved: false,
      rawEvidenceInProviderPayload: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: '2026-06-02T15:18:13.000Z',
    },
    {
      schemaVersion: 'v0.6',
      contractEntryId: 'notification-rule-device-offline-sms-permanent-failure',
      reasonCode: 'device-offline',
      providerChannel: 'sms',
      deliveryAttemptState: 'failed-final',
      deliveryResultState: 'permanent-failure',
      retryPolicyState: 'manual-review',
      quietHoursDecision: 'manual-required',
      escalationDecision: 'manual-review',
      parentPreferenceState: 'manual-setup-required',
      notificationRuleRef: 'notification-rule-device-offline-ref',
      notificationIntentRef: 'notification-intent-device-offline-ref',
      deliveryAttemptRef: 'delivery-attempt-device-offline-sms-ref',
      deliveryResultRef: 'delivery-result-device-offline-permanent-failure-ref',
      retryPolicyRef: 'retry-policy-manual-review-ref',
      quietHoursPolicyRef: 'quiet-hours-manual-required-ref',
      escalationPolicyRef: 'escalation-policy-manual-review-ref',
      parentPreferenceRef: 'parent-preference-sms-setup-required-ref',
      auditRefs: ['notification-audit-device-offline-ref'],
      evidenceRefs: ['device-health-status-ref', 'offline-window-evidence-ref'],
      providerReceiptRefs: [],
      manualProofRequirements: [
        'provider failure artifact and parent preference setup before SMS retry can be claimed',
      ],
      minimalProviderPayloadBoundary:
        'Device-offline SMS payload carries device scope, reason code, and action link only; raw child activity is excluded.',
      providerAdapterImplemented: false,
      deliveryAttemptExecuted: false,
      providerReceiptObserved: false,
      rawEvidenceInProviderPayload: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: '2026-06-02T15:18:13.000Z',
    },
    {
      schemaVersion: 'v0.6',
      contractEntryId: 'notification-rule-sync-failure-whatsapp-quiet-hours-deferred',
      reasonCode: 'sync-failure',
      providerChannel: 'whatsapp',
      deliveryAttemptState: 'suppressed-quiet-hours',
      deliveryResultState: 'manual-required',
      retryPolicyState: 'quiet-hours-deferred',
      quietHoursDecision: 'defer-noncritical',
      escalationDecision: 'wait-window',
      parentPreferenceState: 'quiet-hours-active',
      notificationRuleRef: 'notification-rule-sync-failure-ref',
      notificationIntentRef: 'notification-intent-sync-failure-ref',
      deliveryAttemptRef: 'delivery-attempt-sync-failure-whatsapp-ref',
      deliveryResultRef: 'delivery-result-sync-failure-quiet-hours-deferred-ref',
      retryPolicyRef: 'retry-policy-quiet-hours-deferred-ref',
      quietHoursPolicyRef: 'quiet-hours-defer-noncritical-ref',
      escalationPolicyRef: 'escalation-policy-wait-window-sync-ref',
      parentPreferenceRef: 'parent-preference-whatsapp-quiet-hours-ref',
      auditRefs: ['notification-audit-sync-failure-ref'],
      evidenceRefs: ['sync-failure-state-ref', 'parent-owned-storage-ref'],
      providerReceiptRefs: [],
      manualProofRequirements: ['quiet-hours preference artifact before deferred provider send can be claimed'],
      minimalProviderPayloadBoundary:
        'Sync-failure payload is deferable and references parent-owned storage state without embedding report or raw evidence content.',
      providerAdapterImplemented: false,
      deliveryAttemptExecuted: false,
      providerReceiptObserved: false,
      rawEvidenceInProviderPayload: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: '2026-06-02T15:18:13.000Z',
    },
    {
      schemaVersion: 'v0.6',
      contractEntryId: 'notification-rule-provider-failure-in-app-channel-disabled',
      reasonCode: 'provider-failure',
      providerChannel: 'in-app',
      deliveryAttemptState: 'provider-disabled',
      deliveryResultState: 'not-sent',
      retryPolicyState: 'provider-disabled',
      quietHoursDecision: 'allow',
      escalationDecision: 'none',
      parentPreferenceState: 'channel-disabled',
      notificationRuleRef: 'notification-rule-provider-failure-ref',
      notificationIntentRef: 'notification-intent-provider-failure-ref',
      deliveryAttemptRef: 'delivery-attempt-provider-disabled-ref',
      deliveryResultRef: 'delivery-result-not-sent-provider-disabled-ref',
      retryPolicyRef: 'retry-policy-provider-disabled-ref',
      quietHoursPolicyRef: 'quiet-hours-allow-provider-failure-ref',
      escalationPolicyRef: 'escalation-policy-none-provider-disabled-ref',
      parentPreferenceRef: 'parent-preference-channel-disabled-ref',
      auditRefs: ['notification-audit-provider-disabled-ref'],
      evidenceRefs: ['provider-configuration-state-ref', 'notification-routing-status-ref'],
      providerReceiptRefs: [],
      manualProofRequirements: ['provider enablement and credential review before send or retry can be claimed'],
      minimalProviderPayloadBoundary:
        'Provider-failure row is an audit and preference state only; no provider payload is sent while the channel is disabled.',
      providerAdapterImplemented: false,
      deliveryAttemptExecuted: false,
      providerReceiptObserved: false,
      rawEvidenceInProviderPayload: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: '2026-06-02T15:18:13.000Z',
    },
  ],
} as const;

type GeneratedV3NotificationRuleReasonCode = (typeof GeneratedV3NotificationRuleReasonCodes)[number];
type GeneratedV3NotificationProviderChannel = (typeof GeneratedV3NotificationProviderChannels)[number];
type GeneratedV3NotificationDeliveryResultState = (typeof GeneratedV3NotificationDeliveryResultStates)[number];
type GeneratedV3NotificationRetryPolicyState = (typeof GeneratedV3NotificationRetryPolicyStates)[number];
type GeneratedV3NotificationQuietHoursDecision = (typeof GeneratedV3NotificationQuietHoursDecisions)[number];
type GeneratedV3NotificationEscalationDecision = (typeof GeneratedV3NotificationEscalationDecisions)[number];
type GeneratedV3NotificationParentPreferenceState = (typeof GeneratedV3NotificationParentPreferenceStates)[number];

type GeneratedV3NotificationRuleProviderRetryContractEntry = {
  readonly schemaVersion: string;
  readonly contractEntryId: string;
  readonly reasonCode: GeneratedV3NotificationRuleReasonCode;
  readonly providerChannel: GeneratedV3NotificationProviderChannel;
  readonly deliveryAttemptState: string;
  readonly deliveryResultState: GeneratedV3NotificationDeliveryResultState;
  readonly retryPolicyState: GeneratedV3NotificationRetryPolicyState;
  readonly quietHoursDecision: GeneratedV3NotificationQuietHoursDecision;
  readonly escalationDecision: GeneratedV3NotificationEscalationDecision;
  readonly parentPreferenceState: GeneratedV3NotificationParentPreferenceState;
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
  return [
    !generatedV3NotificationRuleProviderRetryContractHasRuntimeClaim(entry),
    generatedV3NotificationRuleProviderRetryContractHasRequiredRefs(entry),
    generatedV3NotificationRuleProviderRetryContractDeliveryStateIsCoherent(entry),
  ].every(Boolean);
}

export function generatedV3NotificationRuleProviderRetryContractReadModelIsHonest(
  readModel: GeneratedV3NotificationRuleProviderRetryContractReadModel
): boolean {
  const hasUniqueContractEntryIds =
    new Set(readModel.entries.map(generatedV3NotificationRuleProviderRetryContractEntryId)).size ===
    readModel.entries.length;
  return [
    hasUniqueContractEntryIds,
    generatedV3NotificationRuleProviderRetryContractCoversReasonCodes(readModel.entries),
    generatedV3NotificationRuleProviderRetryContractCoversProviderChannels(readModel.entries),
    generatedV3NotificationRuleProviderRetryContractCoversDeliveryAndRetry(readModel.entries),
  ].every(Boolean);
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

function generatedV3NotificationRuleProviderRetryContractEntryId(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): string {
  return entry.contractEntryId;
}

function generatedV3NotificationRuleProviderRetryContractTrimmedTextExists(value: string): boolean {
  return value.trim().length > 0;
}

function generatedV3NotificationRuleProviderRetryContractReasonCode(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): GeneratedV3NotificationRuleReasonCode {
  return entry.reasonCode;
}

function generatedV3NotificationRuleProviderRetryContractProviderChannel(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): GeneratedV3NotificationProviderChannel {
  return entry.providerChannel;
}

function generatedV3NotificationRuleProviderRetryContractDeliveryResultState(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): GeneratedV3NotificationDeliveryResultState {
  return entry.deliveryResultState;
}

function generatedV3NotificationRuleProviderRetryContractRetryPolicyState(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): GeneratedV3NotificationRetryPolicyState {
  return entry.retryPolicyState;
}

function generatedV3NotificationRuleProviderRetryContractQuietHoursDecision(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): GeneratedV3NotificationQuietHoursDecision {
  return entry.quietHoursDecision;
}

function generatedV3NotificationRuleProviderRetryContractEscalationDecision(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): GeneratedV3NotificationEscalationDecision {
  return entry.escalationDecision;
}

function generatedV3NotificationRuleProviderRetryContractParentPreferenceState(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): GeneratedV3NotificationParentPreferenceState {
  return entry.parentPreferenceState;
}

function generatedV3NotificationRuleProviderRetryContractSetContainsAll<T>(
  values: readonly T[],
  set: ReadonlySet<T>
): boolean {
  return values.map(set.has, set).every(Boolean);
}

function generatedV3NotificationRuleProviderRetryContractHasRequiredRefs(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  const requiredRefs = [
    entry.notificationRuleRef,
    entry.notificationIntentRef,
    entry.deliveryAttemptRef,
    entry.deliveryResultRef,
    entry.retryPolicyRef,
    entry.quietHoursPolicyRef,
    entry.escalationPolicyRef,
    entry.parentPreferenceRef,
    entry.minimalProviderPayloadBoundary,
  ];

  return [
    entry.auditRefs.length > 0,
    entry.evidenceRefs.length > 0,
    requiredRefs.every(generatedV3NotificationRuleProviderRetryContractTrimmedTextExists),
  ].every(Boolean);
}

function generatedV3NotificationRuleProviderRetryContractDeliveryStateIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return [
    generatedV3NotificationRuleProviderRetryContractQuietHoursIsCoherent(entry),
    generatedV3NotificationRuleProviderRetryContractParentPreferenceIsCoherent(entry),
    generatedV3NotificationRuleProviderRetryContractRetryableFailureIsCoherent(entry),
    generatedV3NotificationRuleProviderRetryContractReceiptRequiredIsCoherent(entry),
    generatedV3NotificationRuleProviderRetryContractPermanentFailureIsCoherent(entry),
    generatedV3NotificationRuleProviderRetryContractNonReceiptRowsAreCoherent(entry),
  ].every(Boolean);
}

function generatedV3NotificationRuleProviderRetryContractImplicationIsHonest(
  condition: boolean,
  requirements: readonly boolean[]
): boolean {
  return [!condition, requirements.every(Boolean)].includes(true);
}

function generatedV3NotificationRuleProviderRetryContractQuietHoursIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return generatedV3NotificationRuleProviderRetryContractImplicationIsHonest(
    entry.quietHoursDecision === 'defer-noncritical',
    [
      entry.deliveryAttemptState === 'suppressed-quiet-hours',
      entry.retryPolicyState === 'quiet-hours-deferred',
      entry.parentPreferenceState === 'quiet-hours-active',
    ]
  );
}

function generatedV3NotificationRuleProviderRetryContractParentPreferenceIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return generatedV3NotificationRuleProviderRetryContractImplicationIsHonest(
    entry.parentPreferenceState === 'channel-disabled',
    [
      entry.deliveryAttemptState === 'provider-disabled',
      entry.deliveryResultState === 'not-sent',
      entry.retryPolicyState === 'provider-disabled',
    ]
  );
}

function generatedV3NotificationRuleProviderRetryContractRetryableFailureIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return generatedV3NotificationRuleProviderRetryContractImplicationIsHonest(
    entry.deliveryResultState === 'retryable-failure',
    [entry.retryPolicyState === 'exponential-backoff', entry.manualProofRequirements.length > 0]
  );
}

function generatedV3NotificationRuleProviderRetryContractReceiptRequiredIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return generatedV3NotificationRuleProviderRetryContractImplicationIsHonest(
    entry.deliveryResultState === 'receipt-required',
    [
      entry.providerReceiptRefs.length > 0,
      entry.manualProofRequirements.length > 0,
      entry.deliveryAttemptState === 'queued',
    ]
  );
}

function generatedV3NotificationRuleProviderRetryContractPermanentFailureIsCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return generatedV3NotificationRuleProviderRetryContractImplicationIsHonest(
    entry.deliveryResultState === 'permanent-failure',
    [entry.retryPolicyState === 'manual-review', entry.manualProofRequirements.length > 0]
  );
}

function generatedV3NotificationRuleProviderRetryContractNonReceiptRowsAreCoherent(
  entry: GeneratedV3NotificationRuleProviderRetryContractEntry
): boolean {
  return generatedV3NotificationRuleProviderRetryContractImplicationIsHonest(
    entry.deliveryResultState !== 'receipt-required',
    [entry.providerReceiptRefs.length === 0]
  );
}

function generatedV3NotificationRuleProviderRetryContractCoversReasonCodes(
  entries: readonly GeneratedV3NotificationRuleProviderRetryContractEntry[]
): boolean {
  const reasonCodes = new Set(entries.map(generatedV3NotificationRuleProviderRetryContractReasonCode));
  return generatedV3NotificationRuleProviderRetryContractSetContainsAll(
    GeneratedV3NotificationRuleReasonCodes,
    reasonCodes
  );
}

function generatedV3NotificationRuleProviderRetryContractCoversProviderChannels(
  entries: readonly GeneratedV3NotificationRuleProviderRetryContractEntry[]
): boolean {
  const channels = new Set(entries.map(generatedV3NotificationRuleProviderRetryContractProviderChannel));
  return generatedV3NotificationRuleProviderRetryContractSetContainsAll(
    GeneratedV3NotificationProviderChannels,
    channels
  );
}

function generatedV3NotificationRuleProviderRetryContractCoversDeliveryAndRetry(
  entries: readonly GeneratedV3NotificationRuleProviderRetryContractEntry[]
): boolean {
  const deliveryResults = new Set(entries.map(generatedV3NotificationRuleProviderRetryContractDeliveryResultState));
  const retryPolicies = new Set(entries.map(generatedV3NotificationRuleProviderRetryContractRetryPolicyState));
  const quietHours = new Set(entries.map(generatedV3NotificationRuleProviderRetryContractQuietHoursDecision));
  const escalation = new Set(entries.map(generatedV3NotificationRuleProviderRetryContractEscalationDecision));
  const preferences = new Set(entries.map(generatedV3NotificationRuleProviderRetryContractParentPreferenceState));

  return [
    generatedV3NotificationRuleProviderRetryContractSetContainsAll(
      GeneratedV3NotificationDeliveryResultStates,
      deliveryResults
    ),
    generatedV3NotificationRuleProviderRetryContractSetContainsAll(
      GeneratedV3NotificationRetryPolicyStates,
      retryPolicies
    ),
    generatedV3NotificationRuleProviderRetryContractSetContainsAll(
      GeneratedV3NotificationQuietHoursDecisions,
      quietHours
    ),
    generatedV3NotificationRuleProviderRetryContractSetContainsAll(
      GeneratedV3NotificationEscalationDecisions,
      escalation
    ),
    generatedV3NotificationRuleProviderRetryContractSetContainsAll(
      GeneratedV3NotificationParentPreferenceStates,
      preferences
    ),
  ].every(Boolean);
}
