import {
  type SupportCaseResolutionDestination,
  type SupportCaseResolutionEscalationState,
  type SupportCaseResolutionOperatorResponseState,
  SupportCaseResolutionRequiredDataClasses,
  type SupportCaseResolutionSlaState,
  SupportCaseResolutionStatusEntrySchema,
  type SupportCaseResolutionStatusEntry,
  SupportCaseResolutionStatusReadModelSchema,
  type SupportCaseResolutionStatusState,
} from './support-case-resolution-status.js';

type SupportCaseResolutionStatusEntryInput = {
  caseId: string;
  caseStatus: SupportCaseResolutionStatusState;
  operatorResponseState: SupportCaseResolutionOperatorResponseState;
  escalationState: SupportCaseResolutionEscalationState;
  slaState: SupportCaseResolutionSlaState;
  allowedDestinations: readonly SupportCaseResolutionDestination[];
  escalationRefs: readonly string[];
  responseRefs: readonly string[];
  closureRefs: readonly string[];
  slaRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-05T16:20:22.382Z';

export const SupportCaseResolutionStatusReadModel = SupportCaseResolutionStatusReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'production-support-case-resolution-status-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'release-installer-support-case-resolution-expectation',
    'data-custody-support-case-status-boundary',
    'production-support-backend-upload-status-proof',
    'production-support-backend-upload-execution-runtime-proof',
    'production-support-publication-workflow-proof',
  ],
  entries: [
    supportCaseResolutionStatusEntry({
      caseId: 'support-case-opened',
      caseStatus: 'case-opened',
      operatorResponseState: 'manual-required',
      escalationState: 'not-requested',
      slaState: 'not-claimed',
      allowedDestinations: ['support-safe-case-status-boundary'],
      escalationRefs: [],
      responseRefs: ['support-case-opened-parent-visible-status-ref'],
      closureRefs: [],
      slaRefs: [],
      manualProofRequirements: [],
    }),
    supportCaseResolutionStatusEntry({
      caseId: 'support-case-triage-ready',
      caseStatus: 'triage-ready',
      operatorResponseState: 'manual-required',
      escalationState: 'not-requested',
      slaState: 'not-claimed',
      allowedDestinations: ['support-safe-case-status-boundary'],
      escalationRefs: [],
      responseRefs: ['support-case-triage-parent-update-ref'],
      closureRefs: [],
      slaRefs: [],
      manualProofRequirements: [],
    }),
    supportCaseResolutionStatusEntry({
      caseId: 'support-case-parent-update-ready',
      caseStatus: 'parent-update-ready',
      operatorResponseState: 'manual-required',
      escalationState: 'not-requested',
      slaState: 'not-claimed',
      allowedDestinations: ['support-safe-case-status-boundary'],
      escalationRefs: [],
      responseRefs: ['support-case-parent-safe-update-ref'],
      closureRefs: [],
      slaRefs: [],
      manualProofRequirements: [],
    }),
    supportCaseResolutionStatusEntry({
      caseId: 'support-case-escalation-manual-required',
      caseStatus: 'escalation-manual-required',
      operatorResponseState: 'manual-required',
      escalationState: 'manual-required',
      slaState: 'not-claimed',
      allowedDestinations: ['manual-support-operator'],
      escalationRefs: ['support-case-escalation-runbook-ref', 'support-case-provider-contact-manual-ref'],
      responseRefs: [],
      closureRefs: [],
      slaRefs: [],
      manualProofRequirements: ['support escalation operator workflow before escalation execution can be claimed'],
    }),
    supportCaseResolutionStatusEntry({
      caseId: 'support-case-response-manual-required',
      caseStatus: 'response-manual-required',
      operatorResponseState: 'manual-required',
      escalationState: 'not-requested',
      slaState: 'not-claimed',
      allowedDestinations: ['manual-support-operator'],
      escalationRefs: [],
      responseRefs: ['support-case-operator-response-runbook-ref'],
      closureRefs: [],
      slaRefs: [],
      manualProofRequirements: ['support operator response workflow before parent response execution can be claimed'],
    }),
    supportCaseResolutionStatusEntry({
      caseId: 'support-case-closure-ready',
      caseStatus: 'closure-ready',
      operatorResponseState: 'manual-required',
      escalationState: 'not-requested',
      slaState: 'not-claimed',
      allowedDestinations: ['support-safe-case-status-boundary'],
      escalationRefs: [],
      responseRefs: ['support-case-closure-parent-update-ref'],
      closureRefs: ['support-case-closure-audit-ref', 'support-case-parent-closeout-ref'],
      slaRefs: [],
      manualProofRequirements: [],
    }),
    supportCaseResolutionStatusEntry({
      caseId: 'support-case-sla-manual-required',
      caseStatus: 'sla-manual-required',
      operatorResponseState: 'manual-required',
      escalationState: 'not-requested',
      slaState: 'manual-required',
      allowedDestinations: ['manual-support-operator'],
      escalationRefs: [],
      responseRefs: [],
      closureRefs: [],
      slaRefs: ['support-case-sla-policy-manual-ref', 'support-case-sla-publication-manual-ref'],
      manualProofRequirements: ['published production support SLA before support timing commitments can be claimed'],
    }),
  ],
});

function supportCaseResolutionStatusEntry(
  input: SupportCaseResolutionStatusEntryInput
): SupportCaseResolutionStatusEntry {
  return SupportCaseResolutionStatusEntrySchema.parse({
    schemaVersion: 1,
    parentInitiationState: 'parent-initiated',
    parentConsentState: 'parent-approved',
    casePayloadState: 'support-safe-status-and-refs-only',
    custodyState: 'no-ocentra-hosted-family-data',
    disclosedDataClasses: [...SupportCaseResolutionRequiredDataClasses],
    parentConsentRefs: ['parent-support-case-consent-artifact-ref'],
    incidentRefs: ['support-incident-workflow-proof-ref', 'support-incident-status-publication-ref'],
    redactionRefs: ['support-bundle-redaction-proof-ref', 'support-safe-case-summary-ref'],
    auditRefs: ['support-case-status-audit-ref', 'support-case-resolution-audit-ref'],
    uploadStatusRefs: [
      'production-support-backend-upload-status-proof-ref',
      'production-support-backend-upload-execution-runtime-proof-ref',
    ],
    publicationRefs: ['production-support-publication-workflow-proof-ref'],
    containsTokens: false,
    containsRawChildActivity: false,
    containsRawUrls: false,
    containsScreenshots: false,
    containsJournals: false,
    containsSqliteSnapshots: false,
    containsPrivatePaths: false,
    containsCommandLines: false,
    containsKeystrokes: false,
    containsClipboardData: false,
    containsMessageContents: false,
    containsProviderSecrets: false,
    containsRemoteSupportTranscripts: false,
    realSupportBackendUploadExecuted: false,
    accountLookupExecuted: false,
    billingProviderContactExecuted: false,
    remoteSupportSessionExecuted: false,
    productionSlaClaimed: false,
    ocentraHostedFamilyDataDefault: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
