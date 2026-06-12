import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ProductionSupportPublicationExecutionStatusProofSchema,
  type ProductionSupportPublicationExecutionLifecycleStatus,
  type ProductionSupportPublicationExecutionStatusRow,
  type ProductionSupportPublicationExecutionStatusTarget,
} from './production-support-publication-execution-status-proof';
import {
  ForbiddenPublicationExecutionStatusDataClasses,
  PublicationExecutionStatusManualRequirementSchema,
  PublicationExecutionStatusReferenceSchema,
  RequiredPublicationExecutionStatusLifecycleStates,
  RequiredPublicationExecutionStatusNonClaims,
  RequiredPublicationExecutionStatusTargets,
} from './production-support-publication-execution-status-values';

const SafeDataClassesByTarget: Record<
  ProductionSupportPublicationExecutionStatusTarget,
  ProductionSupportPublicationExecutionStatusRow['supportSafeDataClasses']
> = {
  'support-runbook-publication-execution': [
    'publication-status-label',
    'support-runbook-status',
    'manual-proof-reference',
    'runtime-readiness-reference',
    'freshness-policy-reference',
  ],
  'incident-status-publication-execution': [
    'publication-status-label',
    'incident-status',
    'manual-proof-reference',
    'runtime-readiness-reference',
    'freshness-policy-reference',
  ],
  'public-support-contact-publication-execution': [
    'publication-status-label',
    'public-support-contact-status',
    'manual-proof-reference',
    'runtime-readiness-reference',
    'freshness-policy-reference',
  ],
  'support-backend-upload-publication-execution': [
    'publication-status-label',
    'support-upload-status-summary',
    'manual-proof-reference',
    'runtime-readiness-reference',
    'freshness-policy-reference',
  ],
  'privacy-legal-publication-execution': [
    'publication-status-label',
    'privacy-policy-status',
    'legal-review-status',
    'manual-proof-reference',
    'runtime-readiness-reference',
    'freshness-policy-reference',
  ],
  'account-billing-support-publication-execution': [
    'publication-status-label',
    'account-status-summary',
    'billing-support-status',
    'manual-proof-reference',
    'runtime-readiness-reference',
    'freshness-policy-reference',
  ],
};

export const ProductionSupportPublicationExecutionStatusReadModel =
  ProductionSupportPublicationExecutionStatusProofSchema.parse({
    schemaVersion: 'production-support-publication-execution-status-proof',
    rows: RequiredPublicationExecutionStatusTargets.flatMap((target) =>
      RequiredPublicationExecutionStatusLifecycleStates.map((lifecycleStatus) =>
        publicationExecutionStatus(target, lifecycleStatus)
      )
    ),
    nonClaims: RequiredPublicationExecutionStatusNonClaims,
    publicRuntimeExecutionClaim: 'not-implemented',
    publicationRunnerExecutionClaim: 'manual-required',
    statusBackendExecutionClaim: 'manual-required',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountLookupExecutionClaim: 'manual-required',
    billingProviderContactClaim: 'manual-required',
    productionSlaClaim: 'not-implemented',
    legalDisclosureExecutionClaim: 'manual-required',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T05:18:00.000Z'),
  });

export const ProductionSupportPublicationExecutionStatusKnownGaps = [
  'Publication execution status remains a source-contract proof; no family.ocentra.ca public runtime executes these rows.',
  'Support runbook, incident status, contact, upload, privacy/legal, and account/billing publication runners remain manual-required until real runner and status backend evidence exists.',
  'Support backend upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, provider-secret custody, and child activity custody remain unclaimed.',
] as const;

function publicationExecutionStatus(
  target: ProductionSupportPublicationExecutionStatusTarget,
  lifecycleStatus: ProductionSupportPublicationExecutionLifecycleStatus
): ProductionSupportPublicationExecutionStatusRow {
  return {
    schemaVersion: 'production-support-publication-execution-status-proof',
    target,
    sourceProof: sourceProofForTarget(target),
    lifecycleStatus,
    sourceContractState: 'source-contract-ready',
    statusContractState: 'status-contract-ready',
    publicRuntimeState: publicRuntimeStateForTarget(target),
    publicationRunnerState: 'manual-required',
    statusBackendState:
      target === 'public-support-contact-publication-execution' ? 'backend-required' : 'manual-required',
    supportBackendUploadState:
      target === 'support-backend-upload-publication-execution' ? 'manual-required' : 'not-implemented',
    legalExecutionState: target === 'privacy-legal-publication-execution' ? 'legal-review-required' : 'not-implemented',
    supportSafeDataClasses: SafeDataClassesByTarget[target],
    forbiddenDataClasses: ForbiddenPublicationExecutionStatusDataClasses,
    statusReference: Schema.decodeUnknownSync(PublicationExecutionStatusReferenceSchema)(
      `production-support-publication-execution-status-${target}-${lifecycleStatus}`
    ),
    manualRequirement: Schema.decodeUnknownSync(PublicationExecutionStatusManualRequirementSchema)(
      `${target}-${lifecycleStatus}-requires-real-public-runtime-publication-runner-status-backend-and-manual-proof-before-product-claim`
    ),
  };
}

function sourceProofForTarget(
  target: ProductionSupportPublicationExecutionStatusTarget
): ProductionSupportPublicationExecutionStatusRow['sourceProof'] {
  if (target === 'privacy-legal-publication-execution') {
    return 'production-release-public-docs-freshness-proof';
  }
  if (target === 'public-support-contact-publication-execution') {
    return 'public-support-contact-status-proof';
  }
  if (target === 'account-billing-support-publication-execution') {
    return 'production-support-publication-status-freshness-proof';
  }
  return 'production-support-publication-runtime-readiness-proof';
}

function publicRuntimeStateForTarget(
  target: ProductionSupportPublicationExecutionStatusTarget
): ProductionSupportPublicationExecutionStatusRow['publicRuntimeState'] {
  return target === 'support-backend-upload-publication-execution' ? 'backend-required' : 'not-implemented';
}
