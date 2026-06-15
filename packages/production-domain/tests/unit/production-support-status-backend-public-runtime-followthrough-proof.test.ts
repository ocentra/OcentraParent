import { describe, expect, it } from 'vitest';
import {
  ProductionSupportStatusBackendPublicRuntimeFollowthroughProofSchema,
  ProductionSupportStatusBackendPublicRuntimeFollowthroughRowSchema,
  summarizeProductionSupportStatusBackendPublicRuntimeFollowthroughRows,
} from '../../src/production-support-status-backend-public-runtime-followthrough-proof';
import { ProductionSupportStatusBackendPublicRuntimeFollowthroughReadModel } from '../../src/production-support-status-backend-public-runtime-followthrough-read-model';

describe('production support status backend public runtime follow-through proof', () => {
  acceptsFollowthroughRows();
  rejectsRuntimeAndBackendOverclaims();
  rejectsForbiddenFollowthroughData();
  rejectsIncompleteFollowthroughCoverage();
});

function acceptsFollowthroughRows(): void {
  it('accepts each target with requested queued running succeeded failed and manual states', () => {
    const proof = ProductionSupportStatusBackendPublicRuntimeFollowthroughProofSchema.parse(
      ProductionSupportStatusBackendPublicRuntimeFollowthroughReadModel
    );

    for (const targetSummary of Object.values(
      summarizeProductionSupportStatusBackendPublicRuntimeFollowthroughRows(proof.rows)
    )) {
      expect(targetSummary).toEqual({
        requested: 1,
        queued: 1,
        running: 1,
        succeeded: 1,
        failed: 1,
        'manual-required': 1,
      });
    }
    expect(proof.publicRuntimeExecutionClaim).toBe('not-implemented');
    expect(proof.statusBackendExecutionClaim).toBe('manual-required');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsRuntimeAndBackendOverclaims(): void {
  it('rejects implemented or executed public runtime status backend and upload states', () => {
    const supportRuntimeRow = requiredFollowthrough('support-status-public-runtime-followthrough', 'requested');
    const incidentBackendRow = requiredFollowthrough('incident-status-backend-followthrough', 'running');
    const uploadBackendRow = requiredFollowthrough('support-upload-status-backend-followthrough', 'succeeded');

    expect(
      ProductionSupportStatusBackendPublicRuntimeFollowthroughRowSchema.safeParse({
        ...supportRuntimeRow,
        publicRuntimeFollowthroughState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendPublicRuntimeFollowthroughRowSchema.safeParse({
        ...incidentBackendRow,
        statusBackendFollowthroughState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendPublicRuntimeFollowthroughRowSchema.safeParse({
        ...uploadBackendRow,
        supportBackendUploadState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsForbiddenFollowthroughData(): void {
  it('rejects public runtime payloads or omitted provider-secret exclusions', () => {
    const row = requiredFollowthrough('support-status-public-runtime-followthrough', 'queued');

    expect(
      ProductionSupportStatusBackendPublicRuntimeFollowthroughRowSchema.safeParse({
        ...row,
        supportSafeDataClasses: [...row.supportSafeDataClasses, 'public-runtime-payload'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendPublicRuntimeFollowthroughRowSchema.safeParse({
        ...row,
        forbiddenDataClasses: row.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secrets'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteFollowthroughCoverage(): void {
  it('rejects proof missing follow-through state coverage or status backend non-claim', () => {
    expect(
      ProductionSupportStatusBackendPublicRuntimeFollowthroughProofSchema.safeParse({
        ...ProductionSupportStatusBackendPublicRuntimeFollowthroughReadModel,
        rows: ProductionSupportStatusBackendPublicRuntimeFollowthroughReadModel.rows.filter(
          (row) =>
            row.target !== 'public-support-contact-status-backend-followthrough' ||
            row.followthroughState !== 'manual-required'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendPublicRuntimeFollowthroughProofSchema.safeParse({
        ...ProductionSupportStatusBackendPublicRuntimeFollowthroughReadModel,
        nonClaims: ProductionSupportStatusBackendPublicRuntimeFollowthroughReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-status-backend-execution'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredFollowthrough(
  target:
    | 'support-status-public-runtime-followthrough'
    | 'incident-status-backend-followthrough'
    | 'support-upload-status-backend-followthrough',
  followthroughState: 'requested' | 'queued' | 'running' | 'succeeded'
): (typeof ProductionSupportStatusBackendPublicRuntimeFollowthroughReadModel.rows)[number] {
  const row = ProductionSupportStatusBackendPublicRuntimeFollowthroughReadModel.rows.find(
    (entry) => entry.target === target && entry.followthroughState === followthroughState
  );
  if (row === undefined) {
    throw new Error(`missing status backend/public runtime follow-through row: ${target} ${followthroughState}`);
  }
  return row;
}
