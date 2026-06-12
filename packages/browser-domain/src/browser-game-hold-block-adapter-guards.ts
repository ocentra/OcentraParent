import type { Infer } from '@ocentra-parent/schema-domain/effect';
import type {
  BrowserGameHoldBlockActionSchema,
  BrowserGameHoldBlockAdapterStateSchema,
  BrowserGameHoldBlockDeliveryModeSchema,
  BrowserGameHoldBlockFallbackActionSchema,
  BrowserGameHoldBlockReasonSchema,
  BrowserGameHoldBlockAction,
} from './browser-game-hold-block-adapter-values';

export type BrowserGameHoldBlockAdapterPlanGuardInput = {
  requestedAction: Infer<typeof BrowserGameHoldBlockActionSchema>;
  adapterState: Infer<typeof BrowserGameHoldBlockAdapterStateSchema>;
  deliveryMode: Infer<typeof BrowserGameHoldBlockDeliveryModeSchema>;
  fallbackAction: Infer<typeof BrowserGameHoldBlockFallbackActionSchema>;
  policyCandidateRef: unknown | null;
  childUxSurfaceRef: unknown | null;
  managedInterventionAdapterProofRef: unknown | null;
  adapterAuditRef: unknown | null;
  reasonCodes: ReadonlyArray<Infer<typeof BrowserGameHoldBlockReasonSchema>>;
  rawUrlIncluded: boolean;
  rawPageBodyIncluded: boolean;
  rawGamePayloadIncluded: boolean;
  childCookieSessionReused: boolean;
  unmanagedBrowserExactUrlClaimed: boolean;
  browserMutationExecutedClaimed: boolean;
  renderedChildPageClaimed: boolean;
  notificationDeliveredClaimed: boolean;
  finalPolicyDecisionClaimed: boolean;
  timeLimitAppliedClaimed: boolean;
  cloudFrameAnalysisClaimed: boolean;
  nativeGameControlClaimed: boolean;
  enforcementClaimed: boolean;
};

export type BrowserGameHoldBlockAdapterSnapshotGuardInput = {
  plans: ReadonlyArray<BrowserGameHoldBlockAdapterPlanGuardInput>;
};

const RequiredBrowserGameHoldBlockActions = [
  'hold-until-classified',
  'hold-until-parent-approval',
  'block-game-route',
  'warn-before-play',
  'manual-required',
  'unavailable',
] as const satisfies ReadonlyArray<BrowserGameHoldBlockAction>;

export function browserGameHoldBlockAdapterSnapshotIsComplete(
  snapshot: BrowserGameHoldBlockAdapterSnapshotGuardInput
): boolean {
  const actions = new Set(snapshot.plans.map((plan) => plan.requestedAction));
  return RequiredBrowserGameHoldBlockActions.every((action) => actions.has(action));
}

export function browserGameHoldBlockAdapterPlanIsHonest(plan: BrowserGameHoldBlockAdapterPlanGuardInput): boolean {
  if (browserGameHoldBlockAdapterPlanClaimsRuntime(plan)) {
    return false;
  }
  if (managedHoldBlockActionRequiresProof(plan)) {
    return managedHoldBlockProofIsPresent(plan);
  }
  if (plan.requestedAction === 'manual-required') {
    return manualRequiredPlanIsPresent(plan);
  }
  if (plan.requestedAction === 'unavailable') {
    return unavailablePlanIsPresent(plan);
  }
  if (plan.requestedAction === 'allow-educational-game') {
    return candidateOnlyActionIsPresent(plan, 'educational-allow-candidate', 'continue-session');
  }
  if (plan.requestedAction === 'time-limit-candidate') {
    return candidateOnlyActionIsPresent(plan, 'policy-candidate-time-limit', 'manual-review');
  }
  return false;
}

function managedHoldBlockActionRequiresProof(plan: BrowserGameHoldBlockAdapterPlanGuardInput): boolean {
  return (
    plan.requestedAction === 'hold-until-classified' ||
    plan.requestedAction === 'hold-until-parent-approval' ||
    plan.requestedAction === 'block-game-route' ||
    plan.requestedAction === 'warn-before-play'
  );
}

