import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyTamperIntegrityAuditText = Schema.String.pipe(Schema.minLength(1));

const tamperIntegrityAuditText = <Brand extends string>(brand: Brand) =>
  NonEmptyTamperIntegrityAuditText.pipe(Schema.brand(brand));
export const TamperIntegrityAuditReadModelIdSchema = tamperIntegrityAuditText('TamperIntegrityAuditReadModelId');
export const TamperIntegrityAuditEntryIdSchema = tamperIntegrityAuditText('TamperIntegrityAuditEntryId');
export const TamperIntegrityAuditReferenceSchema = tamperIntegrityAuditText('TamperIntegrityAuditReference');
export const TamperIntegrityAuditRequirementSchema = tamperIntegrityAuditText('TamperIntegrityAuditRequirement');
export const TamperIntegrityAuditNonClaimSchema = tamperIntegrityAuditText('TamperIntegrityAuditNonClaim');
export const TamperIntegrityAuditTimestampSchema = tamperIntegrityAuditText('TamperIntegrityAuditTimestamp');
export const TamperIntegrityAuditSignalKindSchema = withParser(
  Schema.Literal(
    'heartbeat-stale',
    'heartbeat-offline',
    'permission-loss',
    'service-stopped',
    'agent-removed',
    'uninstall-detected',
    'tamper-manual-required',
    'admin-removal-flow'
  )
);
export const TamperIntegrityAuditHeartbeatStateSchema = withParser(
  Schema.Literal('fresh', 'stale', 'offline', 'not-applicable')
);
export const TamperIntegrityAuditPermissionStateSchema = withParser(
  Schema.Literal('granted', 'permission-lost', 'manual-required', 'not-applicable')
);
export const TamperIntegrityAuditServicePresenceStateSchema = withParser(
  Schema.Literal('running', 'stale', 'offline', 'stopped', 'removed', 'not-applicable')
);
export const TamperIntegrityAuditUninstallStateSchema = withParser(
  Schema.Literal('not-detected', 'detected', 'manual-required', 'not-applicable')
);
export const TamperIntegrityAuditTamperStateSchema = withParser(
  Schema.Literal('not-detected', 'manual-required', 'unsupported', 'not-applicable')
);
export const TamperIntegrityAuditSeveritySchema = withParser(
  Schema.Literal('info', 'warning', 'critical', 'manual-required')
);
export const TamperIntegrityAuditRedactionStateSchema = withParser(Schema.Literal('minimal-operational-fields-only'));
export const TamperIntegrityAuditPayloadFieldSchema = withParser(
  Schema.Literal(
    'audit-entry-ref',
    'family-scope-ref',
    'device-scope-ref',
    'integrity-state',
    'signal-kind',
    'severity',
    'reason-code',
    'first-seen-at',
    'last-seen-at',
    'parent-drill-in-ref',
    'admin-removal-flow-ref',
    'manual-proof-ref'
  )
);

const RequiredSignalKinds = [
  'heartbeat-stale',
  'heartbeat-offline',
  'permission-loss',
  'service-stopped',
  'agent-removed',
  'uninstall-detected',
  'tamper-manual-required',
  'admin-removal-flow',
] as const satisfies ReadonlyArray<TamperIntegrityAuditSignalKind>;

const RequiredPayloadFields = [
  'audit-entry-ref',
  'family-scope-ref',
  'device-scope-ref',
  'integrity-state',
  'signal-kind',
  'severity',
  'reason-code',
  'first-seen-at',
  'last-seen-at',
  'parent-drill-in-ref',
  'admin-removal-flow-ref',
  'manual-proof-ref',
] as const satisfies ReadonlyArray<TamperIntegrityAuditPayloadField>;

export const TamperIntegrityAuditRequiredPayloadFields = RequiredPayloadFields;

const TamperIntegrityAuditEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  auditEntryId: TamperIntegrityAuditEntryIdSchema,
  signalKind: TamperIntegrityAuditSignalKindSchema,
  heartbeatState: TamperIntegrityAuditHeartbeatStateSchema,
  permissionState: TamperIntegrityAuditPermissionStateSchema,
  servicePresenceState: TamperIntegrityAuditServicePresenceStateSchema,
  uninstallState: TamperIntegrityAuditUninstallStateSchema,
  tamperState: TamperIntegrityAuditTamperStateSchema,
  severity: TamperIntegrityAuditSeveritySchema,
  payloadRedactionState: TamperIntegrityAuditRedactionStateSchema,
  auditRefs: Schema.Array(TamperIntegrityAuditReferenceSchema),
  integrityRefs: Schema.Array(TamperIntegrityAuditReferenceSchema),
  parentAlertRefs: Schema.Array(TamperIntegrityAuditReferenceSchema),
  authenticatedDrillInRefs: Schema.Array(TamperIntegrityAuditReferenceSchema),
  adminRemovalFlowRefs: Schema.Array(TamperIntegrityAuditReferenceSchema),
  manualProofRequirements: Schema.Array(TamperIntegrityAuditRequirementSchema),
  nonClaims: Schema.Array(TamperIntegrityAuditNonClaimSchema),
  redactionSafePayloadFields: Schema.Array(TamperIntegrityAuditPayloadFieldSchema),
  providerDeliveryClaimed: Schema.Boolean,
  stealthBehaviorClaimed: Schema.Boolean,
  privilegeEscalationClaimed: Schema.Boolean,
  hiddenPersistenceClaimed: Schema.Boolean,
  blocksAdminRemovalClaimed: Schema.Boolean,
  rawChildDataIncluded: Schema.Boolean,
  rawEvidencePayloadIncluded: Schema.Boolean,
  rawUrlsIncluded: Schema.Boolean,
  screenshotsIncluded: Schema.Boolean,
  commandLinesIncluded: Schema.Boolean,
  privatePathsIncluded: Schema.Boolean,
  messageContentsIncluded: Schema.Boolean,
  lastCheckedAt: TamperIntegrityAuditTimestampSchema,
});

type TamperIntegrityAuditEntryCandidate = Infer<typeof TamperIntegrityAuditEntryBaseSchema>;

export const TamperIntegrityAuditEntrySchema = withParser(
  TamperIntegrityAuditEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        tamperIntegrityAuditEntryIsSafe(entry) ||
        'Expected tamper integrity audit logs to keep stale/offline heartbeat, permission loss, stopped/removed, uninstall, tamper manual, and admin removal refs redaction-safe without stealth, privilege escalation, provider delivery, or raw child data claims'
    )
  )
);

export const TamperIntegrityAuditReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: TamperIntegrityAuditReadModelIdSchema,
    generatedAt: TamperIntegrityAuditTimestampSchema,
    sourceContractRefs: Schema.Array(TamperIntegrityAuditReferenceSchema),
    entries: Schema.Array(TamperIntegrityAuditEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.auditEntryId)).size === readModel.entries.length ||
        'Expected tamper integrity audit entry ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        requiredValuesArePresent(
          readModel.entries.map((entry) => entry.signalKind),
          RequiredSignalKinds
        ) || 'Expected tamper integrity audit to cover every required integrity/tamper signal kind'
    )
  )
);

function tamperIntegrityAuditEntryIsSafe(entry: TamperIntegrityAuditEntryCandidate): boolean {
  return (
    !tamperIntegrityAuditHasClaimUpgrade(entry) &&
    tamperIntegrityAuditHasRequiredRefs(entry) &&
    tamperIntegrityAuditHasRequiredPayloadFields(entry) &&
    tamperIntegrityAuditHasRequiredNonClaims(entry) &&
    tamperIntegrityAuditStatesAreCoherent(entry)
  );
}

function tamperIntegrityAuditHasClaimUpgrade(entry: TamperIntegrityAuditEntryCandidate): boolean {
  return [
    entry.providerDeliveryClaimed,
    entry.stealthBehaviorClaimed,
    entry.privilegeEscalationClaimed,
    entry.hiddenPersistenceClaimed,
    entry.blocksAdminRemovalClaimed,
    entry.rawChildDataIncluded,
    entry.rawEvidencePayloadIncluded,
    entry.rawUrlsIncluded,
    entry.screenshotsIncluded,
    entry.commandLinesIncluded,
    entry.privatePathsIncluded,
    entry.messageContentsIncluded,
  ].some(Boolean);
}

