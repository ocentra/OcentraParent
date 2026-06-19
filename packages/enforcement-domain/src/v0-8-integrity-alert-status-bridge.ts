import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  enforcementProofClaimFlagsAreUnset,
  enforcementProofEntriesHaveUniqueField,
  enforcementProofRequiredValuesAreCovered,
} from './enforcement-proof-shape';

export const V08IntegrityAlertStatusBridgeReadModelIdSchema = brandedNonEmptyStringSchema('V08IntegrityAlertStatusBridgeReadModelId');
export const V08IntegrityAlertStatusBridgeEntryIdSchema = brandedNonEmptyStringSchema('V08IntegrityAlertStatusBridgeEntryId');
export const V08IntegrityAlertStatusBridgeReferenceSchema = brandedNonEmptyStringSchema('V08IntegrityAlertStatusBridgeReference');
export const V08IntegrityAlertStatusBridgeRequirementSchema = brandedNonEmptyStringSchema('V08IntegrityAlertStatusBridgeRequirement');
export const V08IntegrityAlertStatusBridgeBoundarySchema = brandedNonEmptyStringSchema('V08IntegrityAlertStatusBridgeBoundary');

export const V08IntegrityAlertStateSchema = withParser(
  Schema.Literal('permission-loss', 'stale-heartbeat', 'stopped-or-removed', 'tamper-manual-required')
);

export const V08IntegrityAlertParentVisibleStatusSchema = withParser(
  Schema.Literal(
    'permission-action-required',
    'agent-heartbeat-stale',
    'agent-stopped-or-removed',
    'tamper-review-required'
  )
);

export const V08IntegrityAlertNotificationIntentStateSchema = withParser(
  Schema.Literal('intent-created', 'manual-review-required')
);

export const V08IntegrityAlertDeliveryStateSchema = withParser(Schema.Literal('not-delivered-provider-not-configured'));

export const V08IntegrityAlertAuditStateSchema = withParser(Schema.Literal('audit-ref-backed', 'manual-required'));

const V08IntegrityAlertStatusBridgeEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  bridgeEntryId: V08IntegrityAlertStatusBridgeEntryIdSchema,
  integrityAlertState: V08IntegrityAlertStateSchema,
  parentVisibleStatus: V08IntegrityAlertParentVisibleStatusSchema,
  notificationIntentState: V08IntegrityAlertNotificationIntentStateSchema,
  deliveryState: V08IntegrityAlertDeliveryStateSchema,
  auditState: V08IntegrityAlertAuditStateSchema,
  reasonCodeRef: V08IntegrityAlertStatusBridgeReferenceSchema,
  statusRef: V08IntegrityAlertStatusBridgeReferenceSchema,
  notificationIntentRefs: Schema.Array(V08IntegrityAlertStatusBridgeReferenceSchema),
  notificationStatusRefs: Schema.Array(V08IntegrityAlertStatusBridgeReferenceSchema),
  auditRefs: Schema.Array(V08IntegrityAlertStatusBridgeReferenceSchema),
  integrityRefs: Schema.Array(V08IntegrityAlertStatusBridgeReferenceSchema),
  drillInRefs: Schema.Array(V08IntegrityAlertStatusBridgeReferenceSchema),
  manualProofRequirements: Schema.Array(V08IntegrityAlertStatusBridgeRequirementSchema),
  boundary: V08IntegrityAlertStatusBridgeBoundarySchema,
  providerDeliveryClaimed: Schema.Boolean,
  broadBlockingClaimed: Schema.Boolean,
  tamperResistanceClaimed: Schema.Boolean,
  mobileEnforcementClaimed: Schema.Boolean,
  stealthPersistenceClaimed: Schema.Boolean,
  privilegeEscalationClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08IntegrityAlertStatusBridgeEntryCandidate = Infer<typeof V08IntegrityAlertStatusBridgeEntryBaseSchema>;

export const V08IntegrityAlertStatusBridgeEntrySchema = withParser(
  V08IntegrityAlertStatusBridgeEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        integrityAlertStatusBridgeEntryIsHonest(entry) ||
        'Expected V0.8 integrity alert/status bridge entries to keep notification delivery, anti-tamper, broad blocking, mobile enforcement, stealth, and privilege escalation unclaimed while preserving audit, status, intent, and drill-in references'
    )
  )
);

export const V08IntegrityAlertStatusBridgeReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08IntegrityAlertStatusBridgeReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08IntegrityAlertStatusBridgeReferenceSchema),
    entries: Schema.Array(V08IntegrityAlertStatusBridgeEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        enforcementProofEntriesHaveUniqueField(readModel.entries, (entry) => entry.bridgeEntryId) ||
        'Expected V0.8 integrity alert/status bridge entry ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        integrityAlertStatusBridgeCoversRequiredStates(readModel.entries) ||
        'Expected V0.8 integrity alert/status bridge to cover permission-loss, stale-heartbeat, stopped-or-removed, and tamper-manual-required states'
    )
  )
);

