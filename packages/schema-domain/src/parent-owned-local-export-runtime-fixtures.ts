import type {
  ParentOwnedLocalExportRuntimeJob,
  ParentOwnedLocalExportRuntimeProof,
  ParentOwnedLocalExportRuntimeState,
} from './parent-owned-local-export-runtime';
import type { ParentOwnedSyncExportDataClass } from './parent-owned-sync-export';

const Timestamp = '2026-06-03T14:26:23.877Z';
export const Family = { familyId: 'family-local-export-runtime-proof-1' } as const;
export const Device = {
  deviceId: 'windows-child-device-local-export-proof-1',
  childProfileId: 'child-local-export-proof-1',
  label: 'Windows child device local export proof',
  platform: 'windows',
} as const;
export const ParentAction = {
  actionReferenceId: 'parent-action-local-export-proof-1',
  actor: { actorId: 'parent-local-export-proof-1', role: 'parent' },
  policyVersion: 'parent-owned-local-export-runtime-v1',
  createdAt: Timestamp,
} as const;
export const EvidenceRef = {
  evidenceReferenceId: 'evidence-local-export-runtime-proof-1',
  kind: 'journal-event',
  observedAt: Timestamp,
} as const;

export const BaseScope = {
  family: Family,
  device: Device,
  parentAction: ParentAction,
  requestedDataClasses: ['encrypted-journal-segment', 'sqlite-query-row', 'generated-summary'],
  outputFormat: 'encrypted-machine-readable',
  destinationOwnership: 'parent-device-local',
  destinationRef: 'parent-local-export-folder-proof-1',
  parentAuthorized: true,
  rawEvidenceUploaded: false,
  ocentraHostedFamilyDataStored: false,
} as const;

export const BaseOutput = {
  bundleRef: 'local-export-bundle-proof-1',
  outputRef: 'local-export-output-proof-1',
  outputFormat: 'encrypted-machine-readable',
  destinationOwnership: 'parent-device-local',
  encryptedAtRest: true,
  schemaVersionLabel: 'parent-owned-local-export-runtime-schema-v1',
  byteCountRange: 'support-safe-size-range-1',
  checksumRef: 'local-export-checksum-proof-1',
  createdAt: Timestamp,
  sourceEvidenceRefs: [EvidenceRef],
  childDetailMinimized: true,
  rawEvidenceIncludedByDefault: false,
  ocentraHostedCopyRetained: false,
} as const;

const RequiredStates = ['export-queued', 'export-running', 'export-written', 'delete-requested', 'delete-confirmed', 'delete-failed', 'offline-queued', 'manual-required'] as const satisfies ReadonlyArray<ParentOwnedLocalExportRuntimeState>;

function jobFor(state: ParentOwnedLocalExportRuntimeState, overrides: Record<string, unknown> = {}): ParentOwnedLocalExportRuntimeJob {
  return {
    jobId: `local-export-runtime-job-${state}`,
    operation: state.startsWith('delete') ? 'delete' : 'export',
    state,
    queueRef: `local-export-runtime-queue-${state}`,
    storageState: state === 'offline-queued' ? 'offline' : 'local-folder-ready',
    scope: BaseScope,
    output: state === 'export-written' || state.startsWith('delete') ? BaseOutput : null,
    deleteReceipt: deleteReceiptFor(state),
    queuedAt: Timestamp,
    updatedAt: Timestamp,
    auditRefs: [EvidenceRef],
    localEvidenceMutated: false,
    parentOwnedOutputMutatedByFailure: false,
    localSafetyStatePreserved: true,
    manualActionRequired: state === 'manual-required' || state === 'offline-queued',
    ...overrides,
  };
}

function deleteReceiptFor(state: ParentOwnedLocalExportRuntimeState) {
  if (!state.startsWith('delete')) {
    return null;
  }

  const deleteConfirmed = state === 'delete-confirmed';
  return {
    deleteRequestRef: `local-export-delete-request-${state}`,
    targetBundleRef: BaseOutput.bundleRef,
    requestedAt: Timestamp,
    deletedAt: deleteConfirmed ? Timestamp : null,
    deleteConfirmed,
    auditState: deleteConfirmed ? 'audit-recorded' : state === 'delete-failed' ? 'manual-audit-required' : 'audit-pending',
    sourceEvidenceRetained: false,
    exportedOutputDeleted: deleteConfirmed,
    localSafetyStatePreserved: true,
    failureReasonRef: deleteConfirmed ? null : `local-export-delete-failure-${state}`,
  };
}

function countBy<const T extends string>(values: ReadonlyArray<T>, keys: readonly T[]): Record<T, number> {
  return Object.fromEntries(keys.map((key) => [key, values.filter((value) => value === key).length])) as Record<
    T,
    number
  >;
}

export const ParentOwnedLocalExportRuntimeProofReadModelFixture = {
  schemaVersion: 'parent-owned-local-export-runtime-proof',
  jobs: RequiredStates.map((state) => jobFor(state)),
  nonClaims: [
    'no-cloud-transfer-runtime',
    'no-connector-oauth',
    'no-provider-api',
    'no-portal-ui',
    'no-ocentra-family-data-custody',
    'no-remote-report-compiler',
    'no-child-device-mutation',
    'no-raw-evidence-upload',
  ] as const,
  cloudTransferRuntimeClaimed: false,
  connectorOAuthClaimed: false,
  providerApiClaimed: false,
  portalUiClaimed: false,
  ocentraHostedFamilyDataCustodyClaimed: false,
  remoteReportCompilerClaimed: false,
  childDeviceMutationClaimed: false,
  rawEvidenceUploadClaimed: false,
  updatedAt: Timestamp,
} as const satisfies ParentOwnedLocalExportRuntimeProof;

export function summarizeParentOwnedLocalExportRuntimeStates(
  jobs: ReadonlyArray<ParentOwnedLocalExportRuntimeJob>
): Record<ParentOwnedLocalExportRuntimeState, number> {
  return countBy(
    jobs.map((job) => job.state),
    RequiredStates
  );
}

export function summarizeParentOwnedLocalExportRuntimeDataClasses(
  jobs: ReadonlyArray<ParentOwnedLocalExportRuntimeJob>
): Record<ParentOwnedSyncExportDataClass, number> {
  const dataClasses = jobs.flatMap((job) => job.scope.requestedDataClasses);
  return countBy(dataClasses, [
    'encrypted-journal-segment',
    'sqlite-query-row',
    'parent-rule',
    'approval-decision',
    'device-registry-entry',
    'notification-history',
    'audit-event',
    'generated-summary',
  ] as const);
}
