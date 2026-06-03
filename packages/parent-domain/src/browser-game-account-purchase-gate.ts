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

const AccountRequestKinds: ReadonlyArray<BrowserGameApprovalRequestKind> = [
  'game-account-creation',
  'game-login',
  'secondary-game-account',
] as const;

const PurchaseRequestKinds: ReadonlyArray<BrowserGameApprovalRequestKind> = [
  'game-purchase',
  'subscription-purchase',
  'loot-box-purchase',
  'virtual-currency-purchase',
  'wallet-or-gambling-payment',
] as const;

const DownloadRequestKinds: ReadonlyArray<BrowserGameApprovalRequestKind> = [
  'game-download',
  'install-prompt',
] as const;

function browserGameApprovalRequestIsConsistent(request: BrowserGameApprovalRequestCandidate): boolean {
  if (browserGameApprovalRequestClaimsExecution(request)) {
    return false;
  }
  if (request.requestState === 'pending-contract-only') {
    return (
      request.managedRouteEvidenceRef !== null &&
      request.parentRuleRef !== null &&
      request.expiresAt !== null &&
      request.reasonCodes.includes('parent-rule-requires-approval') &&
      browserGameApprovalRequestKindMatchesReasons(request)
    );
  }
  if (request.requestState === 'blocked-candidate') {
    return (
      request.managedRouteEvidenceRef !== null &&
      request.parentRuleRef !== null &&
      request.reasonCodes.includes('parent-rule-blocks-flow') &&
      browserGameApprovalRequestKindMatchesReasons(request)
    );
  }
  if (request.requestState === 'manual-required') {
    return request.reasonCodes.includes('manual-required');
  }
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
  if (AccountRequestKinds.includes(request.requestKind)) {
    return browserGameAccountRequestKindMatchesReasons(request);
  }
  if (PurchaseRequestKinds.includes(request.requestKind)) {
    return browserGamePurchaseRequestKindMatchesReasons(request);
  }
  if (DownloadRequestKinds.includes(request.requestKind)) {
    return request.reasonCodes.includes('download-or-install-route');
  }
  if (request.requestKind === 'cloud-gaming-start') {
    return request.reasonCodes.includes('cloud-gaming-route');
  }
  return request.reasonCodes.includes('unknown-game-route');
}

function browserGameAccountRequestKindMatchesReasons(request: BrowserGameApprovalRequestCandidate): boolean {
  if (request.requestKind === 'game-account-creation') {
    return request.reasonCodes.includes('account-creation-route');
  }
  if (request.requestKind === 'game-login') {
    return request.reasonCodes.includes('login-route');
  }
  return request.reasonCodes.includes('secondary-account-route');
}

function browserGamePurchaseRequestKindMatchesReasons(request: BrowserGameApprovalRequestCandidate): boolean {
  if (request.requestKind === 'subscription-purchase') {
    return request.reasonCodes.includes('subscription-route');
  }
  if (request.requestKind === 'loot-box-purchase') {
    return request.reasonCodes.includes('loot-box-route');
  }
  if (request.requestKind === 'virtual-currency-purchase') {
    return request.reasonCodes.includes('virtual-currency-route');
  }
  if (request.requestKind === 'wallet-or-gambling-payment') {
    return (
      request.reasonCodes.includes('wallet-payment-risk') || request.reasonCodes.includes('gambling-like-payment-risk')
    );
  }
  return request.reasonCodes.includes('purchase-route');
}

function browserGameApprovalRequestClaimsExecution(request: BrowserGameApprovalRequestCandidate): boolean {
  return (
    request.rawUrlStored ||
    request.rawGameTitleStored ||
    request.rawAccountIdentifierCaptured ||
    request.credentialCaptured ||
    request.formSubmittedClaimed ||
    request.accountCreatedClaimed ||
    request.purchaseExecutedClaimed ||
    request.paymentInfoCaptured ||
    request.launcherDownloadClaimed ||
    request.notificationDeliveredClaimed ||
    request.uiRenderedClaimed ||
    request.childNotifiedClaimed ||
    request.policyDecisionClaimed ||
    request.runtimeGateExecutedClaimed ||
    request.nativeGameControlClaimed ||
    request.cloudFrameAnalysisClaimed ||
    request.enforcementClaimed
  );
}

function browserGameApprovalDecisionClaimsExecution(decision: BrowserGameApprovalDecisionCandidate): boolean {
  return (
    decision.notificationDeliveredClaimed ||
    decision.uiRenderedClaimed ||
    decision.childNotifiedClaimed ||
    decision.policyDecisionClaimed ||
    decision.runtimeGateExecutedClaimed ||
    decision.accountCreatedClaimed ||
    decision.purchaseExecutedClaimed ||
    decision.launcherDownloadClaimed ||
    decision.nativeGameControlClaimed ||
    decision.cloudFrameAnalysisClaimed ||
    decision.enforcementClaimed
  );
}
