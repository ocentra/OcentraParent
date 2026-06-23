import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';

import {
  supportBackendUploadCustodyAuditCoversRequiredStates,
  supportBackendUploadCustodyAuditEntryIsSafe,
} from './support-backend-upload-custody-audit-guards.js';

const supportBackendUploadCustodyAuditText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const SupportBackendUploadCustodyAuditReadModelIdSchema = supportBackendUploadCustodyAuditText(
  'SupportBackendUploadCustodyAuditReadModelId'
);
export const SupportBackendUploadCustodyAuditIdSchema = supportBackendUploadCustodyAuditText(
  'SupportBackendUploadCustodyAuditId'
);
export const SupportBackendUploadCustodyAuditReferenceSchema = supportBackendUploadCustodyAuditText(
  'SupportBackendUploadCustodyAuditReference'
);
export const SupportBackendUploadCustodyAuditRequirementSchema = supportBackendUploadCustodyAuditText(
  'SupportBackendUploadCustodyAuditRequirement'
);
export const SupportBackendUploadCustodyAuditTimestampSchema = supportBackendUploadCustodyAuditText(
  'SupportBackendUploadCustodyAuditTimestamp'
);

export const SupportBackendUploadCustodyAuditStateSchema = withParser(
  Schema.Literal(
    'custody-boundary-recorded',
    'retention-manual-required',
    'delete-request-recorded',
    'deletion-manual-required',
    'audit-export-ready'
  )
);

export const SupportBackendUploadCustodyAuditParentInitiationStateSchema = withParser(
  Schema.Literal('parent-initiated')
);
export const SupportBackendUploadCustodyAuditParentConsentStateSchema = withParser(
  Schema.Literal('parent-approved', 'required', 'revoked')
);
export const SupportBackendUploadCustodyAuditRetentionStateSchema = withParser(
  Schema.Literal('manual-required', 'not-applicable')
);
export const SupportBackendUploadCustodyAuditDeleteStateSchema = withParser(
  Schema.Literal('manual-required', 'not-requested')
);
export const SupportBackendUploadCustodyAuditExportStateSchema = withParser(
  Schema.Literal('support-safe-export-ready', 'manual-required')
);
export const SupportBackendUploadCustodyAuditExecutionClaimStateSchema = withParser(
  Schema.Literal('custody-audit-boundary-only')
);
export const SupportBackendUploadCustodyAuditPayloadStateSchema = withParser(
  Schema.Literal('redacted-audit-refs-only')
);
export const SupportBackendUploadCustodyAuditCustodyStateSchema = withParser(
  Schema.Literal('parent-owned-export-only')
);

export const SupportBackendUploadCustodyAuditDataClassSchema = withParser(
  Schema.Literal(
    'parent-consent-artifact-ref',
    'redaction-summary-ref',
    'support-upload-status-ref',
    'support-upload-runtime-ref',
    'custody-boundary-ref',
    'retention-manual-proof-ref',
    'delete-request-ref',
    'delete-manual-proof-ref',
    'audit-export-ref',
    'manual-runbook-ref'
  )
);

export const SupportBackendUploadCustodyAuditRequiredDataClasses = [
  'parent-consent-artifact-ref',
  'redaction-summary-ref',
  'support-upload-status-ref',
  'support-upload-runtime-ref',
  'custody-boundary-ref',
  'retention-manual-proof-ref',
  'delete-request-ref',
  'delete-manual-proof-ref',
  'audit-export-ref',
  'manual-runbook-ref',
] as const satisfies ReadonlyArray<SupportBackendUploadCustodyAuditDataClass>;

const SupportBackendUploadCustodyAuditEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  auditId: SupportBackendUploadCustodyAuditIdSchema,
  auditState: SupportBackendUploadCustodyAuditStateSchema,
  parentInitiationState: SupportBackendUploadCustodyAuditParentInitiationStateSchema,
  parentConsentState: SupportBackendUploadCustodyAuditParentConsentStateSchema,
  executionClaimState: SupportBackendUploadCustodyAuditExecutionClaimStateSchema,
  payloadState: SupportBackendUploadCustodyAuditPayloadStateSchema,
  custodyState: SupportBackendUploadCustodyAuditCustodyStateSchema,
  retentionState: SupportBackendUploadCustodyAuditRetentionStateSchema,
  deleteState: SupportBackendUploadCustodyAuditDeleteStateSchema,
  auditExportState: SupportBackendUploadCustodyAuditExportStateSchema,
  disclosedDataClasses: Schema.Array(SupportBackendUploadCustodyAuditDataClassSchema),
  consentRefs: Schema.Array(SupportBackendUploadCustodyAuditReferenceSchema),
  redactionRefs: Schema.Array(SupportBackendUploadCustodyAuditReferenceSchema),
  auditRefs: Schema.Array(SupportBackendUploadCustodyAuditReferenceSchema),
  statusRefs: Schema.Array(SupportBackendUploadCustodyAuditReferenceSchema),
  runtimeRefs: Schema.Array(SupportBackendUploadCustodyAuditReferenceSchema),
  custodyRefs: Schema.Array(SupportBackendUploadCustodyAuditReferenceSchema),
  retentionRefs: Schema.Array(SupportBackendUploadCustodyAuditReferenceSchema),
  deleteRefs: Schema.Array(SupportBackendUploadCustodyAuditReferenceSchema),
  manualProofRequirements: Schema.Array(SupportBackendUploadCustodyAuditRequirementSchema),
  containsTokens: Schema.Boolean,
  containsRawChildActivity: Schema.Boolean,
  containsRawUrls: Schema.Boolean,
  containsScreenshots: Schema.Boolean,
  containsJournals: Schema.Boolean,
  containsSqliteSnapshots: Schema.Boolean,
  containsPrivatePaths: Schema.Boolean,
  containsCommandLines: Schema.Boolean,
  containsKeystrokes: Schema.Boolean,
  containsClipboardData: Schema.Boolean,
  containsMessageContents: Schema.Boolean,
  containsProviderSecrets: Schema.Boolean,
  containsRemoteSupportTranscripts: Schema.Boolean,
  realSupportBackendUploadExecuted: Schema.Boolean,
  supportBackendRetainedPayload: Schema.Boolean,
  supportBackendDeletedPayload: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  billingProviderContactExecuted: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  lastCheckedAt: SupportBackendUploadCustodyAuditTimestampSchema,
});

export type SupportBackendUploadCustodyAuditEntryCandidate = Infer<
  typeof SupportBackendUploadCustodyAuditEntryBaseSchema
>;

export const SupportBackendUploadCustodyAuditEntrySchema = withParser(
  SupportBackendUploadCustodyAuditEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        supportBackendUploadCustodyAuditEntryIsSafe(entry, SupportBackendUploadCustodyAuditRequiredDataClasses) ||
        'Expected support backend upload custody audit rows to be parent-consented, redaction-backed, status-linked, runtime-linked, custody-boundary-only, and free of backend execution, hosted payload retention/deletion, child activity custody, provider secrets, account lookup, billing contact, remote support, production SLA, or default Ocentra-hosted family data claims'
    )
  )
);

export const SupportBackendUploadCustodyAuditReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: SupportBackendUploadCustodyAuditReadModelIdSchema,
    generatedAt: SupportBackendUploadCustodyAuditTimestampSchema,
    sourceContractRefs: Schema.Array(SupportBackendUploadCustodyAuditReferenceSchema),
    entries: Schema.Array(SupportBackendUploadCustodyAuditEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.auditId)).size === readModel.entries.length ||
        'Expected support backend upload custody audit ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        supportBackendUploadCustodyAuditCoversRequiredStates(readModel.entries) ||
        'Expected support backend upload custody audit proof to cover custody boundary, retention manual, delete request, deletion manual, and audit export rows'
    )
  )
);

export type SupportBackendUploadCustodyAuditState = Infer<typeof SupportBackendUploadCustodyAuditStateSchema>;
export type SupportBackendUploadCustodyAuditRetentionState = Infer<
  typeof SupportBackendUploadCustodyAuditRetentionStateSchema
>;
export type SupportBackendUploadCustodyAuditDeleteState = Infer<
  typeof SupportBackendUploadCustodyAuditDeleteStateSchema
>;
export type SupportBackendUploadCustodyAuditExportState = Infer<
  typeof SupportBackendUploadCustodyAuditExportStateSchema
>;
export type SupportBackendUploadCustodyAuditDataClass = Infer<typeof SupportBackendUploadCustodyAuditDataClassSchema>;
export type SupportBackendUploadCustodyAuditEntry = Infer<typeof SupportBackendUploadCustodyAuditEntrySchema>;
export type SupportBackendUploadCustodyAuditReadModel = Infer<typeof SupportBackendUploadCustodyAuditReadModelSchema>;

export const decodeSupportBackendUploadCustodyAuditEntry = Schema.decodeUnknownSync(
  SupportBackendUploadCustodyAuditEntrySchema
);
export const decodeSupportBackendUploadCustodyAuditReadModel = Schema.decodeUnknownSync(
  SupportBackendUploadCustodyAuditReadModelSchema
);
