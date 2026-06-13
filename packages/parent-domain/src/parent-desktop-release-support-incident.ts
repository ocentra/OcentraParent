import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { parentDesktopReleaseSupportIncidentHandoffIsHonest } from './parent-desktop-release-support-incident-guards';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const ParentDesktopReleaseSupportIncidentStatusSchema = withParser(
  Schema.Literal('triage-ready', 'waiting-on-parent', 'manual-required', 'closed-no-upload')
);
export const ParentDesktopReleaseSupportIncidentSeveritySchema = withParser(
  Schema.Literal('low', 'medium', 'high', 'manual-required')
);
export const ParentDesktopReleaseSupportIncidentConsentStateSchema = withParser(
  Schema.Literal('parent-approved', 'missing', 'revoked')
);
export const ParentDesktopReleaseSupportIncidentConsentCaptureSchema = withParser(
  Schema.Literal('manual-export-action', 'support-ticket-checkbox', 'not-captured')
);
export const ParentDesktopReleaseSupportIncidentDisclosureStateSchema = withParser(
  Schema.Literal('shown-before-export', 'not-shown', 'manual-required')
);
export const ParentDesktopReleaseSupportIncidentCustodyBoundarySchema = withParser(
  Schema.Literal('parent-exported-local-bundle')
);
export const ParentDesktopReleaseSupportIncidentDestinationSchema = withParser(
  Schema.Literal('parent-controlled-support-channel')
);
export const ParentDesktopReleaseSupportIncidentSupportStateSchema = withParser(
  Schema.Literal('manual-required', 'not-implemented', 'preview-only', 'unavailable')
);
export const ParentDesktopReleaseSupportIncidentDataClassSchema = withParser(
  Schema.Literal(
    'release-version',
    'commit-id',
    'platform-family',
    'package-runtime-state',
    'service-health-state',
    'route-state',
    'capability-state',
    'degraded-state',
    'redaction-summary',
    'manual-proof-reference',
    'incident-status'
  )
);
export const ParentDesktopReleaseSupportIncidentExcludedDataClassSchema = withParser(
  Schema.Literal(
    'tokens',
    'child-activity',
    'raw-urls',
    'screenshots',
    'journals',
    'sqlite-snapshots',
    'private-paths',
    'commands',
    'keystrokes',
    'clipboard-data',
    'message-contents'
  )
);
export const ParentDesktopReleaseSupportIncidentDiagnosticReferenceKindSchema = withParser(
  Schema.Literal('proof-json', 'package-preview-workflow', 'redaction-summary', 'manual-runbook', 'support-status-row')
);

export const ParentDesktopReleaseSupportIncidentIdSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportIncidentId');
export const ParentDesktopReleaseSupportIncidentLabelSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportIncidentLabel');
export const ParentDesktopReleaseSupportIncidentReferenceSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportIncidentReference');
export const ParentDesktopReleaseSupportIncidentNonClaimSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportIncidentNonClaim');

export const ParentDesktopReleaseSupportIncidentMetadataSchema = withParser(
  Schema.Struct({
    incidentId: ParentDesktopReleaseSupportIncidentIdSchema,
    status: ParentDesktopReleaseSupportIncidentStatusSchema,
    severity: ParentDesktopReleaseSupportIncidentSeveritySchema,
    productionSupportState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    supportBackendState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    createdAt: ParentTimestampSchema,
    updatedAt: ParentTimestampSchema,
  })
);

export const ParentDesktopReleaseSupportIncidentConsentSchema = withParser(
  Schema.Struct({
    consentState: ParentDesktopReleaseSupportIncidentConsentStateSchema,
    capturedBy: ParentDesktopReleaseSupportIncidentConsentCaptureSchema,
    disclosureState: ParentDesktopReleaseSupportIncidentDisclosureStateSchema,
    parentActor: ParentDesktopReleaseSupportIncidentLabelSchema,
    consentRecordedAt: ParentTimestampSchema,
    revocationState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
  })
);

