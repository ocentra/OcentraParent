import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ProductionSupportDataExportDeleteLifecycleProofSchema,
  type ProductionSupportDataExportDeleteLifecycleRow,
  type ProductionSupportDataExportDeleteLifecycleSurface,
} from './production-support-data-export-delete-lifecycle-proof';
import {
  ForbiddenProductionSupportDataExportDeleteLifecycleDataClasses,
  RequiredProductionSupportDataExportDeleteLifecycleNonClaims,
} from './production-support-data-export-delete-lifecycle-values';

export const ProductionSupportDataExportDeleteLifecycleReadModel =
  ProductionSupportDataExportDeleteLifecycleProofSchema.parse({
    schemaVersion: 'production-support-data-export-delete-lifecycle-proof',
    rows: [
      lifecycleRow('export-requested', 'export', 'requested', ['parent-request-ref', 'data-custody-ref']),
      lifecycleRow('export-authorized', 'export', 'authorized', ['parent-authorization-ref', 'redaction-audit-ref']),
      lifecycleRow('export-queued', 'export', 'queued', ['local-queue-ref', 'manual-proof-ref']),
      lifecycleRow('export-running', 'export', 'running', ['local-runtime-ref', 'redaction-audit-ref']),
      lifecycleRow('export-succeeded', 'export', 'succeeded', ['local-output-ref', 'redaction-audit-ref']),
      lifecycleRow('export-failed', 'export', 'failed', ['local-runtime-ref', 'manual-proof-ref']),
      lifecycleRow('export-manual-required', 'export', 'manual-required', ['manual-proof-ref', 'data-custody-ref']),
      lifecycleRow('delete-requested', 'delete', 'requested', ['parent-request-ref', 'data-custody-ref']),
      lifecycleRow('delete-authorized', 'delete', 'authorized', ['parent-authorization-ref', 'redaction-audit-ref']),
      lifecycleRow('delete-queued', 'delete', 'queued', ['local-queue-ref', 'manual-proof-ref']),
      lifecycleRow('delete-running', 'delete', 'running', ['local-runtime-ref', 'local-delete-ref']),
      lifecycleRow('delete-succeeded', 'delete', 'succeeded', ['local-delete-ref', 'redaction-audit-ref']),
      lifecycleRow('delete-failed', 'delete', 'failed', ['local-delete-ref', 'manual-proof-ref']),
      lifecycleRow('delete-manual-required', 'delete', 'manual-required', ['manual-proof-ref', 'data-custody-ref']),
    ],
    nonClaims: RequiredProductionSupportDataExportDeleteLifecycleNonClaims,
    backendUploadExecutionState: 'not-implemented',
    publicRuntimeExecutionState: 'not-implemented',
    providerExecutionState: 'not-implemented',
    productionSlaState: 'not-implemented',
    remoteSupportSessionState: 'not-implemented',
    childActivityCustodyState: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T04:21:00.000Z'),
  });

export const ProductionSupportDataExportDeleteLifecycleKnownGaps = [
  'Export/delete lifecycle rows are contract proof only; no real filesystem writer, durable queue, retention scheduler, or delete executor is claimed.',
  'Support backend upload execution, public runtime execution, provider execution, production SLA, remote support sessions, and child activity custody remain not implemented.',
  'Successful rows represent support-safe local output/delete status metadata and require runtime proof before product execution claims.',
] as const;

function lifecycleRow(
  surface: ProductionSupportDataExportDeleteLifecycleSurface,
  operation: ProductionSupportDataExportDeleteLifecycleRow['operation'],
  lifecycleState: ProductionSupportDataExportDeleteLifecycleRow['lifecycleState'],
  lifecycleReferences: ProductionSupportDataExportDeleteLifecycleRow['lifecycleReferences']
) {
  return {
    schemaVersion: 'production-support-data-export-delete-lifecycle-proof',
    surface,
    operation,
    lifecycleState,
    sourceProof: sourceProofFor(surface),
    supportSafeDataClasses: supportSafeDataClassesFor(surface),
    forbiddenDataClasses: ForbiddenProductionSupportDataExportDeleteLifecycleDataClasses,
    lifecycleReferences,
    manualRequirement: `${surface}-requires-runtime-execution-proof`,
    backendUploadExecutionState: 'not-implemented',
    publicRuntimeExecutionState: 'not-implemented',
    providerExecutionState: 'not-implemented',
    productionSlaState: 'not-implemented',
    remoteSupportSessionState: 'not-implemented',
    childActivityCustodyState: 'not-implemented',
  } as const;
}

function sourceProofFor(surface: ProductionSupportDataExportDeleteLifecycleSurface) {
  if (surface.endsWith('manual-required')) {
    return 'production-support-legal-provider-readiness-proof';
  }
  if (surface.endsWith('requested') || surface.endsWith('authorized')) {
    return 'production-incident-support-status-proof';
  }
  return 'data-export-delete-lifecycle-logging-contract';
}

function supportSafeDataClassesFor(surface: ProductionSupportDataExportDeleteLifecycleSurface) {
  if (surface.includes('authorized')) {
    return ['authorization-status', 'redaction-audit-status', 'manual-proof-status'] as const;
  }
  if (surface.includes('queued') || surface.includes('running')) {
    return ['queue-status', 'runtime-status', 'redaction-audit-status'] as const;
  }
  if (surface.includes('succeeded')) {
    return ['runtime-status', 'local-output-status', 'delete-status', 'redaction-audit-status'] as const;
  }
  if (surface.includes('failed') || surface.includes('manual-required')) {
    return ['runtime-status', 'manual-proof-status', 'redaction-audit-status'] as const;
  }
  return ['request-status', 'data-custody-status', 'manual-proof-status'] as const;
}
