import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ProductionReleasePublicRuntimeHandoffSchemaVersionSchema = withParser(
  Schema.Literal('production-release-public-runtime-handoff-proof')
);

export const ProductionReleasePublicRuntimeSurfaceSchema = withParser(
  Schema.Literal(
    'public-download',
    'release-status',
    'update-status',
    'account-status',
    'subscription-status',
    'support-status'
  )
);

export const ProductionReleasePublicRuntimeStateSchema = withParser(
  Schema.Literal(
    'route-contract-only',
    'adapter-boundary-ready',
    'backend-required',
    'manual-required',
    'not-implemented',
    'production-promotion-required',
    'implemented'
  )
);

export const ProductionReleasePublicRuntimeTargetSchema = withParser(
  Schema.Literal(
    'family-public-site-route',
    'download-manifest-route',
    'release-status-route',
    'update-status-route',
    'account-status-route',
    'subscription-status-route',
    'support-status-route'
  )
);

export const ProductionReleasePublicRuntimeSourceProofSchema = withParser(
  Schema.Literal(
    'production-release-public-status-proof',
    'billing-account-endpoint-contract-proof',
    'v8-updater-rollback-runbook-proof',
    'support-bundle-redaction-proof',
    'support-incident-workflow-proof'
  )
);

export const ProductionReleasePublicRuntimeAdapterSchema = withParser(
  Schema.Literal(
    'public-website-runtime',
    'download-status-backend',
    'release-publishing-pipeline',
    'updater-status-runtime',
    'account-backend',
    'billing-provider-runtime',
    'support-backend-upload'
  )
);

export const ProductionReleasePublicRuntimeDataClassSchema = withParser(
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

export const ProductionReleasePublicRuntimeNonClaimSchema = withParser(
  Schema.Literal(
    'no-public-website-runtime',
    'no-account-backend-runtime',
    'no-billing-provider-runtime',
    'no-production-publishing',
    'no-signing-store-proof',
    'no-updater-execution',
    'no-support-backend-upload',
    'no-child-activity-custody',
    'no-real-device-store-proof'
  )
);

export const ProductionReleasePublicRuntimeExecutionClaimSchema = withParser(
  Schema.Literal('not-executed', 'manual-required', 'promotion-required', 'executed')
);

export const PublicRuntimeHandoffReferenceSchema = brandedNonEmptyStringSchema('ProductionReleasePublicRuntimeHandoffReference');
export const PublicRuntimeHandoffRequirementSchema = brandedNonEmptyStringSchema('ProductionReleasePublicRuntimeHandoffRequirement');

export const ForbiddenPublicRuntimeDataClasses = [
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

export const RequiredPublicRuntimeSurfaces = [
  'public-download',
  'release-status',
  'update-status',
  'account-status',
  'subscription-status',
  'support-status',
] as const;

export const RequiredPublicRuntimeAdapters = [
  'public-website-runtime',
  'download-status-backend',
  'release-publishing-pipeline',
  'updater-status-runtime',
  'account-backend',
  'billing-provider-runtime',
  'support-backend-upload',
] as const;

export const RequiredPublicRuntimeNonClaims = [
  'no-public-website-runtime',
  'no-account-backend-runtime',
  'no-billing-provider-runtime',
  'no-production-publishing',
  'no-signing-store-proof',
  'no-updater-execution',
  'no-support-backend-upload',
  'no-child-activity-custody',
  'no-real-device-store-proof',
] as const;

