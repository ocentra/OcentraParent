import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ProductionReleasePublicDocsFreshnessSchemaVersionSchema = withParser(
  Schema.Literal('production-release-public-docs-freshness-proof')
);

export const ProductionReleasePublicDocsFreshnessDocumentSchema = withParser(
  Schema.Literal(
    'privacy-policy',
    'retention-policy',
    'export-delete-process',
    'support-runbook',
    'incident-status-disclosure',
    'legal-disclosure'
  )
);

export const ProductionReleasePublicDocsFreshnessStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'freshness-policy-ready',
    'manual-required',
    'not-implemented',
    'publication-required'
  )
);

export const ProductionReleasePublicDocsFreshnessSignalSchema = withParser(
  Schema.Literal(
    'privacy-review-cadence',
    'retention-review-cadence',
    'export-delete-review-cadence',
    'support-runbook-review-cadence',
    'incident-disclosure-review-cadence',
    'legal-disclosure-review-cadence'
  )
);

export const ProductionReleasePublicDocsFreshnessNonClaimSchema = withParser(
  Schema.Literal(
    'no-public-publication-execution',
    'no-legal-disclosure-execution',
    'no-support-backend-upload',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-remote-support-session',
    'no-production-sla',
    'no-child-activity-custody'
  )
);

export const PublicDocsFreshnessReferenceSchema = brandedNonEmptyStringSchema('ProductionReleasePublicDocsFreshnessReference');

export const PublicDocsFreshnessRequirementSchema = brandedNonEmptyStringSchema('ProductionReleasePublicDocsFreshnessRequirement');

export const RequiredPublicDocsFreshnessDocuments = [
  'privacy-policy',
  'retention-policy',
  'export-delete-process',
  'support-runbook',
  'incident-status-disclosure',
  'legal-disclosure',
] as const;

export const RequiredPublicDocsFreshnessNonClaims = [
  'no-public-publication-execution',
  'no-legal-disclosure-execution',
  'no-support-backend-upload',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-remote-support-session',
  'no-production-sla',
  'no-child-activity-custody',
] as const;

