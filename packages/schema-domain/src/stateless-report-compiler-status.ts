import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentOwnedSyncExportDataClassSchema,
  type ParentOwnedSyncExportDataClass,
  ParentOwnedSyncExportDestinationOwnershipSchema,
} from './parent-owned-sync-export';
import {
  RequiredStatelessReportCompilerNonClaims,
  RequiredStatelessReportCompilerStatuses,
  StatelessReportCompilerKnownGaps,
} from './stateless-report-compiler-status-values';
import { countProductionProofValues } from './production-proof-shape';
import {
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export { StatelessReportCompilerKnownGaps };

export const StatelessReportCompilerSchemaVersionSchema = withParser(
  Schema.Literal('stateless-report-compiler-status-proof')
);
export const StatelessReportCompilerStatusSchema = withParser(
  Schema.Literal(...RequiredStatelessReportCompilerStatuses)
);
export const StatelessReportCompilerNonClaimSchema = withParser(
  Schema.Literal(...RequiredStatelessReportCompilerNonClaims)
);

const StatelessReportCompilerRequestIdSchema = brandedNonEmptyStringSchema('StatelessReportCompilerRequestId');
const StatelessReportCompilerStatusRefSchema = brandedNonEmptyStringSchema('StatelessReportCompilerStatusRef');
const StatelessReportCompilerResultRefSchema = brandedNonEmptyStringSchema('StatelessReportCompilerResultRef');
const StatelessReportCompilerConnectorStatusRefSchema = brandedNonEmptyStringSchema('StatelessReportCompilerConnectorStatusRef');
const StatelessReportCompilerCursorRefSchema = brandedNonEmptyStringSchema('StatelessReportCompilerCursorRef');
const StatelessReportCompilerDestinationRefSchema = brandedNonEmptyStringSchema('StatelessReportCompilerDestinationRef');
const StatelessReportCompilerPolicyRefSchema = brandedNonEmptyStringSchema('StatelessReportCompilerPolicyRef');
const StatelessReportCompilerTempArtifactRefSchema = brandedNonEmptyStringSchema('StatelessReportCompilerTempArtifactRef');

const FinalStatuses = ['succeeded', 'failed', 'expired', 'manual-required'] as const;
const FailureStatuses = ['failed', 'expired', 'manual-required'] as const;

const StatelessReportCompilerTimeWindowSchema = withParser(
  Schema.Struct({
    startsAt: ParentTimestampSchema,
    endsAt: ParentTimestampSchema,
  })
);

const StatelessReportCompilerRedactionPolicyBaseSchema = Schema.Struct({
  childDetailMinimized: Schema.Boolean,
  rawEvidenceExcludedFromOutput: Schema.Boolean,
  logsContainOperationalMetadataOnly: Schema.Boolean,
  outputHumanReadableSummaryOnly: Schema.Boolean,
  minimizationPolicyRef: StatelessReportCompilerPolicyRefSchema,
});

export const StatelessReportCompilerRedactionPolicySchema = withParser(
  StatelessReportCompilerRedactionPolicyBaseSchema.pipe(
    Schema.filter(
      (policy) =>
        reportCompilerRedactionPolicyIsSafe(policy) ||
        'Expected stateless report compiler outputs and logs to minimize child detail and exclude raw evidence'
    )
  )
);

const StatelessReportCompilerTempArtifactTtlBaseSchema = Schema.Struct({
  tempInputRef: StatelessReportCompilerTempArtifactRefSchema,
  tempOutputRef: StatelessReportCompilerTempArtifactRefSchema,
  inputTtlMinutes: Schema.Number,
  outputTtlMinutes: Schema.Number,
  deleteBy: ParentTimestampSchema,
  inputDeletedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  outputDeletedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  deletionConfirmed: Schema.Boolean,
});

export const StatelessReportCompilerTempArtifactTtlSchema = withParser(
  StatelessReportCompilerTempArtifactTtlBaseSchema.pipe(
    Schema.filter(
      (ttl) =>
        reportCompilerTempArtifactTtlIsHonest(ttl) ||
        'Expected temporary compiler input and output TTLs to be positive and deletion confirmation to include both delete timestamps'
    )
  )
);

const StatelessReportCompilerRequestBaseSchema = Schema.Struct({
  schemaVersion: StatelessReportCompilerSchemaVersionSchema,
  requestId: StatelessReportCompilerRequestIdSchema,
  family: FamilyReferenceSchema,
  account: ParentAccountReferenceSchema,
  device: ParentDeviceReferenceSchema,
  parentAction: ParentActionReferenceSchema,
  sourceConnectorStatusRef: StatelessReportCompilerConnectorStatusRefSchema,
  sourceCursorRef: StatelessReportCompilerCursorRefSchema,
  requestedDataClasses: Schema.Array(ParentOwnedSyncExportDataClassSchema),
  requestedTimeWindow: StatelessReportCompilerTimeWindowSchema,
  outputDestinationOwnership: ParentOwnedSyncExportDestinationOwnershipSchema,
  outputDestinationRef: StatelessReportCompilerDestinationRefSchema,
  parentAuthorized: Schema.Boolean,
  parentOwnedSourceRequired: Schema.Boolean,
  rawChildEvidenceRequested: Schema.Boolean,
  auditRefs: Schema.Array(ParentEvidenceReferenceSchema),
});

export const StatelessReportCompilerRequestSchema = withParser(
  StatelessReportCompilerRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        reportCompilerRequestIsHonest(request) ||
        'Expected report compiler requests to be parent-authorized, scoped, parent-owned-storage based, and custody-safe'
    )
  )
);

