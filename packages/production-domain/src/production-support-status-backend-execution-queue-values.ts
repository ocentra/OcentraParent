import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportStatusBackendExecutionQueueSchemaVersionSchema = withParser(
  Schema.Literal('production-support-status-backend-execution-queue-proof')
);

export const ProductionSupportStatusBackendExecutionQueueTargetSchema = withParser(
  Schema.Literal(
    'support-runbook-status-backend-queue',
    'incident-status-backend-queue',
    'public-support-contact-status-backend-queue',
    'support-upload-status-backend-queue',
    'privacy-legal-status-backend-queue',
    'account-billing-status-backend-queue'
  )
);

export const ProductionSupportStatusBackendExecutionQueueStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'status-contract-ready',
    'requested',
    'authorized',
    'queued',
    'running',
    'succeeded',
    'failed',
    'manual-required',
    'backend-unavailable',
    'retry-required',
    'audit-ready',
    'not-implemented',
    'implemented',
    'executed'
  )
);

export const ProductionSupportStatusBackendExecutionQueueSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-status-backend-public-runtime-followthrough-proof',
    'production-support-publication-execution-status-proof',
    'production-support-publication-runtime-readiness-proof',
    'production-support-privacy-legal-disclosure-status-proof',
    'public-support-contact-status-proof'
  )
);

export const ProductionSupportStatusBackendExecutionQueueDataClassSchema = withParser(
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
    'status-backend-execution-payload'
  )
);

export const ProductionSupportStatusBackendExecutionQueueNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-status-backend-execution',
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

export const StatusBackendExecutionQueueReferenceSchema = brandedNonEmptyStringSchema('ProductionSupportStatusBackendExecutionQueueReference');
export const StatusBackendExecutionQueueManualRequirementSchema = brandedNonEmptyStringSchema('ProductionSupportStatusBackendExecutionQueueManualRequirement');

export const ForbiddenStatusBackendExecutionQueueDataClasses = [
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
] as const;

export const RequiredStatusBackendExecutionQueueTargets = [
  'support-runbook-status-backend-queue',
  'incident-status-backend-queue',
  'public-support-contact-status-backend-queue',
  'support-upload-status-backend-queue',
  'privacy-legal-status-backend-queue',
  'account-billing-status-backend-queue',
] as const;

export const RequiredStatusBackendExecutionQueueStates = [
  'requested',
  'authorized',
  'queued',
  'running',
  'succeeded',
  'failed',
  'manual-required',
  'backend-unavailable',
] as const;

export const RequiredStatusBackendExecutionQueueNonClaims = [
  'no-real-status-backend-execution',
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

