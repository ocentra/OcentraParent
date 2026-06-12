import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ProductionSupportStatusBackendRuntimeClosureProofSchema,
  type ProductionSupportStatusBackendRuntimeClosureRow,
  type ProductionSupportStatusBackendRuntimeClosureState,
  type ProductionSupportStatusBackendRuntimeClosureTarget,
} from './production-support-status-backend-runtime-closure-proof';
import {
  ForbiddenRuntimeClosureDataClasses,
  RequiredRuntimeClosureNonClaims,
  RequiredRuntimeClosureSourceProofs,
  RequiredRuntimeClosureStates,
  RequiredRuntimeClosureTargets,
  RuntimeClosureManualRequirementSchema,
  RuntimeClosureReferenceSchema,
  RuntimeClosureSupportSafeDataClasses,
} from './production-support-status-backend-runtime-closure-values';

export const ProductionSupportStatusBackendRuntimeClosureReadModel =
  ProductionSupportStatusBackendRuntimeClosureProofSchema.parse({
    schemaVersion: 'production-support-status-backend-runtime-closure-proof',
    sourceContractRefs: RequiredRuntimeClosureSourceProofs,
    rows: RequiredRuntimeClosureTargets.flatMap((target) =>
      RequiredRuntimeClosureStates.map((closureState) => runtimeClosureReadiness(target, closureState))
    ),
    nonClaims: RequiredRuntimeClosureNonClaims,
    statusBackendExecutionClaim: 'manual-required',
    durableQueueStorageClaim: 'manual-required',
    retryWorkerExecutionClaim: 'manual-required',
    auditPersistenceClaim: 'manual-required',
    deadLetterPayloadCustodyClaim: 'manual-required',
    statusBackendPayloadCustodyClaim: 'manual-required',
    redactionManifestExecutionClaim: 'manual-required',
    publicRuntimeExecutionClaim: 'not-implemented',
    providerExecutionClaim: 'not-implemented',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountLookupExecutionClaim: 'manual-required',
    billingProviderContactClaim: 'manual-required',
    legalDisclosureExecutionClaim: 'manual-required',
    remoteSupportSessionClaim: 'not-implemented',
    productionSlaClaim: 'not-implemented',
    providerSecretCustodyClaim: 'not-implemented',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-07T18:35:00.000Z'),
  });

export const ProductionSupportStatusBackendRuntimeClosureKnownGaps = [
  'Runtime closure proves the existing status backend runtime, queue/audit, payload custody, and redaction manifest contracts can be composed; it does not implement real status backend execution.',
  'Durable queue storage, retry worker execution, audit persistence, dead-letter payload custody, status backend payload custody, and redaction manifest execution remain manual-required until a real backend worker and durable store exist.',
  'Public runtime execution, provider execution, support upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, provider-secret custody, and child activity custody remain unclaimed.',
  'Package exports, parent-domain README guidance, and product checklist proof text are updated in this branch; remaining gaps are limited to the explicit runtime, storage, provider, legal, SLA, and child-custody non-claims above.',
] as const;

function runtimeClosureReadiness(
  target: ProductionSupportStatusBackendRuntimeClosureTarget,
  closureState: ProductionSupportStatusBackendRuntimeClosureState
): ProductionSupportStatusBackendRuntimeClosureRow {
  return {
    schemaVersion: 'production-support-status-backend-runtime-closure-proof',
    target,
    closureState,
    runtimeExecutionRef: runtimeClosureReference(target, closureState, 'runtime-execution'),
    queueAuditPersistenceRef: runtimeClosureReference(target, closureState, 'queue-audit-persistence'),
    deadLetterRef: runtimeClosureReference(target, closureState, 'dead-letter'),
    payloadCustodyRef: runtimeClosureReference(target, closureState, 'payload-custody'),
    redactionManifestRef: runtimeClosureReference(target, closureState, 'redaction-manifest'),
    sourceProofRefs: RequiredRuntimeClosureSourceProofs,
    supportSafeDataClasses: RuntimeClosureSupportSafeDataClasses,
    forbiddenDataClasses: ForbiddenRuntimeClosureDataClasses,
    durableQueueStorageState: closureState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required',
    retryWorkerState: closureState === 'queue-audit-linked' ? 'manual-required' : 'not-implemented',
    auditPersistenceState: closureState === 'queue-audit-linked' ? 'manual-required' : 'not-implemented',
    deadLetterPayloadCustodyState: closureState === 'queue-audit-linked' ? 'manual-required' : 'not-implemented',
    statusBackendPayloadCustodyState: closureState === 'payload-custody-linked' ? 'manual-required' : 'not-implemented',
    redactionManifestExecutionState:
      closureState === 'redaction-manifest-linked' ? 'manual-required' : 'not-implemented',
    statusBackendExecutionState: 'manual-required',
    publicRuntimeExecutionState: 'not-implemented',
    providerExecutionState: 'not-implemented',
    supportBackendUploadState:
      target === 'support-upload-status-backend-closure' ? 'manual-required' : 'not-implemented',
    legalDisclosureExecutionState:
      target === 'privacy-legal-status-backend-closure' ? 'manual-required' : 'not-implemented',
    childActivityCustodyState: 'not-implemented',
    manualRequirement: Schema.decodeUnknownSync(RuntimeClosureManualRequirementSchema)(
      `${target}-${closureState}-requires-real-status-backend-worker-durable-queue-audit-payload-custody-and-redaction-manifest-proof-before-product-claim`
    ),
  };
}

function runtimeClosureReference(
  target: ProductionSupportStatusBackendRuntimeClosureTarget,
  closureState: ProductionSupportStatusBackendRuntimeClosureState,
  referenceKind:
    | 'runtime-execution'
    | 'queue-audit-persistence'
    | 'dead-letter'
    | 'payload-custody'
    | 'redaction-manifest'
): ProductionSupportStatusBackendRuntimeClosureRow['runtimeExecutionRef'] {
  return Schema.decodeUnknownSync(RuntimeClosureReferenceSchema)(
    `production-support-status-backend-runtime-closure-${referenceKind}-${target}-${closureState}`
  );
}
