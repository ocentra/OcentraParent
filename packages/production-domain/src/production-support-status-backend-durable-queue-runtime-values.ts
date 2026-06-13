import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportStatusBackendDurableQueueRuntimeSchemaVersionSchema = withParser(
  Schema.Literal('production-support-status-backend-durable-queue-runtime-proof')
);

export const ProductionSupportStatusBackendDurableQueueRuntimeTargetSchema = withParser(
  Schema.Literal(
    'support-runbook-status-backend-durable-queue-runtime',
    'incident-status-backend-durable-queue-runtime',
    'public-support-contact-status-backend-durable-queue-runtime',
    'support-upload-status-backend-durable-queue-runtime',
    'privacy-legal-status-backend-durable-queue-runtime',
    'account-billing-status-backend-durable-queue-runtime'
  )
);

export const ProductionSupportStatusBackendDurableQueueRuntimeStateSchema = withParser(
  Schema.Literal(
    'queue-storage-boundary-ready',
    'retry-worker-boundary-ready',
    'audit-persistence-boundary-ready',
    'dead-letter-runtime-boundary-ready',
    'runtime-boundary-manual-required',
    'backend-unavailable',
    'manual-required',
    'not-implemented',
    'source-contract-ready',
    'implemented',
    'executed',
    'persisted'
  )
);

export const ProductionSupportStatusBackendDurableQueueRuntimeSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-status-backend-execution-queue-proof',
    'production-support-status-backend-queue-audit-persistence-proof',
    'production-support-status-backend-dead-letter-proof',
    'production-support-status-backend-runtime-execution-proof',
    'production-support-status-backend-runtime-closure-proof'
  )
);

export const ProductionSupportStatusBackendDurableQueueRuntimeDataClassSchema = withParser(
  Schema.Literal(
    'status-label',
    'status-backend-target-ref',
    'queue-boundary-ref',
    'retry-worker-ref',
    'audit-persistence-ref',
    'dead-letter-ref',
    'runtime-evidence-ref',
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

export const ProductionSupportStatusBackendDurableQueueRuntimeNonClaimSchema = withParser(
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
    'no-legal-disclosure-execution',
    'no-remote-support-session',
    'no-production-sla',
    'no-provider-secret-custody',
    'no-child-activity-custody'
  )
);

export const DurableQueueRuntimeReferenceSchema = brandedNonEmptyStringSchema('ProductionSupportStatusBackendDurableQueueRuntimeReference');
export const DurableQueueRuntimeManualRequirementSchema = brandedNonEmptyStringSchema('ProductionSupportStatusBackendDurableQueueRuntimeManualRequirement');

export const RequiredDurableQueueRuntimeTargets = [
  'support-runbook-status-backend-durable-queue-runtime',
  'incident-status-backend-durable-queue-runtime',
  'public-support-contact-status-backend-durable-queue-runtime',
  'support-upload-status-backend-durable-queue-runtime',
  'privacy-legal-status-backend-durable-queue-runtime',
  'account-billing-status-backend-durable-queue-runtime',
] as const;

export const RequiredDurableQueueRuntimeStates = [
  'queue-storage-boundary-ready',
  'retry-worker-boundary-ready',
  'audit-persistence-boundary-ready',
  'dead-letter-runtime-boundary-ready',
  'runtime-boundary-manual-required',
  'backend-unavailable',
] as const;

export const RequiredDurableQueueRuntimeSourceProofs = [
  'production-support-status-backend-execution-queue-proof',
  'production-support-status-backend-queue-audit-persistence-proof',
  'production-support-status-backend-dead-letter-proof',
  'production-support-status-backend-runtime-execution-proof',
  'production-support-status-backend-runtime-closure-proof',
] as const;

export const DurableQueueRuntimeSupportSafeDataClasses = [
  'status-label',
  'status-backend-target-ref',
  'queue-boundary-ref',
  'retry-worker-ref',
  'audit-persistence-ref',
  'dead-letter-ref',
  'runtime-evidence-ref',
  'manual-proof-ref',
] as const;

export const ForbiddenDurableQueueRuntimeDataClasses = [
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

export const RequiredDurableQueueRuntimeNonClaims = [
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
  'no-legal-disclosure-execution',
  'no-remote-support-session',
  'no-production-sla',
  'no-provider-secret-custody',
  'no-child-activity-custody',
] as const;

