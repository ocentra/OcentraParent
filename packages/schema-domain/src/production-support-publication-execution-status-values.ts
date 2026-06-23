import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportPublicationExecutionStatusSchemaVersionSchema = withParser(
  Schema.Literal('production-support-publication-execution-status-proof')
);

export const ProductionSupportPublicationExecutionStatusTargetSchema = withParser(
  Schema.Literal(
    'support-runbook-publication-execution',
    'incident-status-publication-execution',
    'public-support-contact-publication-execution',
    'support-backend-upload-publication-execution',
    'privacy-legal-publication-execution',
    'account-billing-support-publication-execution'
  )
);

export const ProductionSupportPublicationExecutionStatusStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'status-contract-ready',
    'requested',
    'queued',
    'running',
    'succeeded',
    'failed',
    'manual-required',
    'backend-required',
    'legal-review-required',
    'not-implemented',
    'implemented',
    'executed'
  )
);

export const ProductionSupportPublicationExecutionStatusSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-publication-runtime-readiness-proof',
    'production-support-publication-status-freshness-proof',
    'production-support-publication-workflow-proof',
    'public-support-contact-status-proof',
    'production-release-public-docs-freshness-proof',
    'release-installer-expectation',
    'documentation-expectation'
  )
);

export const ProductionSupportPublicationExecutionStatusDataClassSchema = withParser(
  Schema.Literal(
    'publication-status-label',
    'support-runbook-status',
    'incident-status',
    'public-support-contact-status',
    'support-upload-status-summary',
    'privacy-policy-status',
    'legal-review-status',
    'account-status-summary',
    'billing-support-status',
    'manual-proof-reference',
    'runtime-readiness-reference',
    'freshness-policy-reference',
    'child-activity-evidence',
    'raw-support-bundle',
    'provider-secrets',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript',
    'production-sla-commitment',
    'legal-disclosure-payload'
  )
);

export const ProductionSupportPublicationExecutionStatusNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-public-runtime',
    'no-publication-runner-execution',
    'no-support-backend-upload-execution',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-production-sla',
    'no-child-activity-custody',
    'no-legal-disclosure-execution',
    'no-remote-support-session',
    'no-provider-secret-custody',
    'no-status-backend-execution'
  )
);

export const PublicationExecutionStatusReferenceSchema = brandedNonEmptyStringSchema(
  'ProductionSupportPublicationExecutionStatusReference'
);
export const PublicationExecutionStatusManualRequirementSchema = brandedNonEmptyStringSchema(
  'ProductionSupportPublicationExecutionStatusManualRequirement'
);

export const ForbiddenPublicationExecutionStatusDataClasses = [
  'child-activity-evidence',
  'raw-support-bundle',
  'provider-secrets',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
  'production-sla-commitment',
  'legal-disclosure-payload',
] as const;

export const RequiredPublicationExecutionStatusTargets = [
  'support-runbook-publication-execution',
  'incident-status-publication-execution',
  'public-support-contact-publication-execution',
  'support-backend-upload-publication-execution',
  'privacy-legal-publication-execution',
  'account-billing-support-publication-execution',
] as const;

export const RequiredPublicationExecutionStatusLifecycleStates = [
  'requested',
  'queued',
  'running',
  'succeeded',
  'failed',
  'manual-required',
] as const;

export const RequiredPublicationExecutionStatusNonClaims = [
  'no-real-public-runtime',
  'no-publication-runner-execution',
  'no-support-backend-upload-execution',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-production-sla',
  'no-child-activity-custody',
  'no-legal-disclosure-execution',
  'no-remote-support-session',
  'no-provider-secret-custody',
  'no-status-backend-execution',
] as const;
