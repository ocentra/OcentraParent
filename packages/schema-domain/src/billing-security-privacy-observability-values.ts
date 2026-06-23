import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';

export const BillingSecurityPrivacyObservabilitySchemaVersionSchema = withParser(
  Schema.Literal('billing-security-privacy-observability-proof')
);
export const BillingMetadataSurfaceSchema = withParser(
  Schema.Literal('checkout-session', 'billing-portal-session', 'provider-webhook-event', 'support-audit-export')
);
export const BillingMetadataAllowlistedFieldSchema = withParser(
  Schema.Literal(
    'parent-account-ref',
    'family-ref',
    'plan-ref',
    'price-ref',
    'subscription-ref',
    'checkout-request-ref',
    'entitlement-snapshot-ref',
    'billing-failure-ref'
  )
);
export const BillingForbiddenMetadataDataClassSchema = withParser(
  Schema.Literal(
    'child-name',
    'child-activity',
    'location-history',
    'geofence-state',
    'browser-history',
    'app-activity',
    'network-history',
    'screenshot',
    'screen-analysis',
    'policy-detail',
    'ai-safety-analysis'
  )
);
export const BillingLogSignalSchema = withParser(
  Schema.Literal(
    'checkout-request',
    'portal-session',
    'provider-webhook',
    'billing-support-audit',
    'payment-drift-reconciliation'
  )
);
export const BillingLogRedactionStateSchema = withParser(
  Schema.Literal('redacted-identifiers-only', 'hashed-or-truncated-identifiers')
);
export const BillingAbuseProtectedOperationSchema = withParser(
  Schema.Literal('checkout-session-create', 'billing-portal-session-create', 'provider-webhook-ingest')
);
export const BillingBotProtectionModeSchema = withParser(
  Schema.Literal('turnstile-enforced', 'trusted-authenticated-session', 'not-applicable')
);
export const BillingAlertKindSchema = withParser(
  Schema.Literal('webhook-failure', 'payment-drift', 'checkout-abuse', 'fraud-signal', 'secret-exposure')
);
export const BillingSecretScanStateSchema = withParser(Schema.Literal('clean', 'manual-review-required'));
export const BillingSecurityPrivacyNonClaimSchema = withParser(
  Schema.Literal(
    'no-child-data-in-metadata',
    'no-raw-payment-identifiers-in-logs',
    'no-provider-secret-logs',
    'no-child-activity-custody',
    'no-pci-pan-custody'
  )
);
export const BillingMetadataAllowlistClaimSchema = withParser(Schema.Literal('typed-reference-only'));
export const BillingMetadataDenylistClaimSchema = withParser(Schema.Literal('child-safety-data-excluded'));
export const BillingLogRedactionClaimSchema = withParser(Schema.Literal('redacted-billing-identifiers-only'));

export const BillingSecurityPrivacyBoundaryIdSchema = brandedNonEmptyStringSchema('BillingSecurityPrivacyBoundaryId');
export const BillingSecurityPrivacyAuditReferenceSchema = brandedNonEmptyStringSchema(
  'BillingSecurityPrivacyAuditReference'
);

export type BillingMetadataSurface = Infer<typeof BillingMetadataSurfaceSchema>;
export type BillingMetadataAllowlistedField = Infer<typeof BillingMetadataAllowlistedFieldSchema>;
export type BillingForbiddenMetadataDataClass = Infer<typeof BillingForbiddenMetadataDataClassSchema>;
export type BillingLogSignal = Infer<typeof BillingLogSignalSchema>;
export type BillingAbuseProtectedOperation = Infer<typeof BillingAbuseProtectedOperationSchema>;
export type BillingAlertKind = Infer<typeof BillingAlertKindSchema>;
export type BillingSecurityPrivacyNonClaim = Infer<typeof BillingSecurityPrivacyNonClaimSchema>;
