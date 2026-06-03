import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentAccountReferenceSchema, FamilyReferenceSchema, ParentActorReferenceSchema } from './references';
import { ParentDeviceReferenceSchema } from './references';
import { ParentTimestampSchema } from './reference-primitives';
import {
  BillingAuditReferenceSchema,
  BillingChildActivityCustodyClaimSchema,
  BillingChildActivityCustodySchema,
  BillingDeviceLimitDecisionIdSchema,
  BillingDeviceLimitDecisionStateSchema,
  BillingDeviceLimitReasonSchema,
  BillingDisplayTextTokenSchema,
  BillingEntitlementDecisionStateSchema,
  BillingEntitlementNonClaimSchema,
  BillingEntitlementSchemaVersionSchema,
  BillingEntitlementSnapshotIdSchema,
  BillingEntitlementSourceSchema,
  BillingEvidenceExportAccessSchema,
  BillingFailureKindSchema,
  BillingFeatureCodeSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentVisibleStateSchema,
  BillingPlanActiveStateSchema,
  BillingPlanIdSchema,
  BillingPortalUiClaimSchema,
  BillingPriceReferenceSchema,
  BillingProviderBackendClaimSchema,
  BillingProviderBoundarySchema,
  BillingProviderReferenceSchema,
  BillingReasonCodeSchema,
  BillingSignatureStateSchema,
  BillingStripeSdkClaimSchema,
  BillingSubscriptionStatusSchema,
  BillingSyncEventIdSchema,
  PositiveBillingLimitSchema,
} from './billing-entitlement-values';

export * from './billing-entitlement-values';

export const BillingFeatureEntitlementSchema = withParser(
  Schema.Struct({
    featureCode: BillingFeatureCodeSchema,
    included: Schema.Boolean,
    gateable: Schema.Boolean,
    safetyCritical: Schema.Boolean,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
  }).pipe(
    Schema.filter(
      (feature) =>
        !feature.safetyCritical ||
        !feature.gateable ||
        'Expected safety-critical local behavior to stay outside paid entitlement gates'
    )
  )
);

export const BillingPlanSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    planId: BillingPlanIdSchema,
    displayTextToken: BillingDisplayTextTokenSchema,
    activeState: BillingPlanActiveStateSchema,
    deviceLimit: PositiveBillingLimitSchema,
    featureEntitlements: Schema.Array(BillingFeatureEntitlementSchema),
    retentionExportAllowance: Schema.Struct({
      advancedReportDays: PositiveBillingLimitSchema,
      exportAllowed: Schema.Boolean,
      parentOwnedStorageRequired: Schema.Boolean,
    }),
    priceReference: BillingPriceReferenceSchema,
    updatedAt: ParentTimestampSchema,
  })
);

export const BillingFailureStateSchema = withParser(
  Schema.Struct({
    failureKind: BillingFailureKindSchema,
    parentVisibleState: BillingParentVisibleStateSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    retainEvidenceExportAccess: Schema.Boolean,
    retryAllowed: Schema.Boolean,
    retryAfter: Schema.Union(ParentTimestampSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (failure) =>
        failure.retainEvidenceExportAccess ||
        'Expected billing failures to retain evidence export and safety-critical audit access'
    )
  )
);

export const BillingFeatureDecisionSchema = withParser(
  Schema.Struct({
    featureCode: BillingFeatureCodeSchema,
    decision: BillingEntitlementDecisionStateSchema,
    reasonCode: BillingReasonCodeSchema,
    safetyCritical: Schema.Boolean,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
  }).pipe(
    Schema.filter(
      (decision) =>
        !decision.safetyCritical ||
        (decision.decision !== 'locked' && decision.decision !== 'unavailable') ||
        'Expected safety-critical behavior not to be locked or made unavailable by billing state'
    )
  )
);

export const BillingEntitlementSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    snapshotId: BillingEntitlementSnapshotIdSchema,
    family: FamilyReferenceSchema,
    parentAccount: ParentAccountReferenceSchema,
    planId: BillingPlanIdSchema,
    subscriptionStatus: BillingSubscriptionStatusSchema,
    source: BillingEntitlementSourceSchema,
    signatureState: BillingSignatureStateSchema,
    generatedAt: ParentTimestampSchema,
    expiresAt: ParentTimestampSchema,
    deviceLimit: PositiveBillingLimitSchema,
    featureDecisions: Schema.Array(BillingFeatureDecisionSchema),
    failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (snapshot) =>
        snapshot.source !== 'unavailable' ||
        snapshot.failureState !== null ||
        'Expected unavailable entitlement snapshots to carry a parent-visible failure state'
    )
  )
);

