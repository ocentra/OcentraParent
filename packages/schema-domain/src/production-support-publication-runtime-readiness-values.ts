import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportPublicationRuntimeReadinessSchemaVersionSchema = withParser(
  Schema.Literal('production-support-publication-runtime-readiness-proof')
);

export const ProductionSupportPublicationRuntimeReadinessItemSchema = withParser(
  Schema.Literal(
    'public-runtime-publication-adapter-readiness',
    'support-runbook-publication-runner-readiness',
    'incident-status-publication-runner-readiness',
    'support-upload-publication-runtime-readiness',
    'privacy-legal-publication-runtime-readiness',
    'public-support-contact-runtime-readiness'
  )
);

export const ProductionSupportPublicationRuntimeReadinessStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'adapter-required',
    'runner-required',
    'backend-required',
    'legal-review-required',
    'manual-required',
    'not-implemented',
    'implemented',
    'executed'
  )
);

export const ProductionSupportPublicationRuntimeReadinessSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-publication-workflow-proof',
    'production-release-public-runtime-handoff-proof',
    'production-release-public-docs-status-proof',
    'public-support-contact-status-proof',
    'production-support-backend-upload-status-proof',
    'release-installer-expectation',
    'documentation-expectation'
  )
);

export const ProductionSupportPublicationRuntimeReadinessRuntimeRefSchema =
  brandedNonEmptyStringSchema('ProductionSupportPublicationRuntimeReadinessRuntimeRef');
export const ProductionSupportPublicationRuntimeReadinessManualRequirementSchema =
  brandedNonEmptyStringSchema('ProductionSupportPublicationRuntimeReadinessManualRequirement');

export const ProductionSupportPublicationRuntimeReadinessNonClaimSchema = withParser(
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
    'no-provider-secret-custody'
  )
);

export const ProductionSupportPublicationRuntimeReadinessDataClassSchema = withParser(
  Schema.Literal(
    'public-route-status',
    'support-runbook-status',
    'incident-status',
    'support-upload-status-summary',
    'privacy-policy-status',
    'legal-review-status',
    'manual-proof-reference',
    'runtime-adapter-reference',
    'publication-runner-reference',
    'child-activity-evidence',
    'raw-support-bundle',
    'provider-secrets',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript',
    'production-sla-commitment'
  )
);

export const ForbiddenPublicationRuntimeReadinessDataClasses = [
  'child-activity-evidence',
  'raw-support-bundle',
  'provider-secrets',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
  'production-sla-commitment',
] as const;

export const RequiredPublicationRuntimeReadinessItems = [
  'public-runtime-publication-adapter-readiness',
  'support-runbook-publication-runner-readiness',
  'incident-status-publication-runner-readiness',
  'support-upload-publication-runtime-readiness',
  'privacy-legal-publication-runtime-readiness',
  'public-support-contact-runtime-readiness',
] as const;

export const RequiredPublicationRuntimeReadinessNonClaims = [
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
] as const;

