import { describe, expect, it } from 'vitest';
import {
  AppGameSourceFreshnessPolicyConsumptionMatrix,
  AppGameSourceFreshnessPolicyConsumptionRequests,
} from '../src/app-game-source-freshness-policy-consumption-data';
import {
  AppGameSourceFreshnessPolicyReadinessSchema,
  AppGameSourceFreshnessPolicyRequestSchema,
  AppGameSourceFreshnessStatusRowSchema,
  evaluateAppGameSourceFreshnessPolicyReadiness,
} from '../src/app-game-source-freshness-policy-consumption';
import {
  AppGameSourceFreshnessCapabilityStatus,
  AppGameSourceFreshnessPolicyReadinessState,
  AppGameSourceFreshnessReadModelState,
  AppGameSourceFreshnessReasonCode,
  AppGameSourceFreshnessRequirementKind,
  AppGameSourceFreshnessRequirementState,
  AppGameSourceFreshnessSourceKind,
} from '../src/app-game-source-freshness-policy-consumption-values';

const readyAppRequest = sourceFreshnessRequest('source-freshness-native-app-ready-request');
const manualAppRequest = sourceFreshnessRequest('source-freshness-native-app-manual-request');
const readyGameRequest = sourceFreshnessRequest('source-freshness-native-game-ready-request');
const manualGameRequest = sourceFreshnessRequest('source-freshness-native-game-manual-request');

describe('app/game source freshness policy consumption', () => {
  registerPolicyReadyTests();
  registerManualRequiredTests();
  registerValidationTests();
});

function registerPolicyReadyTests() {
  it('allows policy compile only when inventory, runtime, foreground, and launcher requirements are fresh', () => {
    const appReadiness = sourceFreshnessReadiness('source-freshness-native-app-ready-request');
    const gameReadiness = sourceFreshnessReadiness('source-freshness-native-game-ready-request');

    expect(appReadiness.readinessState).toBe(AppGameSourceFreshnessPolicyReadinessState.PolicyReady);
    expect(gameReadiness.readinessState).toBe(AppGameSourceFreshnessPolicyReadinessState.PolicyReady);
    expect(appReadiness.policyCompileAllowed).toBe(true);
    expect(gameReadiness.policyCompileAllowed).toBe(true);
    expect(appReadiness.policyEvidenceRefs).toHaveLength(3);
    expect(gameReadiness.policyEvidenceRefs).toHaveLength(4);
    expect(gameReadiness.requirementResults.map((result) => result.requirementKind)).toEqual([
      AppGameSourceFreshnessRequirementKind.Inventory,
      AppGameSourceFreshnessRequirementKind.Runtime,
      AppGameSourceFreshnessRequirementKind.Foreground,
      AppGameSourceFreshnessRequirementKind.Launcher,
    ]);
  });
}

function registerManualRequiredTests() {
  it('keeps stale, missing, and not-claimed source rows manual-required before policy compile', () => {
    const manualReadiness = evaluateAppGameSourceFreshnessPolicyReadiness(
      manualGameRequest,
      'manual-source-freshness-readiness',
      '2026-06-04T12:55:00.000Z'
    );

    expect(manualReadiness.readinessState).toBe(AppGameSourceFreshnessPolicyReadinessState.ManualRequired);
    expect(manualReadiness.policyCompileAllowed).toBe(false);
    expect(manualReadiness.directAdapterCallRequested).toBe(false);
    expect(manualReadiness.requirementResults.map((result) => result.requirementState)).toEqual([
      AppGameSourceFreshnessRequirementState.Stale,
      AppGameSourceFreshnessRequirementState.Missing,
      AppGameSourceFreshnessRequirementState.NotClaimed,
    ]);
    expect(manualReadiness.requirementResults.map((result) => result.reasonCode)).toEqual([
      AppGameSourceFreshnessReasonCode.StaleSourceStatusRow,
      AppGameSourceFreshnessReasonCode.MissingSourceStatusRow,
      AppGameSourceFreshnessReasonCode.NotClaimedSourceStatus,
    ]);
  });

  it('keeps stale and missing native app source rows manual-required before policy compile', () => {
    const manualReadiness = evaluateAppGameSourceFreshnessPolicyReadiness(
      manualAppRequest,
      'manual-app-source-freshness-readiness',
      '2026-06-04T12:55:00.000Z'
    );

    expect(manualReadiness.readinessState).toBe(AppGameSourceFreshnessPolicyReadinessState.ManualRequired);
    expect(manualReadiness.policyCompileAllowed).toBe(false);
    expect(manualReadiness.directAdapterCallRequested).toBe(false);
    expect(manualReadiness.requirementResults.map((result) => result.requirementState)).toEqual([
      AppGameSourceFreshnessRequirementState.Stale,
      AppGameSourceFreshnessRequirementState.Empty,
      AppGameSourceFreshnessRequirementState.Missing,
    ]);
    expect(manualReadiness.requirementResults.map((result) => result.reasonCode)).toEqual([
      AppGameSourceFreshnessReasonCode.StaleSourceStatusRow,
      AppGameSourceFreshnessReasonCode.EmptySourceStatusRow,
      AppGameSourceFreshnessReasonCode.MissingSourceStatusRow,
    ]);
  });
}

function registerValidationTests() {
  it('rejects read-model source rows that have counts without evidence refs', () => {
    const invalidRow = {
      sourceKind: AppGameSourceFreshnessSourceKind.ProcessSnapshot,
      state: AppGameSourceFreshnessReadModelState.Ready,
      rowCount: 1,
      lastObservedAt: '2026-06-04T12:54:00.000Z',
      capabilityStatus: AppGameSourceFreshnessCapabilityStatus.Available,
      evidence: [],
    };

    expect(AppGameSourceFreshnessStatusRowSchema.safeParse(invalidRow).success).toBe(false);
  });

  it('rejects policy-ready outputs that request adapter dispatch or include private raw source rows', () => {
    const ready = AppGameSourceFreshnessPolicyConsumptionMatrix.readiness[0];

    expect(
      AppGameSourceFreshnessPolicyReadinessSchema.safeParse({
        ...ready,
        directAdapterCallRequested: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceFreshnessPolicyReadinessSchema.safeParse({
        ...ready,
        rawPrivateSourceRowsIncluded: true,
      }).success
    ).toBe(false);
  });

  it('requires concrete native app/game policy targets to carry a target ref', () => {
    expect(
      AppGameSourceFreshnessPolicyRequestSchema.safeParse({
        ...readyAppRequest,
        target: {
          ...readyAppRequest.target,
          targetRef: null,
        },
      }).success
    ).toBe(false);
    expect(AppGameSourceFreshnessPolicyRequestSchema.safeParse(readyGameRequest).success).toBe(true);
  });
}

function sourceFreshnessRequest(policyRequestId: string) {
  const request = AppGameSourceFreshnessPolicyConsumptionRequests.find(
    (candidate) => candidate.policyRequestId === policyRequestId
  );
  if (request === undefined) {
    throw new Error(`Missing source freshness request fixture: ${policyRequestId}`);
  }
  return request;
}

function sourceFreshnessReadiness(policyRequestId: string) {
  const readiness = AppGameSourceFreshnessPolicyConsumptionMatrix.readiness.find(
    (candidate) => candidate.request.policyRequestId === policyRequestId
  );
  if (readiness === undefined) {
    throw new Error(`Missing source freshness readiness fixture: ${policyRequestId}`);
  }
  return readiness;
}
