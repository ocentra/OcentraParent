import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppInstallPurchaseApprovalDecisionActionSchema,
  AppInstallPurchaseApprovalRequestKindSchema,
  AppInstallPurchaseApprovalStateSchema,
} from './app-install-purchase-approval';
import { AppInstallPurchaseApprovalContractProofReadModel } from './app-install-purchase-approval-proof';
import { AppInstallPurchaseApprovedApiEntitlementProofReadModel } from './app-install-purchase-approved-api-entitlement-proof';
import { AppInstallPurchaseReportRuntimeProofReadModel } from './app-install-purchase-report-runtime-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ParentReviewActionProofText = Schema.String.pipe(Schema.minLength(1));
const ParentReviewActionProofVersion = 'app-install-purchase-parent-review-action-proof';
const SourceApprovalContractProofVersion = 'app-install-purchase-approval-contract-proof';
const SourceApprovedApiEntitlementProofVersion = 'app-install-purchase-approved-api-entitlement-proof';
const SourceReportRuntimeProofVersion = 'app-install-purchase-report-runtime-proof';
const ParentReviewActionTimestamp = '2026-06-05T04:05:00.000Z';
const ParentReviewActionClaimBoundary =
  'parent review action proof only; no portal approval UI no parent action runtime delivery no store integration no provider API execution no platform adapter no child-device delivery no runtime report delivery no real install or purchase interception no child activity data not generic app blocking no Ocentra-hosted family data custody';
const RequiredReviewActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RequiredParentActionStates = ['approved', 'denied', 'time-box-active', 'review-needed'] as const;
const RequiredParentReviewNonClaims = [
  'no-portal-approval-ui',
  'no-parent-action-runtime-delivery',
  'no-store-integration',
  'no-provider-api-execution',
  'no-platform-adapter',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'not-generic-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;

export const AppInstallPurchaseParentReviewActionProofSchemaVersionSchema = withParser(
  Schema.Literal(ParentReviewActionProofVersion)
);
const AppInstallPurchaseParentReviewActionStateSchema = withParser(Schema.Literal(...RequiredParentActionStates));
const AppInstallPurchaseParentReviewActionRuntimeClaimSchema = withParser(
  Schema.Literal('contract-action-recorded', 'manual-review-state-only')
);
const AppInstallPurchaseParentReviewActionDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseParentReviewActionPortalClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseParentReviewActionProviderApiClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseParentReviewActionStoreIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentReviewActionAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseParentReviewActionInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentReviewActionBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentReviewActionDataCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchaseParentReviewActionHostedCustodyClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentReviewActionNonClaimSchema = withParser(Schema.Literal(...RequiredParentReviewNonClaims));

const ParentReviewActionRowIdSchema = ParentReviewActionProofText.pipe(
  Schema.brand('AppInstallPurchaseParentReviewActionRowId')
);
const ParentReviewActionDecisionRefSchema = ParentReviewActionProofText.pipe(
  Schema.brand('AppInstallPurchaseParentReviewActionDecisionRef')
);
const ParentReviewActionRequestRefSchema = ParentReviewActionProofText.pipe(
  Schema.brand('AppInstallPurchaseParentReviewActionRequestRef')
);
const ParentReviewActionRefSchema = ParentReviewActionProofText.pipe(
  Schema.brand('AppInstallPurchaseParentReviewActionRef')
);
const ParentReviewActionReportRuntimeRefSchema = ParentReviewActionProofText.pipe(
  Schema.brand('AppInstallPurchaseParentReviewActionReportRuntimeRef')
);
const ParentReviewActionEvidenceRefSchema = ParentReviewActionProofText.pipe(
  Schema.brand('AppInstallPurchaseParentReviewActionEvidenceRef')
);
const ParentReviewActionAuditRefSchema = ParentReviewActionProofText.pipe(
  Schema.brand('AppInstallPurchaseParentReviewActionAuditRef')
);
const ParentReviewActionClaimBoundarySchema = ParentReviewActionProofText.pipe(
  Schema.brand('AppInstallPurchaseParentReviewActionClaimBoundary')
);

const ParentReviewActionRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseParentReviewActionProofSchemaVersionSchema,
  parentReviewActionRowId: ParentReviewActionRowIdSchema,
  sourceApprovalProofVersion: Schema.Literal(SourceApprovalContractProofVersion),
  sourceDecisionId: ParentReviewActionDecisionRefSchema,
  sourceRequestId: ParentReviewActionRequestRefSchema,
  sourceRequestKind: AppInstallPurchaseApprovalRequestKindSchema,
  sourceDecisionAction: AppInstallPurchaseApprovalDecisionActionSchema,
  resultingApprovalState: AppInstallPurchaseApprovalStateSchema,
  parentReviewActionState: AppInstallPurchaseParentReviewActionStateSchema,
  parentActionRecorded: Schema.Boolean,
  parentActionReferenceId: Schema.Union(ParentReviewActionRefSchema, Schema.Null),
  parentActionRuntimeClaim: AppInstallPurchaseParentReviewActionRuntimeClaimSchema,
  auditEventRefs: Schema.Array(ParentReviewActionAuditRefSchema),
  sourceApprovedApiEntitlementProofVersion: Schema.Literal(SourceApprovedApiEntitlementProofVersion),
  sourceApprovedApiEntitlementEvidenceRefs: Schema.Array(ParentReviewActionEvidenceRefSchema),
  sourceReportRuntimeProofVersion: Schema.Literal(SourceReportRuntimeProofVersion),
  sourceReportRuntimeRefs: Schema.Array(ParentReviewActionReportRuntimeRefSchema),
  runtimeActionDeliveryClaim: AppInstallPurchaseParentReviewActionDeliveryClaimSchema,
  portalApprovalUiClaim: AppInstallPurchaseParentReviewActionPortalClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseParentReviewActionProviderApiClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseParentReviewActionStoreIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseParentReviewActionAdapterClaimSchema,
  childDeliveryClaim: AppInstallPurchaseParentReviewActionDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseParentReviewActionDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchaseParentReviewActionInterceptionClaimSchema,
  appBlockingClaim: AppInstallPurchaseParentReviewActionBlockingClaimSchema,
  childDataCustody: AppInstallPurchaseParentReviewActionDataCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseParentReviewActionHostedCustodyClaimSchema,
  claimBoundary: ParentReviewActionClaimBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type ParentReviewActionRowCandidate = Infer<typeof ParentReviewActionRowBaseSchema>;

export const AppInstallPurchaseParentReviewActionRowSchema = withParser(
  ParentReviewActionRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        parentReviewActionRowIsHonest(row) ||
        'Expected parent review action rows to link approval decisions without portal, runtime delivery, provider, adapter, child delivery, custody, interception, or blocking claims'
    )
  )
);

const ParentReviewActionProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseParentReviewActionProofSchemaVersionSchema,
  sourceApprovalProofVersion: Schema.Literal(SourceApprovalContractProofVersion),
  sourceApprovedApiEntitlementProofVersion: Schema.Literal(SourceApprovedApiEntitlementProofVersion),
  sourceReportRuntimeProofVersion: Schema.Literal(SourceReportRuntimeProofVersion),
  parentReviewActionRows: Schema.Array(AppInstallPurchaseParentReviewActionRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseParentReviewActionNonClaimSchema),
  knownGaps: Schema.Array(ParentReviewActionRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseParentReviewActionProof = Infer<typeof ParentReviewActionProofBaseSchema>;

export const AppInstallPurchaseParentReviewActionProofSchema = withParser(
  ParentReviewActionProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        parentReviewActionProofIsHonest(proof) ||
        'Expected app install/purchase parent review action proof to cover approval actions and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseParentReviewActionKnownGaps = [
  'Parent review action rows link contract decisions to approved API/entitlement evidence and report-runtime refs only; no portal approval UI is implemented.',
  'Parent action refs are contract proof rows only and do not deliver runtime actions to child devices or store/provider systems.',
  'Store/provider execution, platform adapters, production child-device package capture, child delivery, report writer/delivery, interception, and app blocking remain unimplemented.',
] as const;

export const AppInstallPurchaseParentReviewActionProofReadModel = AppInstallPurchaseParentReviewActionProofSchema.parse(
  {
    schemaVersion: ParentReviewActionProofVersion,
    sourceApprovalProofVersion: SourceApprovalContractProofVersion,
    sourceApprovedApiEntitlementProofVersion: SourceApprovedApiEntitlementProofVersion,
    sourceReportRuntimeProofVersion: SourceReportRuntimeProofVersion,
    parentReviewActionRows:
      AppInstallPurchaseApprovalContractProofReadModel.approvalDecisions.map(parentReviewActionRow),
    nonClaims: RequiredParentReviewNonClaims,
    knownGaps: AppInstallPurchaseParentReviewActionKnownGaps,
    updatedAt: ParentReviewActionTimestamp,
  }
);

export function summarizeAppInstallPurchaseParentReviewActionProof(proof: AppInstallPurchaseParentReviewActionProof) {
  return {
    parentReviewActionRows: proof.parentReviewActionRows.length,
    parentActionRecordedRows: proof.parentReviewActionRows.filter((row) => row.parentActionRecorded).length,
    manualReviewStateRows: proof.parentReviewActionRows.filter(
      (row) => row.parentActionRuntimeClaim === 'manual-review-state-only'
    ).length,
    reportRuntimeLinkedRows: proof.parentReviewActionRows.filter(
      (row) =>
        row.sourceReportRuntimeRefs.length === AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.length
    ).length,
    portalApprovalUiRows: proof.parentReviewActionRows.filter((row) => row.portalApprovalUiClaim !== 'not-implemented')
      .length,
    runtimeDeliveredRows: proof.parentReviewActionRows.filter(
      (row) => row.runtimeActionDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function parentReviewActionRow(
  decision: (typeof AppInstallPurchaseApprovalContractProofReadModel.approvalDecisions)[number]
) {
  return {
    schemaVersion: ParentReviewActionProofVersion,
    parentReviewActionRowId: `parent-review-action-${decision.decisionAction}`,
    sourceApprovalProofVersion: SourceApprovalContractProofVersion,
    sourceDecisionId: decision.decisionId,
    sourceRequestId: decision.requestId,
    sourceRequestKind: decision.requestKind,
    sourceDecisionAction: decision.decisionAction,
    resultingApprovalState: decision.resultingState.state,
    parentReviewActionState: decision.resultingState.state,
    parentActionRecorded: decision.parentAction !== null,
    parentActionReferenceId: decision.parentAction?.actionReferenceId ?? null,
    parentActionRuntimeClaim: decision.parentAction === null ? 'manual-review-state-only' : 'contract-action-recorded',
    auditEventRefs: decision.auditEventRefs.map((eventRef) => eventRef.auditEventId),
    sourceApprovedApiEntitlementProofVersion: SourceApprovedApiEntitlementProofVersion,
    sourceApprovedApiEntitlementEvidenceRefs: AppInstallPurchaseApprovedApiEntitlementProofReadModel.evidenceRows.map(
      (row) => row.evidenceRowId
    ),
    sourceReportRuntimeProofVersion: SourceReportRuntimeProofVersion,
    sourceReportRuntimeRefs: AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.map(
      (row) => row.reportRuntimeRowId
    ),
    runtimeActionDeliveryClaim: 'not-delivered',
    portalApprovalUiClaim: 'not-implemented',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    interceptionClaim: 'not-claimed',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: ParentReviewActionClaimBoundary,
    linkedAt: ParentReviewActionTimestamp,
  } as const;
}

function parentReviewActionRowIsHonest(row: ParentReviewActionRowCandidate): boolean {
  return (
    parentReviewActionMatchesDecision(row) &&
    parentReviewActionEvidenceIsComplete(row) &&
    parentReviewActionClaimsStayUnimplemented(row) &&
    parentReviewActionBoundaryIsExplicit(row.claimBoundary)
  );
}

function parentReviewActionMatchesDecision(row: ParentReviewActionRowCandidate): boolean {
  const expectedState = expectedParentReviewState(row.sourceDecisionAction);
  const actionStateMatches =
    row.resultingApprovalState === expectedState && row.parentReviewActionState === expectedState;
  if (row.sourceDecisionAction === 'review-needed') {
    return (
      actionStateMatches &&
      !row.parentActionRecorded &&
      row.parentActionReferenceId === null &&
      row.parentActionRuntimeClaim === 'manual-review-state-only'
    );
  }
  return (
    actionStateMatches &&
    row.parentActionRecorded &&
    row.parentActionReferenceId !== null &&
    row.parentActionRuntimeClaim === 'contract-action-recorded'
  );
}

function parentReviewActionEvidenceIsComplete(row: ParentReviewActionRowCandidate): boolean {
  return (
    row.sourceApprovalProofVersion === SourceApprovalContractProofVersion &&
    row.sourceApprovedApiEntitlementProofVersion === SourceApprovedApiEntitlementProofVersion &&
    row.sourceReportRuntimeProofVersion === SourceReportRuntimeProofVersion &&
    row.auditEventRefs.length > 0 &&
    row.sourceApprovedApiEntitlementEvidenceRefs.length ===
      AppInstallPurchaseApprovedApiEntitlementProofReadModel.evidenceRows.length &&
    row.sourceReportRuntimeRefs.length === AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.length
  );
}

function parentReviewActionClaimsStayUnimplemented(row: ParentReviewActionRowCandidate): boolean {
  return (
    row.runtimeActionDeliveryClaim === 'not-delivered' &&
    row.portalApprovalUiClaim === 'not-implemented' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function parentReviewActionProofIsHonest(proof: AppInstallPurchaseParentReviewActionProof): boolean {
  return (
    proof.sourceApprovalProofVersion === SourceApprovalContractProofVersion &&
    proof.sourceApprovedApiEntitlementProofVersion === SourceApprovedApiEntitlementProofVersion &&
    proof.sourceReportRuntimeProofVersion === SourceReportRuntimeProofVersion &&
    parentReviewActionsAreComplete(proof.parentReviewActionRows) &&
    parentReviewActionNonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

function parentReviewActionsAreComplete(rows: readonly ParentReviewActionRowCandidate[]): boolean {
  const actions = new Set(rows.map((row) => row.sourceDecisionAction));
  const states = new Set(rows.map((row) => row.parentReviewActionState));
  return (
    rows.length === RequiredReviewActions.length &&
    RequiredReviewActions.every((action) => actions.has(action)) &&
    RequiredParentActionStates.every((state) => states.has(state)) &&
    rows.every((row) => parentReviewActionRowIsHonest(row))
  );
}

function parentReviewActionNonClaimsAreComplete(
  nonClaims: readonly (typeof RequiredParentReviewNonClaims)[number][]
): boolean {
  const claimSet = new Set(nonClaims);
  return RequiredParentReviewNonClaims.every((claim) => claimSet.has(claim));
}

function expectedParentReviewState(
  action: typeof AppInstallPurchaseApprovalDecisionActionSchema.Type
): typeof AppInstallPurchaseApprovalStateSchema.Type {
  if (action === 'approve') {
    return 'approved';
  }
  if (action === 'deny') {
    return 'denied';
  }
  if (action === 'time-box') {
    return 'time-box-active';
  }
  return 'review-needed';
}

function parentReviewActionBoundaryIsExplicit(boundary: typeof ParentReviewActionClaimBoundarySchema.Type): boolean {
  return (
    boundary.includes('no portal approval UI') &&
    boundary.includes('no parent action runtime delivery') &&
    boundary.includes('no store integration') &&
    boundary.includes('no provider API execution') &&
    boundary.includes('no platform adapter') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('no child activity data') &&
    boundary.includes('not generic app blocking') &&
    boundary.includes('no Ocentra-hosted family data custody')
  );
}
