import {
  type SupportIncidentDisclosureState,
  type SupportIncidentManualBoundaryState,
  type SupportIncidentParentConsentState,
  type SupportIncidentWorkflowDestination,
  type SupportIncidentWorkflowEntry,
  SupportIncidentWorkflowEntrySchema,
  SupportIncidentWorkflowReadModelSchema,
  SupportIncidentWorkflowRequiredDataClasses,
  type SupportIncidentWorkflowState,
} from './support-incident-workflow.js';

type SupportIncidentWorkflowEntryInput = {
  incidentId: string;
  workflowState: SupportIncidentWorkflowState;
  parentConsentState: SupportIncidentParentConsentState;
  privacyDisclosureState: SupportIncidentDisclosureState;
  legalDisclosureState: SupportIncidentDisclosureState;
  backendUploadState: SupportIncidentManualBoundaryState;
  billingEscalationState: SupportIncidentManualBoundaryState;
  accountLookupState: SupportIncidentManualBoundaryState;
  allowedDestinations: readonly SupportIncidentWorkflowDestination[];
  billingRefs: readonly string[];
  accountRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-04T07:05:41.163Z';

export const SupportIncidentWorkflowReadModel = SupportIncidentWorkflowReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'support-incident-workflow-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'static-analysis-security-expectation',
    'data-custody-support-incident-boundary',
    'support-bundle-redaction-proof',
  ],
  entries: [
    supportIncidentWorkflowEntry({
      incidentId: 'support-workflow-parent-consent-gate',
      workflowState: 'parent-consent-gate',
      parentConsentState: 'required',
      privacyDisclosureState: 'not-shown',
      legalDisclosureState: 'not-shown',
      backendUploadState: 'not-applicable',
      billingEscalationState: 'not-applicable',
      accountLookupState: 'not-applicable',
      allowedDestinations: ['none'],
      billingRefs: [],
      accountRefs: [],
      manualProofRequirements: ['parent approval artifact before support incident workflow can proceed'],
    }),
    supportIncidentWorkflowEntry({
      incidentId: 'support-workflow-privacy-legal-disclosure',
      workflowState: 'privacy-legal-disclosure-required',
      parentConsentState: 'parent-approved',
      privacyDisclosureState: 'disclosed-before-export',
      legalDisclosureState: 'disclosed-before-export',
      backendUploadState: 'not-applicable',
      billingEscalationState: 'not-applicable',
      accountLookupState: 'not-applicable',
      allowedDestinations: ['parent-local-export', 'support-safe-redaction-summary'],
      billingRefs: [],
      accountRefs: [],
      manualProofRequirements: ['visible privacy and legal disclosure acknowledgment before export'],
    }),
    supportIncidentWorkflowEntry({
      incidentId: 'support-workflow-redaction-audit-review',
      workflowState: 'redaction-audit-review',
      parentConsentState: 'parent-approved',
      privacyDisclosureState: 'disclosed-before-export',
      legalDisclosureState: 'disclosed-before-export',
      backendUploadState: 'manual-required',
      billingEscalationState: 'manual-required',
      accountLookupState: 'manual-required',
      allowedDestinations: ['parent-local-export', 'support-safe-redaction-summary'],
      billingRefs: ['billing-status-manual-escalation-ref'],
      accountRefs: ['account-status-manual-lookup-ref'],
      manualProofRequirements: ['redaction and custody audit review before any support handoff'],
    }),
    supportIncidentWorkflowEntry({
      incidentId: 'support-workflow-backend-upload-manual-required',
      workflowState: 'backend-upload-manual-required',
      parentConsentState: 'parent-approved',
      privacyDisclosureState: 'disclosed-before-export',
      legalDisclosureState: 'disclosed-before-export',
      backendUploadState: 'manual-required',
      billingEscalationState: 'not-applicable',
      accountLookupState: 'not-applicable',
      allowedDestinations: ['manual-support-backend'],
      billingRefs: [],
      accountRefs: [],
      manualProofRequirements: ['production support backend upload implementation before upload can be claimed'],
    }),
    supportIncidentWorkflowEntry({
      incidentId: 'support-workflow-billing-escalation-manual-required',
      workflowState: 'billing-escalation-manual-required',
      parentConsentState: 'parent-approved',
      privacyDisclosureState: 'disclosed-before-export',
      legalDisclosureState: 'disclosed-before-export',
      backendUploadState: 'not-applicable',
      billingEscalationState: 'manual-required',
      accountLookupState: 'not-applicable',
      allowedDestinations: ['manual-billing-provider'],
      billingRefs: ['billing-status-manual-escalation-ref'],
      accountRefs: [],
      manualProofRequirements: ['billing support runbook and provider boundary before contact can be claimed'],
    }),
    supportIncidentWorkflowEntry({
      incidentId: 'support-workflow-account-lookup-manual-required',
      workflowState: 'account-lookup-manual-required',
      parentConsentState: 'parent-approved',
      privacyDisclosureState: 'disclosed-before-export',
      legalDisclosureState: 'disclosed-before-export',
      backendUploadState: 'not-applicable',
      billingEscalationState: 'not-applicable',
      accountLookupState: 'manual-required',
      allowedDestinations: ['manual-account-lookup'],
      billingRefs: [],
      accountRefs: ['account-status-manual-lookup-ref'],
      manualProofRequirements: ['account lookup backend and access audit before lookup can be claimed'],
    }),
  ],
});

function supportIncidentWorkflowEntry(input: SupportIncidentWorkflowEntryInput): SupportIncidentWorkflowEntry {
  return SupportIncidentWorkflowEntrySchema.parse({
    schemaVersion: 1,
    custodyState: 'no-ocentra-child-activity-custody',
    disclosedDataClasses: [...SupportIncidentWorkflowRequiredDataClasses],
    consentRefs: ['parent-support-consent-artifact-ref'],
    privacyLegalRefs: ['privacy-disclosure-version-ref', 'legal-disclosure-version-ref'],
    redactionRefs: ['support-bundle-redaction-proof-ref', 'support-safe-summary-ref'],
    auditRefs: ['support-incident-audit-event-ref', 'custody-boundary-audit-ref'],
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
    providerSecretPresent: false,
    backendUploadExecuted: false,
    billingProviderContacted: false,
    accountLookupExecuted: false,
    remoteSupportSessionStarted: false,
    productionSlaClaimed: false,
    ocentraHostedChildActivityCustody: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