export const BillingSyncEventSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    syncEventId: BillingSyncEventIdSchema,
    previousStatus: BillingSubscriptionStatusSchema,
    nextStatus: BillingSubscriptionStatusSchema,
    source: BillingEntitlementSourceSchema,
    actor: ParentActorReferenceSchema,
    recordedAt: ParentTimestampSchema,
    providerReference: Schema.Union(BillingProviderReferenceSchema, Schema.Null),
    providerBoundary: BillingProviderBoundarySchema,
    failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (event) =>
        event.providerReference === null ||
        event.providerBoundary === 'backend-reference-only' ||
        'Expected provider references to remain behind the billing backend boundary'
    )
  )
);

export const BillingDeviceLimitDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    decisionId: BillingDeviceLimitDecisionIdSchema,
    requestedDevice: ParentDeviceReferenceSchema,
    entitlementSnapshotId: BillingEntitlementSnapshotIdSchema,
    decision: BillingDeviceLimitDecisionStateSchema,
    reasonCode: BillingDeviceLimitReasonSchema,
    auditReference: BillingAuditReferenceSchema,
    existingLocalSafetyBehavior: BillingLocalSafetyBehaviorSchema,
  }).pipe(
    Schema.filter(
      (decision) =>
        decision.decision !== 'denied' ||
        decision.reasonCode !== 'within-plan' ||
        'Expected denied device-limit decisions to carry a denial reason'
    )
  )
);

export const BillingEntitlementContractProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    plan: BillingPlanSchema,
    entitlementSnapshot: BillingEntitlementSnapshotSchema,
    billingSyncEvents: Schema.Array(BillingSyncEventSchema),
    deviceLimitDecisions: Schema.Array(BillingDeviceLimitDecisionSchema),
    failureStates: Schema.Array(BillingFailureStateSchema),
    nonClaims: Schema.Array(BillingEntitlementNonClaimSchema),
    stripeSdkClaim: BillingStripeSdkClaimSchema,
    billingProviderBackendClaim: BillingProviderBackendClaimSchema,
    portalUiClaim: BillingPortalUiClaimSchema,
    childActivityCustodyClaim: BillingChildActivityCustodyClaimSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        billingEntitlementProofIsHonest(proof) ||
        'Expected billing entitlement proof to keep provider UI custody and safety-shutdown non-claims explicit'
    )
  )
);

export type BillingPlan = Infer<typeof BillingPlanSchema>;
export type BillingEntitlementSnapshot = Infer<typeof BillingEntitlementSnapshotSchema>;
export type BillingSyncEvent = Infer<typeof BillingSyncEventSchema>;
export type BillingDeviceLimitDecision = Infer<typeof BillingDeviceLimitDecisionSchema>;
export type BillingFailureState = Infer<typeof BillingFailureStateSchema>;
export type BillingEntitlementContractProof = Infer<typeof BillingEntitlementContractProofSchema>;

export const decodeBillingEntitlementContractProof = Schema.decodeUnknownSync(BillingEntitlementContractProofSchema);

function billingEntitlementProofIsHonest(proof: {
  readonly billingSyncEvents: ReadonlyArray<{ readonly failureState: BillingFailureState | null }>;
  readonly failureStates: ReadonlyArray<BillingFailureState>;
  readonly nonClaims: ReadonlyArray<string>;
}): boolean {
  const requiredNonClaims = [
    'no-stripe-sdk',
    'no-billing-provider-backend',
    'no-provider-token-custody',
    'no-child-activity-custody',
    'no-safety-shutdown',
    'no-portal-ui',
  ];
  return (
    requiredNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    proof.failureStates.length >= 3 &&
    proof.billingSyncEvents.every(
      (event) => event.failureState === null || event.failureState.retainEvidenceExportAccess
    )
  );
}
