import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyProductionSupportAccountSlaStatusText = Schema.String.pipe(Schema.minLength(1));

export const ProductionSupportAccountSlaStatusSchemaVersionSchema = withParser(
  Schema.Literal('production-support-account-sla-status-proof')
);

export const ProductionSupportAccountSlaStatusSurfaceSchema = withParser(
  Schema.Literal(
    'account-lookup-request-status',
    'account-lookup-result-boundary',
    'billing-provider-contact-status',
    'remote-support-request-status',
    'remote-support-session-boundary',
    'production-sla-status'
  )
);

export const ProductionSupportAccountSlaStatusStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'manual-required',
    'provider-required',
    'parent-visible',
    'not-implemented',
    'implemented',
    'executed'
  )
);

export const ProductionSupportAccountSlaStatusSourceProofSchema = withParser(
  Schema.Literal(
    'production-incident-support-status-proof',
    'billing-support-admin-status-proof',
    'billing-entitlement-runtime-proof',
    'public-support-contact-status-proof',
    'production-support-case-resolution-status-proof',
    'release-installer-expectation',
    'billing-expectation',
    'data-custody-expectation'
  )
);

export const ProductionSupportAccountSlaStatusDataClassSchema = withParser(
  Schema.Literal(
    'support-case-status-ref',
    'account-status-ref',
    'subscription-status-ref',
    'billing-failure-state-ref',
    'parent-consent-reference',
    'redaction-audit-reference',
    'manual-proof-reference',
    'support-runbook-reference',
    'public-status-reference',
    'child-activity-evidence',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript',
    'provider-secret',
    'support-backend-payload',
    'sla-commitment',
    'payment-provider-token',
    'raw-support-bundle'
  )
);

export const ProductionSupportAccountSlaStatusNonClaimSchema = withParser(
  Schema.Literal(
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-remote-support-session',
    'no-production-sla',
    'no-provider-secrets',
    'no-support-backend-upload-execution',
    'no-family-ocentra-runtime',
    'no-child-activity-custody'
  )
);

export const ProductionSupportAccountSlaStatusReferenceSchema = NonEmptyProductionSupportAccountSlaStatusText.pipe(
  Schema.brand('ProductionSupportAccountSlaStatusReference')
);
export const ProductionSupportAccountSlaStatusRequirementSchema = NonEmptyProductionSupportAccountSlaStatusText.pipe(
  Schema.brand('ProductionSupportAccountSlaStatusRequirement')
);

export const ForbiddenProductionSupportAccountSlaStatusDataClasses = [
  'child-activity-evidence',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
  'provider-secret',
  'support-backend-payload',
  'sla-commitment',
  'payment-provider-token',
  'raw-support-bundle',
] as const;

export const RequiredProductionSupportAccountSlaStatusSurfaces = [
  'account-lookup-request-status',
  'account-lookup-result-boundary',
  'billing-provider-contact-status',
  'remote-support-request-status',
  'remote-support-session-boundary',
  'production-sla-status',
] as const;

export const RequiredProductionSupportAccountSlaStatusNonClaims = [
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-remote-support-session',
  'no-production-sla',
  'no-provider-secrets',
  'no-support-backend-upload-execution',
  'no-family-ocentra-runtime',
  'no-child-activity-custody',
] as const;
