import { describe, expect, it } from 'vitest';
import {
  StatelessReportCompilerContractProofReadModel,
  StatelessReportCompilerContractProofSchema,
  StatelessReportCompilerKnownGaps,
  StatelessReportCompilerRequestSchema,
  StatelessReportCompilerResultSchema,
  StatelessReportCompilerStatusRowSchema,
  summarizeStatelessReportCompilerRequestedDataClasses,
  summarizeStatelessReportCompilerStatuses,
} from '../src/stateless-report-compiler-status';

describe('stateless report compiler status contracts', () => {
  acceptsTheContractOnlyCompilerStatusProof();
  rejectsRuntimeCloudConnectorPortalCustodyAndMutationClaims();
  rejectsUnsafeRequestScopeAndOutputOwnership();
  rejectsInvalidStatusResultAndTemporaryArtifactStates();
});

function acceptsTheContractOnlyCompilerStatusProof(): void {
  it('covers parent-authorized request scope lifecycle states results TTL deletion and custody boundaries', () => {
    const proof = StatelessReportCompilerContractProofSchema.parse(StatelessReportCompilerContractProofReadModel);

    expect(summarizeStatelessReportCompilerStatuses(proof.statuses)).toEqual({
      queued: 1,
      running: 1,
      succeeded: 1,
      failed: 1,
      expired: 1,
      'manual-required': 1,
    });
    expect(summarizeStatelessReportCompilerRequestedDataClasses(proof.request)).toEqual({
      'encrypted-journal-segment': 0,
      'sqlite-query-row': 1,
      'parent-rule': 0,
      'approval-decision': 0,
      'device-registry-entry': 0,
      'notification-history': 1,
      'audit-event': 1,
      'generated-summary': 1,
    });
    expect(proof.results.map((result) => result.status)).toEqual(['succeeded', 'failed', 'expired', 'manual-required']);
    expect(proof.request.outputDestinationOwnership).toBe('parent-owned-external-storage');
    expect(proof.results.every((result) => result.tempArtifacts.deletionConfirmed)).toBe(true);
    expect(StatelessReportCompilerKnownGaps).toEqual([
      'No report compiler runtime or cloud worker is implemented by this parent-domain proof.',
      'No connector OAuth, token vault, provider API, upload, or download implementation is claimed.',
      'No portal UI, CLI control, or account/subscription backend is implemented.',
      'No Ocentra-hosted custody of family activity data, generated reports, source bundles, or temporary child evidence is claimed.',
      'Real parent-owned storage reads, report rendering, deletion execution, and audit persistence remain future work.',
    ]);
  });
}

function rejectsRuntimeCloudConnectorPortalCustodyAndMutationClaims(): void {
  it('rejects compiler runtime cloud worker connector portal custody upload child mutation and retained temp evidence claims', () => {
    const proof = StatelessReportCompilerContractProofReadModel;

    for (const invalidProof of [
      { ...proof, reportCompilerRuntimeClaimed: true },
      { ...proof, cloudWorkerClaimed: true },
      { ...proof, connectorOAuthProviderApiClaimed: true },
      { ...proof, portalUiClaimed: true },
      { ...proof, ocentraHostedFamilyDataCustodyClaimed: true },
      { ...proof, uploadDownloadImplementationClaimed: true },
      { ...proof, childDeviceMutationClaimed: true },
      { ...proof, retainedTempChildEvidenceClaimed: true },
      { ...proof, nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-cloud-worker') },
    ]) {
      expect(StatelessReportCompilerContractProofSchema.safeParse(invalidProof).success).toBe(false);
    }
  });
}

function rejectsUnsafeRequestScopeAndOutputOwnership(): void {
  it('rejects requests that are not parent authorized or try to use Ocentra-hosted output custody', () => {
    const request = StatelessReportCompilerContractProofReadModel.request;

    for (const invalidRequest of [
      { ...request, parentAuthorized: false },
      { ...request, parentOwnedSourceRequired: false },
      { ...request, requestedDataClasses: [] },
      { ...request, rawChildEvidenceRequested: true },
      { ...request, auditRefs: [] },
      { ...request, outputDestinationOwnership: 'ocentra-hosted-non-activity-metadata' },
    ]) {
      expect(StatelessReportCompilerRequestSchema.safeParse(invalidRequest).success).toBe(false);
    }
  });
}

function rejectsInvalidStatusResultAndTemporaryArtifactStates(): void {
  it('rejects lifecycle rows and results that omit refs delete proof or failure non-mutation guarantees', () => {
    const succeededStatus = statusFor('succeeded');
    const failedStatus = statusFor('failed');
    const succeededResult = resultFor('succeeded');
    const failedResult = resultFor('failed');

    expect(StatelessReportCompilerStatusRowSchema.safeParse({ ...succeededStatus, resultRef: null }).success).toBe(
      false
    );
    expect(
      StatelessReportCompilerStatusRowSchema.safeParse({
        ...failedStatus,
        parentOwnedStorageMutatedByFailure: true,
      }).success
    ).toBe(false);
    expect(
      StatelessReportCompilerStatusRowSchema.safeParse({
        ...failedStatus,
        tempArtifacts: { ...failedStatus.tempArtifacts, outputDeletedAt: null },
      }).success
    ).toBe(false);
    expect(StatelessReportCompilerResultSchema.safeParse({ ...succeededResult, outputReportRef: null }).success).toBe(
      false
    );
    expect(
      StatelessReportCompilerResultSchema.safeParse({
        ...failedResult,
        outputReportRef: 'failed-output-should-not-exist',
      }).success
    ).toBe(false);
    expect(
      StatelessReportCompilerResultSchema.safeParse({
        ...failedResult,
        redaction: { ...failedResult.redaction, rawEvidenceExcludedFromOutput: false },
      }).success
    ).toBe(false);
  });
}

function statusFor(status: 'succeeded' | 'failed') {
  const row = StatelessReportCompilerContractProofReadModel.statuses.find((candidate) => candidate.status === status);
  if (row === undefined) {
    throw new Error(`missing stateless report compiler status: ${status}`);
  }
  return row;
}

function resultFor(status: 'succeeded' | 'failed') {
  const result = StatelessReportCompilerContractProofReadModel.results.find((candidate) => candidate.status === status);
  if (result === undefined) {
    throw new Error(`missing stateless report compiler result: ${status}`);
  }
  return result;
}
