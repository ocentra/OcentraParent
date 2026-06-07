import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyStatusBackendDeadLetterText = Schema.String.pipe(Schema.minLength(1));

export const ProductionSupportStatusBackendDeadLetterSchemaVersionSchema = withParser(
  Schema.Literal('production-support-status-backend-dead-letter-proof')
);

export const ProductionSupportStatusBackendDeadLetterTargetSchema = withParser(
  Schema.Literal(
    'support-runbook-status-dead-letter',
    'incident-status-dead-letter',
    'public-support-contact-status-dead-letter',
    'support-upload-status-dead-letter',
    'privacy-legal-status-dead-letter',
    'account-billing-status-dead-letter'
  )
);

export const ProductionSupportStatusBackendDeadLetterStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'status-contract-ready',
    'requested',
    'authorized',
    'dead-lettered',
    'triage-ready',
    'retry-blocked',
    'failed',
    'manual-required',
    'backend-unavailable',
    'not-implemented',
    'implemented',
    'executed',
    'persisted'
  )
);

export const ProductionSupportStatusBackendDeadLetterSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-status-backend-execution-queue-proof',
    'production-support-status-backend-queue-audit-persistence-proof',
    'production-support-status-backend-public-runtime-followthrough-proof',
    'production-support-publication-runtime-readiness-proof',
    'production-support-privacy-legal-disclosure-status-proof',
    'public-support-contact-status-proof'
  )
);

export const ProductionSupportStatusBackendDeadLetterDataClassSchema = withParser(
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
    'dead-letter-reference',
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
    'audit-persistence-payload',
    'dead-letter-payload'
  )
);

export const ProductionSupportStatusBackendDeadLetterNonClaimSchema = withParser(
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
    'no-provider-secret-custody'
  )
);

export const StatusBackendDeadLetterReferenceSchema = NonEmptyStatusBackendDeadLetterText.pipe(
  Schema.brand('ProductionSupportStatusBackendDeadLetterReference')
);
export const StatusBackendDeadLetterManualRequirementSchema = NonEmptyStatusBackendDeadLetterText.pipe(
  Schema.brand('ProductionSupportStatusBackendDeadLetterManualRequirement')
);

export const ForbiddenStatusBackendDeadLetterDataClasses = [
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
] as const;

export const RequiredStatusBackendDeadLetterTargets = [
  'support-runbook-status-dead-letter',
  'incident-status-dead-letter',
  'public-support-contact-status-dead-letter',
  'support-upload-status-dead-letter',
  'privacy-legal-status-dead-letter',
  'account-billing-status-dead-letter',
] as const;

export const RequiredStatusBackendDeadLetterStates = [
  'requested',
  'authorized',
  'dead-lettered',
  'triage-ready',
  'retry-blocked',
  'failed',
  'manual-required',
  'backend-unavailable',
] as const;

export const RequiredStatusBackendDeadLetterNonClaims = [
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
] as const;
