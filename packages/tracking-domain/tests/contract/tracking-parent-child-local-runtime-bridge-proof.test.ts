import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingParentChildLocalRuntimeBridgePhaseRefs,
  RequiredTrackingParentChildLocalRuntimeBridgeSourceRefs,
  TrackingParentChildLocalRuntimeBridgeRowSchema,
  buildTrackingParentChildLocalRuntimeBridgeProof,
} from '@ocentra-parent/schema-domain/tracking-parent-child-local-runtime-bridge-proof';

const GeneratedAt = '2026-06-08T18:10:00.000Z';

describe('tracking parent-child local runtime bridge proof', () => {
  it('bridges local parent-child eventing runtime evidence without physical child-device claims', () => {
    const proof = buildTrackingParentChildLocalRuntimeBridgeProof(GeneratedAt, bridgeInput());

    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].status).toBe('local-parent-child-runtime-observed-physical-child-runtime-required');
    expect(proof.rows[0].requiredProofTier).toBe('P4_PHYSICAL_DEVICE');
    expect(proof.rows[0].currentProofTier).toBe('P3_LOCAL_DEV_MACHINE');
    expect(proof.rows[0].sourceProofRefs).toEqual([...RequiredTrackingParentChildLocalRuntimeBridgeSourceRefs]);
    expect(proof.rows[0].publishReportCount).toBe(9);
    expect(proof.rows[0].storedEventCount).toBe(9);
    expect(proof.rows[0].deadLetterCount).toBe(0);
    expect(proof.rows[0].childAgentPhaseCount).toBe(4);
    expect(proof.rows[0].parentReadModelProjectionObserved).toBe(true);
    expect(proof.rows[0].typedLocalServiceTransportObserved).toBe(true);
    expect(proof.rows[0].localParentChildRuntimeObserved).toBe(true);
    expect(proof.rows[0].phaseRefs).toEqual([...RequiredTrackingParentChildLocalRuntimeBridgePhaseRefs]);
    expect(proof.productClaims.localParentChildRuntimeObserved).toBe(true);
    expect(proof.productClaims.childDeviceDeliveryRuntimeClaimed).toBe(false);
    expect(proof.productClaims.childDeviceExecutionRuntimeClaimed).toBe(false);
    expect(proof.productClaims.renderedChildDeviceUiRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('rejects child-device delivery overclaims from local runtime evidence', () => {
    const [row] = buildTrackingParentChildLocalRuntimeBridgeProof(GeneratedAt, bridgeInput()).rows;

    expect(
      TrackingParentChildLocalRuntimeBridgeRowSchema.safeParse({
        ...row,
        childDeviceDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });

  it('rejects incomplete runtime phase coverage', () => {
    const [row] = buildTrackingParentChildLocalRuntimeBridgeProof(GeneratedAt, bridgeInput()).rows;

    expect(
      TrackingParentChildLocalRuntimeBridgeRowSchema.safeParse({
        ...row,
        phaseRefs: RequiredTrackingParentChildLocalRuntimeBridgePhaseRefs.slice(0, 4),
      }).success
    ).toBe(false);
  });

  it('rejects dead-lettered local runtime chains', () => {
    const [row] = buildTrackingParentChildLocalRuntimeBridgeProof(GeneratedAt, bridgeInput()).rows;

    expect(
      TrackingParentChildLocalRuntimeBridgeRowSchema.safeParse({
        ...row,
        deadLetterCount: 1,
      }).success
    ).toBe(false);
  });
});

function bridgeInput() {
  return {
    eventingProofRef: RequiredTrackingParentChildLocalRuntimeBridgeSourceRefs[0],
    eventingRowProofRef: RequiredTrackingParentChildLocalRuntimeBridgeSourceRefs[1],
    runtimeSourceRefs: [
      'crates/agent-core/src/parent_child_event_runtime.rs',
      'crates/agent-core/src/parent_child_event_runtime/build.rs',
      'crates/agent-core/src/parent_child_event_runtime_phase.rs',
      'crates/agent-core/src/parent_child_event_runtime_tests.rs',
    ],
    phaseRefs: [...RequiredTrackingParentChildLocalRuntimeBridgePhaseRefs],
    publishReportCount: 9,
    storedEventCount: 9,
    deadLetterCount: 0,
    childAgentPhaseCount: 4,
    parentReadModelProjectionObserved: true,
    typedLocalServiceTransportObserved: true,
  };
}
