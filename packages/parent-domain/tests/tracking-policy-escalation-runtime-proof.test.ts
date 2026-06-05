import { describe, expect, it } from 'vitest';
import {
  TrackingPolicyEscalationRuntimeOutcomeSchema,
  TrackingPolicyEscalationRuntimeProofReadModel,
  TrackingPolicyEscalationRuntimeProofSchema,
  summarizeTrackingPolicyEscalationRuntimeProof,
} from '../src/tracking-policy-escalation-runtime-proof';

const Proof = TrackingPolicyEscalationRuntimeProofReadModel;
const ExpectedSummary = {
  outcomes: 5,
  parentPolicyAuthorityRows: 5,
  aiAuthorityRows: 0,
  providerDeliveryRows: 0,
  emergencyContactRows: 0,
  deviceRuntimeRows: 0,
  escalationRows: 2,
  resolvedRows: 3,
} as const;
const RequiredNonClaims = [
  'no-provider-delivery-attempted',
  'no-emergency-contact-automation',
  'no-child-device-runtime',
  'no-background-location-claim',
  'no-physical-device-proof',
  'no-ai-final-authority',
] as const;

describe('tracking policy escalation runtime proof', () => {
  it('summarizes parent-policy runtime outcomes without runtime overclaims', () => {
    expect(summarizeTrackingPolicyEscalationRuntimeProof(Proof)).toEqual(ExpectedSummary);
    expect(Proof.nonClaims).toEqual(RequiredNonClaims);
    expect(Proof.productClaimReady).toBe(false);
  });

  it('derives policy, acknowledgement, check-in, and escalation rows from the runtime helpers', () => {
    assertRuntimeOutcomeRows();
  });

  it('rejects provider delivery, emergency/device runtime claims, suppressed critical rows, and missing proof coverage', () => {
    assertRuntimeNegativeCases();
  });
});

function assertRuntimeOutcomeRows() {
  const aiBoundary = outcomeByKind('ai-analysis-cannot-trigger-alert-directly');
  const warningAcknowledgement = outcomeByKind('parent-acknowledgement-suppresses-warning');
  const criticalAcknowledgement = outcomeByKind('critical-alert-remains-visible');
  const safeCheckIn = outcomeByKind('safe-child-check-in-resolves');
  const expiredCheckIn = outcomeByKind('expired-child-check-in-escalates-by-policy');

  expect(aiBoundary.aiFinalAuthorityClaimed).toBe(false);
  expect(aiBoundary.aiDirectAlertClaimed).toBe(false);
  expect(aiBoundary.parentPolicyFinalAuthority).toBe(true);
  expect(aiBoundary.runtimeState).toBe('ai-advisory-only');
  expect(warningAcknowledgement.parentAlertSuppressed).toBe(true);
  expect(warningAcknowledgement.runtimeState).toBe('suppressed-by-acknowledgement');
  expect(criticalAcknowledgement.parentAlertSuppressed).toBe(false);
  expect(criticalAcknowledgement.runtimeState).toBe('critical-still-alert');
  expect(safeCheckIn.checkInId).toBe('tracking-runtime-checkin-safe');
  expect(safeCheckIn.escalates).toBe(false);
  expect(safeCheckIn.resolved).toBe(true);
  expect(expiredCheckIn.checkInId).toBe('tracking-runtime-checkin-expired');
  expect(expiredCheckIn.escalationId).toBe('tracking-runtime-escalation-expired-checkin');
  expect(expiredCheckIn.escalates).toBe(true);
  expect(expiredCheckIn.providerDeliveryAttempted).toBe(false);
  expect(expiredCheckIn.emergencyContactClaimed).toBe(false);
  expect(expiredCheckIn.deviceRuntimeClaimed).toBe(false);
}

function assertRuntimeNegativeCases() {
  const providerDeliveryClaim = TrackingPolicyEscalationRuntimeOutcomeSchema.safeParse({
    ...Proof.outcomes[0],
    providerDeliveryAttempted: true,
  });
  const emergencyContactClaim = TrackingPolicyEscalationRuntimeOutcomeSchema.safeParse({
    ...Proof.outcomes[0],
    emergencyContactClaimed: true,
  });
  const deviceRuntimeClaim = TrackingPolicyEscalationRuntimeOutcomeSchema.safeParse({
    ...Proof.outcomes[0],
    deviceRuntimeClaimed: true,
  });
  const suppressedCritical = TrackingPolicyEscalationRuntimeOutcomeSchema.safeParse({
    ...outcomeByKind('critical-alert-remains-visible'),
    parentAlertSuppressed: true,
  });
  const missingOutcome = TrackingPolicyEscalationRuntimeProofSchema.safeParse({
    ...Proof,
    outcomes: Proof.outcomes.filter((outcome) => outcome.outcomeKind !== 'expired-child-check-in-escalates-by-policy'),
  });
  const missingNonClaim = TrackingPolicyEscalationRuntimeProofSchema.safeParse({
    ...Proof,
    nonClaims: Proof.nonClaims.filter((nonClaim) => nonClaim !== 'no-ai-final-authority'),
  });

  expect(providerDeliveryClaim.success).toBe(false);
  expect(emergencyContactClaim.success).toBe(false);
  expect(deviceRuntimeClaim.success).toBe(false);
  expect(suppressedCritical.success).toBe(false);
  expect(missingOutcome.success).toBe(false);
  expect(missingNonClaim.success).toBe(false);
}

function outcomeByKind(kind: (typeof Proof.outcomes)[number]['outcomeKind']) {
  const outcome = Proof.outcomes.find((candidate) => candidate.outcomeKind === kind);

  if (outcome === undefined) {
    throw new Error(`Missing tracking policy escalation runtime proof outcome ${kind}`);
  }

  return outcome;
}
