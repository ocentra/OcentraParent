import { describe, expect, it } from 'vitest';
import {
  BrowserGameHoldBlockAdapterPlanSchema,
  BrowserGameHoldBlockAdapterSnapshotSchema,
} from '../../src/browser-game-hold-block-adapter';
import { adapterPlan, adapterSnapshot } from './browser-game-hold-block-adapter-fixtures';

describe('browser game managed hold block adapter contracts', () => {
  it('accepts managed browser-game hold, parent approval, block, and warning adapter proof plans', acceptsManagedPlans);
  it('accepts candidate-only allow and time-limit rows without adapter execution claims', acceptsCandidateOnlyRows);
  it('accepts manual-required and unavailable fallback rows', acceptsManualFallbackRows);
  it('rejects raw payload, session reuse, UI, mutation, native, cloud-frame, and enforcement claims', rejectsClaims);
  it('rejects action-specific proof mismatches', rejectsActionMismatches);
});

function acceptsManagedPlans() {
  const snapshot = BrowserGameHoldBlockAdapterSnapshotSchema.parse(adapterSnapshot());
  const actions = snapshot.plans.map((plan) => plan.requestedAction);
  const blockPlan = snapshot.plans.find((plan) => plan.requestedAction === 'block-game-route');

  expect(actions).toEqual([
    'hold-until-classified',
    'hold-until-parent-approval',
    'block-game-route',
    'warn-before-play',
    'allow-educational-game',
    'time-limit-candidate',
    'manual-required',
    'unavailable',
  ]);
  expect(blockPlan?.deliveryMode).toBe('managed-intervention-proof-ref');
  expect(blockPlan?.browserMutationExecutedClaimed).toBe(false);
  expect(blockPlan?.enforcementClaimed).toBe(false);
}

function acceptsCandidateOnlyRows() {
  const allowPlan = adapterPlan({
    planId: 'browser-game-hold-block-plan-allow-educational',
    requestedAction: 'allow-educational-game',
    adapterState: 'candidate-only',
    deliveryMode: 'contract-only',
    fallbackAction: 'continue-session',
    managedInterventionAdapterProofRef: null,
    adapterAuditRef: null,
    reasonCodes: ['educational-allow-candidate'],
  });
  const timeLimitPlan = adapterPlan({
    planId: 'browser-game-hold-block-plan-time-limit',
    requestedAction: 'time-limit-candidate',
    adapterState: 'candidate-only',
    deliveryMode: 'contract-only',
    fallbackAction: 'manual-review',
    managedInterventionAdapterProofRef: null,
    adapterAuditRef: null,
    reasonCodes: ['policy-candidate-time-limit'],
  });

  expect(BrowserGameHoldBlockAdapterPlanSchema.safeParse(allowPlan).success).toBe(true);
  expect(BrowserGameHoldBlockAdapterPlanSchema.safeParse(timeLimitPlan).success).toBe(true);
}

function acceptsManualFallbackRows() {
  const manualRequired = adapterPlan({
    planId: 'browser-game-hold-block-plan-cloud-manual-required',
    targetKind: 'manual-required',
    requestedAction: 'manual-required',
    adapterState: 'manual-required',
    deliveryMode: 'manual-required',
    fallbackAction: 'manual-review',
    policyCandidateRef: null,
    childUxSurfaceRef: null,
    managedInterventionAdapterProofRef: null,
    adapterAuditRef: null,
    reasonCodes: ['cloud-gaming-proof-manual-required'],
  });
  const unavailable = adapterPlan({
    planId: 'browser-game-hold-block-plan-native-unavailable',
    targetKind: 'manual-required',
    requestedAction: 'unavailable',
    adapterState: 'unavailable',
    deliveryMode: 'unavailable',
    fallbackAction: 'no-action',
    policyCandidateRef: null,
    childUxSurfaceRef: null,
    managedInterventionAdapterProofRef: null,
    adapterAuditRef: null,
    reasonCodes: ['native-game-control-unavailable'],
  });

  expect(BrowserGameHoldBlockAdapterPlanSchema.safeParse(manualRequired).success).toBe(true);
  expect(BrowserGameHoldBlockAdapterPlanSchema.safeParse(unavailable).success).toBe(true);
}

function rejectsClaims() {
  const valid = adapterPlan();
  const invalidRows = [
    { ...valid, rawUrlIncluded: true },
    { ...valid, rawPageBodyIncluded: true },
    { ...valid, rawGamePayloadIncluded: true },
    { ...valid, childCookieSessionReused: true },
    { ...valid, unmanagedBrowserExactUrlClaimed: true },
    { ...valid, browserMutationExecutedClaimed: true },
    { ...valid, renderedChildPageClaimed: true },
    { ...valid, notificationDeliveredClaimed: true },
    { ...valid, finalPolicyDecisionClaimed: true },
    { ...valid, timeLimitAppliedClaimed: true },
    { ...valid, cloudFrameAnalysisClaimed: true },
    { ...valid, nativeGameControlClaimed: true },
    { ...valid, enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameHoldBlockAdapterPlanSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsActionMismatches() {
  const valid = adapterPlan();
  const invalidRows = [
    { ...valid, policyCandidateRef: null },
    { ...valid, childUxSurfaceRef: null },
    { ...valid, managedInterventionAdapterProofRef: null },
    { ...valid, adapterAuditRef: null },
    { ...valid, deliveryMode: 'contract-only' },
    { ...valid, fallbackAction: 'show-warning-page' },
    { ...valid, reasonCodes: ['managed-intervention-proof-present'] },
    {
      ...valid,
      requestedAction: 'manual-required',
      adapterState: 'adapter-proof-present',
      deliveryMode: 'managed-intervention-proof-ref',
    },
    {
      ...valid,
      requestedAction: 'allow-educational-game',
      adapterState: 'adapter-proof-present',
      deliveryMode: 'managed-intervention-proof-ref',
      reasonCodes: ['educational-allow-candidate'],
    },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameHoldBlockAdapterPlanSchema.safeParse(invalid).success).toBe(false);
  }
}
