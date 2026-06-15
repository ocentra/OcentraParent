export function adapterSnapshot() {
  return {
    schemaVersion: 'browser-game-hold-block-adapter-contract',
    familyId: 'family-main',
    childProfileId: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    generatedAt: '2026-06-03T12:30:00.000Z',
    plans: browserGameHoldBlockAdapterPlans(),
    claimBoundaries: claimBoundaries(),
  };
}

function browserGameHoldBlockAdapterPlans() {
  return [...managedAdapterPlans(), ...candidateAdapterPlans(), ...fallbackAdapterPlans()];
}

function managedAdapterPlans() {
  return [
    adapterPlan(),
    adapterPlan({
      planId: 'browser-game-hold-block-plan-parent-approval',
      requestedAction: 'hold-until-parent-approval',
      fallbackAction: 'show-approval-page',
      reasonCodes: ['policy-candidate-parent-review', 'managed-intervention-proof-present'],
    }),
    adapterPlan({
      planId: 'browser-game-hold-block-plan-block',
      requestedAction: 'block-game-route',
      fallbackAction: 'show-block-page',
      reasonCodes: ['policy-candidate-block', 'managed-intervention-proof-present'],
    }),
    adapterPlan({
      planId: 'browser-game-hold-block-plan-warn',
      requestedAction: 'warn-before-play',
      fallbackAction: 'show-warning-page',
      reasonCodes: ['policy-candidate-warn', 'managed-intervention-proof-present'],
    }),
  ];
}

function candidateAdapterPlans() {
  return [
    adapterPlan({
      planId: 'browser-game-hold-block-plan-allow-educational',
      requestedAction: 'allow-educational-game',
      adapterState: 'candidate-only',
      deliveryMode: 'contract-only',
      fallbackAction: 'continue-session',
      managedInterventionAdapterProofRef: null,
      adapterAuditRef: null,
      reasonCodes: ['educational-allow-candidate'],
    }),
    adapterPlan({
      planId: 'browser-game-hold-block-plan-time-limit',
      requestedAction: 'time-limit-candidate',
      adapterState: 'candidate-only',
      deliveryMode: 'contract-only',
      fallbackAction: 'manual-review',
      managedInterventionAdapterProofRef: null,
      adapterAuditRef: null,
      reasonCodes: ['policy-candidate-time-limit'],
    }),
  ];
}

function fallbackAdapterPlans() {
  return [
    adapterPlan({
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
    }),
    adapterPlan({
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
    }),
  ];
}

function claimBoundaries() {
  return {
    rawUrlStorage: 'not-claimed',
    rawPageBodyStorage: 'not-claimed',
    rawGamePayloadStorage: 'not-claimed',
    childCookieSessionReuse: 'not-claimed',
    unmanagedExactUrl: 'not-claimed',
    browserMutationExecution: 'not-claimed',
    renderedChildPage: 'not-claimed',
    notificationDelivery: 'not-claimed',
    finalPolicyDecision: 'not-claimed',
    timeLimitApplication: 'not-claimed',
    cloudFrameAnalysis: 'not-claimed',
    nativeGameControl: 'not-claimed',
    enforcement: 'not-claimed',
  };
}

export function adapterPlan(overrides = {}) {
  return {
    schemaVersion: 'browser-game-hold-block-adapter-contract',
    planId: 'browser-game-hold-block-plan-checking',
    familyId: 'family-main',
    childProfileId: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    createdAt: '2026-06-03T12:25:00.000Z',
    targetKind: 'managed-browser-game-page',
    requestedAction: 'hold-until-classified',
    adapterState: 'adapter-proof-present',
    deliveryMode: 'managed-intervention-proof-ref',
    fallbackAction: 'show-checking-page',
    sourceEvidenceRefs: ['parent-evidence-browser-game-route', 'parent-evidence-browser-game-policy-candidate'],
    policyCandidateRef: 'parent-evidence-browser-game-policy-candidate',
    childUxSurfaceRef: 'parent-evidence-browser-game-child-surface',
    managedInterventionAdapterProofRef: 'parent-evidence-managed-browser-intervention-proof',
    adapterAuditRef: 'parent-evidence-managed-browser-game-adapter-audit',
    reasonCodes: ['unknown-game-needs-classification', 'managed-intervention-proof-present'],
    rawUrlIncluded: false,
    rawPageBodyIncluded: false,
    rawGamePayloadIncluded: false,
    childCookieSessionReused: false,
    unmanagedBrowserExactUrlClaimed: false,
    browserMutationExecutedClaimed: false,
    renderedChildPageClaimed: false,
    notificationDeliveredClaimed: false,
    finalPolicyDecisionClaimed: false,
    timeLimitAppliedClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
