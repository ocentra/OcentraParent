import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyPublicationWorkflowText = Schema.String.pipe(Schema.minLength(1));

export const ProductionSupportPublicationWorkflowSchemaVersionSchema = withParser(
  Schema.Literal('production-support-publication-workflow-proof')
);

export const ProductionSupportPublicationWorkflowItemSchema = withParser(
  Schema.Literal(
    'public-privacy-policy-publication',
    'privacy-legal-disclosure-execution',
    'support-runbook-publication',
    'support-incident-status-publication',
    'support-backend-upload-publication-handoff',
    'public-support-contact-publication'
  )
);

export const ProductionSupportPublicationWorkflowStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'manual-required',
    'publication-required',
    'legal-review-required',
    'backend-required',
    'not-implemented',
    'implemented',
    'executed'
  )
);

export const ProductionSupportPublicationWorkflowSourceProofSchema = withParser(
  Schema.Literal(
    'production-release-public-docs-status-proof',
    'production-release-public-surface-publication-proof',
    'production-support-backend-upload-status-proof',
    'documentation-expectation',
    'data-custody-expectation'
  )
);

export const ProductionSupportPublicationWorkflowDataClassSchema = withParser(
  Schema.Literal(
    'public-policy-text',
    'retention-summary',
    'export-delete-summary',
    'support-runbook-status',
    'incident-status',
    'legal-review-status',
    'support-upload-status-summary',
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

export const ProductionSupportPublicationWorkflowNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-public-runtime',
    'no-support-backend-upload-execution',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-production-sla',
    'no-child-activity-custody',
    'no-legal-disclosure-execution',
    'no-remote-support-session'
  )
);

export const PublicationWorkflowReferenceSchema = NonEmptyPublicationWorkflowText.pipe(
  Schema.brand('ProductionSupportPublicationWorkflowReference')
);
export const PublicationWorkflowRequirementSchema = NonEmptyPublicationWorkflowText.pipe(
  Schema.brand('ProductionSupportPublicationWorkflowRequirement')
);

export const ForbiddenPublicationWorkflowDataClasses = [
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

export const RequiredPublicationWorkflowItems = [
  'public-privacy-policy-publication',
  'privacy-legal-disclosure-execution',
  'support-runbook-publication',
  'support-incident-status-publication',
  'support-backend-upload-publication-handoff',
  'public-support-contact-publication',
] as const;

export const RequiredPublicationWorkflowNonClaims = [
  'no-real-public-runtime',
  'no-support-backend-upload-execution',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-production-sla',
  'no-child-activity-custody',
  'no-legal-disclosure-execution',
  'no-remote-support-session',
] as const;
