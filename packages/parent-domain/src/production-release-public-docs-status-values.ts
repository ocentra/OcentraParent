import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyPublicDocsStatusText = Schema.String.pipe(Schema.minLength(1));

export const ProductionReleasePublicDocsStatusSchemaVersionSchema = withParser(
  Schema.Literal('production-release-public-docs-status-proof')
);

export const ProductionReleasePublicDocsStatusDocumentSchema = withParser(
  Schema.Literal(
    'privacy-policy',
    'retention-policy',
    'export-delete-process',
    'support-runbook',
    'incident-status-disclosure',
    'legal-disclosure'
  )
);

export const ProductionReleasePublicDocsStatusStateSchema = withParser(
  Schema.Literal('source-contract-ready', 'manual-required', 'not-implemented', 'published', 'implemented')
);

export const ProductionReleasePublicDocsStatusSourceSchema = withParser(
  Schema.Literal(
    'documentation-expectation',
    'data-custody-expectation',
    'release-installer-expectation',
    'support-incident-workflow-proof',
    'production-release-public-runtime-handoff-proof'
  )
);

export const ProductionReleasePublicDocsStatusAudienceSchema = withParser(
  Schema.Literal('public-family', 'support-operator', 'legal-review')
);

export const ProductionReleasePublicDocsStatusDataClassSchema = withParser(
  Schema.Literal(
    'public-policy-text',
    'data-custody-summary',
    'retention-window-summary',
    'export-delete-process-summary',
    'support-runbook-status',
    'incident-status',
    'legal-disclosure-status',
    'release-status',
    'redaction-policy-summary',
    'manual-proof-reference',
    'contact-channel-status',
    'child-activity-evidence',
    'browser-url-history',
    'screenshots',
    'journals',
    'sqlite-snapshots',
    'private-paths',
    'provider-secrets',
    'raw-support-bundle',
    'parent-rules-source-of-truth',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript'
  )
);

export const ProductionReleasePublicDocsStatusNonClaimSchema = withParser(
  Schema.Literal(
    'no-public-website-publication',
    'no-support-backend-upload',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-remote-support-session',
    'no-production-sla',
    'no-child-activity-custody',
    'no-legal-disclosure-execution'
  )
);

export const ProductionReleasePublicDocsStatusExecutionClaimSchema = withParser(
  Schema.Literal('not-executed', 'manual-required', 'not-implemented', 'executed')
);

export const PublicDocsStatusReferenceSchema = NonEmptyPublicDocsStatusText.pipe(
  Schema.brand('ProductionReleasePublicDocsStatusReference')
);
export const PublicDocsStatusRequirementSchema = NonEmptyPublicDocsStatusText.pipe(
  Schema.brand('ProductionReleasePublicDocsStatusRequirement')
);

export const ForbiddenPublicDocsStatusDataClasses = [
  'child-activity-evidence',
  'browser-url-history',
  'screenshots',
  'journals',
  'sqlite-snapshots',
  'private-paths',
  'provider-secrets',
  'raw-support-bundle',
  'parent-rules-source-of-truth',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
] as const;

export const RequiredPublicDocsStatusDocuments = [
  'privacy-policy',
  'retention-policy',
  'export-delete-process',
  'support-runbook',
  'incident-status-disclosure',
  'legal-disclosure',
] as const;

export const RequiredPublicDocsStatusNonClaims = [
  'no-public-website-publication',
  'no-support-backend-upload',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-remote-support-session',
  'no-production-sla',
  'no-child-activity-custody',
  'no-legal-disclosure-execution',
] as const;