function managedHoldBlockProofIsPresent(plan: BrowserGameHoldBlockAdapterPlanGuardInput): boolean {
  return (
    plan.adapterState === 'adapter-proof-present' &&
    plan.deliveryMode === 'managed-intervention-proof-ref' &&
    plan.policyCandidateRef !== null &&
    plan.childUxSurfaceRef !== null &&
    plan.managedInterventionAdapterProofRef !== null &&
    plan.adapterAuditRef !== null &&
    expectedReasonForManagedActionIsPresent(plan)
  );
}

function expectedReasonForManagedActionIsPresent(plan: BrowserGameHoldBlockAdapterPlanGuardInput): boolean {
  if (!plan.reasonCodes.includes('managed-intervention-proof-present')) {
    return false;
  }
  if (plan.requestedAction === 'hold-until-classified') {
    return (
      plan.reasonCodes.includes('unknown-game-needs-classification') && plan.fallbackAction === 'show-checking-page'
    );
  }
  if (plan.requestedAction === 'hold-until-parent-approval') {
    return plan.reasonCodes.includes('policy-candidate-parent-review') && plan.fallbackAction === 'show-approval-page';
  }
  if (plan.requestedAction === 'block-game-route') {
    return plan.reasonCodes.includes('policy-candidate-block') && plan.fallbackAction === 'show-block-page';
  }
  return plan.reasonCodes.includes('policy-candidate-warn') && plan.fallbackAction === 'show-warning-page';
}

function manualRequiredPlanIsPresent(plan: BrowserGameHoldBlockAdapterPlanGuardInput): boolean {
  return (
    plan.adapterState === 'manual-required' &&
    plan.deliveryMode === 'manual-required' &&
    plan.fallbackAction === 'manual-review' &&
    manualRequiredReasonIsPresent(plan)
  );
}

function unavailablePlanIsPresent(plan: BrowserGameHoldBlockAdapterPlanGuardInput): boolean {
  return (
    plan.adapterState === 'unavailable' &&
    plan.deliveryMode === 'unavailable' &&
    plan.fallbackAction === 'no-action' &&
    unavailableReasonIsPresent(plan)
  );
}

function candidateOnlyActionIsPresent(
  plan: BrowserGameHoldBlockAdapterPlanGuardInput,
  reason: Infer<typeof BrowserGameHoldBlockReasonSchema>,
  fallbackAction: Infer<typeof BrowserGameHoldBlockFallbackActionSchema>
): boolean {
  return (
    plan.adapterState === 'candidate-only' &&
    plan.deliveryMode === 'contract-only' &&
    plan.policyCandidateRef !== null &&
    plan.managedInterventionAdapterProofRef === null &&
    plan.reasonCodes.includes(reason) &&
    plan.fallbackAction === fallbackAction
  );
}

function manualRequiredReasonIsPresent(plan: BrowserGameHoldBlockAdapterPlanGuardInput): boolean {
  return (
    plan.reasonCodes.includes('cloud-gaming-proof-manual-required') ||
    plan.reasonCodes.includes('missing-managed-route-proof') ||
    plan.reasonCodes.includes('missing-adapter-proof')
  );
}

function unavailableReasonIsPresent(plan: BrowserGameHoldBlockAdapterPlanGuardInput): boolean {
  return (
    plan.reasonCodes.includes('native-game-control-unavailable') ||
    plan.reasonCodes.includes('unmanaged-browser-not-supported')
  );
}

function browserGameHoldBlockAdapterPlanClaimsRuntime(plan: BrowserGameHoldBlockAdapterPlanGuardInput): boolean {
  return BrowserGameHoldBlockAdapterRuntimeClaimFields.some((field) => plan[field] === true);
}

const BrowserGameHoldBlockAdapterRuntimeClaimFields = [
  'rawUrlIncluded',
  'rawPageBodyIncluded',
  'rawGamePayloadIncluded',
  'childCookieSessionReused',
  'unmanagedBrowserExactUrlClaimed',
  'browserMutationExecutedClaimed',
  'renderedChildPageClaimed',
  'notificationDeliveredClaimed',
  'finalPolicyDecisionClaimed',
  'timeLimitAppliedClaimed',
  'cloudFrameAnalysisClaimed',
  'nativeGameControlClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameHoldBlockAdapterPlanGuardInput>;
