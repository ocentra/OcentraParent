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
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  BrowserGameApprovalConfidenceSchema,
  BrowserGameApprovalDecisionIdSchema,
  BrowserGameApprovalDecisionKindSchema,
  BrowserGameApprovalDecisionStateSchema,
  BrowserGameApprovalEvidenceRefsSchema,
  BrowserGameApprovalReasonCodesSchema,
  BrowserGameApprovalRequestIdSchema,
  type BrowserGameApprovalRequestKind,
  BrowserGameApprovalRequestKindSchema,
  BrowserGameApprovalRequestStateSchema,
} from './browser-game-account-purchase-gate-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const OptionalParentActionRefSchema = Schema.Union(ParentActionReferenceIdSchema, Schema.Null);
const OptionalParentActorRefSchema = Schema.Union(ParentActorIdSchema, Schema.Null);
const OptionalParentPolicyVersionSchema = Schema.Union(ParentPolicyVersionSchema, Schema.Null);
const OptionalParentTimestampSchema = Schema.Union(ParentTimestampSchema, Schema.Null);

const BrowserGameApprovalRequestBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  approvalRequestId: BrowserGameApprovalRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  requestedByDeviceId: ParentDeviceIdSchema,
  requestedAt: ParentTimestampSchema,
  expiresAt: OptionalParentTimestampSchema,
  requestKind: BrowserGameApprovalRequestKindSchema,
  requestState: BrowserGameApprovalRequestStateSchema,
  confidence: BrowserGameApprovalConfidenceSchema,
  sourceEvidenceRefs: BrowserGameApprovalEvidenceRefsSchema,
  managedRouteEvidenceRef: OptionalParentEvidenceRefSchema,
  gameTitleEvidenceRef: OptionalParentEvidenceRefSchema,
  aiAnalysisRef: OptionalParentEvidenceRefSchema,
  parentRuleRef: OptionalParentEvidenceRefSchema,
  reasonCodes: BrowserGameApprovalReasonCodesSchema,
  deliveryState: Schema.Literal('contract-only'),
  rawUrlStored: Schema.Boolean,
  rawGameTitleStored: Schema.Boolean,
  rawAccountIdentifierCaptured: Schema.Boolean,
  credentialCaptured: Schema.Boolean,
  formSubmittedClaimed: Schema.Boolean,
  accountCreatedClaimed: Schema.Boolean,
  purchaseExecutedClaimed: Schema.Boolean,
  paymentInfoCaptured: Schema.Boolean,
  launcherDownloadClaimed: Schema.Boolean,
  notificationDeliveredClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  childNotifiedClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameApprovalRequestCandidate = Infer<typeof BrowserGameApprovalRequestBaseSchema>;

const BrowserGameApprovalDecisionBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  approvalDecisionId: BrowserGameApprovalDecisionIdSchema,
  approvalRequestId: BrowserGameApprovalRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  decidedAt: ParentTimestampSchema,
  decidedByActorId: OptionalParentActorRefSchema,
  decisionKind: BrowserGameApprovalDecisionKindSchema,
  decisionState: BrowserGameApprovalDecisionStateSchema,
  sourceEvidenceRefs: BrowserGameApprovalEvidenceRefsSchema,
  policyVersionRef: OptionalParentPolicyVersionSchema,
  actionCandidateRef: OptionalParentActionRefSchema,
  reasonCodes: BrowserGameApprovalReasonCodesSchema,
  deliveryState: Schema.Literal('contract-only'),
  notificationDeliveredClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  childNotifiedClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  accountCreatedClaimed: Schema.Boolean,
  purchaseExecutedClaimed: Schema.Boolean,
  launcherDownloadClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameApprovalDecisionCandidate = Infer<typeof BrowserGameApprovalDecisionBaseSchema>;

export const BrowserGameApprovalRequestSchema = withParser(
  BrowserGameApprovalRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        browserGameApprovalRequestIsConsistent(request) ||
        'Expected browser-game approval request to stay contract-only, evidence-backed, and non-executing'
    )
  )
);

export const BrowserGameApprovalDecisionSchema = withParser(
  BrowserGameApprovalDecisionBaseSchema.pipe(
    Schema.filter(
      (decision) =>
        browserGameApprovalDecisionIsConsistent(decision) ||
        'Expected browser-game approval decision to stay recorded candidate state without execution or enforcement claims'
    )
  )
);

export const decodeBrowserGameApprovalRequest = Schema.decodeUnknownSync(BrowserGameApprovalRequestSchema);
export const decodeBrowserGameApprovalDecision = Schema.decodeUnknownSync(BrowserGameApprovalDecisionSchema);

export type BrowserGameApprovalRequest = Infer<typeof BrowserGameApprovalRequestSchema>;
export type BrowserGameApprovalDecision = Infer<typeof BrowserGameApprovalDecisionSchema>;

