import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

import {
  supportIncidentWorkflowCoversRequiredStates,
  supportIncidentWorkflowEntryIsSafe,
} from './support-incident-workflow-guards.js';

const supportIncidentText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const SupportIncidentWorkflowReadModelIdSchema = supportIncidentText('SupportIncidentWorkflowReadModelId');
export const SupportIncidentWorkflowIncidentIdSchema = supportIncidentText('SupportIncidentWorkflowIncidentId');
export const SupportIncidentWorkflowReferenceSchema = supportIncidentText('SupportIncidentWorkflowReference');
export const SupportIncidentWorkflowRequirementSchema = supportIncidentText('SupportIncidentWorkflowRequirement');
export const SupportIncidentWorkflowTimestampSchema = supportIncidentText('SupportIncidentWorkflowTimestamp');

export const SupportIncidentWorkflowStateSchema = withParser(
  Schema.Literal(
    'parent-consent-gate',
    'privacy-legal-disclosure-required',
    'redaction-audit-review',
    'backend-upload-manual-required',
    'billing-escalation-manual-required',
    'account-lookup-manual-required'
  )
);

export const SupportIncidentParentConsentStateSchema = withParser(
  Schema.Literal('required', 'parent-approved', 'revoked')
);

export const SupportIncidentDisclosureStateSchema = withParser(
  Schema.Literal('not-shown', 'disclosed-before-export', 'manual-required')
);

export const SupportIncidentManualBoundaryStateSchema = withParser(
  Schema.Literal('not-applicable', 'manual-required', 'not-implemented')
);

export const SupportIncidentCustodyStateSchema = withParser(
  Schema.Literal('local-parent-approved-export-only', 'no-ocentra-child-activity-custody')
);

export const SupportIncidentWorkflowDataClassSchema = withParser(
  Schema.Literal(
    'incident-status',
    'consent-artifact-ref',
    'privacy-disclosure-ref',
    'legal-disclosure-ref',
    'redaction-summary-ref',
    'support-bundle-ref',
    'audit-event-ref',
    'billing-status-ref',
    'account-status-ref',
    'release-package-runtime-ref'
  )
);

export const SupportIncidentWorkflowDestinationSchema = withParser(
  Schema.Literal(
    'parent-local-export',
    'support-safe-redaction-summary',
    'manual-support-backend',
    'manual-billing-provider',
    'manual-account-lookup',
    'none'
  )
);

export const SupportIncidentWorkflowRequiredDataClasses = [
  'incident-status',
  'consent-artifact-ref',
  'privacy-disclosure-ref',
  'legal-disclosure-ref',
  'redaction-summary-ref',
  'support-bundle-ref',
  'audit-event-ref',
  'billing-status-ref',
  'account-status-ref',
  'release-package-runtime-ref',
] as const satisfies ReadonlyArray<SupportIncidentWorkflowDataClass>;

const SupportIncidentWorkflowEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  incidentId: SupportIncidentWorkflowIncidentIdSchema,
  workflowState: SupportIncidentWorkflowStateSchema,
  parentConsentState: SupportIncidentParentConsentStateSchema,
  privacyDisclosureState: SupportIncidentDisclosureStateSchema,
  legalDisclosureState: SupportIncidentDisclosureStateSchema,
  backendUploadState: SupportIncidentManualBoundaryStateSchema,
  billingEscalationState: SupportIncidentManualBoundaryStateSchema,
  accountLookupState: SupportIncidentManualBoundaryStateSchema,
  remoteSupportState: SupportIncidentManualBoundaryStateSchema,
  productionSlaState: SupportIncidentManualBoundaryStateSchema,
  custodyState: SupportIncidentCustodyStateSchema,
  disclosedDataClasses: Schema.Array(SupportIncidentWorkflowDataClassSchema),
  allowedDestinations: Schema.Array(SupportIncidentWorkflowDestinationSchema),
  consentRefs: Schema.Array(SupportIncidentWorkflowReferenceSchema),
  privacyLegalRefs: Schema.Array(SupportIncidentWorkflowReferenceSchema),
  redactionRefs: Schema.Array(SupportIncidentWorkflowReferenceSchema),
  auditRefs: Schema.Array(SupportIncidentWorkflowReferenceSchema),
  billingRefs: Schema.Array(SupportIncidentWorkflowReferenceSchema),
  accountRefs: Schema.Array(SupportIncidentWorkflowReferenceSchema),
  manualProofRequirements: Schema.Array(SupportIncidentWorkflowRequirementSchema),
  containsTokens: Schema.Boolean,
  containsChildActivity: Schema.Boolean,
  containsRawUrls: Schema.Boolean,
  containsScreenshots: Schema.Boolean,
  containsJournals: Schema.Boolean,
  containsSqliteSnapshots: Schema.Boolean,
  containsPrivatePaths: Schema.Boolean,
  containsCommandLines: Schema.Boolean,
  containsKeystrokes: Schema.Boolean,
  containsClipboardData: Schema.Boolean,
  containsMessageContents: Schema.Boolean,
  providerSecretPresent: Schema.Boolean,
  backendUploadExecuted: Schema.Boolean,
  billingProviderContacted: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  remoteSupportSessionStarted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  ocentraHostedChildActivityCustody: Schema.Boolean,
  lastCheckedAt: SupportIncidentWorkflowTimestampSchema,
});

export type SupportIncidentWorkflowEntryCandidate = Infer<typeof SupportIncidentWorkflowEntryBaseSchema>;

export const SupportIncidentWorkflowEntrySchema = withParser(
  SupportIncidentWorkflowEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        supportIncidentWorkflowEntryIsSafe(entry, SupportIncidentWorkflowRequiredDataClasses) ||
        'Expected production support incident workflow rows to require parent consent, privacy/legal disclosure, redaction audit refs, manual upload/billing/account boundaries, and no child activity custody'
    )
  )
);

export const SupportIncidentWorkflowReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: SupportIncidentWorkflowReadModelIdSchema,
    generatedAt: SupportIncidentWorkflowTimestampSchema,
    sourceContractRefs: Schema.Array(SupportIncidentWorkflowReferenceSchema),
    entries: Schema.Array(SupportIncidentWorkflowEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.incidentId)).size === readModel.entries.length ||
        'Expected production support incident workflow ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        supportIncidentWorkflowCoversRequiredStates(readModel.entries) ||
        'Expected production support incident workflow proof to cover consent, privacy/legal, redaction/audit, backend-upload, billing, and account states'
    )
  )
);

export type SupportIncidentWorkflowState = Infer<typeof SupportIncidentWorkflowStateSchema>;
export type SupportIncidentParentConsentState = Infer<typeof SupportIncidentParentConsentStateSchema>;
export type SupportIncidentDisclosureState = Infer<typeof SupportIncidentDisclosureStateSchema>;
export type SupportIncidentManualBoundaryState = Infer<typeof SupportIncidentManualBoundaryStateSchema>;
export type SupportIncidentCustodyState = Infer<typeof SupportIncidentCustodyStateSchema>;
export type SupportIncidentWorkflowDataClass = Infer<typeof SupportIncidentWorkflowDataClassSchema>;
export type SupportIncidentWorkflowDestination = Infer<typeof SupportIncidentWorkflowDestinationSchema>;
export type SupportIncidentWorkflowEntry = Infer<typeof SupportIncidentWorkflowEntrySchema>;
export type SupportIncidentWorkflowReadModel = Infer<typeof SupportIncidentWorkflowReadModelSchema>;

export const decodeSupportIncidentWorkflowEntry = Schema.decodeUnknownSync(SupportIncidentWorkflowEntrySchema);
export const decodeSupportIncidentWorkflowReadModel = Schema.decodeUnknownSync(SupportIncidentWorkflowReadModelSchema);

