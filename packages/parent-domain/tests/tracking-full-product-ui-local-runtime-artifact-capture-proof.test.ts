import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures,
  TrackingFullProductUiLocalRuntimeArtifactCaptureRowSchema,
  buildTrackingFullProductUiLocalRuntimeArtifactCaptureProof,
} from '../src/tracking-full-product-ui-local-runtime-artifact-capture-proof';

const generatedAt = '2026-06-08T04:35:00.000Z';

describe('tracking full product UI local runtime artifact capture proof', () => {
  it('captures only locally provable parent UI artifacts without product claims', () => {
    const proof = buildTrackingFullProductUiLocalRuntimeArtifactCaptureProof(
      generatedAt,
      ['test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json'],
      [
        capture('parent-overview-runtime-ui', '01-parent-overview-runtime.png', 2048, 1200, 900),
        capture('parent-device-detail-runtime-ui', '02-parent-device-detail-runtime.png', 4096, 1200, 900),
        capture(
          'parent-notification-history-preferences-runtime',
          '03-parent-notification-history-preferences-runtime.png',
          3072,
          1200,
          900
        ),
        capture('cross-surface-accessibility-report', '08-cross-surface-accessibility-report.json', 1024),
        capture('product-ui-end-to-end-trace', '09-product-ui-end-to-end-trace.json', 1536),
      ],
      closureEvidenceInput()
    );

    expect(proof.rows).toHaveLength(RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures.length);
    expect(proof.localArtifactCount).toBe(5);
    expect(proof.rows.map((row) => row.status)).toEqual([
      'local-artifact-captured',
      'local-artifact-captured',
      'local-artifact-captured',
      'local-artifact-captured',
      'local-artifact-captured',
    ]);
    expect(proof.productClaims.parentOverviewLocalArtifactCaptured).toBe(true);
    expect(proof.productClaims.parentDeviceDetailLocalArtifactCaptured).toBe(true);
    expect(proof.productClaims.parentNotificationHistoryPreferencesLocalArtifactCaptured).toBe(true);
    expect(proof.productClaims.crossSurfaceAccessibilityLocalArtifactCaptured).toBe(true);
    expect(proof.productClaims.productUiEndToEndTraceCaptured).toBe(true);
    expect(proof.productClaims.fullProductUiRuntimeClaimed).toBe(false);
    expect(proof.productClaims.childDeviceRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.closureEvidence.retentionWritableExecutionRowCount).toBe(1);
    expect(proof.closureEvidence.retentionWritableExecutionDerivationCount).toBe(1);
    expect(proof.closureEvidence.retentionWritableExecutionArtifactRefs).toEqual([
      'tracking-retention/product-settings-writable-execution.json',
    ]);
    expect(proof.closureEvidence.retentionLocalProductSettingsWritableExecutionObserved).toBe(true);
    expect(proof.closureEvidence.childRuntimeRequiredArtifactCount).toBe(2);
    expect(proof.closureEvidence.childRuntimeMissingArtifactCount).toBe(2);
    expect(proof.closureEvidence.childRuntimeArtifactSetComplete).toBe(false);
    expect(proof.closureEvidence.productClaimReady).toBe(false);
  });

  it('rejects copied screenshot rows when byte sizes drift', () => {
    const invalid = TrackingFullProductUiLocalRuntimeArtifactCaptureRowSchema.safeParse({
      schemaVersion: 'v0.6-parent',
      artifactId: 'parent-overview-runtime-ui',
      status: 'local-artifact-captured',
      generatedAt,
      sourceArtifactRef:
        'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-parent-overview-shell.png',
      outputArtifactRef: 'output/tracking-plan-proof/product-parent-child-ui-runtime/01-parent-overview-runtime.png',
      sourceBytes: 2048,
      outputBytes: 2047,
      width: 1200,
      height: 900,
      currentProofTier: 'P2_HOSTED_CI',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      localParentUiArtifactCaptured: true,
      fullProductUiRuntimeClaimed: false,
      childDeviceRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});

function capture(
  artifactId:
    | 'parent-overview-runtime-ui'
    | 'parent-device-detail-runtime-ui'
    | 'parent-notification-history-preferences-runtime'
    | 'cross-surface-accessibility-report'
    | 'product-ui-end-to-end-trace',
  fileName: string,
  bytes: number,
  width?: number,
  height?: number
) {
  return {
    artifactId,
    sourceArtifactRef:
      artifactId === 'cross-surface-accessibility-report'
        ? 'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json'
        : artifactId === 'product-ui-end-to-end-trace'
          ? 'test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json'
          : `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-${fileName}`,
    outputArtifactRef: `output/tracking-plan-proof/product-parent-child-ui-runtime/${fileName}`,
    sourceBytes: bytes,
    outputBytes: bytes,
    width,
    height,
  };
}

function closureEvidenceInput() {
  return {
    retentionWritableExecutionProofRef:
      'output/tracking-plan-proof/tracking-retention-product-settings-writable-execution-proof/proof.json',
    retentionWritableExecutionProof: retentionWritableExecutionProof(),
    childRuntimeArtifactGateProofRef:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/50-child-runtime-artifact-gate-proof.json',
    childRuntimeArtifactGateProof: childRuntimeArtifactGateProof(),
  };
}

function retentionWritableExecutionProof() {
  return {
    schemaVersion: 'v0.5-tracking',
    proofMode: 'tracking-retention-product-settings-writable-execution-proof',
    generatedAt,
    rows: [retentionWritableExecutionRow()],
    proofClaims: {
      writeCommandAccepted: true,
      serviceMutationExecuted: true,
      localServiceStateReadbackClaimed: true,
      durableSettingsPersisted: true,
      localProductSettingsWritableExecutionObserved: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      portalWritableUiClaimed: false,
      platformRuntimeRetentionEnforcementClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
    derivationMatrix: [retentionWritableExecutionDerivation()],
  };
}

function retentionWritableExecutionRow() {
  return {
    ...retentionWritableExecutionSourceRefs(),
    schemaVersion: 'v0.5-tracking',
    rowId: 'retention-row',
    generatedAt,
    settingsKind: 'retention-window-setting',
    outputArtifactRef: 'tracking-retention/product-settings-writable-execution.json',
    auditRefs: ['retention-row-audit'],
    localServiceStateRevision: 1,
    localServiceStateSnapshotRef: 'agent-service-local-retention-settings-state',
    durableSettingsStoreRef: 'agent-service-local-retention-settings-durable-json',
    appliedRetentionWindowHours: 168,
    appliedDeleteAfterAlertResolved: false,
    parentExportPrepared: false,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    writeCommandAccepted: true,
    serviceMutationExecuted: true,
    localServiceStateReadbackClaimed: true,
    durableSettingsPersisted: true,
    localProductSettingsWritableExecutionObserved: true,
    portalWritableUiClaimed: false,
    platformRuntimeRetentionEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  };
}

function retentionWritableExecutionDerivation() {
  return {
    ...retentionWritableExecutionSourceRefs(),
    rowId: 'retention-row',
    localServiceStateRevision: 1,
    localServiceStateSnapshotRef: 'agent-service-local-retention-settings-state',
    durableSettingsStoreRef: 'agent-service-local-retention-settings-durable-json',
    appliedRetentionWindowHours: 168,
    appliedDeleteAfterAlertResolved: false,
    outputArtifactRef: 'tracking-retention/product-settings-writable-execution.json',
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    portalWritableUiClaimed: false,
    platformRuntimeRetentionEnforcementClaimed: false,
    productClaimReady: false,
  };
}

function retentionWritableExecutionSourceRefs() {
  return {
    sourceLocalServiceStateProofRef: 'output/tracking-plan-proof/07-retention-and-custody-model/22.json',
    sourceWriteCommandProofRef: 'output/tracking-plan-proof/07-retention-and-custody-model/21.json',
    sourceReadModelProofRefs: ['output/tracking-plan-proof/07-retention-and-custody-model/18.json'],
    sourceMutationProofRefs: ['output/tracking-plan-proof/07-retention-and-custody-model/20.json'],
  };
}

function childRuntimeArtifactGateProof() {
  return {
    schemaVersion: 'v0.5-tracking',
    proofMode: 'tracking-child-runtime-artifact-gate-proof',
    generatedAt,
    rows: [
      {
        schemaVersion: 'v0.5-tracking',
        rowId: 'tracking-child-runtime-artifacts-device-execution',
        generatedAt,
        proofRoot: 'output/tracking-plan-proof/child-device-runtime-execution',
        requiredProofTier: 'P4_PHYSICAL_DEVICE',
        currentProofTier: 'P3_LOCAL_DEV_MACHINE',
        status: 'manual-required',
        requiredArtifacts: ['00-run-metadata.json', '01-child-device-metadata.json'],
        presentArtifacts: [],
        missingArtifacts: ['00-run-metadata.json', '01-child-device-metadata.json'],
        auditRefs: ['tracking-child-runtime-artifacts-device-execution-audit'],
        childRuntimeArtifactSetComplete: false,
        childDeviceDeliveryRuntimeClaimed: false,
        childDeviceExecutionRuntimeClaimed: false,
        renderedChildDeviceUiRuntimeClaimed: false,
        parentReceiptRuntimeClaimed: false,
        runtimeObservationClaimed: false,
        physicalDeviceProofClaimed: false,
        authorityProofClaimed: false,
        providerDeliveryClaimed: false,
        productionWorkerClaimed: false,
        productClaimReady: false,
      },
    ],
    proofClaims: {
      childRuntimeArtifactGateChecked: true,
      noChildDeviceDeliveryRuntimeClaim: true,
      noChildDeviceExecutionRuntimeClaim: true,
      noRenderedChildDeviceUiRuntimeClaim: true,
      noParentReceiptRuntimeClaim: true,
      noRuntimeObservationClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      parentReceiptRuntimeClaimed: false,
      runtimeObservationClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  };
}