const BrowserGameApprovalRequestKindValidators = {
  'game-account-creation': (request: BrowserGameApprovalRequestCandidate) =>
    request.reasonCodes.includes('account-creation-route'),
  'game-login': (request: BrowserGameApprovalRequestCandidate) => request.reasonCodes.includes('login-route'),
  'secondary-game-account': (request: BrowserGameApprovalRequestCandidate) =>
    request.reasonCodes.includes('secondary-account-route'),
  'game-purchase': (request: BrowserGameApprovalRequestCandidate) => request.reasonCodes.includes('purchase-route'),
  'subscription-purchase': (request: BrowserGameApprovalRequestCandidate) =>
    request.reasonCodes.includes('subscription-route'),
  'loot-box-purchase': (request: BrowserGameApprovalRequestCandidate) => request.reasonCodes.includes('loot-box-route'),
  'virtual-currency-purchase': (request: BrowserGameApprovalRequestCandidate) =>
    request.reasonCodes.includes('virtual-currency-route'),
  'wallet-or-gambling-payment': (request: BrowserGameApprovalRequestCandidate) =>
    request.reasonCodes.includes('wallet-payment-risk') || request.reasonCodes.includes('gambling-like-payment-risk'),
  'game-download': (request: BrowserGameApprovalRequestCandidate) =>
    request.reasonCodes.includes('download-or-install-route'),
  'install-prompt': (request: BrowserGameApprovalRequestCandidate) =>
    request.reasonCodes.includes('download-or-install-route'),
  'cloud-gaming-start': (request: BrowserGameApprovalRequestCandidate) =>
    request.reasonCodes.includes('cloud-gaming-route'),
  'unknown-game-start': (request: BrowserGameApprovalRequestCandidate) =>
    request.reasonCodes.includes('unknown-game-route'),
} satisfies Record<BrowserGameApprovalRequestKind, (request: BrowserGameApprovalRequestCandidate) => boolean>;

const BrowserGameApprovalRequestStateValidators = {
  'pending-contract-only': pendingApprovalRequestIsConsistent,
  'blocked-candidate': blockedApprovalRequestIsConsistent,
  'manual-required': manualRequiredApprovalRequestIsConsistent,
  unavailable: unavailableApprovalRequestIsConsistent,
} satisfies Record<
  BrowserGameApprovalRequestCandidate['requestState'],
  (request: BrowserGameApprovalRequestCandidate) => boolean
>;

function browserGameApprovalRequestIsConsistent(request: BrowserGameApprovalRequestCandidate): boolean {
  if (browserGameApprovalRequestClaimsExecution(request)) {
    return false;
  }
  return BrowserGameApprovalRequestStateValidators[request.requestState](request);
}

function pendingApprovalRequestIsConsistent(request: BrowserGameApprovalRequestCandidate): boolean {
  return (
    request.managedRouteEvidenceRef !== null &&
    request.parentRuleRef !== null &&
    request.expiresAt !== null &&
    request.reasonCodes.includes('parent-rule-requires-approval') &&
    browserGameApprovalRequestKindMatchesReasons(request)
  );
}

function blockedApprovalRequestIsConsistent(request: BrowserGameApprovalRequestCandidate): boolean {
  return (
    request.managedRouteEvidenceRef !== null &&
    request.parentRuleRef !== null &&
    request.reasonCodes.includes('parent-rule-blocks-flow') &&
    browserGameApprovalRequestKindMatchesReasons(request)
  );
}

function manualRequiredApprovalRequestIsConsistent(request: BrowserGameApprovalRequestCandidate): boolean {
  return request.reasonCodes.includes('manual-required');
}

function unavailableApprovalRequestIsConsistent(request: BrowserGameApprovalRequestCandidate): boolean {
  return request.managedRouteEvidenceRef === null && request.reasonCodes.includes('missing-route-proof');
}

function browserGameApprovalDecisionIsConsistent(decision: BrowserGameApprovalDecisionCandidate): boolean {
  if (browserGameApprovalDecisionClaimsExecution(decision)) {
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

function browserGameApprovalRequestKindMatchesReasons(request: BrowserGameApprovalRequestCandidate): boolean {
  return BrowserGameApprovalRequestKindValidators[request.requestKind](request);
}

const BrowserGameApprovalRequestExecutionClaimFields = [
  'rawUrlStored',
  'rawGameTitleStored',
  'rawAccountIdentifierCaptured',
  'credentialCaptured',
  'formSubmittedClaimed',
  'accountCreatedClaimed',
  'purchaseExecutedClaimed',
  'paymentInfoCaptured',
  'launcherDownloadClaimed',
  'notificationDeliveredClaimed',
  'uiRenderedClaimed',
  'childNotifiedClaimed',
  'policyDecisionClaimed',
  'runtimeGateExecutedClaimed',
  'nativeGameControlClaimed',
  'cloudFrameAnalysisClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameApprovalRequestCandidate>;

const BrowserGameApprovalDecisionExecutionClaimFields = [
  'notificationDeliveredClaimed',
  'uiRenderedClaimed',
  'childNotifiedClaimed',
  'policyDecisionClaimed',
  'runtimeGateExecutedClaimed',
  'accountCreatedClaimed',
  'purchaseExecutedClaimed',
  'launcherDownloadClaimed',
  'nativeGameControlClaimed',
  'cloudFrameAnalysisClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameApprovalDecisionCandidate>;

function browserGameApprovalRequestClaimsExecution(request: BrowserGameApprovalRequestCandidate): boolean {
  return BrowserGameApprovalRequestExecutionClaimFields.some((field) => request[field] === true);
}

function browserGameApprovalDecisionClaimsExecution(decision: BrowserGameApprovalDecisionCandidate): boolean {
  return BrowserGameApprovalDecisionExecutionClaimFields.some((field) => decision[field] === true);
}
