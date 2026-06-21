import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportPublicationStatusFreshnessSchemaVersionSchema = withParser(
  Schema.Literal('production-support-publication-status-freshness-proof')
);

export const ProductionSupportPublicationStatusFreshnessSurfaceSchema = withParser(
  Schema.Literal(
    'support-runbook-publication-freshness',
    'incident-status-publication-freshness',
    'public-support-contact-publication-freshness',
    'support-backend-upload-publication-freshness',
    'privacy-legal-publication-freshness',
    'account-billing-support-publication-freshness'
  )
);

export const ProductionSupportPublicationStatusFreshnessStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'freshness-policy-ready',
    'publication-required',
    'manual-required',
    'backend-required',
    'not-implemented',
    'executed',
    'implemented'
  )
);

export const ProductionSupportPublicationStatusFreshnessSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-publication-workflow-proof',
    'public-support-contact-status-proof',
    'production-release-public-docs-freshness-proof',
    'production-release-public-status-freshness-proof',
    'documentation-expectation'
  )
);

export const ProductionSupportPublicationStatusFreshnessDataClassSchema = withParser(
  Schema.Literal(
    'support-runbook-status',
    'incident-status',
    'public-support-contact-status',
    'support-upload-status-summary',
    'privacy-policy-status',
    'legal-review-status',
    'account-status-summary',
    'billing-support-status',
    'freshness-policy-reference',
    'manual-proof-reference',
    'publication-reference',
    'child-activity-evidence',
    'browser-url-history',
    'screenshots',
    'journals',
    'sqlite-snapshots',
    'private-paths',
    'provider-secrets',
    'raw-support-bundle',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript',
    'parent-rules-source-of-truth'
  )
);

export const ProductionSupportPublicationStatusFreshnessNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-public-runtime',
    'no-support-publication-execution',
    'no-support-backend-upload-execution',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-production-sla',
    'no-child-activity-custody',
    'no-legal-disclosure-execution',
    'no-remote-support-session'
  )
);

export const PublicationStatusFreshnessReferenceSchema = brandedNonEmptyStringSchema('ProductionSupportPublicationStatusFreshnessReference');
export const PublicationStatusFreshnessRequirementSchema = brandedNonEmptyStringSchema('ProductionSupportPublicationStatusFreshnessRequirement');

export const ForbiddenPublicationStatusFreshnessDataClasses = [
  'child-activity-evidence',
  'browser-url-history',
  'screenshots',
  'journals',
  'sqlite-snapshots',
  'private-paths',
  'provider-secrets',
  'raw-support-bundle',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
  'parent-rules-source-of-truth',
] as const;

export const RequiredPublicationStatusFreshnessSurfaces = [
  'support-runbook-publication-freshness',
  'incident-status-publication-freshness',
  'public-support-contact-publication-freshness',
  'support-backend-upload-publication-freshness',
  'privacy-legal-publication-freshness',
  'account-billing-support-publication-freshness',
] as const;

export const RequiredPublicationStatusFreshnessNonClaims = [
  'no-real-public-runtime',
  'no-support-publication-execution',
  'no-support-backend-upload-execution',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-production-sla',
  'no-child-activity-custody',
  'no-legal-disclosure-execution',
  'no-remote-support-session',
] as const;

