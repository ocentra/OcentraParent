/* generated from crates/schema/src/v0_8_notification_provider_status_boundary_ts.rs */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './generated-family-reference-primitives';

export const V08NotificationProviderStatusBoundaryReadModelIdSchema = brandedNonEmptyStringSchema(
  'V08NotificationProviderStatusBoundaryReadModelId'
);
export const V08NotificationProviderStatusBoundaryEntryIdSchema = brandedNonEmptyStringSchema(
  'V08NotificationProviderStatusBoundaryEntryId'
);
export const V08NotificationProviderStatusBoundaryReferenceSchema = brandedNonEmptyStringSchema(
  'V08NotificationProviderStatusBoundaryReference'
);
export const V08NotificationProviderStatusBoundaryRequirementSchema = brandedNonEmptyStringSchema(
  'V08NotificationProviderStatusBoundaryRequirement'
);
export const V08NotificationProviderStatusBoundaryTextSchema = brandedNonEmptyStringSchema(
  'V08NotificationProviderStatusBoundaryText'
);

export const V08NotificationProviderStatusSchema = withParser(
  Schema.Literal('queued', 'delivered', 'failed', 'unavailable', 'manual-required')
);

export const V08NotificationProviderStatusProofStateSchema = withParser(
  Schema.Literal(
    'queued-contract-only',
    'delivery-receipt-required',
    'failure-contract-only',
    'provider-unavailable-contract',
    'manual-action-required'
  )
);

export const V08NotificationQuietHoursReadinessSchema = withParser(
  Schema.Literal('ready', 'defer-noncritical', 'manual-required', 'unavailable')
);

export const V08NotificationEscalationReadinessSchema = withParser(
  Schema.Literal('ready', 'waiting-window', 'manual-required', 'unavailable')
);

export const V08NotificationProviderDeliveryClaimSchema = withParser(
  Schema.Literal('not-implemented', 'not-observed', 'receipt-required')
);

const V08NotificationProviderStatusBoundaryEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  statusEntryId: V08NotificationProviderStatusBoundaryEntryIdSchema,
  providerStatus: V08NotificationProviderStatusSchema,
  statusProofState: V08NotificationProviderStatusProofStateSchema,
  quietHoursReadiness: V08NotificationQuietHoursReadinessSchema,
  escalationReadiness: V08NotificationEscalationReadinessSchema,
  deliveryClaimState: V08NotificationProviderDeliveryClaimSchema,
  notificationIntentRef: V08NotificationProviderStatusBoundaryReferenceSchema,
  notificationStatusRef: V08NotificationProviderStatusBoundaryReferenceSchema,
  providerAttemptRef: V08NotificationProviderStatusBoundaryReferenceSchema,
  auditRefs: Schema.Array(V08NotificationProviderStatusBoundaryReferenceSchema),
  preferenceRefs: Schema.Array(V08NotificationProviderStatusBoundaryReferenceSchema),
  readinessRefs: Schema.Array(V08NotificationProviderStatusBoundaryReferenceSchema),
  providerReceiptRefs: Schema.Array(V08NotificationProviderStatusBoundaryReferenceSchema),
  manualProofRequirements: Schema.Array(V08NotificationProviderStatusBoundaryRequirementSchema),
  minimalPayloadBoundary: V08NotificationProviderStatusBoundaryTextSchema,
  providerDeliveryImplemented: Schema.Boolean,
  providerDeliveryObserved: Schema.Boolean,
  deliveredNotificationClaimed: Schema.Boolean,
  sensitiveProviderPayloadClaimed: Schema.Boolean,
  providerStoresChildEvidenceClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08NotificationProviderStatusBoundaryEntryCandidate = Infer<
  typeof V08NotificationProviderStatusBoundaryEntryBaseSchema
>;

