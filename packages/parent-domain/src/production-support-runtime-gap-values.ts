import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyRuntimeGapText = Schema.String.pipe(Schema.minLength(1));

export const ProductionSupportRuntimeGapSchemaVersionSchema = withParser(
  Schema.Literal('production-support-runtime-gap-proof')
);

export const ProductionSupportRuntimeGapItemSchema = withParser(
  Schema.Literal(
    'public-website-runtime-gap',
    'support-publication-execution-gap',
    'support-backend-upload-execution-gap',
    'account-billing-provider-runtime-gap',
    'legal-export-delete-runtime-gap',
    'remote-support-sla-runtime-gap'
  )
);

export const ProductionSupportRuntimeGapStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'manual-required',
    'backend-required',
    'provider-required',
    'publication-required',
    'legal-review-required',
    'not-implemented',
    'implemented',
    'executed'
  )
);

export const ProductionSupportRuntimeGapSourceProofSchema = withParser(
  Schema.Literal(
    'production-release-public-runtime-handoff-proof',
    'production-release-public-surface-publication-proof',
    'production-support-publication-workflow-proof',
    'production-support-backend-upload-execution-runtime-proof',
    'production-support-account-sla-status-proof',
    'production-incident-support-status-proof',
    'production-release-public-docs-freshness-proof'
  )
);

export const ProductionSupportRuntimeGapDataClassSchema = withParser(
  Schema.Literal(
    'public-route-status',
    'publication-status',
    'support-runbook-status',
    'support-upload-status-summary',
    'account-status-summary',
    'billing-support-status-summary',
    'legal-review-status',
    'export-delete-status-summary',
    'remote-support-status-summary',
    'manual-proof-reference',
    'source-contract-reference',
    'child-activity-evidence',
    'raw-support-bundle',
    'provider-secrets',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript',
    'hosted-family-data',
    'backend-upload-payload',
    'private-paths',
    'screenshots',
    'journals',
    'sqlite-snapshots'
  )
);

export const ProductionSupportRuntimeGapNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-public-runtime',
    'no-support-publication-execution',
    'no-support-backend-upload-execution',
    'no-account-backend-runtime',
    'no-account-lookup-execution',
    'no-billing-provider-runtime',
    'no-billing-provider-contact',
    'no-legal-disclosure-execution',
    'no-export-delete-runtime-execution',
    'no-remote-support-session',
    'no-production-sla',
    'no-child-activity-custody',
    'no-provider-secrets',
    'no-hosted-family-data'
  )
);

export const RuntimeGapReferenceSchema = NonEmptyRuntimeGapText.pipe(
  Schema.brand('ProductionSupportRuntimeGapReference')
);
export const RuntimeGapRequirementSchema = NonEmptyRuntimeGapText.pipe(
  Schema.brand('ProductionSupportRuntimeGapRequirement')
);

export const ForbiddenRuntimeGapDataClasses = [
  'child-activity-evidence',
  'raw-support-bundle',
  'provider-secrets',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
  'hosted-family-data',
  'backend-upload-payload',
  'private-paths',
  'screenshots',
  'journals',
  'sqlite-snapshots',
] as const;

export const RequiredRuntimeGapItems = [
  'public-website-runtime-gap',
  'support-publication-execution-gap',
  'support-backend-upload-execution-gap',
  'account-billing-provider-runtime-gap',
  'legal-export-delete-runtime-gap',
  'remote-support-sla-runtime-gap',
] as const;

export const RequiredRuntimeGapNonClaims = [
  'no-real-public-runtime',
  'no-support-publication-execution',
  'no-support-backend-upload-execution',
  'no-account-backend-runtime',
  'no-account-lookup-execution',
  'no-billing-provider-runtime',
  'no-billing-provider-contact',
  'no-legal-disclosure-execution',
  'no-export-delete-runtime-execution',
  'no-remote-support-session',
  'no-production-sla',
  'no-child-activity-custody',
  'no-provider-secrets',
  'no-hosted-family-data',
] as const;
