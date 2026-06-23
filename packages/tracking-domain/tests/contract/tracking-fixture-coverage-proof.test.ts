import { describe, expect, it } from 'vitest';
import {
  TrackingFixtureCoverageReadModelSchema,
  buildTrackingFixtureCoverageReadModel,
} from '@ocentra-parent/schema-domain/tracking-fixture-coverage-proof';

const RequiredStates = [
  'fresh',
  'stale',
  'offline',
  'permission-denied',
  'low-accuracy',
  'ambiguous-nearby-place',
  'exception-active',
  'parent-acknowledged',
  'child-check-in-requested',
  'temporary-live-expired',
  'missing-device',
  'retention-deleted',
  'remote-sync-disabled',
  'remote-ai-disabled',
] as const;

describe('tracking fixture coverage proof', () => {
  registerCoverageSummaryCases();
  registerRemoteDisabledCases();
  registerValidationCases();
});

function registerCoverageSummaryCases() {
  it('covers every required fixture state with artifact refs and no product claims', () => {
    const readModel = buildTrackingFixtureCoverageReadModel();

    expect(readModel.rows.map((row) => row.state)).toEqual(RequiredStates);
    expect(readModel.summary).toEqual({
      requiredStateCount: 14,
      coveredStateCount: 14,
      manualRequiredStateCount: 0,
      productClaimReadyRows: 0,
      liveDeviceClaimedRows: 0,
      providerDeliveryClaimedRows: 0,
      childDeviceRuntimeClaimedRows: 0,
      physicalDeviceClaimedRows: 0,
      productionWorkerClaimedRows: 0,
    });
    expect(readModel.rows.every((row) => row.artifactRefs.length > 0)).toBe(true);
  });
}

function registerRemoteDisabledCases() {
  it('keeps remote-sync disabled at contract tier only', () => {
    const readModel = buildTrackingFixtureCoverageReadModel();
    const remoteSync = readModel.rows.find((row) => row.state === 'remote-sync-disabled');

    expect(remoteSync).toEqual({
      state: 'remote-sync-disabled',
      requiredProofTier: 'P1_FIXTURE_SIMULATION',
      currentProofTier: 'P0_CONTRACT',
      status: 'covered',
      artifactRefs: ['output/tracking-plan-proof/07-retention-and-custody-model/'],
      proofRequirement: 'Remote sync disabled-by-default contract proof exists.',
      productClaimReady: false,
      liveDeviceClaimed: false,
      providerDeliveryClaimed: false,
      childDeviceRuntimeClaimed: false,
      physicalDeviceClaimed: false,
      productionWorkerClaimed: false,
      claimBoundary:
        'Fixture coverage proves parser/read-model/UI-state evidence only; live devices, provider delivery, production workers, and product readiness remain separate gates.',
    });
  });

  it('keeps remote-AI disabled at contract tier only', () => {
    const readModel = buildTrackingFixtureCoverageReadModel();
    const remoteAi = readModel.rows.find((row) => row.state === 'remote-ai-disabled');

    expect(remoteAi).toEqual({
      state: 'remote-ai-disabled',
      requiredProofTier: 'P1_FIXTURE_SIMULATION',
      currentProofTier: 'P0_CONTRACT',
      status: 'covered',
      artifactRefs: ['output/tracking-plan-proof/24-ai-provider-routing/'],
      proofRequirement: 'Remote AI disabled-by-default provider routing proof exists.',
      productClaimReady: false,
      liveDeviceClaimed: false,
      providerDeliveryClaimed: false,
      childDeviceRuntimeClaimed: false,
      physicalDeviceClaimed: false,
      productionWorkerClaimed: false,
      claimBoundary:
        'Fixture coverage proves parser/read-model/UI-state evidence only; live devices, provider delivery, production workers, and product readiness remain separate gates.',
    });
  });
}

function registerValidationCases() {
  it('rejects missing fixture states and product-ready overclaims', () => {
    const readModel = buildTrackingFixtureCoverageReadModel();
    const missingState = TrackingFixtureCoverageReadModelSchema.safeParse({
      ...readModel,
      rows: readModel.rows.filter((row) => row.state !== 'missing-device'),
      summary: {
        ...readModel.summary,
        requiredStateCount: 13,
        coveredStateCount: 13,
      },
    });
    const overclaim = TrackingFixtureCoverageReadModelSchema.safeParse({
      ...readModel,
      rows: readModel.rows.map((row) =>
        row.state === 'fresh' ? { ...row, productClaimReady: true, liveDeviceClaimed: true } : row
      ),
      summary: {
        ...readModel.summary,
        productClaimReadyRows: 1,
        liveDeviceClaimedRows: 1,
      },
    });

    expect(missingState.success).toBe(false);
    expect(overclaim.success).toBe(false);
  });
}