const StatelessReportCompilerStatusRowBaseSchema = Schema.Struct({
  requestId: StatelessReportCompilerRequestIdSchema,
  status: StatelessReportCompilerStatusSchema,
  statusRef: StatelessReportCompilerStatusRefSchema,
  sourceConnectorStatusRef: StatelessReportCompilerConnectorStatusRefSchema,
  sourceCursorRef: StatelessReportCompilerCursorRefSchema,
  resultRef: Schema.Union(StatelessReportCompilerResultRefSchema, Schema.Null),
  failureReasonRef: Schema.Union(StatelessReportCompilerPolicyRefSchema, Schema.Null),
  manualActionRef: Schema.Union(StatelessReportCompilerPolicyRefSchema, Schema.Null),
  statusUpdatedAt: ParentTimestampSchema,
  tempArtifacts: StatelessReportCompilerTempArtifactTtlSchema,
  auditRefs: Schema.Array(ParentEvidenceReferenceSchema),
  localEvidenceMutated: Schema.Boolean,
  parentOwnedStorageMutatedByFailure: Schema.Boolean,
});

export const StatelessReportCompilerStatusRowSchema = withParser(
  StatelessReportCompilerStatusRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        reportCompilerStatusRowIsHonest(row) ||
        'Expected compiler status rows to cover lifecycle states without mutating local evidence or parent-owned storage on failure'
    )
  )
);

const StatelessReportCompilerResultBaseSchema = Schema.Struct({
  requestId: StatelessReportCompilerRequestIdSchema,
  resultRef: StatelessReportCompilerResultRefSchema,
  status: StatelessReportCompilerStatusSchema,
  outputDestinationOwnership: ParentOwnedSyncExportDestinationOwnershipSchema,
  outputReportRef: Schema.Union(StatelessReportCompilerDestinationRefSchema, Schema.Null),
  failureReasonRef: Schema.Union(StatelessReportCompilerPolicyRefSchema, Schema.Null),
  redaction: StatelessReportCompilerRedactionPolicySchema,
  tempArtifacts: StatelessReportCompilerTempArtifactTtlSchema,
  auditRefs: Schema.Array(ParentEvidenceReferenceSchema),
  localEvidenceMutated: Schema.Boolean,
  parentOwnedStorageMutatedByFailure: Schema.Boolean,
  ocentraHostedReportRetained: Schema.Boolean,
});

export const StatelessReportCompilerResultSchema = withParser(
  StatelessReportCompilerResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        reportCompilerResultIsHonest(result) ||
        'Expected compiler results to keep outputs parent-owned, temporary artifacts deleted, and failures non-mutating'
    )
  )
);

const StatelessReportCompilerContractProofBaseSchema = Schema.Struct({
  schemaVersion: StatelessReportCompilerSchemaVersionSchema,
  request: StatelessReportCompilerRequestSchema,
  statuses: Schema.Array(StatelessReportCompilerStatusRowSchema),
  results: Schema.Array(StatelessReportCompilerResultSchema),
  nonClaims: Schema.Array(StatelessReportCompilerNonClaimSchema),
  reportCompilerRuntimeClaimed: Schema.Boolean,
  cloudWorkerClaimed: Schema.Boolean,
  connectorOAuthProviderApiClaimed: Schema.Boolean,
  portalUiClaimed: Schema.Boolean,
  ocentraHostedFamilyDataCustodyClaimed: Schema.Boolean,
  uploadDownloadImplementationClaimed: Schema.Boolean,
  childDeviceMutationClaimed: Schema.Boolean,
  retainedTempChildEvidenceClaimed: Schema.Boolean,
  updatedAt: ParentTimestampSchema,
});

