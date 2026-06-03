import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentActionReferenceIdSchema,
  ParentActorIdSchema,
  ParentContractSchemaVersionSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  BrowserGameCloudGateActionCandidateSchema,
  BrowserGameCloudGateConfidenceSchema,
  BrowserGameCloudGateDecisionIdSchema,
  BrowserGameCloudGateDecisionKindSchema,
  BrowserGameCloudGateDecisionStateSchema,
  BrowserGameCloudGateEvidenceRefsSchema,
  BrowserGameCloudGateReasonCodesSchema,
  BrowserGameCloudGateRequestIdSchema,
  BrowserGameCloudGateSignalKindsSchema,
  BrowserGameCloudGateStateSchema,
  BrowserGameCloudGateSubjectSchema,
  BrowserGameCloudPlatformSchema,
} from './browser-game-cloud-gaming-gate-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const OptionalParentActionRefSchema = Schema.Union(ParentActionReferenceIdSchema, Schema.Null);
const OptionalParentActorRefSchema = Schema.Union(ParentActorIdSchema, Schema.Null);
const OptionalParentPolicyVersionSchema = Schema.Union(ParentPolicyVersionSchema, Schema.Null);
const OptionalParentTimestampSchema = Schema.Union(ParentTimestampSchema, Schema.Null);

const BrowserGameCloudGateRequestBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  gateRequestId: BrowserGameCloudGateRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  requestedByDeviceId: ParentDeviceIdSchema,
  requestedAt: ParentTimestampSchema,
  expiresAt: OptionalParentTimestampSchema,
  platform: BrowserGameCloudPlatformSchema,
  gateSubject: BrowserGameCloudGateSubjectSchema,
  gateState: BrowserGameCloudGateStateSchema,
  actionCandidate: BrowserGameCloudGateActionCandidateSchema,
  confidence: BrowserGameCloudGateConfidenceSchema,
  sourceEvidenceRefs: BrowserGameCloudGateEvidenceRefsSchema,
  signalKinds: BrowserGameCloudGateSignalKindsSchema,
  managedRouteEvidenceRef: OptionalParentEvidenceRefSchema,
  platformTitleEvidenceRef: OptionalParentEvidenceRefSchema,
  platformRatingEvidenceRef: OptionalParentEvidenceRefSchema,
  policyCandidateRef: OptionalParentEvidenceRefSchema,
  parentApprovalRequestRef: OptionalParentEvidenceRefSchema,
  scheduleContextRef: OptionalParentEvidenceRefSchema,
  mobileCapabilityRef: OptionalParentEvidenceRefSchema,
  reasonCodes: BrowserGameCloudGateReasonCodesSchema,
  deliveryState: Schema.Literal('contract-only'),
  rawCloudTitleStored: Schema.Boolean,
  rawStreamFrameStored: Schema.Boolean,
  cloudStreamFrameAnalysisClaimed: Schema.Boolean,
  perGameCloudTitleClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  nativeLauncherControlClaimed: Schema.Boolean,
  gameChatContentClaimed: Schema.Boolean,
  accountOrPurchaseFlowClaimed: Schema.Boolean,
  notificationDeliveredClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  childNotifiedClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameCloudGateRequestCandidate = Infer<typeof BrowserGameCloudGateRequestBaseSchema>;

const BrowserGameCloudGateDecisionBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  gateDecisionId: BrowserGameCloudGateDecisionIdSchema,
  gateRequestId: BrowserGameCloudGateRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  decidedAt: ParentTimestampSchema,
  decidedByActorId: OptionalParentActorRefSchema,
  decisionKind: BrowserGameCloudGateDecisionKindSchema,
  decisionState: BrowserGameCloudGateDecisionStateSchema,
  sourceEvidenceRefs: BrowserGameCloudGateEvidenceRefsSchema,
  policyVersionRef: OptionalParentPolicyVersionSchema,
  actionCandidateRef: OptionalParentActionRefSchema,
  reasonCodes: BrowserGameCloudGateReasonCodesSchema,
  deliveryState: Schema.Literal('contract-only'),
  notificationDeliveredClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  childNotifiedClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  cloudStreamFrameAnalysisClaimed: Schema.Boolean,
  perGameCloudTitleClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  nativeLauncherControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameCloudGateDecisionCandidate = Infer<typeof BrowserGameCloudGateDecisionBaseSchema>;

export const BrowserGameCloudGateRequestSchema = withParser(
  BrowserGameCloudGateRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        browserGameCloudGateRequestIsHonest(request) ||
        'Expected browser-game cloud gate request to remain evidence-backed, candidate-only, and non-executing'
    )
  )
);

export const BrowserGameCloudGateDecisionSchema = withParser(
  BrowserGameCloudGateDecisionBaseSchema.pipe(
    Schema.filter(
      (decision) =>
        browserGameCloudGateDecisionIsHonest(decision) ||
        'Expected browser-game cloud gate decision to remain recorded candidate state without runtime or enforcement claims'
    )
  )
);

export const decodeBrowserGameCloudGateRequest = Schema.decodeUnknownSync(BrowserGameCloudGateRequestSchema);
export const decodeBrowserGameCloudGateDecision = Schema.decodeUnknownSync(BrowserGameCloudGateDecisionSchema);

export type BrowserGameCloudGateRequest = Infer<typeof BrowserGameCloudGateRequestSchema>;
export type BrowserGameCloudGateDecision = Infer<typeof BrowserGameCloudGateDecisionSchema>;

