import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportStatusBackendRuntimeClosureSchemaVersionSchema = withParser(
  Schema.Literal('production-support-status-backend-runtime-closure-proof')
);

export const ProductionSupportStatusBackendRuntimeClosureTargetSchema = withParser(
  Schema.Literal(
    'support-runbook-status-backend-closure',
    'incident-status-backend-closure',
    'public-support-contact-status-backend-closure',
    'support-upload-status-backend-closure',
    'privacy-legal-status-backend-closure',
    'account-billing-status-backend-closure'
  )
);

export const ProductionSupportStatusBackendRuntimeClosureStateSchema = withParser(
  Schema.Literal(
    'runtime-row-validated',
    'queue-audit-linked',
    'payload-custody-linked',
    'redaction-manifest-linked',
    'closure-manual-required',
    'backend-unavailable',
    'manual-required',
    'not-implemented',
    'source-contract-ready',
    'implemented',
    'executed',
    'persisted'
  )
);

export const ProductionSupportStatusBackendRuntimeClosureSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-status-backend-runtime-execution-proof',
    'production-support-status-backend-queue-audit-persistence-proof',
    'production-support-status-backend-dead-letter-proof',
    'production-support-status-backend-payload-custody-proof',
    'production-support-status-backend-redaction-manifest-proof',
    'production-support-status-backend-public-runtime-followthrough-proof'
  )
);

export const ProductionSupportStatusBackendRuntimeClosureDataClassSchema = withParser(
  Schema.Literal(
    'status-label',
    'status-backend-target-ref',
    'status-backend-queue-ref',
    'retry-policy-ref',
    'audit-ref',
    'dead-letter-ref',
    'runtime-evidence-ref',
    'payload-custody-ref',
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
    'legal-execution-payload'
  )
);

export const ProductionSupportStatusBackendRuntimeClosureNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-status-backend-execution',
    'no-durable-queue-storage',
    'no-retry-worker-execution',
    'no-audit-persistence',
    'no-dead-letter-payload-custody',
    'no-status-backend-payload-custody',
    'no-redaction-manifest-execution',
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

export const RuntimeClosureReferenceSchema = brandedNonEmptyStringSchema(
  'ProductionSupportStatusBackendRuntimeClosureReference'
);
export const RuntimeClosureManualRequirementSchema = brandedNonEmptyStringSchema(
  'ProductionSupportStatusBackendRuntimeClosureManualRequirement'
);

export const RequiredRuntimeClosureTargets = [
  'support-runbook-status-backend-closure',
  'incident-status-backend-closure',
  'public-support-contact-status-backend-closure',
  'support-upload-status-backend-closure',
  'privacy-legal-status-backend-closure',
  'account-billing-status-backend-closure',
] as const;

export const RequiredRuntimeClosureStates = [
  'runtime-row-validated',
  'queue-audit-linked',
  'payload-custody-linked',
  'redaction-manifest-linked',
  'closure-manual-required',
  'backend-unavailable',
] as const;

export const RequiredRuntimeClosureSourceProofs = [
  'production-support-status-backend-runtime-execution-proof',
  'production-support-status-backend-queue-audit-persistence-proof',
  'production-support-status-backend-dead-letter-proof',
  'production-support-status-backend-payload-custody-proof',
  'production-support-status-backend-redaction-manifest-proof',
  'production-support-status-backend-public-runtime-followthrough-proof',
] as const;

export const RuntimeClosureSupportSafeDataClasses = [
  'status-label',
  'status-backend-target-ref',
  'status-backend-queue-ref',
  'retry-policy-ref',
  'audit-ref',
  'dead-letter-ref',
  'runtime-evidence-ref',
  'payload-custody-ref',
  'redaction-manifest-ref',
  'manual-proof-ref',
] as const;

export const ForbiddenRuntimeClosureDataClasses = [
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
  'legal-execution-payload',
] as const;

export const RequiredRuntimeClosureNonClaims = [
  'no-real-status-backend-execution',
  'no-durable-queue-storage',
  'no-retry-worker-execution',
  'no-audit-persistence',
  'no-dead-letter-payload-custody',
  'no-status-backend-payload-custody',
  'no-redaction-manifest-execution',
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
