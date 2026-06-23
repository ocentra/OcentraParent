import { describe, expect, it } from 'vitest';
import {
  TrackingChildRuntimeSnapshotRequirementsReadModelSchema,
  TrackingChildRuntimeSnapshotRequirementsRowSchema,
  buildTrackingChildRuntimeSnapshotRequirementsReadModel,
} from '@ocentra-parent/schema-domain/tracking-child-runtime-snapshot-requirements-proof';
import { buildTrackingChildRuntimeExecutionReadinessReadModel } from '@ocentra-parent/schema-domain/tracking-child-runtime-execution-readiness-proof';
import { TrackingPolicySchemaVersion } from '@ocentra-parent/schema-domain/tracking-location-policy';

describe('tracking child runtime snapshot requirements proof', () => {
  it('derives snapshot requirement rows from execution readiness rows', () => {
    const readModel = snapshotRequirementsReadModel();

    expect(readModel.rows).toHaveLength(4);
    expect(readModel.requiredSnapshotKindCount).toBe(20);
    expect(readModel.deliveryEnvelopeRequirementCount).toBe(4);
    expect(readModel.executionResultRequirementCount).toBe(4);
    expect(readModel.visibleSnapshotRequirementCount).toBe(4);
    expect(readModel.parentReceiptRequirementCount).toBe(4);
    expect(readModel.runtimeObservationRequirementCount).toBe(8);
    expect(readModel.childDeviceDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.childDeviceExecutionRuntimeClaimed).toBe(false);
    expect(readModel.renderedChildDeviceUiRuntimeClaimed).toBe(false);
    expect(readModel.physicalDeviceProofClaimed).toBe(false);
    expect(readModel.authorityProofClaimed).toBe(false);
    expect(readModel.productReadyClaimed).toBe(false);
  });

  it('keeps visible snapshot and runtime observation refs tied to hosted readiness rows', () => {
    const readModel = snapshotRequirementsReadModel();
    const safe = readModel.rows.find((row) => row.sourceCheckInId === 'tracking-check-in-safe');

    expect(safe?.sourceReadinessState).toBe('safe-response-execution-ready');
    expect(safe?.sourceSnapshotKind).toBe('safe-response-snapshot');
    expect(safe?.deliveryEnvelopeRef).toBe('tracking-child-runtime-delivery-envelope-tracking-check-in-safe');
    expect(safe?.requiredSnapshotKinds).toEqual([
      'delivery-envelope',
      'execution-result',
      'visible-snapshot',
      'parent-receipt',
      'runtime-observation',
    ]);
    expect(safe?.executionResultRequirementRefs).toContain(
      'child-runtime-execution-result-proof-required-tracking-check-in-safe'
    );
    expect(safe?.visibleSnapshotRequirementRefs).toContain(
      'child-runtime-visible-snapshot-proof-required-tracking-check-in-safe'
    );
    expect(safe?.parentReceiptRequirementRefs).toContain(
      'child-runtime-parent-receipt-proof-required-tracking-check-in-safe'
    );
    expect(safe?.runtimeObservationRequirementRefs).toContain(
      'child-runtime-device-observation-required-tracking-check-in-safe'
    );
  });

  it('rejects snapshot rows and read models that claim runtime execution or product readiness', () => {
    const readModel = snapshotRequirementsReadModel();
    const unsafeRow = TrackingChildRuntimeSnapshotRequirementsRowSchema.safeParse({
      ...readModel.rows[0],
      childDeviceExecutionRuntimeClaimed: true,
    });
    const unsafeReadModel = TrackingChildRuntimeSnapshotRequirementsReadModelSchema.safeParse({
      ...readModel,
      renderedChildDeviceUiRuntimeClaimed: true,
    });

    expect(unsafeRow.success).toBe(false);
    expect(unsafeReadModel.success).toBe(false);
  });
});

function snapshotRequirementsReadModel() {
  return buildTrackingChildRuntimeSnapshotRequirementsReadModel(
    {
      generatedAt: '2026-06-07T15:05:00.000Z',
      snapshotRequirementsId: 'tracking-child-runtime-snapshot-requirements-proof',
      sourceContractRefs: [
        'tracking-child-runtime-execution-readiness-proof',
        'tracking-child-runtime-snapshot-requirements-proof',
      ],
    },
    executionReadinessReadModel()
  );
}

function executionReadinessReadModel() {
  return buildTrackingChildRuntimeExecutionReadinessReadModel(
    {
      generatedAt: '2026-06-07T14:45:00.000Z',
      readinessId: 'tracking-child-runtime-execution-readiness-proof',
      sourceContractRefs: [
        'tracking-child-runtime-delivery-boundary-proof',
        'tracking-child-runtime-execution-readiness-proof',
      ],
    },
    boundaryReadModel()
  );
}

function boundaryReadModel() {
  const rows = [
    boundaryRow('tracking-check-in-waiting', 'hosted-copy-only-waiting', 'delivery-disclosure'),
    boundaryRow('tracking-check-in-safe', 'hosted-copy-only-safe-response', 'safe-response-disclosure'),
    boundaryRow('tracking-check-in-help', 'hosted-copy-only-escalation-ready', 'help-response-disclosure'),
    boundaryRow('tracking-check-in-expired', 'hosted-copy-only-escalation-ready', 'timeout-disclosure'),
  ] as const;

  return {
    schemaVersion: TrackingPolicySchemaVersion,
    readinessId: 'tracking-child-runtime-delivery-boundary-proof',
    generatedAt: '2026-06-07T14:10:00.000Z',
    sourceTimeoutReadinessId: 'tracking-child-check-in-timeout-escalation-proof',
    sourceTimeoutGeneratedAt: '2026-06-07T14:05:00.000Z',
    sourceContractRefs: ['tracking-child-check-in-timeout-escalation-proof'],
    hostedUiProofRefs: ['hosted-child-runtime-ui-proof'],
    rows,
    hostedCopyOnlyCount: 4,
    safeResponseDisclosureCount: 1,
    escalationDisclosureCount: 2,
    manualRuntimeProofRequiredCount: 0,
    requiredRuntimeProofRefCount: 20,
    readinessNonClaims: [
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
  } as const;
}

function boundaryRow(sourceCheckInId: string, boundaryState: string, hostedUiState: string) {
  return {
    rowId: `tracking-child-runtime-delivery-boundary-${sourceCheckInId}`,
    sourceCheckInId,
    sourceResolutionState: 'fixture-resolution-state',
    boundaryState,
    hostedUiState,
    hostedUiProofRefs: ['hosted-child-runtime-ui-proof'],
    sourceEvidenceRefs: [`${sourceCheckInId.replace('tracking-check-in-', 'tracking-child-')}-evidence`],
    sourceAuditRefs: [`${sourceCheckInId.replace('tracking-check-in-', 'tracking-child-')}-audit`],
    requiredRuntimeProofRefs: [
      `child-device-delivery-runtime-proof-required-${sourceCheckInId}`,
      `child-device-execution-runtime-proof-required-${sourceCheckInId}`,
      `rendered-child-device-ui-runtime-proof-required-${sourceCheckInId}`,
      `physical-device-proof-required-${sourceCheckInId}`,
      `authority-proof-required-${sourceCheckInId}`,
    ],
    parentVisibleStatusRefs: [`hosted-child-runtime-disclosure-${sourceCheckInId}`],
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
  } as const;
}