export const StatelessReportCompilerContractProofSchema = withParser(
  StatelessReportCompilerContractProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        reportCompilerContractProofIsHonest(proof) ||
        'Expected stateless report compiler proof to cover request scope, lifecycle statuses, results, TTL deletion, and custody non-claims'
    )
  )
);

type RedactionPolicyCandidate = Infer<typeof StatelessReportCompilerRedactionPolicyBaseSchema>;
type TempArtifactTtlCandidate = Infer<typeof StatelessReportCompilerTempArtifactTtlBaseSchema>;
type RequestCandidate = Infer<typeof StatelessReportCompilerRequestBaseSchema>;
type StatusRowCandidate = Infer<typeof StatelessReportCompilerStatusRowBaseSchema>;
type ResultCandidate = Infer<typeof StatelessReportCompilerResultBaseSchema>;
type ContractProofCandidate = Infer<typeof StatelessReportCompilerContractProofBaseSchema>;

export type StatelessReportCompilerStatus = Infer<typeof StatelessReportCompilerStatusSchema>;
export type StatelessReportCompilerNonClaim = Infer<typeof StatelessReportCompilerNonClaimSchema>;
export type StatelessReportCompilerRequest = Infer<typeof StatelessReportCompilerRequestSchema>;
export type StatelessReportCompilerStatusRow = Infer<typeof StatelessReportCompilerStatusRowSchema>;
export type StatelessReportCompilerResult = Infer<typeof StatelessReportCompilerResultSchema>;
export type StatelessReportCompilerContractProof = Infer<typeof StatelessReportCompilerContractProofSchema>;

const Timestamp = '2026-06-03T10:25:39.382Z';
const RequestId = 'stateless-report-compiler-request-proof-1';
const Family = { familyId: 'family-stateless-report-compiler-proof-1' } as const;
const Account = { parentAccountId: 'parent-account-stateless-report-compiler-proof-1' } as const;
const Device = {
  deviceId: 'windows-child-device-report-compiler-proof-1',
  childProfileId: 'child-report-compiler-proof-1',
  label: 'Windows child device report compiler proof',
  platform: 'windows',
} as const;
const ParentAction = {
  actionReferenceId: 'parent-action-report-compiler-proof-1',
  actor: { actorId: 'parent-report-compiler-proof-1', role: 'parent' },
  policyVersion: 'stateless-report-compiler-policy-v1',
  createdAt: Timestamp,
} as const;
const EvidenceRef = {
  evidenceReferenceId: 'evidence-report-compiler-proof-1',
  kind: 'query-store-summary',
  observedAt: Timestamp,
} as const;
const ConnectorStatusRef = 'connector-status-google-drive-ready';
const CursorRef = 'cursor-parent-owned-storage-report-window';

export const StatelessReportCompilerContractProofReadModel = StatelessReportCompilerContractProofSchema.parse({
  schemaVersion: 'stateless-report-compiler-status-proof',
  request: {
    schemaVersion: 'stateless-report-compiler-status-proof',
    requestId: RequestId,
    family: Family,
    account: Account,
    device: Device,
    parentAction: ParentAction,
    sourceConnectorStatusRef: ConnectorStatusRef,
    sourceCursorRef: CursorRef,
    requestedDataClasses: ['sqlite-query-row', 'audit-event', 'notification-history', 'generated-summary'],
    requestedTimeWindow: {
      startsAt: '2026-06-01T00:00:00.000Z',
      endsAt: '2026-06-03T00:00:00.000Z',
    },
    outputDestinationOwnership: 'parent-owned-external-storage',
    outputDestinationRef: 'parent-owned-storage-report-output-ref',
    parentAuthorized: true,
    parentOwnedSourceRequired: true,
    rawChildEvidenceRequested: false,
    auditRefs: [EvidenceRef],
  },
  statuses: [
    status('queued', null, null, null, tempArtifacts(false)),
    status('running', null, null, null, tempArtifacts(false)),
    status('succeeded', 'result-report-compiler-succeeded-proof', null, null, tempArtifacts(true)),
    status('failed', null, 'provider-read-failed-without-source-mutation', null, tempArtifacts(true)),
    status('expired', null, 'temporary-input-expired-before-compile', null, tempArtifacts(true)),
    status(
      'manual-required',
      null,
      'parent-owned-storage-manual-review-required',
      'manual-review-report-source',
      tempArtifacts(true)
    ),
  ],
  results: [
    result('result-report-compiler-succeeded-proof', 'succeeded', 'parent-owned-storage-report-output-ref', null),
    result('result-report-compiler-failed-proof', 'failed', null, 'provider-read-failed-without-source-mutation'),
    result('result-report-compiler-expired-proof', 'expired', null, 'temporary-input-expired-before-compile'),
    result(
      'result-report-compiler-manual-proof',
      'manual-required',
      null,
      'parent-owned-storage-manual-review-required'
    ),
  ],
  nonClaims: [...RequiredStatelessReportCompilerNonClaims],
  reportCompilerRuntimeClaimed: false,
  cloudWorkerClaimed: false,
  connectorOAuthProviderApiClaimed: false,
  portalUiClaimed: false,
  ocentraHostedFamilyDataCustodyClaimed: false,
  uploadDownloadImplementationClaimed: false,
  childDeviceMutationClaimed: false,
  retainedTempChildEvidenceClaimed: false,
  updatedAt: Timestamp,
});

