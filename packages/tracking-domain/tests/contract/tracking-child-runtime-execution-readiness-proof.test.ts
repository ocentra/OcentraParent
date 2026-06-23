import { describe, expect, it } from 'vitest';
import {
  TrackingChildRuntimeExecutionReadinessReadModelSchema,
  TrackingChildRuntimeExecutionReadinessRowSchema,
  buildTrackingChildRuntimeExecutionReadinessReadModel,
} from '@ocentra-parent/schema-domain/tracking-child-runtime-execution-readiness-proof';
import { TrackingPolicySchemaVersion } from '@ocentra-parent/schema-domain/tracking-location-policy';

describe('tracking child runtime execution readiness proof', () => {
  it('derives execution-readiness rows from child runtime delivery boundary rows', () => {
    const readModel = proofReadModel();

    expect(readModel.rows).toHaveLength(4);
    expect(readModel.deliveryEnvelopeReadyCount).toBe(4);
    expect(readModel.safeResponseExecutionReadyCount).toBe(1);
    expect(readModel.escalationExecutionReadyCount).toBe(2);
    expect(readModel.manualRuntimeProofRequiredCount).toBe(0);
    expect(readModel.executionRequirementRefCount).toBe(16);
    expect(readModel.runtimeObservationRequirementRefCount).toBe(8);
    expect(readModel.childDeviceDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.childDeviceExecutionRuntimeClaimed).toBe(false);
    expect(readModel.renderedChildDeviceUiRuntimeClaimed).toBe(false);
    expect(readModel.physicalDeviceProofClaimed).toBe(false);
    expect(readModel.authorityProofClaimed).toBe(false);
    expect(readModel.productReadyClaimed).toBe(false);
  });

  it('preserves boundary refs and assigns snapshot requirements for safe and escalation rows', () => {
    const readModel = proofReadModel();
    const safe = readModel.rows.find((row) => row.sourceCheckInId === 'tracking-check-in-safe');
    const help = readModel.rows.find((row) => row.sourceCheckInId === 'tracking-check-in-help');

    expect(safe?.readinessState).toBe('safe-response-execution-ready');
    expect(safe?.snapshotKind).toBe('safe-response-snapshot');
    expect(safe?.deliveryEnvelopeRef).toBe('tracking-child-runtime-delivery-envelope-tracking-check-in-safe');
    expect(safe?.executionRequirementRefs).toContain(
      'child-runtime-execution-result-proof-required-tracking-check-in-safe'
    );
    expect(safe?.runtimeObservationRequirementRefs).toContain(
      'child-runtime-result-receipt-required-tracking-check-in-safe'
    );
    expect(safe?.boundaryRuntimeProofRefs).toContain(
      'child-device-execution-runtime-proof-required-tracking-check-in-safe'
    );

    expect(help?.readinessState).toBe('escalation-execution-ready');
    expect(help?.snapshotKind).toBe('help-response-snapshot');
    expect(help?.sourceEvidenceRefs).toContain('tracking-child-help-evidence');
    expect(help?.sourceAuditRefs).toContain('tracking-child-help-audit');
  });

  it('rejects rows and read models that claim runtime, provider, authority, or product proof', () => {
    const readModel = proofReadModel();
    const unsafeRow = TrackingChildRuntimeExecutionReadinessRowSchema.safeParse({
      ...readModel.rows[0],
      childDeviceDeliveryRuntimeClaimed: true,
    });
    const unsafeReadModel = TrackingChildRuntimeExecutionReadinessReadModelSchema.safeParse({
      ...readModel,
      productReadyClaimed: true,
    });

    expect(unsafeRow.success).toBe(false);
    expect(unsafeReadModel.success).toBe(false);
  });
});

function proofReadModel() {
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
