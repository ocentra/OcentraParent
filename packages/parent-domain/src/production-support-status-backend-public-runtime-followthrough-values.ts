import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyStatusBackendPublicRuntimeFollowthroughText = Schema.String.pipe(Schema.minLength(1));

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughSchemaVersionSchema = withParser(
  Schema.Literal('production-support-status-backend-public-runtime-followthrough-proof')
);

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughTargetSchema = withParser(
  Schema.Literal(
    'support-status-public-runtime-followthrough',
    'support-runbook-status-backend-followthrough',
    'incident-status-backend-followthrough',
    'public-support-contact-status-backend-followthrough',
    'support-upload-status-backend-followthrough',
    'account-billing-status-backend-followthrough'
  )
);

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema = withParser(
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
    'public-runtime-required',
    'not-implemented',
    'implemented',
    'executed'
  )
);

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-publication-execution-status-proof',
    'production-support-publication-runtime-readiness-proof',
    'production-release-public-runtime-handoff-proof',
    'production-support-publication-status-freshness-proof',
    'public-support-contact-status-proof'
  )
);

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughDataClassSchema = withParser(
  Schema.Literal(
    'publication-status-label',
    'support-runbook-status',
    'incident-status',
    'public-support-contact-status',
    'support-upload-status-summary',
    'account-status-summary',
    'billing-support-status',
    'public-runtime-handoff-reference',
    'status-backend-handoff-reference',
    'manual-proof-reference',
    'child-activity-evidence',
    'raw-support-bundle',
    'provider-secrets',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript',
    'production-sla-commitment',
    'public-runtime-payload',
    'backend-upload-payload'
  )
);

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-public-runtime-execution',
    'no-status-backend-execution',
    'no-support-backend-upload-execution',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-production-sla',
    'no-child-activity-custody',
    'no-legal-disclosure-execution',
    'no-remote-support-session',
    'no-provider-secret-custody',
    'no-public-runtime-payload-custody'
  )
);

export const StatusBackendPublicRuntimeFollowthroughReferenceSchema =
  NonEmptyStatusBackendPublicRuntimeFollowthroughText.pipe(
    Schema.brand('ProductionSupportStatusBackendPublicRuntimeFollowthroughReference')
  );
export const StatusBackendPublicRuntimeFollowthroughManualRequirementSchema =
  NonEmptyStatusBackendPublicRuntimeFollowthroughText.pipe(
    Schema.brand('ProductionSupportStatusBackendPublicRuntimeFollowthroughManualRequirement')
  );

export const ForbiddenStatusBackendPublicRuntimeFollowthroughDataClasses = [
  'child-activity-evidence',
  'raw-support-bundle',
  'provider-secrets',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
  'production-sla-commitment',
  'public-runtime-payload',
  'backend-upload-payload',
] as const;

export const RequiredStatusBackendPublicRuntimeFollowthroughTargets = [
  'support-status-public-runtime-followthrough',
  'support-runbook-status-backend-followthrough',
  'incident-status-backend-followthrough',
  'public-support-contact-status-backend-followthrough',
  'support-upload-status-backend-followthrough',
  'account-billing-status-backend-followthrough',
] as const;

export const RequiredStatusBackendPublicRuntimeFollowthroughStates = [
  'requested',
  'queued',
  'running',
  'succeeded',
  'failed',
  'manual-required',
] as const;

export const RequiredStatusBackendPublicRuntimeFollowthroughNonClaims = [
  'no-real-public-runtime-execution',
  'no-status-backend-execution',
  'no-support-backend-upload-execution',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-production-sla',
  'no-child-activity-custody',
  'no-legal-disclosure-execution',
  'no-remote-support-session',
  'no-provider-secret-custody',
  'no-public-runtime-payload-custody',
] as const;
