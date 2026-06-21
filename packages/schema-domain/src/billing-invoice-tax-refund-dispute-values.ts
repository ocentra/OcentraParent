import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from './effect';

export const BillingInvoiceTaxRefundDisputeSchemaVersionSchema = withParser(
  Schema.Literal('billing-invoice-tax-refund-dispute')
);

export const BillingInvoiceVisibilityStateSchema = withParser(
  Schema.Literal('customer-portal-hosted', 'download-link-issued', 'manual-support-required')
);

export const BillingInvoiceProviderModeSchema = withParser(
  Schema.Literal('stripe-hosted', 'manual-invoice')
);

export const BillingInvoiceCurrencyCodeSchema = withParser(Schema.Literal('USD'));

export const BillingTaxModeDecisionSchema = withParser(
  Schema.Literal('stripe-automatic-tax', 'merchant-configured-tax', 'manual-support-required')
);

export const BillingTaxRegionStateSchema = withParser(
  Schema.Literal('launch-supported', 'manual-support-required')
);

export const BillingRefundLifecycleStateSchema = withParser(
  Schema.Literal('none', 'refund-requested', 'refund-issued', 'refund-settled', 'refund-denied')
);

export const BillingDisputeLifecycleStateSchema = withParser(
  Schema.Literal('none', 'dispute-opened', 'dispute-won', 'dispute-lost')
);

export const BillingCollectionRecoveryStateSchema = withParser(
  Schema.Literal('active', 'trialing', 'past-due', 'grace', 'cancelled', 'unpaid', 'support-required')
);

export const BillingEntitlementSideEffectSchema = withParser(
  Schema.Literal(
    'retain-paid-access',
    'grace-paid-access',
    'limit-paid-access',
    'revoke-paid-access',
    'manual-review-required'
  )
);

export const BillingSupportAuditStateSchema = withParser(Schema.Literal('audited'));

export const BillingInvoiceLifecycleBoundaryIdSchema = brandedNonEmptyStringSchema(
  'BillingInvoiceLifecycleBoundaryId'
);

export const BillingInvoiceLifecycleAuditReferenceSchema = brandedNonEmptyStringSchema(
  'BillingInvoiceLifecycleAuditReference'
);

export const BillingInvoiceNumberSchema = brandedNonEmptyStringSchema(
  'BillingInvoiceNumber'
);

export const BillingInvoiceLifecycleNonClaimSchema = withParser(
  Schema.Literal(
    'no-invoice-pdf-custody',
    'no-self-service-refund',
    'no-self-service-dispute',
    'no-child-activity-custody'
  )
);

export const BillingInvoiceHostedSurfaceClaimSchema = withParser(
  Schema.Literal('customer-portal-hosted-only')
);

export const BillingInvoiceManualSupportClaimSchema = withParser(
  Schema.Literal('audited-required')
);

export const BillingInvoiceChildActivityCustodyClaimSchema = withParser(
  Schema.Literal('not-included')
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