export function summarizeStatelessReportCompilerStatuses(
  rows: ReadonlyArray<StatelessReportCompilerStatusRow>
): Record<StatelessReportCompilerStatus, number> {
  return countProductionProofValues(
    rows.map((row) => row.status),
    RequiredStatelessReportCompilerStatuses
  );
}

export function summarizeStatelessReportCompilerRequestedDataClasses(
  request: StatelessReportCompilerRequest
): Record<ParentOwnedSyncExportDataClass, number> {
  return countProductionProofValues(request.requestedDataClasses, [
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

function reportCompilerRedactionPolicyIsSafe(policy: RedactionPolicyCandidate): boolean {
  return (
    policy.childDetailMinimized &&
    policy.rawEvidenceExcludedFromOutput &&
    policy.logsContainOperationalMetadataOnly &&
    policy.outputHumanReadableSummaryOnly
  );
}

function reportCompilerTempArtifactTtlIsHonest(ttl: TempArtifactTtlCandidate): boolean {
  if (ttl.inputTtlMinutes <= 0 || ttl.outputTtlMinutes <= 0) {
    return false;
  }
  if (ttl.deletionConfirmed) {
    return ttl.inputDeletedAt !== null && ttl.outputDeletedAt !== null;
  }
  return ttl.inputDeletedAt === null && ttl.outputDeletedAt === null;
}

function reportCompilerRequestIsHonest(request: RequestCandidate): boolean {
  return (
    request.parentAuthorized &&
    request.parentOwnedSourceRequired &&
    request.requestedDataClasses.length > 0 &&
    request.auditRefs.length > 0 &&
    !request.rawChildEvidenceRequested &&
    request.outputDestinationOwnership !== 'ocentra-hosted-non-activity-metadata'
  );
}

function reportCompilerStatusRowIsHonest(row: StatusRowCandidate): boolean {
  if (row.auditRefs.length === 0 || row.localEvidenceMutated) {
    return false;
  }
  if (FailureStatuses.includes(row.status as (typeof FailureStatuses)[number])) {
    return (
      row.resultRef === null &&
      row.failureReasonRef !== null &&
      !row.parentOwnedStorageMutatedByFailure &&
      row.tempArtifacts.deletionConfirmed
    );
  }
  if (row.status === 'succeeded') {
    return row.resultRef !== null && row.failureReasonRef === null && row.tempArtifacts.deletionConfirmed;
  }
  return row.resultRef === null && row.failureReasonRef === null && !row.tempArtifacts.deletionConfirmed;
}

function reportCompilerResultIsHonest(result: ResultCandidate): boolean {
  if (
    !FinalStatuses.includes(result.status as (typeof FinalStatuses)[number]) ||
    result.auditRefs.length === 0 ||
    result.localEvidenceMutated ||
    result.ocentraHostedReportRetained ||
    result.outputDestinationOwnership === 'ocentra-hosted-non-activity-metadata' ||
    !result.tempArtifacts.deletionConfirmed
  ) {
    return false;
  }
  if (result.status === 'succeeded') {
    return result.outputReportRef !== null && result.failureReasonRef === null;
  }
  return (
    result.outputReportRef === null && result.failureReasonRef !== null && !result.parentOwnedStorageMutatedByFailure
  );
}

function reportCompilerContractProofIsHonest(proof: ContractProofCandidate): boolean {
  return (
    reportCompilerProofCoversStatuses(proof.statuses) &&
    reportCompilerProofCoversFinalResults(proof.results) &&
    reportCompilerProofHasRequiredNonClaims(proof.nonClaims) &&
    proof.statuses.every((row) => row.requestId === proof.request.requestId) &&
    proof.results.every((row) => row.requestId === proof.request.requestId) &&
    reportCompilerRuntimeClaimsAreFalse(proof)
  );
}

function reportCompilerRuntimeClaimsAreFalse(proof: ContractProofCandidate): boolean {
  return (
    !proof.reportCompilerRuntimeClaimed &&
    !proof.cloudWorkerClaimed &&
    !proof.connectorOAuthProviderApiClaimed &&
    !proof.portalUiClaimed &&
    !proof.ocentraHostedFamilyDataCustodyClaimed &&
    !proof.uploadDownloadImplementationClaimed &&
    !proof.childDeviceMutationClaimed &&
    !proof.retainedTempChildEvidenceClaimed
  );
}

function reportCompilerProofCoversStatuses(rows: readonly StatelessReportCompilerStatusRow[]): boolean {
  const statuses = new Set(rows.map((row) => row.status));
  return RequiredStatelessReportCompilerStatuses.every((statusEntry) => statuses.has(statusEntry));
}

function reportCompilerProofCoversFinalResults(results: readonly StatelessReportCompilerResult[]): boolean {
  const statuses = new Set(results.map((resultEntry) => resultEntry.status));
  return FinalStatuses.every((statusEntry) => statuses.has(statusEntry));
}

function reportCompilerProofHasRequiredNonClaims(nonClaims: readonly StatelessReportCompilerNonClaim[]): boolean {
  const claims = new Set(nonClaims);
  return (
    claims.size === nonClaims.length && RequiredStatelessReportCompilerNonClaims.every((claim) => claims.has(claim))
  );
}

function status(
  statusValue: StatelessReportCompilerStatus,
  resultRef: string | null,
  failureReasonRef: string | null,
  manualActionRef: string | null,
  tempArtifactsValue: unknown
): StatelessReportCompilerStatusRow {
  return StatelessReportCompilerStatusRowSchema.parse({
    requestId: RequestId,
    status: statusValue,
    statusRef: `status-${statusValue}`,
    sourceConnectorStatusRef: ConnectorStatusRef,
    sourceCursorRef: CursorRef,
    resultRef,
    failureReasonRef,
    manualActionRef,
    statusUpdatedAt: Timestamp,
    tempArtifacts: tempArtifactsValue,
    auditRefs: [EvidenceRef],
    localEvidenceMutated: false,
    parentOwnedStorageMutatedByFailure: false,
  });
}

function result(
  resultRef: string,
  statusValue: (typeof FinalStatuses)[number],
  outputReportRef: string | null,
  failureReasonRef: string | null
): StatelessReportCompilerResult {
  return StatelessReportCompilerResultSchema.parse({
    requestId: RequestId,
    resultRef,
    status: statusValue,
    outputDestinationOwnership: 'parent-owned-external-storage',
    outputReportRef,
    failureReasonRef,
    redaction: redactionPolicy(),
    tempArtifacts: tempArtifacts(true),
    auditRefs: [EvidenceRef],
    localEvidenceMutated: false,
    parentOwnedStorageMutatedByFailure: false,
    ocentraHostedReportRetained: false,
  });
}

function redactionPolicy() {
  return {
    childDetailMinimized: true,
    rawEvidenceExcludedFromOutput: true,
    logsContainOperationalMetadataOnly: true,
    outputHumanReadableSummaryOnly: true,
    minimizationPolicyRef: 'redaction-minimization-report-compiler-proof',
  };
}

function tempArtifacts(deleted: boolean) {
  return {
    tempInputRef: 'temp-input-report-compiler-proof',
    tempOutputRef: 'temp-output-report-compiler-proof',
    inputTtlMinutes: 15,
    outputTtlMinutes: 15,
    deleteBy: '2026-06-03T10:40:39.382Z',
    inputDeletedAt: deleted ? Timestamp : null,
    outputDeletedAt: deleted ? Timestamp : null,
    deletionConfirmed: deleted,
  };
}

