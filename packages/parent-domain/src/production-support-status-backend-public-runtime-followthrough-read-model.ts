import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ProductionSupportStatusBackendPublicRuntimeFollowthroughProofSchema,
  type ProductionSupportStatusBackendPublicRuntimeFollowthroughRow,
  type ProductionSupportStatusBackendPublicRuntimeFollowthroughState,
  type ProductionSupportStatusBackendPublicRuntimeFollowthroughTarget,
} from './production-support-status-backend-public-runtime-followthrough-proof';
import {
  ForbiddenStatusBackendPublicRuntimeFollowthroughDataClasses,
  RequiredStatusBackendPublicRuntimeFollowthroughNonClaims,
  RequiredStatusBackendPublicRuntimeFollowthroughStates,
  RequiredStatusBackendPublicRuntimeFollowthroughTargets,
  StatusBackendPublicRuntimeFollowthroughManualRequirementSchema,
  StatusBackendPublicRuntimeFollowthroughReferenceSchema,
} from './production-support-status-backend-public-runtime-followthrough-values';

const SafeDataClassesByTarget: Record<
  ProductionSupportStatusBackendPublicRuntimeFollowthroughTarget,
  ProductionSupportStatusBackendPublicRuntimeFollowthroughRow['supportSafeDataClasses']
> = {
  'support-status-public-runtime-followthrough': [
    'publication-status-label',
    'support-runbook-status',
    'incident-status',
    'public-runtime-handoff-reference',
    'manual-proof-reference',
  ],
  'support-runbook-status-backend-followthrough': [
    'publication-status-label',
    'support-runbook-status',
    'status-backend-handoff-reference',
    'manual-proof-reference',
  ],
  'incident-status-backend-followthrough': [
    'publication-status-label',
    'incident-status',
    'status-backend-handoff-reference',
    'manual-proof-reference',
  ],
  'public-support-contact-status-backend-followthrough': [
    'publication-status-label',
    'public-support-contact-status',
    'status-backend-handoff-reference',
    'manual-proof-reference',
  ],
  'support-upload-status-backend-followthrough': [
    'publication-status-label',
    'support-upload-status-summary',
    'status-backend-handoff-reference',
    'manual-proof-reference',
  ],
  'account-billing-status-backend-followthrough': [
    'publication-status-label',
    'account-status-summary',
    'billing-support-status',
    'status-backend-handoff-reference',
    'manual-proof-reference',
  ],
};

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughReadModel =
  ProductionSupportStatusBackendPublicRuntimeFollowthroughProofSchema.parse({
    schemaVersion: 'production-support-status-backend-public-runtime-followthrough-proof',
    rows: RequiredStatusBackendPublicRuntimeFollowthroughTargets.flatMap((target) =>
      RequiredStatusBackendPublicRuntimeFollowthroughStates.map((followthroughState) =>
        statusBackendPublicRuntimeFollowthrough(target, followthroughState)
      )
    ),
    nonClaims: RequiredStatusBackendPublicRuntimeFollowthroughNonClaims,
    publicRuntimeExecutionClaim: 'not-implemented',
    statusBackendExecutionClaim: 'manual-required',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountLookupExecutionClaim: 'manual-required',
    billingProviderContactClaim: 'manual-required',
    productionSlaClaim: 'not-implemented',
    legalDisclosureExecutionClaim: 'manual-required',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T10:30:00.000Z'),
  });

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughKnownGaps = [
  'Status backend/public runtime follow-through remains a deterministic contract proof; no family.ocentra.ca public runtime executes these rows.',
  'Status backend execution and durable queue execution remain manual-required until real backend evidence exists.',
  'Support backend upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, provider-secret custody, and child activity custody remain unclaimed.',
] as const;

function statusBackendPublicRuntimeFollowthrough(
  target: ProductionSupportStatusBackendPublicRuntimeFollowthroughTarget,
  followthroughState: ProductionSupportStatusBackendPublicRuntimeFollowthroughState
): ProductionSupportStatusBackendPublicRuntimeFollowthroughRow {
  return {
    schemaVersion: 'production-support-status-backend-public-runtime-followthrough-proof',
    target,
    sourceProof: sourceProofForTarget(target),
    followthroughState,
    sourceContractState: 'source-contract-ready',
    statusContractState: 'status-contract-ready',
    publicRuntimeFollowthroughState: publicRuntimeStateForTarget(target),
    statusBackendFollowthroughState: statusBackendStateForTarget(target),
    supportBackendUploadState:
      target === 'support-upload-status-backend-followthrough' ? 'manual-required' : 'not-implemented',
    supportSafeDataClasses: SafeDataClassesByTarget[target],
    forbiddenDataClasses: ForbiddenStatusBackendPublicRuntimeFollowthroughDataClasses,
    publicRuntimeReference: Schema.decodeUnknownSync(StatusBackendPublicRuntimeFollowthroughReferenceSchema)(
      `production-support-public-runtime-followthrough-${target}-${followthroughState}`
    ),
    statusBackendReference: Schema.decodeUnknownSync(StatusBackendPublicRuntimeFollowthroughReferenceSchema)(
      `production-support-status-backend-followthrough-${target}-${followthroughState}`
    ),
    manualRequirement: Schema.decodeUnknownSync(StatusBackendPublicRuntimeFollowthroughManualRequirementSchema)(
      `${target}-${followthroughState}-requires-real-status-backend-public-runtime-and-manual-proof-before-product-claim`
    ),
  };
}

function sourceProofForTarget(
  target: ProductionSupportStatusBackendPublicRuntimeFollowthroughTarget
): ProductionSupportStatusBackendPublicRuntimeFollowthroughRow['sourceProof'] {
  if (target === 'support-status-public-runtime-followthrough') {
    return 'production-release-public-runtime-handoff-proof';
  }
  if (target === 'public-support-contact-status-backend-followthrough') {
    return 'public-support-contact-status-proof';
  }
  if (target === 'account-billing-status-backend-followthrough') {
    return 'production-support-publication-status-freshness-proof';
  }
  if (target === 'support-upload-status-backend-followthrough') {
    return 'production-support-publication-runtime-readiness-proof';
  }
  return 'production-support-publication-execution-status-proof';
}

function publicRuntimeStateForTarget(
  target: ProductionSupportStatusBackendPublicRuntimeFollowthroughTarget
): ProductionSupportStatusBackendPublicRuntimeFollowthroughRow['publicRuntimeFollowthroughState'] {
  return target === 'support-status-public-runtime-followthrough' ? 'public-runtime-required' : 'not-implemented';
}

function statusBackendStateForTarget(
  target: ProductionSupportStatusBackendPublicRuntimeFollowthroughTarget
): ProductionSupportStatusBackendPublicRuntimeFollowthroughRow['statusBackendFollowthroughState'] {
  return target === 'support-status-public-runtime-followthrough' ? 'manual-required' : 'backend-required';
}
