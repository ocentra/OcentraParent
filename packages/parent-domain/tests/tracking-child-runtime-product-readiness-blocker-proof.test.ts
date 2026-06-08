import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingChildRuntimeProductReadinessBlockers,
  TrackingChildRuntimeProductReadinessBlockerRowSchema,
  buildTrackingChildRuntimeProductReadinessBlockerProof,
} from '../src/tracking-child-runtime-product-readiness-blocker-proof';
import { TrackingPolicySchemaVersion } from '../src/tracking-location-policy-primitives';

const GeneratedAt = '2026-06-07T16:05:00.000Z';
const SourceSnapshotRequirementsProofRef =
  'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/28-child-runtime-snapshot-requirements-proof.json';
const SourceAndroidEmulatorBridgeProofRef =
  'output/tracking-plan-proof/tracking-child-runtime-android-emulator-readiness-bridge-proof/proof.json';

describe('tracking child runtime product readiness blocker proof', () => {
  it('blocks product-ready child runtime claims while preserving requirement coverage refs', () => {
    const proof = buildTrackingChildRuntimeProductReadinessBlockerProof(
      GeneratedAt,
      SourceSnapshotRequirementsProofRef,
      snapshotRequirementsProof(),
      SourceAndroidEmulatorBridgeProofRef,
      androidEmulatorBridgeProof()
    );

    expect(proof.proofClaims).toEqual({
      snapshotRequirementRowsObserved: true,
      androidEmulatorBridgeObserved: true,
      deliveryEnvelopeRequirementsObserved: true,
      executionResultRequirementsObserved: true,
      visibleSnapshotRequirementsObserved: true,
      parentReceiptRequirementsObserved: true,
      runtimeObservationRequirementsObserved: true,
      productReadinessBlocked: true,
      noProductReadyClaim: true,
    });
    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].blockerRefs).toEqual([...RequiredTrackingChildRuntimeProductReadinessBlockers]);
    expect(proof.rows[0].executionResultRequirementRefCount).toBe(1);
    expect(proof.rows[0].runtimeObservationRequirementRefCount).toBe(2);
    expect(proof.rows[0].androidEmulatorPrerequisitesObserved).toBe(true);
    expect(proof.rows[0].androidLocalGeofenceTransitionCount).toBe(3);
    expect(proof.rows[0].androidEmulatorChildRuntimeMissingArtifactCount).toBe(10);
    expect(proof.productClaims.childRuntimeRequirementCoverageClaimed).toBe(true);
    expect(proof.productClaims.androidEmulatorPrerequisitesObserved).toBe(true);
    expect(proof.productClaims.childDeviceDeliveryRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productReadyClaimed).toBe(false);
  });

  it('rejects child-device runtime delivery overclaims', () => {
    const [row] = buildTrackingChildRuntimeProductReadinessBlockerProof(
      GeneratedAt,
      SourceSnapshotRequirementsProofRef,
      snapshotRequirementsProof(),
      SourceAndroidEmulatorBridgeProofRef,
      androidEmulatorBridgeProof()
    ).rows;

    expect(
      TrackingChildRuntimeProductReadinessBlockerRowSchema.safeParse({
        ...row,
        childDeviceDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });

  it('rejects missing product-readiness blocker refs', () => {
    const [row] = buildTrackingChildRuntimeProductReadinessBlockerProof(
      GeneratedAt,
      SourceSnapshotRequirementsProofRef,
      snapshotRequirementsProof(),
      SourceAndroidEmulatorBridgeProofRef,
      androidEmulatorBridgeProof()
    ).rows;

    expect(
      TrackingChildRuntimeProductReadinessBlockerRowSchema.safeParse({
        ...row,
        blockerRefs: RequiredTrackingChildRuntimeProductReadinessBlockers.slice(0, 2),
      }).success
    ).toBe(false);
  });
});

function snapshotRequirementsProof(): unknown {
  return {
    status: 'proved',
    readModel: {
      schemaVersion: TrackingPolicySchemaVersion,
      snapshotRequirementsId: 'tracking-child-runtime-snapshot-requirements-proof',
      generatedAt: GeneratedAt,
      sourceExecutionReadinessId: 'tracking-child-runtime-execution-readiness-proof',
      sourceExecutionReadinessGeneratedAt: GeneratedAt,
      sourceContractRefs: ['packages/parent-domain/src/tracking-child-runtime-snapshot-requirements-proof.ts'],
      rows: [snapshotRequirementsRow()],
      requiredSnapshotKindCount: 5,
      deliveryEnvelopeRequirementCount: 1,
      executionResultRequirementCount: 1,
      visibleSnapshotRequirementCount: 1,
      parentReceiptRequirementCount: 1,
      runtimeObservationRequirementCount: 2,
      snapshotRequirementsNonClaims: [
        'no-child-device-delivery-runtime',
        'no-child-device-execution-runtime',
        'no-rendered-child-device-ui-runtime',
        'no-provider-delivery',
        'no-notification-receipt-ingestion',
        'no-live-location-runtime',
        'no-physical-device-proof',
        'no-authority-proof',
        'no-production-worker',
        'no-product-ready-claim',
      ],
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptIngestionClaimed: false,
      liveLocationRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productReadyClaimed: false,
    },
  };
}

function snapshotRequirementsRow(): Record<string, unknown> {
  return {
    rowId: 'tracking-child-runtime-snapshot-requirements-check-in-timeout',
    sourceReadinessRowId: 'tracking-child-runtime-execution-readiness-check-in-timeout',
    sourceCheckInId: 'check-in-timeout',
    sourceReadinessState: 'escalation-execution-ready',
    sourceSnapshotKind: 'timeout-snapshot',
    requiredSnapshotKinds: [
      'delivery-envelope',
      'execution-result',
      'visible-snapshot',
      'parent-receipt',
      'runtime-observation',
    ],
    deliveryEnvelopeRef: 'tracking-child-runtime-delivery-envelope-check-in-timeout',
    executionResultRequirementRefs: ['child-runtime-execution-result-proof-required-check-in-timeout'],
    visibleSnapshotRequirementRefs: ['child-runtime-visible-snapshot-proof-required-check-in-timeout'],
    parentReceiptRequirementRefs: ['child-runtime-parent-receipt-proof-required-check-in-timeout'],
    runtimeObservationRequirementRefs: [
      'child-runtime-device-observation-required-check-in-timeout',
      'child-runtime-result-receipt-required-check-in-timeout',
    ],
    hostedUiProofRefs: ['output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/19-child-runtime-ui-proof.json'],
    sourceEvidenceRefs: ['tracking-child-check-in-timeout-evidence'],
    parentVisibleStatusRefs: ['hosted-child-runtime-disclosure-check-in-timeout'],
    boundaryRuntimeProofRefs: ['child-device-delivery-runtime-proof-required-check-in-timeout'],
    ...runtimeNonClaims(),
  };
}

function androidEmulatorBridgeProof(): unknown {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-child-runtime-android-emulator-readiness-bridge-proof',
    generatedAt: GeneratedAt,
    rows: [
      {
        schemaVersion: TrackingPolicySchemaVersion,
        rowId: 'tracking-child-runtime-android-emulator-readiness-bridge',
        generatedAt: GeneratedAt,
        requiredProofTier: 'P4_PHYSICAL_DEVICE',
        currentProofTier: 'P3_LOCAL_DEV_MACHINE',
        status: 'emulator-prerequisites-observed-manual-runtime-required',
        sourceProofRefs: [
          'test-results/tracking-plan-android-emulator-proof/proof.json',
          'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/50-child-runtime-artifact-gate-proof.json',
        ],
        androidEmulatorProofRef: 'test-results/tracking-plan-android-emulator-proof/proof.json',
        childRuntimeArtifactGateProofRef:
          'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/50-child-runtime-artifact-gate-proof.json',
        androidProofStatus: 'emulator_scaffold_observed_nonvisual_screenshot',
        androidEvidenceRefs: ['test-results/tracking-plan-android-emulator-proof/proof.json'],
        childRuntimeMissingArtifacts: childRuntimeMissingArtifacts(),
        missingProofReasonRefs: ['child-runtime-delivery-envelope-physical-run-required'],
        auditRefs: ['tracking-child-runtime-android-emulator-readiness-bridge-audit'],
        packageLaunchObserved: true,
        foregroundServiceObserved: true,
        foregroundPermissionGranted: true,
        backgroundPermissionGranted: true,
        localGeofenceTransitionCount: 3,
        emulatorPrerequisitesObserved: true,
        childRuntimeArtifactSetComplete: false,
        childDeviceDeliveryRuntimeClaimed: false,
        childDeviceExecutionRuntimeClaimed: false,
        renderedChildDeviceUiRuntimeClaimed: false,
        parentReceiptRuntimeClaimed: false,
        physicalDeviceProofClaimed: false,
        authorityProofClaimed: false,
        providerDeliveryClaimed: false,
        productionWorkerClaimed: false,
        productClaimReady: false,
      },
    ],
    proofClaims: {
      androidEmulatorPrerequisitesObserved: true,
      childRuntimeArtifactGateLinked: true,
      childRuntimePhysicalProofStillRequired: true,
      noChildDeviceDeliveryRuntimeClaim: true,
      noChildDeviceExecutionRuntimeClaim: true,
      noRenderedChildDeviceUiRuntimeClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      androidEmulatorPrerequisitesObserved: true,
      childRuntimeArtifactSetComplete: false,
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      parentReceiptRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  };
}

function childRuntimeMissingArtifacts(): string[] {
  return [
    '00-run-metadata.json',
    '01-child-device-metadata.json',
    '02-delivery-envelope.json',
    '03-execution-result.json',
    '04-visible-child-ui-snapshot.png',
    '05-parent-receipt.json',
    '06-runtime-observation.ndjson',
    '07-permission-consent-state.json',
    '08-device-log.txt',
    '09-result-summary.md',
  ];
}

function runtimeNonClaims(): Record<string, false> {
  return {
    childDeviceDeliveryRuntimeClaimed: false,
    childDeviceExecutionRuntimeClaimed: false,
    renderedChildDeviceUiRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptIngestionClaimed: false,
    liveLocationRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    productReadyClaimed: false,
  };
}