function integrityAlertStatusBridgeEntryIsHonest(entry: V08IntegrityAlertStatusBridgeEntryCandidate): boolean {
  if (integrityAlertStatusBridgeEntryHasClaimUpgrade(entry)) {
    return false;
  }

  return (
    entry.deliveryState === 'not-delivered-provider-not-configured' &&
    entry.notificationIntentRefs.length > 0 &&
    entry.notificationStatusRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.integrityRefs.length > 0 &&
    entry.drillInRefs.length > 0 &&
    integrityAlertStatusBridgeManualProofMatchesState(entry)
  );
}

function integrityAlertStatusBridgeEntryHasClaimUpgrade(entry: V08IntegrityAlertStatusBridgeEntryCandidate): boolean {
  return !enforcementProofClaimFlagsAreUnset([
    entry.providerDeliveryClaimed,
    entry.broadBlockingClaimed,
    entry.tamperResistanceClaimed,
    entry.mobileEnforcementClaimed,
    entry.stealthPersistenceClaimed,
    entry.privilegeEscalationClaimed,
  ]);
}

function integrityAlertStatusBridgeManualProofMatchesState(
  entry: V08IntegrityAlertStatusBridgeEntryCandidate
): boolean {
  if (entry.integrityAlertState === 'tamper-manual-required') {
    return entry.auditState === 'manual-required' && entry.manualProofRequirements.length > 0;
  }

  return entry.auditState === 'audit-ref-backed';
}

function integrityAlertStatusBridgeCoversRequiredStates(
  entries: readonly V08IntegrityAlertStatusBridgeEntry[]
): boolean {
  return enforcementProofRequiredValuesAreCovered(
    entries.map((entry) => entry.integrityAlertState),
    RequiredIntegrityAlertStates
  );
}

export type V08IntegrityAlertStatusBridgeReadModelId = typeof V08IntegrityAlertStatusBridgeReadModelIdSchema.Type;
export type V08IntegrityAlertStatusBridgeEntryId = typeof V08IntegrityAlertStatusBridgeEntryIdSchema.Type;
export type V08IntegrityAlertStatusBridgeReference = typeof V08IntegrityAlertStatusBridgeReferenceSchema.Type;
export type V08IntegrityAlertStatusBridgeRequirement = typeof V08IntegrityAlertStatusBridgeRequirementSchema.Type;
export type V08IntegrityAlertStatusBridgeBoundary = typeof V08IntegrityAlertStatusBridgeBoundarySchema.Type;
export type V08IntegrityAlertState = Infer<typeof V08IntegrityAlertStateSchema>;
export type V08IntegrityAlertParentVisibleStatus = Infer<typeof V08IntegrityAlertParentVisibleStatusSchema>;
export type V08IntegrityAlertNotificationIntentState = Infer<typeof V08IntegrityAlertNotificationIntentStateSchema>;
export type V08IntegrityAlertDeliveryState = Infer<typeof V08IntegrityAlertDeliveryStateSchema>;
export type V08IntegrityAlertAuditState = Infer<typeof V08IntegrityAlertAuditStateSchema>;
export type V08IntegrityAlertStatusBridgeEntry = Infer<typeof V08IntegrityAlertStatusBridgeEntrySchema>;
export type V08IntegrityAlertStatusBridgeReadModel = Infer<typeof V08IntegrityAlertStatusBridgeReadModelSchema>;

const RequiredIntegrityAlertStates = [
  'permission-loss',
  'stale-heartbeat',
  'stopped-or-removed',
  'tamper-manual-required',
] as const satisfies ReadonlyArray<V08IntegrityAlertState>;

type V08IntegrityAlertStatusBridgeEntryInput = {
  bridgeEntryId: string;
  integrityAlertState: V08IntegrityAlertState;
  parentVisibleStatus: V08IntegrityAlertParentVisibleStatus;
  notificationIntentState: V08IntegrityAlertNotificationIntentState;
  auditState: V08IntegrityAlertAuditState;
  reasonCodeRef: string;
  statusRef: string;
  notificationIntentRefs: readonly string[];
  notificationStatusRefs: readonly string[];
  auditRefs: readonly string[];
  integrityRefs: readonly string[];
  drillInRefs: readonly string[];
  manualProofRequirements: readonly string[];
  boundary: string;
};

const generatedAt = '2026-06-02T13:44:41.000Z';

const SourceReadModelIds = {
  EnforcementIntegrityRuntimeAudit: 'v0-8-enforcement-integrity-runtime-audit',
  SupportedAdapterRuntimeProof: 'v0-8-supported-adapter-runtime-proof',
  ReportsNotificationsSync: 'reports-notifications-sync-intent-status',
} as const;

