import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingFullProductUiRuntimeArtifactPlan,
  buildTrackingFullProductUiRuntimeArtifactGateProof,
} from '../src/tracking-full-product-ui-runtime-artifact-gate-proof';
import {
  RequiredTrackingFullProductUiRuntimePreflightPlan,
  TrackingFullProductUiRuntimePreflightRowSchema,
  buildTrackingFullProductUiRuntimePreflightProof,
} from '../src/tracking-full-product-ui-runtime-preflight-proof';

describe('tracking full product UI runtime preflight proof', () => {
  it('keeps hard product UI runtime artifacts manual-required with concrete acceptance rows', () => {
    const runtimeGateProof = buildTrackingFullProductUiRuntimeArtifactGateProof('2026-06-08T12:30:00.000Z', {
      presentArtifacts: RequiredTrackingFullProductUiRuntimeArtifactPlan.requiredArtifacts.filter(
        (artifact) =>
          !artifact.includes('04-retention-settings-production-write-result') &&
          !artifact.includes('05-child-device-rendered-check-in-runtime') &&
          !artifact.includes('06-child-device-rendered-location-consent-runtime') &&
          !artifact.includes('07-child-device-safe-help-response-runtime')
      ),
    });

    const proof = buildTrackingFullProductUiRuntimePreflightProof('2026-06-08T12:30:00.000Z', runtimeGateProof);

    expect(proof.rows).toHaveLength(4);
    expect(proof.rows.map((row) => row.area)).toEqual([
      'retention-settings-production-write-result',
      'rendered-child-device-check-in',
      'rendered-child-device-location-consent',
      'child-device-safe-help-response',
    ]);
    expect(proof.rows.every((row) => row.status === 'manual-required')).toBe(true);
    expect(proof.rows.every((row) => row.acceptanceCriteria.length >= 3)).toBe(true);
    expect(proof.rows.every((row) => row.manualValidationCommands.length >= 2)).toBe(true);
    expect(proof.summary).toMatchObject({
      rowCount: 4,
      manualRequiredRowCount: 4,
      requiredArtifactCount: 4,
      presentArtifactCount: 0,
      missingArtifactCount: 4,
      productReadyRowCount: 0,
    });
    expect(proof.productClaims.fullProductUiRuntimeClaimed).toBe(false);
    expect(proof.productClaims.childDeviceRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('rejects preflight when every full product UI runtime artifact is present', () => {
    const runtimeGateProof = buildTrackingFullProductUiRuntimeArtifactGateProof('2026-06-08T12:30:00.000Z', {
      presentArtifacts: RequiredTrackingFullProductUiRuntimeArtifactPlan.requiredArtifacts,
    });

    expect(() => buildTrackingFullProductUiRuntimePreflightProof('2026-06-08T12:30:00.000Z', runtimeGateProof)).toThrow(
      'requires missing artifact'
    );
  });
});

describe('tracking full product UI runtime preflight row schema', () => {
  it('rejects rows that convert preflight into a product-ready UI claim', () => {
    const invalid = TrackingFullProductUiRuntimePreflightRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-full-product-ui-runtime-preflight-invalid',
      generatedAt: '2026-06-08T12:30:00.000Z',
      area: 'rendered-child-device-check-in',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      status: 'manual-required',
      sourceRuntimeArtifactGateProofRef:
        RequiredTrackingFullProductUiRuntimePreflightPlan.sourceRuntimeArtifactGateProofRef,
      sourceMissingArtifactRef:
        'output/tracking-plan-proof/product-parent-child-ui-runtime/05-child-device-rendered-check-in-runtime.png',
      acceptanceCriteria: ['criterion-one', 'criterion-two', 'criterion-three'],
      manualValidationCommands: ['command-one', 'command-two'],
      requiredArtifacts: [
        'output/tracking-plan-proof/product-parent-child-ui-runtime/05-child-device-rendered-check-in-runtime.png',
      ],
      presentArtifacts: [],
      missingArtifacts: [
        'output/tracking-plan-proof/product-parent-child-ui-runtime/05-child-device-rendered-check-in-runtime.png',
      ],
      artifactAcceptanceNotes: ['note-one'],
      auditRefs: ['tracking-full-product-ui-runtime-preflight-invalid-audit'],
      fullProductUiRuntimeClaimed: true,
      childDeviceRuntimeClaimed: false,
      retentionProductionWriteClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});
