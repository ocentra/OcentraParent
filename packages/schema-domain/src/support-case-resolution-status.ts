import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from './effect';

import {
  supportCaseResolutionStatusCoversRequiredStates,
  supportCaseResolutionStatusEntryIsSafe,
} from './support-case-resolution-status-guards.js';

const supportCaseResolutionText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const SupportCaseResolutionReadModelIdSchema = supportCaseResolutionText('SupportCaseResolutionReadModelId');
export const SupportCaseResolutionCaseIdSchema = supportCaseResolutionText('SupportCaseResolutionCaseId');
export const SupportCaseResolutionReferenceSchema = supportCaseResolutionText('SupportCaseResolutionReference');
export const SupportCaseResolutionRequirementSchema = supportCaseResolutionText('SupportCaseResolutionRequirement');
export const SupportCaseResolutionTimestampSchema = supportCaseResolutionText('SupportCaseResolutionTimestamp');

export const SupportCaseResolutionStatusStateSchema = withParser(
  Schema.Literal(
    'case-opened',
    'triage-ready',
    'parent-update-ready',
    'escalation-manual-required',
    'response-manual-required',
    'closure-ready',
    'sla-manual-required'
  )
);

export const SupportCaseResolutionParentInitiationStateSchema = withParser(Schema.Literal('parent-initiated'));
export const SupportCaseResolutionParentConsentStateSchema = withParser(
  Schema.Literal('parent-approved', 'required', 'revoked')
);
export const SupportCaseResolutionOperatorResponseStateSchema = withParser(
  Schema.Literal('manual-required', 'not-started', 'not-applicable')
);
export const SupportCaseResolutionEscalationStateSchema = withParser(
  Schema.Literal('not-requested', 'manual-required')
);
export const SupportCaseResolutionSlaStateSchema = withParser(Schema.Literal('manual-required', 'not-claimed'));
export const SupportCaseResolutionPayloadStateSchema = withParser(Schema.Literal('support-safe-status-and-refs-only'));
export const SupportCaseResolutionCustodyStateSchema = withParser(Schema.Literal('no-ocentra-hosted-family-data'));

export const SupportCaseResolutionDataClassSchema = withParser(
  Schema.Literal(
    'case-status',
    'parent-consent-ref',
    'incident-status-ref',
    'redaction-summary-ref',
    'support-upload-status-ref',
    'support-contact-ref',
    'escalation-ref',
    'manual-proof-ref',
    'publication-status-ref'
  )
);

export const SupportCaseResolutionDestinationSchema = withParser(
  Schema.Literal('support-safe-case-status-boundary', 'manual-support-operator', 'none')
);

export const SupportCaseResolutionRequiredDataClasses = [
  'case-status',
  'parent-consent-ref',
  'incident-status-ref',
  'redaction-summary-ref',
  'support-upload-status-ref',
  'support-contact-ref',
  'escalation-ref',
  'manual-proof-ref',
  'publication-status-ref',
] as const satisfies ReadonlyArray<SupportCaseResolutionDataClass>;

const SupportCaseResolutionStatusEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  caseId: SupportCaseResolutionCaseIdSchema,
  caseStatus: SupportCaseResolutionStatusStateSchema,
  parentInitiationState: SupportCaseResolutionParentInitiationStateSchema,
  parentConsentState: SupportCaseResolutionParentConsentStateSchema,
  operatorResponseState: SupportCaseResolutionOperatorResponseStateSchema,
  escalationState: SupportCaseResolutionEscalationStateSchema,
  slaState: SupportCaseResolutionSlaStateSchema,
  casePayloadState: SupportCaseResolutionPayloadStateSchema,
  custodyState: SupportCaseResolutionCustodyStateSchema,
  disclosedDataClasses: Schema.Array(SupportCaseResolutionDataClassSchema),
  allowedDestinations: Schema.Array(SupportCaseResolutionDestinationSchema),
  parentConsentRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  incidentRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  redactionRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  auditRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  uploadStatusRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  publicationRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  escalationRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  responseRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  closureRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  slaRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
  manualProofRequirements: Schema.Array(SupportCaseResolutionRequirementSchema),
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
  accountLookupExecuted: Schema.Boolean,
  billingProviderContactExecuted: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  lastCheckedAt: SupportCaseResolutionTimestampSchema,
});

export type SupportCaseResolutionStatusEntryCandidate = Infer<typeof SupportCaseResolutionStatusEntryBaseSchema>;

export const SupportCaseResolutionStatusEntrySchema = withParser(
  SupportCaseResolutionStatusEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        supportCaseResolutionStatusEntryIsSafe(entry, SupportCaseResolutionRequiredDataClasses) ||
        'Expected support case resolution rows to be parent-initiated and consented, support-safe, audit-backed, manual-response aware, and free of child activity custody, provider secrets, remote transcripts, backend execution, account lookup, billing contact, remote support session, production SLA, or hosted family data claims'
    )
  )
);

export const SupportCaseResolutionStatusReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: SupportCaseResolutionReadModelIdSchema,
    generatedAt: SupportCaseResolutionTimestampSchema,
    sourceContractRefs: Schema.Array(SupportCaseResolutionReferenceSchema),
    entries: Schema.Array(SupportCaseResolutionStatusEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.caseId)).size === readModel.entries.length ||
        'Expected support case resolution ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        supportCaseResolutionStatusCoversRequiredStates(readModel.entries) ||
        'Expected support case resolution proof to cover opened, triage, parent update, escalation manual-required, response manual-required, closure, and SLA manual-required rows'
    )
  )
);

export type SupportCaseResolutionStatusState = Infer<typeof SupportCaseResolutionStatusStateSchema>;
export type SupportCaseResolutionParentInitiationState = Infer<typeof SupportCaseResolutionParentInitiationStateSchema>;
export type SupportCaseResolutionParentConsentState = Infer<typeof SupportCaseResolutionParentConsentStateSchema>;
export type SupportCaseResolutionOperatorResponseState = Infer<typeof SupportCaseResolutionOperatorResponseStateSchema>;
export type SupportCaseResolutionEscalationState = Infer<typeof SupportCaseResolutionEscalationStateSchema>;
export type SupportCaseResolutionSlaState = Infer<typeof SupportCaseResolutionSlaStateSchema>;
export type SupportCaseResolutionPayloadState = Infer<typeof SupportCaseResolutionPayloadStateSchema>;
export type SupportCaseResolutionCustodyState = Infer<typeof SupportCaseResolutionCustodyStateSchema>;
export type SupportCaseResolutionDataClass = Infer<typeof SupportCaseResolutionDataClassSchema>;
export type SupportCaseResolutionDestination = Infer<typeof SupportCaseResolutionDestinationSchema>;
export type SupportCaseResolutionStatusEntry = Infer<typeof SupportCaseResolutionStatusEntrySchema>;
export type SupportCaseResolutionStatusReadModel = Infer<typeof SupportCaseResolutionStatusReadModelSchema>;

export const decodeSupportCaseResolutionStatusEntry = Schema.decodeUnknownSync(SupportCaseResolutionStatusEntrySchema);
export const decodeSupportCaseResolutionStatusReadModel = Schema.decodeUnknownSync(
  SupportCaseResolutionStatusReadModelSchema
);

