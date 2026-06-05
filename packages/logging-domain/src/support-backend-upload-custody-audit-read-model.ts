import {
  type SupportBackendUploadCustodyAuditDeleteState,
  type SupportBackendUploadCustodyAuditEntry,
  SupportBackendUploadCustodyAuditEntrySchema,
  type SupportBackendUploadCustodyAuditExportState,
  SupportBackendUploadCustodyAuditReadModelSchema,
  SupportBackendUploadCustodyAuditRequiredDataClasses,
  type SupportBackendUploadCustodyAuditRetentionState,
  type SupportBackendUploadCustodyAuditState,
} from './support-backend-upload-custody-audit.js';

type SupportBackendUploadCustodyAuditEntryInput = {
  auditId: string;
  auditState: SupportBackendUploadCustodyAuditState;
  retentionState: SupportBackendUploadCustodyAuditRetentionState;
  deleteState: SupportBackendUploadCustodyAuditDeleteState;
  auditExportState: SupportBackendUploadCustodyAuditExportState;
  retentionRefs: readonly string[];
  deleteRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-05T15:35:49.895Z';

export const SupportBackendUploadCustodyAuditReadModel = SupportBackendUploadCustodyAuditReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'production-support-backend-upload-custody-audit-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'data-custody-support-upload-boundary',
    'release-installer-support-backend-upload-expectation',
    'production-support-backend-upload-status-proof',
    'production-support-backend-upload-execution-runtime-proof',
  ],
  entries: [
    supportBackendUploadCustodyAuditEntry({
      auditId: 'support-upload-custody-boundary-recorded',
      auditState: 'custody-boundary-recorded',
      retentionState: 'manual-required',
      deleteState: 'not-requested',
      auditExportState: 'manual-required',
      retentionRefs: ['support-upload-retention-manual-proof-ref'],
      deleteRefs: [],
      manualProofRequirements: ['retention policy proof before backend custody can be claimed'],
    }),
    supportBackendUploadCustodyAuditEntry({
      auditId: 'support-upload-retention-manual-required',
      auditState: 'retention-manual-required',
      retentionState: 'manual-required',
      deleteState: 'not-requested',
      auditExportState: 'manual-required',
      retentionRefs: ['support-upload-retention-window-runbook-ref'],
      deleteRefs: [],
      manualProofRequirements: ['published retention runbook before support backend retention can be claimed'],
    }),
    supportBackendUploadCustodyAuditEntry({
      auditId: 'support-upload-delete-request-recorded',
      auditState: 'delete-request-recorded',
      retentionState: 'not-applicable',
      deleteState: 'manual-required',
      auditExportState: 'manual-required',
      retentionRefs: [],
      deleteRefs: ['parent-support-upload-delete-request-ref'],
      manualProofRequirements: ['delete request execution proof before backend deletion can be claimed'],
    }),
    supportBackendUploadCustodyAuditEntry({
      auditId: 'support-upload-deletion-manual-required',
      auditState: 'deletion-manual-required',
      retentionState: 'not-applicable',
      deleteState: 'manual-required',
      auditExportState: 'manual-required',
      retentionRefs: [],
      deleteRefs: ['support-upload-delete-manual-proof-ref'],
      manualProofRequirements: ['operator deletion runbook before support backend deletion can be claimed'],
    }),
    supportBackendUploadCustodyAuditEntry({
      auditId: 'support-upload-custody-audit-export-ready',
      auditState: 'audit-export-ready',
      retentionState: 'manual-required',
      deleteState: 'manual-required',
      auditExportState: 'support-safe-export-ready',
      retentionRefs: ['support-upload-retention-audit-ref'],
      deleteRefs: ['support-upload-delete-audit-ref'],
      manualProofRequirements: [
        'support-safe custody audit export review before backend upload execution can be claimed',
      ],
    }),
  ],
});

function supportBackendUploadCustodyAuditEntry(
  input: SupportBackendUploadCustodyAuditEntryInput
): SupportBackendUploadCustodyAuditEntry {
  return SupportBackendUploadCustodyAuditEntrySchema.parse({
    schemaVersion: 1,
    parentInitiationState: 'parent-initiated',
    parentConsentState: 'parent-approved',
    executionClaimState: 'custody-audit-boundary-only',
    payloadState: 'redacted-audit-refs-only',
    custodyState: 'parent-owned-export-only',
    disclosedDataClasses: [...SupportBackendUploadCustodyAuditRequiredDataClasses],
    consentRefs: ['parent-support-upload-consent-artifact-ref'],
    redactionRefs: ['support-bundle-redaction-proof-ref', 'support-upload-redaction-preflight-ref'],
    auditRefs: ['support-upload-custody-audit-event-ref'],
    statusRefs: ['production-support-backend-upload-status-proof-ref'],
    runtimeRefs: ['production-support-backend-upload-execution-runtime-proof-ref'],
    custodyRefs: ['data-custody-support-upload-boundary-ref'],
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
    supportBackendRetainedPayload: false,
    supportBackendDeletedPayload: false,
    ocentraHostedFamilyDataDefault: false,
    accountLookupExecuted: false,
    billingProviderContactExecuted: false,
    remoteSupportSessionExecuted: false,
    productionSlaClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