function tamperIntegrityAuditHasRequiredRefs(entry: TamperIntegrityAuditEntryCandidate): boolean {
  return (
    entry.auditRefs.length > 0 &&
    entry.integrityRefs.length > 0 &&
    entry.parentAlertRefs.length > 0 &&
    entry.authenticatedDrillInRefs.length > 0
  );
}

function tamperIntegrityAuditHasRequiredPayloadFields(entry: TamperIntegrityAuditEntryCandidate): boolean {
  return requiredValuesArePresent(entry.redactionSafePayloadFields, RequiredPayloadFields);
}

function tamperIntegrityAuditHasRequiredNonClaims(entry: TamperIntegrityAuditEntryCandidate): boolean {
  return (
    nonClaimIsPresent(entry, 'no stealth behavior is claimed by this audit contract') &&
    nonClaimIsPresent(entry, 'no privilege escalation is claimed by this audit contract') &&
    nonClaimIsPresent(entry, 'no notification provider delivery is claimed by this audit contract') &&
    nonClaimIsPresent(entry, 'no admin removal blocking is claimed by this audit contract')
  );
}

function nonClaimIsPresent(entry: TamperIntegrityAuditEntryCandidate, expected: string): boolean {
  return entry.nonClaims.some((claim) => claim === expected);
}

function tamperIntegrityAuditStatesAreCoherent(entry: TamperIntegrityAuditEntryCandidate): boolean {
  const manualProofPresent = entry.manualProofRequirements.length > 0;
  const adminFlowPresent = entry.adminRemovalFlowRefs.length > 0;
  return (
    (entry.signalKind !== 'heartbeat-stale' || entry.heartbeatState === 'stale') &&
    (entry.signalKind !== 'heartbeat-offline' || entry.heartbeatState === 'offline') &&
    (entry.signalKind !== 'permission-loss' || entry.permissionState === 'permission-lost') &&
    (entry.signalKind !== 'service-stopped' || entry.servicePresenceState === 'stopped') &&
    (entry.signalKind !== 'agent-removed' || entry.servicePresenceState === 'removed') &&
    (entry.signalKind !== 'uninstall-detected' ||
      (entry.uninstallState === 'detected' && adminFlowPresent && manualProofPresent)) &&
    (entry.signalKind !== 'tamper-manual-required' ||
      (entry.tamperState === 'manual-required' && manualProofPresent)) &&
    (entry.signalKind !== 'admin-removal-flow' || adminFlowPresent)
  );
}

function requiredValuesArePresent<T extends string>(
  actualValues: ReadonlyArray<T>,
  requiredValues: ReadonlyArray<T>
): boolean {
  const actual = new Set(actualValues);
  return actual.size === actualValues.length && requiredValues.every((value) => actual.has(value));
}

export type TamperIntegrityAuditSignalKind = Infer<typeof TamperIntegrityAuditSignalKindSchema>;
export type TamperIntegrityAuditHeartbeatState = Infer<typeof TamperIntegrityAuditHeartbeatStateSchema>;
export type TamperIntegrityAuditPermissionState = Infer<typeof TamperIntegrityAuditPermissionStateSchema>;
export type TamperIntegrityAuditServicePresenceState = Infer<typeof TamperIntegrityAuditServicePresenceStateSchema>;
export type TamperIntegrityAuditUninstallState = Infer<typeof TamperIntegrityAuditUninstallStateSchema>;
export type TamperIntegrityAuditTamperState = Infer<typeof TamperIntegrityAuditTamperStateSchema>;
export type TamperIntegrityAuditSeverity = Infer<typeof TamperIntegrityAuditSeveritySchema>;
export type TamperIntegrityAuditPayloadField = Infer<typeof TamperIntegrityAuditPayloadFieldSchema>;
export type TamperIntegrityAuditEntry = Infer<typeof TamperIntegrityAuditEntrySchema>;
export type TamperIntegrityAuditReadModel = Infer<typeof TamperIntegrityAuditReadModelSchema>;

export const decodeTamperIntegrityAuditEntry = Schema.decodeUnknownSync(TamperIntegrityAuditEntrySchema);
export const decodeTamperIntegrityAuditReadModel = Schema.decodeUnknownSync(TamperIntegrityAuditReadModelSchema);
