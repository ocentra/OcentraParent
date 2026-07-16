import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  GeneratedBillingEntitlementRuntimeChildCustodyClaims,
  GeneratedBillingEntitlementRuntimeConsumptionStates,
  GeneratedBillingEntitlementRuntimeNonClaims,
  GeneratedBillingEntitlementRuntimeOperations,
  GeneratedBillingEntitlementRuntimePortalUiClaims,
  GeneratedBillingEntitlementRuntimeProductionBillingClaims,
  GeneratedBillingEntitlementRuntimeProviderContactClaims,
  GeneratedBillingEntitlementRuntimeProviderExecutionClaims,
  GeneratedBillingEntitlementRuntimeRefundCreditClaims,
  GeneratedBillingEntitlementRuntimeSchemaVersion,
  GeneratedBillingEntitlementRuntimeSnapshotStates,
  GeneratedBillingEntitlementRuntimeSources,
} from './generated-billing-entitlement-runtime-proof-values';

export const BillingEntitlementRuntimeSchemaVersionSchema = withParser(
  Schema.Literal(GeneratedBillingEntitlementRuntimeSchemaVersion)
);

export const BillingEntitlementRuntimeSnapshotStateSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeSnapshotStates)
);

export const BillingEntitlementRuntimeSourceSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeSources)
);

export const BillingEntitlementRuntimeOperationSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeOperations)
);

export const BillingEntitlementRuntimeConsumptionStateSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeConsumptionStates)
);

export const BillingEntitlementRuntimeProviderExecutionClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeProviderExecutionClaims)
);
export const BillingEntitlementRuntimeProviderContactClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeProviderContactClaims)
);
export const BillingEntitlementRuntimeRefundCreditClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeRefundCreditClaims)
);
export const BillingEntitlementRuntimeProductionBillingClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeProductionBillingClaims)
);
export const BillingEntitlementRuntimePortalUiClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimePortalUiClaims)
);
export const BillingEntitlementRuntimeChildCustodyClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeChildCustodyClaims)
);

export const BillingEntitlementRuntimeNonClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingEntitlementRuntimeNonClaims)
);

export const BillingEntitlementRuntimeBoundaryIdSchema = brandedNonEmptyStringSchema(
  'BillingEntitlementRuntimeBoundaryId'
);

export const BillingEntitlementRuntimeAuditReferenceSchema = brandedNonEmptyStringSchema(
  'BillingEntitlementRuntimeAuditReference'
);

export type BillingEntitlementRuntimeSnapshotState = Infer<typeof BillingEntitlementRuntimeSnapshotStateSchema>;
export type BillingEntitlementRuntimeOperation = Infer<typeof BillingEntitlementRuntimeOperationSchema>;
export type BillingEntitlementRuntimeConsumptionState = Infer<typeof BillingEntitlementRuntimeConsumptionStateSchema>;
export type BillingEntitlementRuntimeNonClaim = Infer<typeof BillingEntitlementRuntimeNonClaimSchema>;
