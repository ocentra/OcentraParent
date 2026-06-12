import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyPublicStatusText = Schema.String.pipe(Schema.minLength(1));

export const ProductionReleasePublicStatusSchemaVersionSchema = withParser(
  Schema.Literal('production-release-public-status-proof')
);
export const ProductionReleasePublicSurfaceSchema = withParser(
  Schema.Literal(
    'public-download',
    'release-status',
    'update-status',
    'account-status',
    'subscription-status',
    'support-status'
  )
);
export const ProductionReleasePublicSurfaceStateSchema = withParser(
  Schema.Literal(
    'route-contract-only',
    'manual-required',
    'not-implemented',
    'backend-required',
    'production-promotion-required',
    'implemented'
  )
);
export const ProductionReleasePublicSourceSchema = withParser(
  Schema.Literal(
    'family-ocentra-public-surface',
    'billing-account-endpoint-contract',
    'release-support-proof',
    'support-redaction-proof'
  )
);
export const ProductionReleasePublicDataClassSchema = withParser(
  Schema.Literal(
    'release-version',
    'commit',
    'platform',
    'package-artifact',
    'download-status',
    'update-status',
    'account-status',
    'subscription-status',
    'entitlement-summary',
    'support-runbook-status',
    'incident-status',
    'child-activity-evidence',
    'browser-url-history',
    'screenshots',
    'journals',
    'sqlite-snapshots',
    'private-paths',
    'provider-secrets',
    'raw-support-bundle',
    'parent-rules-source-of-truth'
  )
);
export const ProductionReleasePublicNonClaimSchema = withParser(
  Schema.Literal(
    'no-public-website-runtime',
    'no-account-backend',
    'no-billing-provider-runtime',
    'no-production-publishing',
    'no-signing-store-proof',
    'no-updater-execution',
    'no-support-backend-upload',
    'no-child-activity-custody'
  )
);

export const PublicStatusLabelSchema = NonEmptyPublicStatusText.pipe(
  Schema.brand('ProductionReleasePublicStatusLabel')
);
export const PublicStatusRequirementSchema = NonEmptyPublicStatusText.pipe(
  Schema.brand('ProductionReleasePublicStatusRequirement')
);
export const PublicStatusReferenceSchema = NonEmptyPublicStatusText.pipe(
  Schema.brand('ProductionReleasePublicStatusReference')
);

export const ForbiddenPublicDataClasses = [
  'child-activity-evidence',
  'browser-url-history',
  'screenshots',
  'journals',
  'sqlite-snapshots',
  'private-paths',
  'provider-secrets',
  'raw-support-bundle',
  'parent-rules-source-of-truth',
] as const;

export const RequiredPublicSurfaces = [
  'public-download',
  'release-status',
  'update-status',
  'account-status',
  'subscription-status',
  'support-status',
] as const;

export const RequiredNonClaims = [
  'no-public-website-runtime',
  'no-account-backend',
  'no-billing-provider-runtime',
  'no-production-publishing',
  'no-signing-store-proof',
  'no-updater-execution',
  'no-support-backend-upload',
  'no-child-activity-custody',
] as const;