type BrowserGameCloudGateActionValidator = (request: BrowserGameCloudGateRequestCandidate) => boolean;

const BrowserGameCloudGateActionValidators = {
  'parent-review-candidate': askParentCloudGateActionIsHonest,
  'block-candidate': blockCloudGateActionIsHonest,
  'time-limit-candidate': timeLimitCloudGateActionIsHonest,
  'allow-window-candidate': allowWindowCloudGateActionIsHonest,
  'manual-review-candidate': unsupportedCandidateCloudGateActionIsHonest,
  'unknown-fallback-candidate': unsupportedCandidateCloudGateActionIsHonest,
} satisfies Record<BrowserGameCloudGateRequestCandidate['actionCandidate'], BrowserGameCloudGateActionValidator>;

function browserGameCloudGateRequestIsHonest(request: BrowserGameCloudGateRequestCandidate): boolean {
  if (browserGameCloudGateRequestClaimsRuntime(request)) {
    return false;
  }
  if (request.gateState === 'candidate') {
    return (
      request.managedRouteEvidenceRef !== null &&
      request.expiresAt !== null &&
      browserGameCloudGateRequestActionIsHonest(request)
    );
  }
  if (request.gateState === 'manual-required') {
    return (
      request.actionCandidate === 'manual-review-candidate' &&
      request.reasonCodes.includes('manual-required') &&
      request.reasonCodes.includes('content-frame-unavailable')
    );
  }
  return (
    request.actionCandidate === 'unknown-fallback-candidate' &&
    request.managedRouteEvidenceRef === null &&
    request.reasonCodes.includes('missing-platform-proof')
  );
}

function browserGameCloudGateDecisionIsHonest(decision: BrowserGameCloudGateDecisionCandidate): boolean {
  if (browserGameCloudGateDecisionClaimsRuntime(decision)) {
    return false;
  }
  if (decision.decisionKind === 'manual-required') {
    return (
      decision.decisionState === 'manual-required' &&
      decision.decidedByActorId === null &&
      decision.policyVersionRef === null &&
      decision.actionCandidateRef === null &&
      decision.reasonCodes.includes('manual-required')
    );
  }
  return (
    decision.decisionState === 'recorded-contract-only' &&
    decision.decidedByActorId !== null &&
    decision.policyVersionRef !== null &&
    decision.actionCandidateRef !== null
  );
}

function browserGameCloudGateRequestActionIsHonest(request: BrowserGameCloudGateRequestCandidate): boolean {
  return BrowserGameCloudGateActionValidators[request.actionCandidate](request);
}

function askParentCloudGateActionIsHonest(request: BrowserGameCloudGateRequestCandidate): boolean {
  return (
    request.parentApprovalRequestRef !== null &&
    request.reasonCodes.includes('parent-approval-required') &&
    request.reasonCodes.includes('unknown-cloud-title')
  );
}

function blockCloudGateActionIsHonest(request: BrowserGameCloudGateRequestCandidate): boolean {
  return (
    request.policyCandidateRef !== null &&
    (request.reasonCodes.includes('mature-title-risk') || request.reasonCodes.includes('schedule-blocked'))
  );
}

function timeLimitCloudGateActionIsHonest(request: BrowserGameCloudGateRequestCandidate): boolean {
  return (
    request.policyCandidateRef !== null &&
    request.scheduleContextRef !== null &&
    request.reasonCodes.includes('time-budget-candidate')
  );
}

function allowWindowCloudGateActionIsHonest(request: BrowserGameCloudGateRequestCandidate): boolean {
  return (
    request.policyCandidateRef !== null &&
    request.reasonCodes.includes('known-cloud-domain') &&
    request.signalKinds.includes('known-cloud-domain')
  );
}

function unsupportedCandidateCloudGateActionIsHonest(): boolean {
  return false;
}

function browserGameCloudGateRequestClaimsRuntime(request: BrowserGameCloudGateRequestCandidate): boolean {
  return BrowserGameCloudGateRequestRuntimeClaimFields.some((field) => request[field] === true);
}

const BrowserGameCloudGateRequestRuntimeClaimFields = [
  'rawCloudTitleStored',
  'rawStreamFrameStored',
  'cloudStreamFrameAnalysisClaimed',
  'perGameCloudTitleClaimed',
  'nativeGameControlClaimed',
  'nativeLauncherControlClaimed',
  'gameChatContentClaimed',
  'accountOrPurchaseFlowClaimed',
  'notificationDeliveredClaimed',
  'uiRenderedClaimed',
  'childNotifiedClaimed',
  'finalPolicyDecisionClaimed',
  'runtimeGateExecutedClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameCloudGateRequestCandidate>;

function browserGameCloudGateDecisionClaimsRuntime(decision: BrowserGameCloudGateDecisionCandidate): boolean {
  return (
    decision.notificationDeliveredClaimed ||
    decision.uiRenderedClaimed ||
    decision.childNotifiedClaimed ||
    decision.finalPolicyDecisionClaimed ||
    decision.runtimeGateExecutedClaimed ||
    decision.cloudStreamFrameAnalysisClaimed ||
    decision.perGameCloudTitleClaimed ||
    decision.nativeGameControlClaimed ||
    decision.nativeLauncherControlClaimed ||
    decision.enforcementClaimed
  );
}
