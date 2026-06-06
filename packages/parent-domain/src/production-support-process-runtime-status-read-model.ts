import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ProductionSupportProcessRuntimeStatusProofSchema,
  ProductionSupportProcessRuntimeStatusRowSchema,
  type ProductionSupportProcessRuntimeStatusRow,
  type ProductionSupportProcessRuntimeStatusSurface,
} from './production-support-process-runtime-status-proof';
import {
  ForbiddenProductionSupportProcessRuntimeStatusDataClasses,
  RequiredProductionSupportProcessRuntimeStatusNonClaims,
} from './production-support-process-runtime-status-values';

export const ProductionSupportProcessRuntimeStatusReadModel = ProductionSupportProcessRuntimeStatusProofSchema.parse({
  schemaVersion: 'production-support-process-runtime-status-proof',
  rows: [
    processStatus('support-process-requested', 'support-incident-workflow-proof', 'requested'),
    processStatus('parent-consent-authorized', 'support-incident-workflow-proof', 'authorized'),
    processStatus('privacy-legal-queued', 'production-incident-support-status-proof', 'queued'),
    processStatus('redaction-review-running', 'support-incident-workflow-proof', 'running'),
    processStatus('backend-upload-failed', 'production-support-backend-upload-status-proof', 'failed'),
    processStatus('case-resolution-succeeded', 'production-support-case-resolution-status-proof', 'succeeded'),
    processStatus(
      'support-process-manual-required',
      'production-support-publication-runtime-readiness-proof',
      'manual-required'
    ),
  ],
  nonClaims: RequiredProductionSupportProcessRuntimeStatusNonClaims,
  backendUploadExecutionState: 'manual-required',
  publicRuntimeExecutionState: 'not-implemented',
  providerExecutionState: 'not-implemented',
  productionSlaState: 'not-implemented',
  remoteSupportSessionState: 'not-implemented',
  childActivityCustodyState: 'not-implemented',
  defaultOcentraHostedFamilyDataState: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T05:00:00.000Z'),
});

export const ProductionSupportProcessRuntimeStatusKnownGaps = [
  'Support process runtime status rows are deterministic local proof only; public runtime execution remains unimplemented.',
  'Support backend upload execution, provider execution, remote support sessions, and production SLA remain manual-required or not implemented.',
  'No child activity evidence, raw support bundles, provider secrets, account lookup results, billing contact records, remote transcripts, or default Ocentra-hosted family data are included.',
] as const;

function processStatus(
  surface: ProductionSupportProcessRuntimeStatusSurface,
  sourceProof: ProductionSupportProcessRuntimeStatusRow['sourceProof'],
  runtimeState: ProductionSupportProcessRuntimeStatusRow['runtimeState']
): ProductionSupportProcessRuntimeStatusRow {
  return ProductionSupportProcessRuntimeStatusRowSchema.parse({
    schemaVersion: 'production-support-process-runtime-status-proof',
    surface,
    sourceProof,
    runtimeState,
    parentConsentState: surface === 'support-process-requested' ? 'requested' : 'authorized',
    privacyLegalState:
      surface === 'privacy-legal-queued' || surface === 'redaction-review-running' ? 'queued' : 'manual-required',
    redactionReviewState: surface === 'redaction-review-running' ? 'running' : 'manual-required',
    backendUploadState: surface === 'backend-upload-failed' ? 'failed' : 'manual-required',
    caseResolutionState: surface === 'case-resolution-succeeded' ? 'succeeded' : 'manual-required',
    supportSafeDataClasses: supportSafeDataClassesFor(surface),
    forbiddenDataClasses: ForbiddenProductionSupportProcessRuntimeStatusDataClasses,
    runtimeReference: `production-support-process-runtime-${surface}`,
    auditReference: `production-support-process-audit-${surface}`,
    manualRequirement: `${surface}-requires-runtime-publication-support-and-manual-proof`,
  });
}

function supportSafeDataClassesFor(
  surface: ProductionSupportProcessRuntimeStatusSurface
): ProductionSupportProcessRuntimeStatusRow['supportSafeDataClasses'] {
  if (surface === 'backend-upload-failed') {
    return ['support-case-status', 'backend-upload-status', 'audit-status', 'manual-proof-status'];
  }
  if (surface === 'case-resolution-succeeded') {
    return ['support-case-status', 'case-resolution-status', 'audit-status'];
  }
  if (surface === 'support-process-manual-required') {
    return ['support-case-status', 'support-runbook-status', 'manual-proof-status'];
  }
  return ['support-case-status', 'parent-consent-status', 'privacy-legal-status', 'redaction-review-status'];
}
