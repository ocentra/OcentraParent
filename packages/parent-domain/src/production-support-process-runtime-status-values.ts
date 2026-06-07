import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyProductionSupportProcessRuntimeStatusText = Schema.String.pipe(Schema.minLength(1));

export const ProductionSupportProcessRuntimeStatusSchemaVersionSchema = withParser(
  Schema.Literal('production-support-process-runtime-status-proof')
);

export const ProductionSupportProcessRuntimeStatusSurfaceSchema = withParser(
  Schema.Literal(
    'support-process-requested',
    'parent-consent-authorized',
    'privacy-legal-queued',
    'redaction-review-running',
    'backend-upload-failed',
    'case-resolution-succeeded',
    'support-process-manual-required',
    'incident-runtime-requested',
    'incident-runtime-authorized',
    'incident-runtime-running',
    'incident-runtime-evidence-ready',
    'incident-runtime-manual-required'
  )
);

export const ProductionSupportProcessRuntimeStatusStateSchema = withParser(
  Schema.Literal(
    'requested',
    'authorized',
    'queued',
    'running',
    'succeeded',
    'failed',
    'manual-required',
    'not-implemented',
    'runtime-evidence-ready',
    'executed'
  )
);

export const ProductionSupportProcessRuntimeStatusSourceProofSchema = withParser(
  Schema.Literal(
    'support-incident-workflow-proof',
    'production-incident-support-status-proof',
    'production-support-backend-upload-status-proof',
    'production-support-backend-upload-execution-runtime-proof',
    'production-support-case-resolution-status-proof',
    'production-support-publication-runtime-readiness-proof',
    'production-support-status-backend-runtime-execution-proof',
    'data-custody-expectation',
    'documentation-expectation'
  )
);

export const ProductionSupportProcessRuntimeStatusDataClassSchema = withParser(
  Schema.Literal(
    'support-case-status',
    'parent-consent-status',
    'privacy-legal-status',
    'redaction-review-status',
    'backend-upload-status',
    'case-resolution-status',
    'manual-proof-status',
    'support-runbook-status',
    'audit-status',
    'incident-runtime-status',
    'runtime-evidence-status',
    'child-activity-evidence',
    'raw-support-bundle',
    'provider-secret',
    'remote-support-session-transcript',
    'account-lookup-result',
    'billing-provider-contact-record',
    'backend-upload-payload',
    'public-runtime-payload',
    'sla-commitment'
  )
);

export const ProductionSupportProcessRuntimeStatusNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-backend-upload-execution',
    'no-public-runtime-execution',
    'no-provider-execution',
    'no-production-sla',
    'no-remote-support-session',
    'no-child-activity-custody',
    'no-provider-secrets',
    'no-incident-runtime-execution',
    'no-default-ocentra-hosted-family-data'
  )
);

export const ProductionSupportProcessRuntimeStatusReferenceSchema =
  NonEmptyProductionSupportProcessRuntimeStatusText.pipe(
    Schema.brand('ProductionSupportProcessRuntimeStatusReference')
  );
export const ProductionSupportProcessRuntimeStatusRequirementSchema =
  NonEmptyProductionSupportProcessRuntimeStatusText.pipe(
    Schema.brand('ProductionSupportProcessRuntimeStatusRequirement')
  );

export const ForbiddenProductionSupportProcessRuntimeStatusDataClasses = [
  'child-activity-evidence',
  'raw-support-bundle',
  'provider-secret',
  'remote-support-session-transcript',
  'account-lookup-result',
  'billing-provider-contact-record',
  'backend-upload-payload',
  'public-runtime-payload',
  'sla-commitment',
] as const;

export const RequiredProductionSupportProcessRuntimeStatusSurfaces = [
  'support-process-requested',
  'parent-consent-authorized',
  'privacy-legal-queued',
  'redaction-review-running',
  'backend-upload-failed',
  'case-resolution-succeeded',
  'support-process-manual-required',
  'incident-runtime-requested',
  'incident-runtime-authorized',
  'incident-runtime-running',
  'incident-runtime-evidence-ready',
  'incident-runtime-manual-required',
] as const;

export const RequiredProductionSupportProcessRuntimeStatusNonClaims = [
  'no-real-backend-upload-execution',
  'no-public-runtime-execution',
  'no-provider-execution',
  'no-production-sla',
  'no-remote-support-session',
  'no-child-activity-custody',
  'no-provider-secrets',
  'no-incident-runtime-execution',
  'no-default-ocentra-hosted-family-data',
] as const;
