import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingRetentionRuntimeArtifactPlan,
  buildTrackingRetentionRuntimeArtifactGateProof,
} from '../src/tracking-retention-runtime-artifact-gate-proof';
import {
  RequiredTrackingRetentionPlatformEnforcementArtifactRef,
  RequiredTrackingRetentionPlatformEnforcementPreflightPlan,
  TrackingRetentionPlatformEnforcementPreflightRowSchema,
  buildTrackingRetentionPlatformEnforcementPreflightProof,
} from '../src/tracking-retention-platform-enforcement-preflight-proof';

describe('tracking retention platform enforcement preflight proof', () => {
  it('keeps platform runtime retention enforcement manual-required with concrete platform rows', () => {
    const runtimeGateProof = buildTrackingRetentionRuntimeArtifactGateProof('2026-06-08T11:50:00.000Z', {
      presentArtifacts: ['tracking-retention/product-settings-writable-execution.json'],
    });

    const proof = buildTrackingRetentionPlatformEnforcementPreflightProof('2026-06-08T11:50:00.000Z', runtimeGateProof);

    expect(proof.rows).toHaveLength(3);
    expect(proof.rows.map((row) => row.platform)).toEqual([
      'android-device-policy',
      'ios-family-controls',
      'desktop-managed-policy',
    ]);
    expect(proof.rows.every((row) => row.status === 'manual-required')).toBe(true);
    expect(proof.rows.every((row) => row.acceptanceCriteria.length >= 3)).toBe(true);
    expect(proof.rows.every((row) => row.manualValidationCommands.length >= 2)).toBe(true);
    expect(proof.rows.every((row) => row.requiredArtifacts.length === row.missingArtifacts.length)).toBe(true);
    expect(proof.sourceMissingArtifactRef).toBe(RequiredTrackingRetentionPlatformEnforcementArtifactRef);
    expect(proof.summary).toMatchObject({
      rowCount: 3,
      manualRequiredRowCount: 3,
      requiredArtifactCount: 6,
      presentArtifactCount: 0,
      missingArtifactCount: 6,
      productReadyRowCount: 0,
    });
    expect(proof.productClaims.platformRuntimeRetentionEnforcementClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('rejects a preflight when the runtime gate no longer has the platform enforcement artifact missing', () => {
    const runtimeGateProof = buildTrackingRetentionRuntimeArtifactGateProof('2026-06-08T11:50:00.000Z', {
      presentArtifacts: [...RequiredTrackingRetentionRuntimeArtifactPlan.requiredArtifacts],
    });

    expect(() =>
      buildTrackingRetentionPlatformEnforcementPreflightProof('2026-06-08T11:50:00.000Z', runtimeGateProof)
    ).toThrow('platform runtime artifact to be missing');
  });

  it('rejects rows that turn a preflight into a product-ready retention claim', () => {
    const invalid = TrackingRetentionPlatformEnforcementPreflightRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-retention-platform-enforcement-invalid',
      generatedAt: '2026-06-08T11:50:00.000Z',
      platform: 'android-device-policy',
      requiredProofTier: 'P4_PRODUCTION_RUNTIME',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      status: 'manual-required',
      sourceRuntimeArtifactGateProofRef:
        RequiredTrackingRetentionPlatformEnforcementPreflightPlan.sourceRuntimeArtifactGateProofRef,
      sourceMissingArtifactRef: RequiredTrackingRetentionPlatformEnforcementArtifactRef,
      acceptanceCriteria: ['criterion-one', 'criterion-two', 'criterion-three'],
      manualValidationCommands: ['command-one', 'command-two'],
      requiredArtifacts: ['tracking-retention/platform-runtime-retention-enforcement/android-device-policy-write.json'],
      presentArtifacts: [],
      missingArtifacts: ['tracking-retention/platform-runtime-retention-enforcement/android-device-policy-write.json'],
      artifactAcceptanceNotes: ['note-one'],
      auditRefs: ['tracking-retention-platform-enforcement-invalid-audit'],
      platformRuntimeRetentionEnforcementClaimed: true,
      writableProductSettingsExecutionClaimed: false,
      childDeviceDeliveryClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});
