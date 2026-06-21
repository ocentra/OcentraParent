import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ProductionReleasePublicStatusFreshnessSchemaVersionSchema = withParser(
  Schema.Literal('production-release-public-status-freshness-proof')
);

export const ProductionReleasePublicStatusFreshnessSurfaceSchema = withParser(
  Schema.Literal(
    'public-download',
    'release-status',
    'update-status',
    'account-status',
    'subscription-status',
    'support-status'
  )
);

export const ProductionReleasePublicStatusFreshnessStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'freshness-policy-ready',
    'manual-required',
    'backend-required',
    'not-implemented',
    'publication-required'
  )
);

export const ProductionReleasePublicStatusFreshnessSignalSchema = withParser(
  Schema.Literal(
    'build-version',
    'download-manifest',
    'release-channel',
    'update-channel',
    'account-snapshot',
    'subscription-snapshot',
    'support-incident-status',
    'support-runbook-status'
  )
);

export const ProductionReleasePublicStatusFreshnessNonClaimSchema = withParser(
  Schema.Literal(
    'no-public-runtime-execution',
    'no-account-backend-runtime',
    'no-billing-provider-runtime',
    'no-production-publishing',
    'no-signing-store-proof',
    'no-updater-execution',
    'no-support-backend-upload',
    'no-production-sla',
    'no-child-activity-custody'
  )
);

export const PublicStatusFreshnessReferenceSchema = brandedNonEmptyStringSchema('ProductionReleasePublicStatusFreshnessReference');

export const PublicStatusFreshnessRequirementSchema = brandedNonEmptyStringSchema('ProductionReleasePublicStatusFreshnessRequirement');

export const RequiredPublicStatusFreshnessSurfaces = [
  'public-download',
  'release-status',
  'update-status',
  'account-status',
  'subscription-status',
  'support-status',
] as const;

export const RequiredPublicStatusFreshnessNonClaims = [
  'no-public-runtime-execution',
  'no-account-backend-runtime',
  'no-billing-provider-runtime',
  'no-production-publishing',
  'no-signing-store-proof',
  'no-updater-execution',
  'no-support-backend-upload',
  'no-production-sla',
  'no-child-activity-custody',
] as const;

