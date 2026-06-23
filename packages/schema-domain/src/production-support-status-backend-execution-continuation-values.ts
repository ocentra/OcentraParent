import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportStatusBackendExecutionContinuationSchemaVersionSchema = withParser(
  Schema.Literal('production-support-status-backend-execution-continuation-proof')
);

export const ProductionSupportStatusBackendExecutionContinuationTargetSchema = withParser(
  Schema.Literal(
    'support-runbook-status-backend-execution-continuation',
    'incident-status-backend-execution-continuation',
    'public-support-contact-status-backend-execution-continuation',
    'support-upload-status-backend-execution-continuation',
    'privacy-legal-status-backend-execution-continuation',
    'account-billing-status-backend-execution-continuation'
  )
);

export const ProductionSupportStatusBackendExecutionContinuationStateSchema = withParser(
  Schema.Literal(
    'execution-preflight-ready',
    'runtime-worker-required',
    'durable-storage-required',
    'payload-custody-required',
    'redaction-manifest-required',
    'manual-required',
    'backend-unavailable',
    'not-implemented',
    'implemented',
    'executed',
    'persisted'
  )
);

export const ProductionSupportStatusBackendExecutionContinuationSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-status-backend-durable-queue-runtime-proof',
    'production-support-status-backend-runtime-closure-proof',
    'production-support-status-backend-payload-custody-proof',
    'production-support-status-backend-redaction-manifest-proof'
  )
);

export const ProductionSupportStatusBackendExecutionContinuationDataClassSchema = withParser(
  Schema.Literal(
    'status-label',
    'status-backend-target-ref',
    'durable-queue-runtime-ref',
    'payload-custody-boundary-ref',
    'redaction-manifest-ref',
    'manual-proof-ref',
    'child-activity-evidence',
    'raw-support-bundle',
    'provider-secrets',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript',
    'production-sla-commitment',
    'public-runtime-payload',
    'backend-upload-payload',
    'status-backend-payload',
    'durable-queue-payload',
    'retry-worker-payload',
    'audit-persistence-payload',
    'dead-letter-payload',
    'provider-secret-payload',
    'legal-execution-payload'
  )
);

export const ProductionSupportStatusBackendExecutionContinuationNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-status-backend-execution',
    'no-durable-queue-storage',
    'no-retry-worker-execution',
    'no-audit-persistence',
    'no-dead-letter-payload-custody',
    'no-status-backend-payload-custody',
    'no-redaction-manifest-execution',
    'no-public-runtime-execution',
    'no-provider-execution',
    'no-support-backend-upload-execution',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-legal-disclosure-execution',
    'no-remote-support-session',
    'no-production-sla',
    'no-provider-secret-custody',
    'no-default-hosted-family-data',
    'no-child-activity-custody'
  )
);

export const ExecutionContinuationReferenceSchema = brandedNonEmptyStringSchema(
  'ProductionSupportStatusBackendExecutionContinuationReference'
);
export const ExecutionContinuationManualRequirementSchema = brandedNonEmptyStringSchema(
  'ProductionSupportStatusBackendExecutionContinuationManualRequirement'
);

export const RequiredExecutionContinuationTargets = [
  'support-runbook-status-backend-execution-continuation',
  'incident-status-backend-execution-continuation',
  'public-support-contact-status-backend-execution-continuation',
  'support-upload-status-backend-execution-continuation',
  'privacy-legal-status-backend-execution-continuation',
  'account-billing-status-backend-execution-continuation',
] as const;

export const RequiredExecutionContinuationStates = [
  'execution-preflight-ready',
  'runtime-worker-required',
  'durable-storage-required',
  'payload-custody-required',
  'redaction-manifest-required',
  'manual-required',
  'backend-unavailable',
] as const;

export const RequiredExecutionContinuationSourceProofs = [
  'production-support-status-backend-durable-queue-runtime-proof',
  'production-support-status-backend-runtime-closure-proof',
  'production-support-status-backend-payload-custody-proof',
  'production-support-status-backend-redaction-manifest-proof',
] as const;

export const ExecutionContinuationSupportSafeDataClasses = [
  'status-label',
  'status-backend-target-ref',
  'durable-queue-runtime-ref',
  'payload-custody-boundary-ref',
  'redaction-manifest-ref',
  'manual-proof-ref',
] as const;

export const ForbiddenExecutionContinuationDataClasses = [
  'child-activity-evidence',
  'raw-support-bundle',
  'provider-secrets',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
  'production-sla-commitment',
  'public-runtime-payload',
  'backend-upload-payload',
  'status-backend-payload',
  'durable-queue-payload',
  'retry-worker-payload',
  'audit-persistence-payload',
  'dead-letter-payload',
  'provider-secret-payload',
  'legal-execution-payload',
] as const;

export const RequiredExecutionContinuationNonClaims = [
  'no-real-status-backend-execution',
  'no-durable-queue-storage',
  'no-retry-worker-execution',
  'no-audit-persistence',
  'no-dead-letter-payload-custody',
  'no-status-backend-payload-custody',
  'no-redaction-manifest-execution',
  'no-public-runtime-execution',
  'no-provider-execution',
  'no-support-backend-upload-execution',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-legal-disclosure-execution',
  'no-remote-support-session',
  'no-production-sla',
  'no-provider-secret-custody',
  'no-default-hosted-family-data',
  'no-child-activity-custody',
] as const;