export const ParentDesktopReleaseSupportIncidentBundleManifestSchema = withParser(
  Schema.Struct({
    manifestId: ParentDesktopReleaseSupportIncidentIdSchema,
    custodyBoundary: ParentDesktopReleaseSupportIncidentCustodyBoundarySchema,
    destination: ParentDesktopReleaseSupportIncidentDestinationSchema,
    disclosureState: ParentDesktopReleaseSupportIncidentDisclosureStateSchema,
    retentionState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    includedDataClasses: Schema.Array(ParentDesktopReleaseSupportIncidentDataClassSchema),
    excludedDataClasses: Schema.Array(ParentDesktopReleaseSupportIncidentExcludedDataClassSchema),
    containsChildActivity: Schema.Boolean,
    containsRawUrls: Schema.Boolean,
    containsScreenshots: Schema.Boolean,
    containsJournals: Schema.Boolean,
    containsSqliteSnapshots: Schema.Boolean,
    containsPrivatePaths: Schema.Boolean,
    containsCommands: Schema.Boolean,
    containsKeystrokes: Schema.Boolean,
    containsClipboardData: Schema.Boolean,
    containsMessageContents: Schema.Boolean,
  })
);

export const ParentDesktopReleaseSupportIncidentDiagnosticReferenceSchema = withParser(
  Schema.Struct({
    kind: ParentDesktopReleaseSupportIncidentDiagnosticReferenceKindSchema,
    reference: ParentDesktopReleaseSupportIncidentReferenceSchema,
    sourceState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    includesSensitiveData: Schema.Boolean,
  })
);

export const ParentDesktopReleaseSupportIncidentManualStateSchema = withParser(
  Schema.Struct({
    supportBackendUploadState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    supportStaffAccessState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    accountLookupState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    billingEscalationState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    remoteControlState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    productionSlaState: ParentDesktopReleaseSupportIncidentSupportStateSchema,
    nonClaims: Schema.Array(ParentDesktopReleaseSupportIncidentNonClaimSchema),
  })
);

const ParentDesktopReleaseSupportIncidentHandoffBaseSchema = Schema.Struct({
  metadata: ParentDesktopReleaseSupportIncidentMetadataSchema,
  parentConsent: ParentDesktopReleaseSupportIncidentConsentSchema,
  supportBundleManifest: ParentDesktopReleaseSupportIncidentBundleManifestSchema,
  diagnosticReferences: Schema.Array(ParentDesktopReleaseSupportIncidentDiagnosticReferenceSchema),
  manualProductionSupportStates: ParentDesktopReleaseSupportIncidentManualStateSchema,
});

export type ParentDesktopReleaseSupportIncidentHandoff = Infer<
  typeof ParentDesktopReleaseSupportIncidentHandoffBaseSchema
>;

export const ParentDesktopReleaseSupportIncidentHandoffSchema = withParser(
  ParentDesktopReleaseSupportIncidentHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        parentDesktopReleaseSupportIncidentHandoffIsHonest(handoff) ||
        'Expected production support incident handoff to require parent consent, safe bundle data-class disclosure, redacted diagnostic references, no child activity/raw evidence/private data inclusion, and manual-required production support states'
    )
  )
);

export type ParentDesktopReleaseSupportIncidentStatus = Infer<typeof ParentDesktopReleaseSupportIncidentStatusSchema>;
export type ParentDesktopReleaseSupportIncidentDataClass = Infer<
  typeof ParentDesktopReleaseSupportIncidentDataClassSchema
>;
export type ParentDesktopReleaseSupportIncidentExcludedDataClass = Infer<
  typeof ParentDesktopReleaseSupportIncidentExcludedDataClassSchema
>;
export type ParentDesktopReleaseSupportIncidentDiagnosticReferenceKind = Infer<
  typeof ParentDesktopReleaseSupportIncidentDiagnosticReferenceKindSchema
>;

