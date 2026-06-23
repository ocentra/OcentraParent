import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const ProductionSupportProofStatusMatrixClosureSchemaVersionSchema = withParser(
  Schema.Literal('production-support-proof-status-matrix-closure-proof')
);

export const ProductionSupportProofStatusMatrixClosureAreaSchema = withParser(
  Schema.Literal(
    'status-backend-runtime',
    'public-runtime-publication',
    'privacy-legal-disclosure',
    'provider-secret-custody',
    'export-delete-lifecycle',
    'release-installer-support'
  )
);

export const ProductionSupportProofStatusMatrixClosureStateSchema = withParser(
  Schema.Literal('source-proof-present', 'manual-required', 'not-implemented', 'unclaimed')
);

export const ProductionSupportProofStatusMatrixClosureSourceProofSchema = withParser(
  Schema.Literal(
    'production-support-status-backend-execution-continuation-proof',
    'production-support-status-backend-runtime-closure-proof',
    'production-support-status-backend-durable-queue-runtime-proof',
    'production-support-status-backend-payload-custody-proof',
    'production-support-status-backend-redaction-manifest-proof',
    'production-release-public-surface-publication-proof',
    'production-support-publication-execution-status-proof',
    'production-support-publication-status-freshness-proof',
    'production-support-privacy-legal-disclosure-status-proof',
    'production-support-legal-provider-readiness-proof',
    'production-support-provider-secret-rotation-revocation-status-proof',
    'production-support-data-export-delete-lifecycle-proof',
    'production-support-delete-executor-proof',
    'parent-desktop-release-support-proof'
  )
);

export const ProductionSupportProofStatusMatrixClosureNonClaimSchema = withParser(
  Schema.Literal(
    'no-real-public-runtime',
    'no-status-backend-execution',
    'no-signing-store-proof',
    'no-updater-execution',
    'no-support-backend-upload-execution',
    'no-account-billing-provider-execution',
    'no-legal-disclosure-execution',
    'no-production-sla',
    'no-provider-secret-custody',
    'no-child-activity-custody'
  )
);

export const ProofStatusMatrixClosureReferenceSchema = brandedNonEmptyStringSchema(
  'ProductionSupportProofStatusMatrixClosureReference'
);

export const RequiredProofStatusMatrixClosureAreas = [
  'status-backend-runtime',
  'public-runtime-publication',
  'privacy-legal-disclosure',
  'provider-secret-custody',
  'export-delete-lifecycle',
  'release-installer-support',
] as const;

export const RequiredProofStatusMatrixClosureSourceProofs = [
  'production-support-status-backend-execution-continuation-proof',
  'production-support-status-backend-runtime-closure-proof',
  'production-support-status-backend-durable-queue-runtime-proof',
  'production-support-status-backend-payload-custody-proof',
  'production-support-status-backend-redaction-manifest-proof',
  'production-release-public-surface-publication-proof',
  'production-support-publication-execution-status-proof',
  'production-support-publication-status-freshness-proof',
  'production-support-privacy-legal-disclosure-status-proof',
  'production-support-legal-provider-readiness-proof',
  'production-support-provider-secret-rotation-revocation-status-proof',
  'production-support-data-export-delete-lifecycle-proof',
  'production-support-delete-executor-proof',
  'parent-desktop-release-support-proof',
] as const;

export const RequiredProofStatusMatrixClosureNonClaims = [
  'no-real-public-runtime',
  'no-status-backend-execution',
  'no-signing-store-proof',
  'no-updater-execution',
  'no-support-backend-upload-execution',
  'no-account-billing-provider-execution',
  'no-legal-disclosure-execution',
  'no-production-sla',
  'no-provider-secret-custody',
  'no-child-activity-custody',
] as const;