export const V08NotificationProviderStatusBoundaryEntrySchema = withParser(
  V08NotificationProviderStatusBoundaryEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        notificationProviderStatusBoundaryEntryIsHonest(entry) ||
        'Expected V0.8 notification provider status rows to represent queued, delivered, failed, unavailable, and manual-required states without implementing provider delivery, observing delivered notifications, embedding sensitive payloads, or storing child evidence in provider metadata'
    )
  )
);

export const V08NotificationProviderStatusBoundaryReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08NotificationProviderStatusBoundaryReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08NotificationProviderStatusBoundaryReferenceSchema),
    entries: Schema.Array(V08NotificationProviderStatusBoundaryEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.statusEntryId)).size === readModel.entries.length ||
        'Expected V0.8 notification provider status entry ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        notificationProviderStatusBoundaryCoversStatuses(readModel.entries) ||
        'Expected V0.8 notification provider status boundary to cover queued, delivered, failed, unavailable, and manual-required provider states'
    ),
    Schema.filter(
      (readModel) =>
        notificationProviderStatusBoundaryCoversReadiness(readModel.entries) ||
        'Expected V0.8 notification provider status boundary to cover quiet-hours and escalation readiness states'
    )
  )
);

function notificationProviderStatusBoundaryEntryIsHonest(
  entry: V08NotificationProviderStatusBoundaryEntryCandidate
): boolean {
  return (
    !notificationProviderStatusBoundaryHasClaimUpgrade(entry) &&
    notificationProviderStatusBoundaryHasRequiredRefs(entry) &&
    notificationProviderStatusBoundaryStatusIsHonest(entry)
  );
}

function notificationProviderStatusBoundaryHasClaimUpgrade(
  entry: V08NotificationProviderStatusBoundaryEntryCandidate
): boolean {
  return [
    entry.providerDeliveryImplemented,
    entry.providerDeliveryObserved,
    entry.deliveredNotificationClaimed,
    entry.sensitiveProviderPayloadClaimed,
    entry.providerStoresChildEvidenceClaimed,
  ].some(Boolean);
}

function notificationProviderStatusBoundaryHasRequiredRefs(
  entry: V08NotificationProviderStatusBoundaryEntryCandidate
): boolean {
  return (
    entry.auditRefs.length > 0 &&
    entry.preferenceRefs.length > 0 &&
    entry.readinessRefs.length > 0 &&
    entry.minimalPayloadBoundary.trim().length > 0
  );
}

function notificationProviderStatusBoundaryStatusIsHonest(
  entry: V08NotificationProviderStatusBoundaryEntryCandidate
): boolean {
  if (entry.providerStatus === 'delivered') {
    return notificationProviderStatusBoundaryDeliveredIsHonest(entry);
  }

  if (entry.providerStatus === 'manual-required') {
    return entry.statusProofState === 'manual-action-required' && entry.manualProofRequirements.length > 0;
  }

  return entry.providerReceiptRefs.length === 0 && entry.deliveryClaimState !== 'receipt-required';
}

