import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportStatusBackendRuntimeExecutionSchemaVersionSchema = withParser(
  Schema.Literal('production-support-status-backend-runtime-execution-proof')
);

export const ProductionSupportStatusBackendRuntimeExecutionTargetSchema = withParser(
  Schema.Literal(
    'support-runbook-status-runtime-execution',
    'incident-status-runtime-execution',
    'public-support-contact-status-runtime-execution',
    'support-upload-status-runtime-execution',
    'privacy-legal-status-runtime-execution',
    'account-billing-status-runtime-execution'
  )
);

export const ProductionSupportStatusBackendRuntimeExecutionStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'status-contract-ready',
    'requested',
    'authorized',
    'queued',
    'running',
    'runtime-evidence-ready',
    'audit-ready',
    'failed',
    'manual-required',
    'backend-unavailable',
    'not-implemented',
    'implemented',
    'executed',
    'persisted'
  )
);

export const ProductionSupportStatusBackendRuntimeExecutionSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-status-backend-dead-letter-proof',
    'production-support-status-backend-queue-audit-persistence-proof',
    'production-support-status-backend-execution-queue-proof',
    'production-support-status-backend-public-runtime-followthrough-proof',
    'production-support-status-backend-redaction-manifest-proof',
    'production-support-status-backend-payload-custody-proof',
    'production-support-publication-runtime-readiness-proof',
    'production-support-privacy-legal-disclosure-status-proof',
    'public-support-contact-status-proof'
  )
);

export const ProductionSupportStatusBackendRuntimeExecutionDataClassSchema = withParser(
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
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'dead-letter-reference',
    'runtime-evidence-reference',
    'manual-proof-reference',
    'child-activity-evidence',
    'raw-support-bundle',
    'provider-secrets',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript',
    'production-sla-commitment',
    'public-runtime-payload',
    'backend-upload-payload',
    'status-backend-execution-payload',
    'durable-queue-payload',
    'retry-worker-payload',
    'audit-persistence-payload',
    'dead-letter-payload',
    'status-backend-payload'
  )
);

export const ProductionSupportStatusBackendRuntimeExecutionNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-status-backend-execution',
    'no-durable-queue-storage',
    'no-retry-worker-execution',
    'no-audit-persistence',
    'no-dead-letter-payload-custody',
    'no-real-public-runtime-execution',
    'no-provider-execution',
    'no-support-backend-upload-execution',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-production-sla',
    'no-child-activity-custody',
    'no-legal-disclosure-execution',
    'no-remote-support-session',
    'no-provider-secret-custody',
    'no-status-backend-payload-custody'
  )
);

export const RuntimeExecutionReferenceSchema = brandedNonEmptyStringSchema('ProductionSupportStatusBackendRuntimeExecutionReference');
export const RuntimeExecutionManualRequirementSchema = brandedNonEmptyStringSchema('ProductionSupportStatusBackendRuntimeExecutionManualRequirement');

export const ForbiddenRuntimeExecutionDataClasses = [
  'child-activity-evidence',
  'raw-support-bundle',
  'provider-secrets',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
  'production-sla-commitment',
  'public-runtime-payload',
  'backend-upload-payload',
  'status-backend-execution-payload',
  'durable-queue-payload',
  'retry-worker-payload',
  'audit-persistence-payload',
  'dead-letter-payload',
  'status-backend-payload',
] as const;

export const RequiredRuntimeExecutionTargets = [
  'support-runbook-status-runtime-execution',
  'incident-status-runtime-execution',
  'public-support-contact-status-runtime-execution',
  'support-upload-status-runtime-execution',
  'privacy-legal-status-runtime-execution',
  'account-billing-status-runtime-execution',
] as const;

export const RequiredRuntimeExecutionStates = [
  'requested',
  'authorized',
  'queued',
  'running',
  'runtime-evidence-ready',
  'audit-ready',
  'failed',
  'manual-required',
  'backend-unavailable',
] as const;

export const RequiredRuntimeExecutionNonClaims = [
  'no-real-status-backend-execution',
  'no-durable-queue-storage',
  'no-retry-worker-execution',
  'no-audit-persistence',
  'no-dead-letter-payload-custody',
  'no-real-public-runtime-execution',
  'no-provider-execution',
  'no-support-backend-upload-execution',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-production-sla',
  'no-child-activity-custody',
  'no-legal-disclosure-execution',
  'no-remote-support-session',
  'no-provider-secret-custody',
  'no-status-backend-payload-custody',
] as const;

