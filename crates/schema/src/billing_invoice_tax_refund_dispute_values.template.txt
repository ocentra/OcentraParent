/* generated from crates/schema/src/billing_invoice_tax_refund_dispute_values_ts.rs */

export const GeneratedBillingInvoiceTaxRefundDisputeSchemaVersion = 'billing-invoice-tax-refund-dispute' as const;

export const GeneratedBillingInvoiceVisibilityStates = [
  'customer-portal-hosted',
  'download-link-issued',
  'manual-support-required',
] as const;

export const GeneratedBillingInvoiceProviderModes = ['stripe-hosted', 'manual-invoice'] as const;

export const GeneratedBillingInvoiceCurrencyCodes = ['USD'] as const;

export const GeneratedBillingTaxModeDecisions = [
  'stripe-automatic-tax',
  'merchant-configured-tax',
  'manual-support-required',
] as const;

export const GeneratedBillingTaxRegionStates = ['launch-supported', 'manual-support-required'] as const;

export const GeneratedBillingRefundLifecycleStates = [
  'none',
  'refund-requested',
  'refund-issued',
  'refund-settled',
  'refund-denied',
] as const;

export const GeneratedBillingDisputeLifecycleStates = [
  'none',
  'dispute-opened',
  'dispute-won',
  'dispute-lost',
] as const;

export const GeneratedBillingCollectionRecoveryStates = [
  'active',
  'trialing',
  'past-due',
  'grace',
  'cancelled',
  'unpaid',
  'support-required',
] as const;

export const GeneratedBillingEntitlementSideEffects = [
  'retain-paid-access',
  'grace-paid-access',
  'limit-paid-access',
  'revoke-paid-access',
  'manual-review-required',
] as const;

export const GeneratedBillingSupportAuditStates = ['audited'] as const;

export const GeneratedBillingInvoiceLifecycleNonClaims = [
  'no-invoice-pdf-custody',
  'no-self-service-refund',
  'no-self-service-dispute',
  'no-child-activity-custody',
] as const;

export const GeneratedBillingInvoiceHostedSurfaceClaims = ['customer-portal-hosted-only'] as const;

export const GeneratedBillingInvoiceManualSupportClaims = ['audited-required'] as const;

export const GeneratedBillingInvoiceChildActivityCustodyClaims = ['not-included'] as const;