function notificationProviderStatusBoundaryDeliveredIsHonest(
  entry: V08NotificationProviderStatusBoundaryEntryCandidate
): boolean {
  return (
    entry.statusProofState === 'delivery-receipt-required' &&
    entry.deliveryClaimState === 'receipt-required' &&
    entry.providerReceiptRefs.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function notificationProviderStatusBoundaryCoversStatuses(
  entries: readonly V08NotificationProviderStatusBoundaryEntry[]
): boolean {
  const statuses = new Set(entries.map((entry) => entry.providerStatus));
  return ['queued', 'delivered', 'failed', 'unavailable', 'manual-required'].every((status) =>
    statuses.has(status as V08NotificationProviderStatus)
  );
}

function notificationProviderStatusBoundaryCoversReadiness(
  entries: readonly V08NotificationProviderStatusBoundaryEntry[]
): boolean {
  const quietHours = new Set(entries.map((entry) => entry.quietHoursReadiness));
  const escalation = new Set(entries.map((entry) => entry.escalationReadiness));
  return (
    ['ready', 'defer-noncritical', 'manual-required', 'unavailable'].every((state) =>
      quietHours.has(state as V08NotificationQuietHoursReadiness)
    ) &&
    ['ready', 'waiting-window', 'manual-required', 'unavailable'].every((state) =>
      escalation.has(state as V08NotificationEscalationReadiness)
    )
  );
}

export type V08NotificationProviderStatusBoundaryReadModelId =
  typeof V08NotificationProviderStatusBoundaryReadModelIdSchema.Type;
export type V08NotificationProviderStatusBoundaryEntryId =
  typeof V08NotificationProviderStatusBoundaryEntryIdSchema.Type;
export type V08NotificationProviderStatusBoundaryReference =
  typeof V08NotificationProviderStatusBoundaryReferenceSchema.Type;
export type V08NotificationProviderStatusBoundaryRequirement =
  typeof V08NotificationProviderStatusBoundaryRequirementSchema.Type;
export type V08NotificationProviderStatusBoundaryText = typeof V08NotificationProviderStatusBoundaryTextSchema.Type;
export type V08NotificationProviderStatus = Infer<typeof V08NotificationProviderStatusSchema>;
export type V08NotificationProviderStatusProofState = Infer<typeof V08NotificationProviderStatusProofStateSchema>;
export type V08NotificationQuietHoursReadiness = Infer<typeof V08NotificationQuietHoursReadinessSchema>;
export type V08NotificationEscalationReadiness = Infer<typeof V08NotificationEscalationReadinessSchema>;
export type V08NotificationProviderDeliveryClaim = Infer<typeof V08NotificationProviderDeliveryClaimSchema>;
export type V08NotificationProviderStatusBoundaryEntry = Infer<typeof V08NotificationProviderStatusBoundaryEntrySchema>;
export type V08NotificationProviderStatusBoundaryReadModel = Infer<
  typeof V08NotificationProviderStatusBoundaryReadModelSchema
>;

type V08NotificationProviderStatusBoundaryEntryInput = {
  statusEntryId: string;
  providerStatus: V08NotificationProviderStatus;
  statusProofState: V08NotificationProviderStatusProofState;
  quietHoursReadiness: V08NotificationQuietHoursReadiness;
  escalationReadiness: V08NotificationEscalationReadiness;
  deliveryClaimState: V08NotificationProviderDeliveryClaim;
  notificationStatusRef: string;
  providerAttemptRef: string;
  readinessRefs: readonly string[];
  providerReceiptRefs: readonly string[];
  manualProofRequirements: readonly string[];
  minimalPayloadBoundary: string;
};

const generatedAt = '2026-06-02T15:18:13.000Z';

const SourceReadModelIds = {
  ReportsNotificationsSync: 'reports-notifications-sync-provider-status',
  IntegrityAlertStatusBridge: 'v0-8-integrity-alert-status-bridge',
  DataCustody: 'data-custody-provider-boundary',
} as const;

export const V08NotificationProviderStatusBoundaryReadModel =
  V08NotificationProviderStatusBoundaryReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'v0-8-notification-provider-status-boundary',
    generatedAt,
    sourceReadModelIds: Object.values(SourceReadModelIds),
    entries: [
      entry({
        statusEntryId: 'notification-provider-queued-contract',
        providerStatus: 'queued',
        statusProofState: 'queued-contract-only',
        quietHoursReadiness: 'ready',
        escalationReadiness: 'ready',
        deliveryClaimState: 'not-implemented',
        notificationStatusRef: 'notification-status-queued-ref',
        providerAttemptRef: 'provider-attempt-queued-ref',
        readinessRefs: ['quiet-hours-ready-ref', 'escalation-ready-ref'],
        providerReceiptRefs: [],
        manualProofRequirements: [],
        minimalPayloadBoundary:
          'Queued means a typed provider attempt can be represented; no provider adapter sends or delivers the alert in this proof.',
      }),
      entry({
        statusEntryId: 'notification-provider-delivered-receipt-required',
        providerStatus: 'delivered',
        statusProofState: 'delivery-receipt-required',
        quietHoursReadiness: 'defer-noncritical',
        escalationReadiness: 'waiting-window',
        deliveryClaimState: 'receipt-required',
        notificationStatusRef: 'notification-status-delivered-contract-ref',
        providerAttemptRef: 'provider-attempt-delivered-contract-ref',
        readinessRefs: ['quiet-hours-defer-noncritical-ref', 'escalation-waiting-window-ref'],
        providerReceiptRefs: ['provider-delivery-receipt-required-ref'],
        manualProofRequirements: ['real provider receipt artifact before delivery can be claimed'],
        minimalPayloadBoundary:
          'Delivered is a contract state for future provider receipts; this read model records no observed delivered notification.',
      }),
      entry({
        statusEntryId: 'notification-provider-failed-contract',
        providerStatus: 'failed',
        statusProofState: 'failure-contract-only',
        quietHoursReadiness: 'ready',
        escalationReadiness: 'manual-required',
        deliveryClaimState: 'not-observed',
        notificationStatusRef: 'notification-status-failed-ref',
        providerAttemptRef: 'provider-attempt-failed-ref',
        readinessRefs: ['quiet-hours-ready-ref', 'escalation-manual-required-ref'],
        providerReceiptRefs: [],
        manualProofRequirements: ['provider error artifact before retry behavior is claimed'],
        minimalPayloadBoundary:
          'Failed status is visible and auditable as a contract state; retry behavior and provider error ingestion remain unimplemented.',
      }),
      entry({
        statusEntryId: 'notification-provider-unavailable-contract',
        providerStatus: 'unavailable',
        statusProofState: 'provider-unavailable-contract',
        quietHoursReadiness: 'unavailable',
        escalationReadiness: 'unavailable',
        deliveryClaimState: 'not-implemented',
        notificationStatusRef: 'notification-status-provider-unavailable-ref',
        providerAttemptRef: 'provider-attempt-unavailable-ref',
        readinessRefs: ['quiet-hours-unavailable-ref', 'escalation-unavailable-ref'],
        providerReceiptRefs: [],
        manualProofRequirements: ['provider configuration or credential review'],
        minimalPayloadBoundary:
          'Unavailable status keeps child safety local and records that no provider adapter is configured or reachable.',
      }),
      entry({
        statusEntryId: 'notification-provider-manual-required-contract',
        providerStatus: 'manual-required',
        statusProofState: 'manual-action-required',
        quietHoursReadiness: 'manual-required',
        escalationReadiness: 'manual-required',
        deliveryClaimState: 'not-observed',
        notificationStatusRef: 'notification-status-manual-required-ref',
        providerAttemptRef: 'provider-attempt-manual-required-ref',
        readinessRefs: ['quiet-hours-manual-required-ref', 'escalation-manual-required-ref'],
        providerReceiptRefs: [],
        manualProofRequirements: ['parent/provider preference setup', 'security review before provider enablement'],
        minimalPayloadBoundary:
          'Manual-required status covers provider setup, quiet-hours, and escalation readiness gaps without sending third-party payloads.',
      }),
    ],
  });

function entry(input: V08NotificationProviderStatusBoundaryEntryInput): V08NotificationProviderStatusBoundaryEntry {
  return V08NotificationProviderStatusBoundaryEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    notificationIntentRef: 'notification-intent-provider-status-boundary-ref',
    auditRefs: ['notification-provider-status-audit-ref'],
    preferenceRefs: ['notification-parent-preferences-ref'],
    providerDeliveryImplemented: false,
    providerDeliveryObserved: false,
    deliveredNotificationClaimed: false,
    sensitiveProviderPayloadClaimed: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}

export const decodeV08NotificationProviderStatusBoundaryEntry = Schema.decodeUnknownSync(
  V08NotificationProviderStatusBoundaryEntrySchema
);
export const decodeV08NotificationProviderStatusBoundaryReadModel = Schema.decodeUnknownSync(
  V08NotificationProviderStatusBoundaryReadModelSchema
);
