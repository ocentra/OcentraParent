import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

import {
  privacyLegalDisclosureCoversRequiredStates,
  privacyLegalDisclosureEntryIsSafe,
} from './privacy-legal-disclosure-status-guards.js';

const privacyLegalDisclosureText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const PrivacyLegalDisclosureReadModelIdSchema = privacyLegalDisclosureText('PrivacyLegalDisclosureReadModelId');
export const PrivacyLegalDisclosureIdSchema = privacyLegalDisclosureText('PrivacyLegalDisclosureId');
export const PrivacyLegalDisclosureReferenceSchema = privacyLegalDisclosureText('PrivacyLegalDisclosureReference');
export const PrivacyLegalDisclosureRequirementSchema = privacyLegalDisclosureText('PrivacyLegalDisclosureRequirement');
export const PrivacyLegalDisclosureTimestampSchema = privacyLegalDisclosureText('PrivacyLegalDisclosureTimestamp');

export const PrivacyLegalDisclosureStateSchema = withParser(
  Schema.Literal(
    'disclosure-requested',
    'parent-authorized',
    'legal-review-queued',
    'legal-review-running',
    'parent-notification-ready',
    'publication-ready',
    'disclosure-failed',
    'manual-required'
  )
);

export const PrivacyLegalDisclosureParentAuthorizationStateSchema = withParser(Schema.Literal('parent-authorized'));
export const PrivacyLegalDisclosurePayloadStateSchema = withParser(
  Schema.Literal('support-safe-disclosure-status-only')
);
export const PrivacyLegalDisclosureCustodyStateSchema = withParser(Schema.Literal('no-child-activity-custody'));

export const PrivacyLegalDisclosureDataClassSchema = withParser(
  Schema.Literal(
    'parent-consent-ref',
    'privacy-policy-ref',
    'legal-review-ref',
    'disclosure-status-ref',
    'publication-status-ref',
    'support-runbook-ref',
    'redaction-audit-ref',
    'manual-proof-ref'
  )
);

export const PrivacyLegalDisclosureDestinationSchema = withParser(
  Schema.Literal('support-safe-disclosure-status-boundary', 'manual-legal-review', 'none')
);

export const PrivacyLegalDisclosureRequiredDataClasses = [
  'parent-consent-ref',
  'privacy-policy-ref',
  'legal-review-ref',
  'disclosure-status-ref',
  'publication-status-ref',
  'support-runbook-ref',
  'redaction-audit-ref',
  'manual-proof-ref',
] as const satisfies ReadonlyArray<PrivacyLegalDisclosureDataClass>;

const PrivacyLegalDisclosureEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  disclosureId: PrivacyLegalDisclosureIdSchema,
  disclosureState: PrivacyLegalDisclosureStateSchema,
  parentAuthorizationState: PrivacyLegalDisclosureParentAuthorizationStateSchema,
  payloadState: PrivacyLegalDisclosurePayloadStateSchema,
  custodyState: PrivacyLegalDisclosureCustodyStateSchema,
  disclosedDataClasses: Schema.Array(PrivacyLegalDisclosureDataClassSchema),
  allowedDestinations: Schema.Array(PrivacyLegalDisclosureDestinationSchema),
  parentConsentRefs: Schema.Array(PrivacyLegalDisclosureReferenceSchema),
  privacyPolicyRefs: Schema.Array(PrivacyLegalDisclosureReferenceSchema),
  legalReviewRefs: Schema.Array(PrivacyLegalDisclosureReferenceSchema),
  disclosureStatusRefs: Schema.Array(PrivacyLegalDisclosureReferenceSchema),
  publicationRefs: Schema.Array(PrivacyLegalDisclosureReferenceSchema),
  supportRunbookRefs: Schema.Array(PrivacyLegalDisclosureReferenceSchema),
  auditRefs: Schema.Array(PrivacyLegalDisclosureReferenceSchema),
  failureRefs: Schema.Array(PrivacyLegalDisclosureReferenceSchema),
  manualProofRequirements: Schema.Array(PrivacyLegalDisclosureRequirementSchema),
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
  legalDisclosureExecuted: Schema.Boolean,
  publicRuntimeExecuted: Schema.Boolean,
  supportBackendUploadExecuted: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  billingProviderContactExecuted: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  childActivityCustodyClaimed: Schema.Boolean,
  lastCheckedAt: PrivacyLegalDisclosureTimestampSchema,
});

export type PrivacyLegalDisclosureEntryCandidate = Infer<typeof PrivacyLegalDisclosureEntryBaseSchema>;

export const PrivacyLegalDisclosureEntrySchema = withParser(
  PrivacyLegalDisclosureEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        privacyLegalDisclosureEntryIsSafe(entry, PrivacyLegalDisclosureRequiredDataClasses) ||
        'Expected privacy/legal disclosure rows to be parent-authorized, support-safe, legal-review-backed, audit-backed, publication-aware, manual-proof aware, and free of sensitive data, execution, SLA, remote support, provider secret, or child activity custody claims'
    )
  )
);

export const PrivacyLegalDisclosureReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: PrivacyLegalDisclosureReadModelIdSchema,
    generatedAt: PrivacyLegalDisclosureTimestampSchema,
    sourceContractRefs: Schema.Array(PrivacyLegalDisclosureReferenceSchema),
    entries: Schema.Array(PrivacyLegalDisclosureEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.disclosureId)).size === readModel.entries.length ||
        'Expected privacy/legal disclosure ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        privacyLegalDisclosureCoversRequiredStates(readModel.entries) ||
        'Expected privacy/legal disclosure proof to cover requested, authorized, queued, running, notification, publication, failed, and manual-required rows'
    )
  )
);

export type PrivacyLegalDisclosureState = Infer<typeof PrivacyLegalDisclosureStateSchema>;
export type PrivacyLegalDisclosureParentAuthorizationState = Infer<
  typeof PrivacyLegalDisclosureParentAuthorizationStateSchema
>;
export type PrivacyLegalDisclosurePayloadState = Infer<typeof PrivacyLegalDisclosurePayloadStateSchema>;
export type PrivacyLegalDisclosureCustodyState = Infer<typeof PrivacyLegalDisclosureCustodyStateSchema>;
export type PrivacyLegalDisclosureDataClass = Infer<typeof PrivacyLegalDisclosureDataClassSchema>;
export type PrivacyLegalDisclosureDestination = Infer<typeof PrivacyLegalDisclosureDestinationSchema>;
export type PrivacyLegalDisclosureEntry = Infer<typeof PrivacyLegalDisclosureEntrySchema>;
export type PrivacyLegalDisclosureReadModel = Infer<typeof PrivacyLegalDisclosureReadModelSchema>;

export const decodePrivacyLegalDisclosureEntry = Schema.decodeUnknownSync(PrivacyLegalDisclosureEntrySchema);
export const decodePrivacyLegalDisclosureReadModel = Schema.decodeUnknownSync(PrivacyLegalDisclosureReadModelSchema);

