/* generated from crates/schema/src/billing_entitlement_runtime_proof_schema_ts.rs */

import { type Infer, Schema, withParser } from './effect';
import {
  BillingDeviceLimitDecisionSchema,
  BillingEntitlementSnapshotSchema,
  BillingFailureStateSchema,
} from './billing-entitlement';
import {
  BillingChildActivityCustodyClaimSchema,
  BillingChildActivityCustodySchema,
  BillingEvidenceExportAccessSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingStripeSdkClaimSchema,
} from './billing-entitlement-values';
import { ParentTimestampSchema } from './family-reference-primitives';
import {
  BillingEntitlementRuntimeAuditReferenceSchema,
  BillingEntitlementRuntimeBoundaryIdSchema,
  BillingEntitlementRuntimeChildCustodyClaimSchema,
  BillingEntitlementRuntimeConsumptionStateSchema,
  BillingEntitlementRuntimeNonClaimSchema,
  BillingEntitlementRuntimeOperationSchema,
  BillingEntitlementRuntimePortalUiClaimSchema,
  BillingEntitlementRuntimeProductionBillingClaimSchema,
  BillingEntitlementRuntimeProviderContactClaimSchema,
  BillingEntitlementRuntimeProviderExecutionClaimSchema,
  BillingEntitlementRuntimeRefundCreditClaimSchema,
  BillingEntitlementRuntimeSchemaVersionSchema,
  BillingEntitlementRuntimeSnapshotStateSchema,
  BillingEntitlementRuntimeSourceSchema,
  type BillingEntitlementRuntimeConsumptionState as GeneratedBillingEntitlementRuntimeConsumptionState,
  type BillingEntitlementRuntimeNonClaim as GeneratedBillingEntitlementRuntimeNonClaim,
  type BillingEntitlementRuntimeOperation as GeneratedBillingEntitlementRuntimeOperation,
  type BillingEntitlementRuntimeSnapshotState as GeneratedBillingEntitlementRuntimeSnapshotState,
} from './billing-entitlement-runtime-proof-values';
import { GeneratedBillingEntitlementRuntimeProofReadModel } from './generated-billing-entitlement-runtime-proof';

export const BillingEntitlementRuntimeSnapshotConsumptionSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementRuntimeSchemaVersionSchema,
    boundaryId: BillingEntitlementRuntimeBoundaryIdSchema,
    operation: BillingEntitlementRuntimeOperationSchema,
    runtimeState: BillingEntitlementRuntimeSnapshotStateSchema,
    source: BillingEntitlementRuntimeSourceSchema,
    entitlementSnapshot: BillingEntitlementSnapshotSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
    auditReference: BillingEntitlementRuntimeAuditReferenceSchema,
  })
);

export const BillingEntitlementRuntimeDeviceLimitConsumptionSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementRuntimeSchemaVersionSchema,
    boundaryId: BillingEntitlementRuntimeBoundaryIdSchema,
    operation: BillingEntitlementRuntimeOperationSchema,
    deviceLimitDecision: BillingDeviceLimitDecisionSchema,
    consumptionState: BillingEntitlementRuntimeConsumptionStateSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
    auditReference: BillingEntitlementRuntimeAuditReferenceSchema,
  })
);

export const BillingEntitlementRuntimeFailureConsumptionSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementRuntimeSchemaVersionSchema,
    boundaryId: BillingEntitlementRuntimeBoundaryIdSchema,
    operation: BillingEntitlementRuntimeOperationSchema,
    failureState: BillingFailureStateSchema,
    consumedFor: Schema.Array(BillingEntitlementRuntimeOperationSchema),
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    auditReference: BillingEntitlementRuntimeAuditReferenceSchema,
  })
);

export const BillingEntitlementRuntimeProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementRuntimeSchemaVersionSchema,
    snapshotConsumptions: Schema.Array(BillingEntitlementRuntimeSnapshotConsumptionSchema),
    deviceLimitConsumptions: Schema.Array(BillingEntitlementRuntimeDeviceLimitConsumptionSchema),
    failureConsumptions: Schema.Array(BillingEntitlementRuntimeFailureConsumptionSchema),
    nonClaims: Schema.Array(BillingEntitlementRuntimeNonClaimSchema),
    stripeSdkClaim: BillingStripeSdkClaimSchema,
    providerExecutionClaim: BillingEntitlementRuntimeProviderExecutionClaimSchema,
    providerContactClaim: BillingEntitlementRuntimeProviderContactClaimSchema,
    refundCreditClaim: BillingEntitlementRuntimeRefundCreditClaimSchema,
    productionBillingClaim: BillingEntitlementRuntimeProductionBillingClaimSchema,
    portalUiClaim: BillingEntitlementRuntimePortalUiClaimSchema,
    childCustodyClaim: BillingEntitlementRuntimeChildCustodyClaimSchema,
    childActivityCustodyClaim: BillingChildActivityCustodyClaimSchema,
    updatedAt: ParentTimestampSchema,
  })
);

export type BillingEntitlementRuntimeSnapshotConsumption = Infer<
  typeof BillingEntitlementRuntimeSnapshotConsumptionSchema
>;
export type BillingEntitlementRuntimeDeviceLimitConsumption = Infer<
  typeof BillingEntitlementRuntimeDeviceLimitConsumptionSchema
>;
export type BillingEntitlementRuntimeFailureConsumption = Infer<
  typeof BillingEntitlementRuntimeFailureConsumptionSchema
>;
export type BillingEntitlementRuntimeProof = Infer<typeof BillingEntitlementRuntimeProofSchema>;

export const BillingEntitlementRuntimeProofReadModel = BillingEntitlementRuntimeProofSchema.parse(
  GeneratedBillingEntitlementRuntimeProofReadModel
);

export {
  BillingEntitlementRuntimeAuditReferenceSchema,
  BillingEntitlementRuntimeBoundaryIdSchema,
  BillingEntitlementRuntimeChildCustodyClaimSchema,
  BillingEntitlementRuntimeConsumptionStateSchema,
  BillingEntitlementRuntimeNonClaimSchema,
  BillingEntitlementRuntimeOperationSchema,
  BillingEntitlementRuntimePortalUiClaimSchema,
  BillingEntitlementRuntimeProductionBillingClaimSchema,
  BillingEntitlementRuntimeProviderContactClaimSchema,
  BillingEntitlementRuntimeProviderExecutionClaimSchema,
  BillingEntitlementRuntimeRefundCreditClaimSchema,
  BillingEntitlementRuntimeSchemaVersionSchema,
  BillingEntitlementRuntimeSnapshotStateSchema,
  BillingEntitlementRuntimeSourceSchema,
};

export type BillingEntitlementRuntimeConsumptionState = GeneratedBillingEntitlementRuntimeConsumptionState;
export type BillingEntitlementRuntimeNonClaim = GeneratedBillingEntitlementRuntimeNonClaim;
export type BillingEntitlementRuntimeOperation = GeneratedBillingEntitlementRuntimeOperation;
export type BillingEntitlementRuntimeSnapshotState = GeneratedBillingEntitlementRuntimeSnapshotState;
