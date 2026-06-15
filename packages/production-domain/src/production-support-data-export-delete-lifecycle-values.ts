import { Schema } from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportDataExportDeleteLifecycleSchemaVersion =
  'production-support-data-export-delete-lifecycle-proof' as const;

export const ProductionSupportDataExportDeleteLifecycleSchemaVersionSchema = Schema.Literal(
  ProductionSupportDataExportDeleteLifecycleSchemaVersion
);

export const ProductionSupportDataExportDeleteLifecycleSurfaces = [
  'export-requested',
  'export-authorized',
  'export-queued',
  'export-running',
  'export-succeeded',
  'export-failed',
  'export-manual-required',
  'delete-requested',
  'delete-authorized',
  'delete-queued',
  'delete-running',
  'delete-succeeded',
  'delete-failed',
  'delete-manual-required',
] as const;

export const RequiredProductionSupportDataExportDeleteLifecycleSurfaces =
  ProductionSupportDataExportDeleteLifecycleSurfaces;

export const ProductionSupportDataExportDeleteLifecycleSurfaceSchema = Schema.Literal(
  ...ProductionSupportDataExportDeleteLifecycleSurfaces
);

export const ProductionSupportDataExportDeleteLifecycleStates = [
  'requested',
  'authorized',
  'queued',
  'running',
  'succeeded',
  'failed',
  'manual-required',
  'not-implemented',
] as const;

export const ProductionSupportDataExportDeleteLifecycleStateSchema = Schema.Literal(
  ...ProductionSupportDataExportDeleteLifecycleStates
);

export const ProductionSupportDataExportDeleteLifecycleOperations = ['export', 'delete'] as const;

export const ProductionSupportDataExportDeleteLifecycleOperationSchema = Schema.Literal(
  ...ProductionSupportDataExportDeleteLifecycleOperations
);

export const ProductionSupportDataExportDeleteLifecycleSourceProofs = [
  'data-export-delete-lifecycle-logging-contract',
  'production-support-legal-provider-readiness-proof',
  'production-incident-support-status-proof',
  'production-release-public-docs-status-proof',
  'data-custody-expectation',
  'documentation-expectation',
] as const;

export const ProductionSupportDataExportDeleteLifecycleSourceProofSchema = Schema.Literal(
  ...ProductionSupportDataExportDeleteLifecycleSourceProofs
);

export const ProductionSupportDataExportDeleteLifecycleReferences = [
  'parent-request-ref',
  'parent-authorization-ref',
  'local-queue-ref',
  'local-runtime-ref',
  'local-output-ref',
  'local-delete-ref',
  'redaction-audit-ref',
  'manual-proof-ref',
  'data-custody-ref',
] as const;

export const ProductionSupportDataExportDeleteLifecycleReferenceSchema = Schema.Literal(
  ...ProductionSupportDataExportDeleteLifecycleReferences
);

export const ProductionSupportDataExportDeleteLifecycleDataClasses = [
  'request-status',
  'authorization-status',
  'queue-status',
  'runtime-status',
  'local-output-status',
  'delete-status',
  'redaction-audit-status',
  'manual-proof-status',
  'data-custody-status',
  'raw-child-activity',
  'raw-support-bundle-payload',
  'provider-secret',
  'backend-upload-payload',
  'public-runtime-payload',
  'remote-support-transcript',
  'production-sla-commitment',
  'default-ocentra-hosted-family-data',
] as const;

export const ProductionSupportDataExportDeleteLifecycleDataClassSchema = Schema.Literal(
  ...ProductionSupportDataExportDeleteLifecycleDataClasses
);

export const ForbiddenProductionSupportDataExportDeleteLifecycleDataClasses = [
  'raw-child-activity',
  'raw-support-bundle-payload',
  'provider-secret',
  'backend-upload-payload',
  'public-runtime-payload',
  'remote-support-transcript',
  'production-sla-commitment',
  'default-ocentra-hosted-family-data',
] as const;

export const RequiredProductionSupportDataExportDeleteLifecycleNonClaims = [
  'no-real-backend-upload-execution',
  'no-public-runtime-execution',
  'no-provider-execution',
  'no-production-sla',
  'no-remote-support-session',
  'no-child-activity-custody',
  'no-default-ocentra-hosted-family-data',
] as const;

export const ProductionSupportDataExportDeleteLifecycleNonClaimSchema = Schema.Literal(
  ...RequiredProductionSupportDataExportDeleteLifecycleNonClaims
);

export const ProductionSupportDataExportDeleteLifecycleRequirementSchema = Schema.TemplateLiteral(
  Schema.String,
  '-requires-runtime-execution-proof'
);
