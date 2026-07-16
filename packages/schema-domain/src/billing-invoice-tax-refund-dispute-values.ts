import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  GeneratedBillingCollectionRecoveryStates,
  GeneratedBillingDisputeLifecycleStates,
  GeneratedBillingEntitlementSideEffects,
  GeneratedBillingInvoiceChildActivityCustodyClaims,
  GeneratedBillingInvoiceCurrencyCodes,
  GeneratedBillingInvoiceHostedSurfaceClaims,
  GeneratedBillingInvoiceLifecycleNonClaims,
  GeneratedBillingInvoiceManualSupportClaims,
  GeneratedBillingInvoiceTaxRefundDisputeSchemaVersion,
  GeneratedBillingInvoiceVisibilityStates,
  GeneratedBillingRefundLifecycleStates,
  GeneratedBillingSupportAuditStates,
  GeneratedBillingTaxModeDecisions,
  GeneratedBillingTaxRegionStates,
  GeneratedBillingInvoiceProviderModes,
} from './generated-billing-invoice-tax-refund-dispute-values';

export const BillingInvoiceTaxRefundDisputeSchemaVersionSchema = withParser(
  Schema.Literal(GeneratedBillingInvoiceTaxRefundDisputeSchemaVersion)
);

export const BillingInvoiceVisibilityStateSchema = withParser(
  Schema.Literal(...GeneratedBillingInvoiceVisibilityStates)
);

export const BillingInvoiceProviderModeSchema = withParser(Schema.Literal(...GeneratedBillingInvoiceProviderModes));

export const BillingInvoiceCurrencyCodeSchema = withParser(Schema.Literal(...GeneratedBillingInvoiceCurrencyCodes));

export const BillingTaxModeDecisionSchema = withParser(Schema.Literal(...GeneratedBillingTaxModeDecisions));

export const BillingTaxRegionStateSchema = withParser(Schema.Literal(...GeneratedBillingTaxRegionStates));

export const BillingRefundLifecycleStateSchema = withParser(Schema.Literal(...GeneratedBillingRefundLifecycleStates));

export const BillingDisputeLifecycleStateSchema = withParser(Schema.Literal(...GeneratedBillingDisputeLifecycleStates));

export const BillingCollectionRecoveryStateSchema = withParser(
  Schema.Literal(...GeneratedBillingCollectionRecoveryStates)
);

export const BillingEntitlementSideEffectSchema = withParser(Schema.Literal(...GeneratedBillingEntitlementSideEffects));

export const BillingSupportAuditStateSchema = withParser(Schema.Literal(...GeneratedBillingSupportAuditStates));

export const BillingInvoiceLifecycleBoundaryIdSchema = brandedNonEmptyStringSchema('BillingInvoiceLifecycleBoundaryId');

export const BillingInvoiceLifecycleAuditReferenceSchema = brandedNonEmptyStringSchema(
  'BillingInvoiceLifecycleAuditReference'
);

export const BillingInvoiceNumberSchema = brandedNonEmptyStringSchema('BillingInvoiceNumber');

export const BillingInvoiceLifecycleNonClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingInvoiceLifecycleNonClaims)
);

export const BillingInvoiceHostedSurfaceClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingInvoiceHostedSurfaceClaims)
);

export const BillingInvoiceManualSupportClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingInvoiceManualSupportClaims)
);

export const BillingInvoiceChildActivityCustodyClaimSchema = withParser(
  Schema.Literal(...GeneratedBillingInvoiceChildActivityCustodyClaims)
);

export type BillingInvoiceVisibilityState = Infer<typeof BillingInvoiceVisibilityStateSchema>;
export type BillingInvoiceProviderMode = Infer<typeof BillingInvoiceProviderModeSchema>;
export type BillingInvoiceCurrencyCode = Infer<typeof BillingInvoiceCurrencyCodeSchema>;
export type BillingTaxModeDecision = Infer<typeof BillingTaxModeDecisionSchema>;
export type BillingTaxRegionState = Infer<typeof BillingTaxRegionStateSchema>;
export type BillingRefundLifecycleState = Infer<typeof BillingRefundLifecycleStateSchema>;
export type BillingDisputeLifecycleState = Infer<typeof BillingDisputeLifecycleStateSchema>;
export type BillingCollectionRecoveryState = Infer<typeof BillingCollectionRecoveryStateSchema>;
export type BillingEntitlementSideEffect = Infer<typeof BillingEntitlementSideEffectSchema>;
export type BillingInvoiceLifecycleNonClaim = Infer<typeof BillingInvoiceLifecycleNonClaimSchema>;
export type BillingInvoiceNumber = Infer<typeof BillingInvoiceNumberSchema>;
