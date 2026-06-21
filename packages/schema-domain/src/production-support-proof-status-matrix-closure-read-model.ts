import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  type ProductionSupportProofStatusMatrixClosureArea,
  type ProductionSupportProofStatusMatrixClosureRow,
  ProductionSupportProofStatusMatrixClosureProofSchema,
} from './production-support-proof-status-matrix-closure-proof';
import {
  ProofStatusMatrixClosureReferenceSchema,
  RequiredProofStatusMatrixClosureNonClaims,
  RequiredProofStatusMatrixClosureSourceProofs,
} from './production-support-proof-status-matrix-closure-values';

export const ProductionSupportProofStatusMatrixClosureReadModel =
  ProductionSupportProofStatusMatrixClosureProofSchema.parse({
    schemaVersion: 'production-support-proof-status-matrix-closure-proof',
    sourceProofRefs: RequiredProofStatusMatrixClosureSourceProofs,
    rows: [
      matrixClosureRow('status-backend-runtime', [
        'production-support-status-backend-execution-continuation-proof',
        'production-support-status-backend-runtime-closure-proof',
        'production-support-status-backend-durable-queue-runtime-proof',
        'production-support-status-backend-payload-custody-proof',
        'production-support-status-backend-redaction-manifest-proof',
      ]),
      matrixClosureRow('public-runtime-publication', [
        'production-release-public-surface-publication-proof',
        'production-support-publication-execution-status-proof',
        'production-support-publication-status-freshness-proof',
      ]),
      matrixClosureRow('privacy-legal-disclosure', [
        'production-support-privacy-legal-disclosure-status-proof',
        'production-support-legal-provider-readiness-proof',
      ]),
      matrixClosureRow('provider-secret-custody', [
        'production-support-provider-secret-rotation-revocation-status-proof',
      ]),
      matrixClosureRow('export-delete-lifecycle', [
        'production-support-data-export-delete-lifecycle-proof',
        'production-support-delete-executor-proof',
      ]),
      matrixClosureRow('release-installer-support', ['parent-desktop-release-support-proof']),
    ],
    nonClaims: RequiredProofStatusMatrixClosureNonClaims,
    publicRuntimeClaim: 'not-implemented',
    statusBackendExecutionClaim: 'manual-required',
    signingStoreClaim: 'manual-required',
    updaterExecutionClaim: 'manual-required',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountBillingProviderExecutionClaim: 'manual-required',
    legalDisclosureExecutionClaim: 'manual-required',
    productionSlaClaim: 'not-implemented',
    providerSecretCustodyClaim: 'not-implemented',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-07T22:52:55.104Z'),
  });

export const ProductionSupportProofStatusMatrixClosureKnownGaps = [
  'Proof/status matrix closure reconciles existing backend, public runtime, privacy/legal, provider-secret, export/delete, and release-support source proofs after PR534 without duplicating their rows.',
  'Real public runtime, status backend execution, signing/store proof, updater execution, support backend upload execution, account/billing provider execution, legal disclosure execution, production SLA, provider-secret custody, and child activity custody remain unimplemented or manual-required.',
] as const;

function matrixClosureRow(
  area: ProductionSupportProofStatusMatrixClosureArea,
  sourceProofRefs: ProductionSupportProofStatusMatrixClosureRow['sourceProofRefs']
): ProductionSupportProofStatusMatrixClosureRow {
  return {
    schemaVersion: 'production-support-proof-status-matrix-closure-proof',
    area,
    proofState: 'source-proof-present',
    runtimeState: area === 'status-backend-runtime' ? 'manual-required' : 'not-implemented',
    backendExecutionState: 'manual-required',
    publicRuntimeState: 'not-implemented',
    legalExecutionState: area === 'privacy-legal-disclosure' ? 'manual-required' : 'not-implemented',
    providerSecretCustodyState: 'not-implemented',
    childActivityCustodyState: 'not-implemented',
    sourceProofRefs,
    matrixRef: matrixClosureReference(area, 'matrix'),
    nextManualProofRef: matrixClosureReference(area, 'manual-proof'),
  };
}

function matrixClosureReference(
  area: ProductionSupportProofStatusMatrixClosureArea,
  referenceKind: 'matrix' | 'manual-proof'
): ProductionSupportProofStatusMatrixClosureRow['matrixRef'] {
  return Schema.decodeUnknownSync(ProofStatusMatrixClosureReferenceSchema)(
    `production-support-proof-status-matrix-closure-${area}-${referenceKind}`
  );
}
