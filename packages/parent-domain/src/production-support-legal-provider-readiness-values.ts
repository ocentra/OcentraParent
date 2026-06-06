import { Schema } from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportLegalProviderReadinessSchemaVersion =
  'production-support-legal-provider-readiness-proof' as const;

export const ProductionSupportLegalProviderReadinessSchemaVersionSchema = Schema.Literal(
  ProductionSupportLegalProviderReadinessSchemaVersion
);

export const ProductionSupportLegalProviderReadinessSurfaces = [
  'privacy-legal-review-readiness',
  'data-export-delete-runtime-readiness',
  'provider-secret-custody-boundary',
  'billing-provider-contact-readiness',
  'remote-support-legal-session-boundary',
  'production-sla-legal-boundary',
] as const;

export const RequiredProductionSupportLegalProviderReadinessSurfaces = ProductionSupportLegalProviderReadinessSurfaces;

export const ProductionSupportLegalProviderReadinessSurfaceSchema = Schema.Literal(
  ...ProductionSupportLegalProviderReadinessSurfaces
);

export const ProductionSupportLegalProviderReadinessStates = [
  'source-contract-ready',
  'manual-required',
  'legal-review-required',
  'provider-required',
  'backend-required',
  'not-implemented',
  'implemented',
  'executed',
] as const;

export const ProductionSupportLegalProviderReadinessStateSchema = Schema.Literal(
  ...ProductionSupportLegalProviderReadinessStates
);

export const ProductionSupportLegalProviderReadinessSourceProofs = [
  'production-release-public-docs-status-proof',
  'production-release-public-docs-freshness-proof',
  'production-incident-support-status-proof',
  'production-support-account-sla-status-proof',
  'billing-support-admin-status-proof',
  'production-support-case-resolution-status-proof',
  'release-installer-expectation',
  'billing-expectation',
  'data-custody-expectation',
  'documentation-expectation',
] as const;

export const ProductionSupportLegalProviderReadinessSourceProofSchema = Schema.Literal(
  ...ProductionSupportLegalProviderReadinessSourceProofs
);

export const ProductionSupportLegalProviderReadinessReferences = [
  'privacy-policy-status-ref',
  'legal-disclosure-status-ref',
  'export-delete-status-ref',
  'support-case-status-ref',
  'billing-status-ref',
  'redaction-audit-reference',
  'provider-boundary-reference',
  'support-runbook-reference',
  'manual-proof-reference',
  'data-custody-reference',
] as const;

export const ProductionSupportLegalProviderReadinessReferenceSchema = Schema.Literal(
  ...ProductionSupportLegalProviderReadinessReferences
);

export const ProductionSupportLegalProviderReadinessDataClasses = [
  'public-policy-status',
  'legal-disclosure-status',
  'export-delete-status',
  'support-case-status',
  'billing-status',
  'redaction-audit-status',
  'manual-proof-status',
  'provider-boundary-status',
  'raw-child-activity',
  'provider-secret',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-transcript',
  'production-sla-commitment',
  'raw-support-bundle-payload',
  'default-ocentra-hosted-family-data',
  'payment-provider-token',
  'parent-rule',
] as const;

export const ProductionSupportLegalProviderReadinessDataClassSchema = Schema.Literal(
  ...ProductionSupportLegalProviderReadinessDataClasses
);

export const ForbiddenProductionSupportLegalProviderReadinessDataClasses = [
  'raw-child-activity',
  'provider-secret',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-transcript',
  'production-sla-commitment',
  'raw-support-bundle-payload',
  'default-ocentra-hosted-family-data',
  'payment-provider-token',
  'parent-rule',
] as const;

export const RequiredProductionSupportLegalProviderReadinessNonClaims = [
  'no-legal-disclosure-execution',
  'no-data-export-delete-runtime',
  'no-provider-secret-custody',
  'no-billing-provider-contact-execution',
  'no-account-lookup-execution',
  'no-remote-support-session',
  'no-production-sla',
  'no-support-backend-upload-execution',
  'no-public-runtime-execution',
  'no-child-activity-custody',
  'no-default-ocentra-hosted-family-data',
] as const;

export const ProductionSupportLegalProviderReadinessNonClaimSchema = Schema.Literal(
  ...RequiredProductionSupportLegalProviderReadinessNonClaims
);

export const ProductionSupportLegalProviderReadinessRequirementSchema = Schema.TemplateLiteral(
  Schema.String,
  '-requires-legal-provider-runtime-proof'
);