export const V08IntegrityAlertStatusBridgeReadModel = V08IntegrityAlertStatusBridgeReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'v0-8-integrity-alert-status-bridge',
  generatedAt,
  sourceReadModelIds: Object.values(SourceReadModelIds),
  entries: [
    entry({
      bridgeEntryId: 'permission-loss-alert-status',
      integrityAlertState: 'permission-loss',
      parentVisibleStatus: 'permission-action-required',
      notificationIntentState: 'intent-created',
      auditState: 'audit-ref-backed',
      reasonCodeRef: 'reason-permission-loss-ref',
      statusRef: 'status-permission-action-required-ref',
      notificationIntentRefs: ['notification-intent-permission-loss-ref'],
      notificationStatusRefs: ['notification-status-provider-not-configured-ref'],
      auditRefs: ['enforcement-audit-permission-loss-ref'],
      integrityRefs: ['integrity-permission-state-ref'],
      drillInRefs: ['drill-in-permission-loss-audit-ref'],
      manualProofRequirements: ['permission restoration artifact'],
      boundary:
        'Permission loss is parent-visible unavailable status with notification intent and audit refs; provider delivery remains unconfigured and unclaimed.',
    }),
    entry({
      bridgeEntryId: 'stale-heartbeat-alert-status',
      integrityAlertState: 'stale-heartbeat',
      parentVisibleStatus: 'agent-heartbeat-stale',
      notificationIntentState: 'intent-created',
      auditState: 'audit-ref-backed',
      reasonCodeRef: 'reason-stale-heartbeat-ref',
      statusRef: 'status-agent-heartbeat-stale-ref',
      notificationIntentRefs: ['notification-intent-stale-heartbeat-ref'],
      notificationStatusRefs: ['notification-status-provider-not-configured-ref'],
      auditRefs: ['enforcement-audit-stale-heartbeat-ref'],
      integrityRefs: ['integrity-heartbeat-ref'],
      drillInRefs: ['drill-in-stale-heartbeat-audit-ref'],
      manualProofRequirements: ['fresh heartbeat proof'],
      boundary:
        'Stale heartbeat is a degraded integrity status and alert intent; it is not anti-tamper hardening or provider-delivery proof.',
    }),
    entry({
      bridgeEntryId: 'stopped-or-removed-alert-status',
      integrityAlertState: 'stopped-or-removed',
      parentVisibleStatus: 'agent-stopped-or-removed',
      notificationIntentState: 'intent-created',
      auditState: 'audit-ref-backed',
      reasonCodeRef: 'reason-agent-stopped-or-removed-ref',
      statusRef: 'status-agent-stopped-or-removed-ref',
      notificationIntentRefs: ['notification-intent-agent-stopped-or-removed-ref'],
      notificationStatusRefs: ['notification-status-provider-not-configured-ref'],
      auditRefs: ['enforcement-audit-agent-stopped-or-removed-ref'],
      integrityRefs: ['integrity-service-state-ref'],
      drillInRefs: ['drill-in-agent-stopped-or-removed-audit-ref'],
      manualProofRequirements: ['service restart recovery proof', 'uninstall detection artifact'],
      boundary:
        'Stopped or removed service state is represented as an auditable parent status and alert intent; it does not install persistence or anti-removal behavior.',
    }),
    entry({
      bridgeEntryId: 'tamper-manual-alert-status',
      integrityAlertState: 'tamper-manual-required',
      parentVisibleStatus: 'tamper-review-required',
      notificationIntentState: 'manual-review-required',
      auditState: 'manual-required',
      reasonCodeRef: 'reason-tamper-manual-required-ref',
      statusRef: 'status-tamper-review-required-ref',
      notificationIntentRefs: ['notification-intent-tamper-manual-ref'],
      notificationStatusRefs: ['notification-status-provider-not-configured-ref'],
      auditRefs: ['enforcement-audit-tamper-manual-ref'],
      integrityRefs: ['integrity-tamper-signal-ref'],
      drillInRefs: ['drill-in-tamper-manual-audit-ref'],
      manualProofRequirements: [
        'service-manager stop proof',
        'uninstall detection artifact',
        'security review before hardening',
      ],
      boundary:
        'Tamper/uninstall remains manual-required evidence review only; no stealth, persistence, privilege escalation, or anti-tamper resistance is claimed.',
    }),
  ],
});

function entry(input: V08IntegrityAlertStatusBridgeEntryInput): V08IntegrityAlertStatusBridgeEntry {
  return V08IntegrityAlertStatusBridgeEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    deliveryState: 'not-delivered-provider-not-configured',
    providerDeliveryClaimed: false,
    broadBlockingClaimed: false,
    tamperResistanceClaimed: false,
    mobileEnforcementClaimed: false,
    stealthPersistenceClaimed: false,
    privilegeEscalationClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}

export const decodeV08IntegrityAlertStatusBridgeEntry = Schema.decodeUnknownSync(
  V08IntegrityAlertStatusBridgeEntrySchema
);
export const decodeV08IntegrityAlertStatusBridgeReadModel = Schema.decodeUnknownSync(
  V08IntegrityAlertStatusBridgeReadModelSchema
);

