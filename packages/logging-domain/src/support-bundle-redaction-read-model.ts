import {
  type SupportBundleIncidentStatus,
  type SupportBundleManualBoundaryState,
  type SupportBundleParentConsentState,
  type SupportBundleRedactionEntry,
  SupportBundleRedactionEntrySchema,
  SupportBundleRedactionReadModelSchema,
  SupportBundleRequiredDataClasses,
  SupportBundleRequiredDiagnosticReferenceKinds,
  SupportBundleRequiredPayloadFields,
} from './support-bundle-redaction.js';

type SupportBundleRedactionEntryInput = {
  incidentId: string;
  incidentStatus: SupportBundleIncidentStatus;
  parentConsentState: SupportBundleParentConsentState;
  backendUploadState: SupportBundleManualBoundaryState;
  billingEscalationState: SupportBundleManualBoundaryState;
  accountLookupState: SupportBundleManualBoundaryState;
  statusBackendRefs?: readonly string[];
  billingRefs: readonly string[];
  accountRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-03T22:26:41.370Z';

export const SupportBundleRedactionReadModel = SupportBundleRedactionReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'support-bundle-redaction-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'release-installer-support-diagnostics-expectation',
    'production-support-status-backend-execution-queue-proof',
    'production-support-status-backend-queue-audit-persistence-proof',
    'billing-account-support-manual-boundary',
    'static-analysis-security-redaction-boundary',
  ],
  entries: [
    supportBundleRedactionEntry({
      incidentId: 'support-incident-parent-consent-required',
      incidentStatus: 'parent-consent-required',
      parentConsentState: 'required',
      backendUploadState: 'not-applicable',
      billingEscalationState: 'not-applicable',
      accountLookupState: 'not-applicable',
      billingRefs: [],
      accountRefs: [],
      manualProofRequirements: ['parent approval artifact before support bundle export can be prepared'],
    }),
    supportBundleRedactionEntry({
      incidentId: 'support-incident-bundle-ready',
      incidentStatus: 'support-bundle-ready',
      parentConsentState: 'parent-approved',
      backendUploadState: 'manual-required',
      billingEscalationState: 'manual-required',
      accountLookupState: 'manual-required',
      billingRefs: ['billing-status-manual-escalation-ref'],
      accountRefs: ['account-status-manual-lookup-ref'],
      manualProofRequirements: ['manual review before any support upload account lookup or billing escalation'],
    }),
    supportBundleRedactionEntry({
      incidentId: 'support-incident-manual-review-required',
      incidentStatus: 'manual-review-required',
      parentConsentState: 'parent-approved',
      backendUploadState: 'manual-required',
      billingEscalationState: 'manual-required',
      accountLookupState: 'manual-required',
      billingRefs: ['billing-status-manual-escalation-ref'],
      accountRefs: ['account-status-manual-lookup-ref'],
      manualProofRequirements: ['support operator redaction review before handoff can proceed'],
    }),
    supportBundleRedactionEntry({
      incidentId: 'support-incident-backend-upload-manual-required',
      incidentStatus: 'backend-upload-manual-required',
      parentConsentState: 'parent-approved',
      backendUploadState: 'manual-required',
      billingEscalationState: 'not-applicable',
      accountLookupState: 'not-applicable',
      billingRefs: [],
      accountRefs: [],
      manualProofRequirements: ['production support backend upload implementation before upload can be claimed'],
    }),
    supportBundleRedactionEntry({
      incidentId: 'support-incident-status-backend-redaction-ready',
      incidentStatus: 'status-backend-redaction-ready',
      parentConsentState: 'parent-approved',
      backendUploadState: 'not-applicable',
      billingEscalationState: 'not-applicable',
      accountLookupState: 'not-applicable',
      statusBackendRefs: [
        'status-backend-execution-queue-ref',
        'status-backend-queue-audit-persistence-ref',
        'status-backend-redaction-manifest-ref',
      ],
      billingRefs: [],
      accountRefs: [],
      manualProofRequirements: ['status backend receives only redacted manifest refs before publication'],
    }),
    supportBundleRedactionEntry({
      incidentId: 'support-incident-status-backend-redaction-manual-required',
      incidentStatus: 'status-backend-redaction-manual-required',
      parentConsentState: 'parent-approved',
      backendUploadState: 'not-applicable',
      billingEscalationState: 'not-applicable',
      accountLookupState: 'not-applicable',
      statusBackendRefs: ['status-backend-redaction-runbook-manual-required-ref'],
      billingRefs: [],
      accountRefs: [],
      manualProofRequirements: [
        'manual status backend redaction review before any status backend payload storage can be claimed',
      ],
    }),
    supportBundleRedactionEntry({
      incidentId: 'support-incident-billing-escalation-manual-required',
      incidentStatus: 'billing-escalation-manual-required',
      parentConsentState: 'parent-approved',
      backendUploadState: 'not-applicable',
      billingEscalationState: 'manual-required',
      accountLookupState: 'not-applicable',
      billingRefs: ['billing-status-manual-escalation-ref'],
      accountRefs: [],
      manualProofRequirements: [
        'billing backend and support escalation runbook before provider contact can be claimed',
      ],
    }),
    supportBundleRedactionEntry({
      incidentId: 'support-incident-account-lookup-manual-required',
      incidentStatus: 'account-lookup-manual-required',
      parentConsentState: 'parent-approved',
      backendUploadState: 'not-applicable',
      billingEscalationState: 'not-applicable',
      accountLookupState: 'manual-required',
      billingRefs: [],
      accountRefs: ['account-status-manual-lookup-ref'],
      manualProofRequirements: ['account lookup backend and access audit before account lookup can be claimed'],
    }),
  ],
});

function supportBundleRedactionEntry(input: SupportBundleRedactionEntryInput): SupportBundleRedactionEntry {
  return SupportBundleRedactionEntrySchema.parse({
    schemaVersion: 1,
    payloadRedactionState: 'support-safe-metadata-only',
    childActivityCustodyState: 'no-child-activity-custody',
    disclosedDataClasses: [...SupportBundleRequiredDataClasses],
    diagnosticReferenceKinds: [...SupportBundleRequiredDiagnosticReferenceKinds],
    redactionSafePayloadFields: [...SupportBundleRequiredPayloadFields],
    incidentRefs: ['support-incident-status-ref'],
    releaseRefs: ['package-preview-release-boundary-ref'],
    diagnosticRefs: [
      'support-safe-proof-json-ref',
      'package-preview-workflow-ref',
      'support-redaction-summary-ref',
      'status-backend-redaction-manifest-ref',
      'manual-support-runbook-ref',
      'production-support-status-row-ref',
    ],
    statusBackendRefs: input.statusBackendRefs ?? ['status-backend-redaction-manifest-ref'],
    remoteSupportState: 'not-implemented',
    productionSlaState: 'not-implemented',
    containsTokens: false,
    containsChildActivity: false,
    containsRawUrls: false,
    containsScreenshots: false,
    containsJournals: false,
    containsSqliteSnapshots: false,
    containsPrivatePaths: false,
    containsCommandLines: false,
    containsKeystrokes: false,
    containsClipboardData: false,
    containsMessageContents: false,
    containsStatusBackendPayload: false,
    publicRuntimePayloadIncluded: false,
    providerSecretPresent: false,
    backendUploadExecuted: false,
    statusBackendExecutionClaimed: false,
    billingProviderContacted: false,
    accountLookupExecuted: false,
    remoteSupportSessionStarted: false,
    productionSlaClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
