import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportStatusBackendQueueAuditPersistenceSchemaVersionSchema = withParser(
  Schema.Literal('production-support-status-backend-queue-audit-persistence-proof')
);

export const ProductionSupportStatusBackendQueueAuditPersistenceTargetSchema = withParser(
  Schema.Literal(
    'support-runbook-status-queue-audit-persistence',
    'incident-status-queue-audit-persistence',
    'public-support-contact-status-queue-audit-persistence',
    'support-upload-status-queue-audit-persistence',
    'privacy-legal-status-queue-audit-persistence',
    'account-billing-status-queue-audit-persistence'
  )
);

export const ProductionSupportStatusBackendQueueAuditPersistenceStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'status-contract-ready',
    'requested',
    'authorized',
    'queued',
    'retry-scheduled',
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

export const ProductionSupportStatusBackendQueueAuditPersistenceSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-status-backend-execution-queue-proof',
    'production-support-status-backend-public-runtime-followthrough-proof',
    'production-support-publication-execution-status-proof',
    'production-support-publication-runtime-readiness-proof',
    'production-support-privacy-legal-disclosure-status-proof',
    'public-support-contact-status-proof'
  )
);

export const ProductionSupportStatusBackendQueueAuditPersistenceDataClassSchema = withParser(
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
    'audit-persistence-payload'
  )
);

export const ProductionSupportStatusBackendQueueAuditPersistenceNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-status-backend-execution',
    'no-durable-queue-storage',
    'no-retry-worker-execution',
    'no-audit-persistence',
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

export const QueueAuditPersistenceReferenceSchema = brandedNonEmptyStringSchema('ProductionSupportStatusBackendQueueAuditPersistenceReference');
export const QueueAuditPersistenceManualRequirementSchema = brandedNonEmptyStringSchema('ProductionSupportStatusBackendQueueAuditPersistenceManualRequirement');

export const ForbiddenQueueAuditPersistenceDataClasses = [
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
] as const;

export const RequiredQueueAuditPersistenceTargets = [
  'support-runbook-status-queue-audit-persistence',
  'incident-status-queue-audit-persistence',
  'public-support-contact-status-queue-audit-persistence',
  'support-upload-status-queue-audit-persistence',
  'privacy-legal-status-queue-audit-persistence',
  'account-billing-status-queue-audit-persistence',
] as const;

export const RequiredQueueAuditPersistenceStates = [
  'requested',
  'authorized',
  'queued',
  'retry-scheduled',
  'audit-ready',
  'failed',
  'manual-required',
  'backend-unavailable',
] as const;

export const RequiredQueueAuditPersistenceNonClaims = [
  'no-real-status-backend-execution',
  'no-durable-queue-storage',
  'no-retry-worker-execution',
  'no-audit-persistence',
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

