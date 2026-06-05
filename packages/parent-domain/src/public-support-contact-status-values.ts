import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyPublicSupportContactStatusText = Schema.String.pipe(Schema.minLength(1));

export const PublicSupportContactStatusSchemaVersionSchema = withParser(
  Schema.Literal('public-support-contact-status-proof')
);

export const PublicSupportContactStatusSurfaceSchema = withParser(
  Schema.Literal(
    'public-support-contact',
    'support-status-page-contact',
    'support-runbook-contact',
    'incident-status-contact',
    'backend-upload-support-contact',
    'billing-support-contact'
  )
);

export const PublicSupportContactStatusStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'route-contract-only',
    'publication-required',
    'legal-review-required',
    'backend-required',
    'manual-required',
    'not-implemented',
    'implemented',
    'executed'
  )
);

export const PublicSupportContactStatusSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-publication-workflow-proof',
    'production-release-public-runtime-handoff-proof',
    'production-release-public-docs-status-proof',
    'production-support-backend-upload-status-proof',
    'support-incident-workflow-proof',
    'billing-support-admin-boundary-proof',
    'documentation-expectation'
  )
);

export const PublicSupportContactStatusDataClassSchema = withParser(
  Schema.Literal(
    'contact-channel-status',
    'support-runbook-status',
    'incident-status',
    'support-upload-status-summary',
    'billing-support-status',
    'account-status',
    'manual-proof-reference',
    'publication-reference',
    'legal-review-status',
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

export const PublicSupportContactStatusNonClaimSchema = withParser(
  Schema.Literal(
    'no-public-runtime-execution',
    'no-support-backend-upload-execution',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-remote-support-session',
    'no-production-sla',
    'no-child-activity-custody',
    'no-provider-secrets',
    'no-legal-disclosure-execution'
  )
);

export const PublicSupportContactStatusReferenceSchema = NonEmptyPublicSupportContactStatusText.pipe(
  Schema.brand('PublicSupportContactStatusReference')
);
export const PublicSupportContactStatusRequirementSchema = NonEmptyPublicSupportContactStatusText.pipe(
  Schema.brand('PublicSupportContactStatusRequirement')
);

export const ForbiddenPublicSupportContactStatusDataClasses = [
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

export const RequiredPublicSupportContactStatusSurfaces = [
  'public-support-contact',
  'support-status-page-contact',
  'support-runbook-contact',
  'incident-status-contact',
  'backend-upload-support-contact',
  'billing-support-contact',
] as const;

export const RequiredPublicSupportContactStatusNonClaims = [
  'no-public-runtime-execution',
  'no-support-backend-upload-execution',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-remote-support-session',
  'no-production-sla',
  'no-child-activity-custody',
  'no-provider-secrets',
  'no-legal-disclosure-execution',
] as const;
