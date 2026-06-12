import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ProductionSupportPublicationRuntimeReadinessProofSchema,
  type ProductionSupportPublicationRuntimeReadinessItem,
  type ProductionSupportPublicationRuntimeReadinessRow,
} from './production-support-publication-runtime-readiness-proof';
import {
  ForbiddenPublicationRuntimeReadinessDataClasses,
  RequiredPublicationRuntimeReadinessNonClaims,
} from './production-support-publication-runtime-readiness-values';

export const ProductionSupportPublicationRuntimeReadinessReadModel =
  ProductionSupportPublicationRuntimeReadinessProofSchema.parse({
    schemaVersion: 'production-support-publication-runtime-readiness-proof',
    rows: [
      publicationRuntimeReadiness(
        'public-runtime-publication-adapter-readiness',
        'production-release-public-runtime-handoff-proof',
        ['public-route-status', 'runtime-adapter-reference', 'manual-proof-reference'],
        'adapter-required',
        'manual-required',
        'not-implemented'
      ),
      publicationRuntimeReadiness(
        'support-runbook-publication-runner-readiness',
        'production-support-publication-workflow-proof',
        ['support-runbook-status', 'publication-runner-reference', 'manual-proof-reference'],
        'manual-required',
        'runner-required',
        'manual-required'
      ),
      publicationRuntimeReadiness(
        'incident-status-publication-runner-readiness',
        'production-support-publication-workflow-proof',
        ['incident-status', 'support-runbook-status', 'publication-runner-reference', 'manual-proof-reference'],
        'manual-required',
        'runner-required',
        'manual-required'
      ),
      publicationRuntimeReadiness(
        'support-upload-publication-runtime-readiness',
        'production-support-backend-upload-status-proof',
        ['support-upload-status-summary', 'runtime-adapter-reference', 'manual-proof-reference'],
        'backend-required',
        'manual-required',
        'manual-required'
      ),
      publicationRuntimeReadiness(
        'privacy-legal-publication-runtime-readiness',
        'production-release-public-docs-status-proof',
        ['privacy-policy-status', 'legal-review-status', 'manual-proof-reference'],
        'legal-review-required',
        'manual-required',
        'not-implemented'
      ),
      publicationRuntimeReadiness(
        'public-support-contact-runtime-readiness',
        'public-support-contact-status-proof',
        ['public-route-status', 'support-runbook-status', 'manual-proof-reference'],
        'adapter-required',
        'manual-required',
        'not-implemented'
      ),
    ],
    nonClaims: RequiredPublicationRuntimeReadinessNonClaims,
    publicRuntimeExecutionClaim: 'not-implemented',
    publicationRunnerExecutionClaim: 'manual-required',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountLookupExecutionClaim: 'manual-required',
    billingProviderContactClaim: 'manual-required',
    productionSlaClaim: 'not-implemented',
    legalDisclosureExecutionClaim: 'manual-required',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T02:10:01.650Z'),
  });

export const ProductionSupportPublicationRuntimeReadinessKnownGaps = [
  'Publication runtime readiness remains a source-backed readiness proof; family.ocentra.ca public runtime execution is not implemented.',
  'Support runbook and incident status publication runners remain manual-required until a real publication runner and status backend exist.',
  'Support backend upload publication, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, and child activity custody remain unclaimed.',
] as const;

function publicationRuntimeReadiness(
  item: ProductionSupportPublicationRuntimeReadinessItem,
  sourceProof: ProductionSupportPublicationRuntimeReadinessRow['sourceProof'],
  supportSafeDataClasses: ProductionSupportPublicationRuntimeReadinessRow['supportSafeDataClasses'],
  runtimeAdapterState: ProductionSupportPublicationRuntimeReadinessRow['runtimeAdapterState'],
  publicationRunnerState: ProductionSupportPublicationRuntimeReadinessRow['publicationRunnerState'],
  publicRuntimeState: ProductionSupportPublicationRuntimeReadinessRow['publicRuntimeState']
) {
  return {
    schemaVersion: 'production-support-publication-runtime-readiness-proof',
    item,
    sourceProof,
    sourceContractState: 'source-contract-ready',
    runtimeAdapterState,
    publicationRunnerState,
    supportBackendUploadState:
      item === 'support-upload-publication-runtime-readiness' ? 'manual-required' : 'not-implemented',
    publicRuntimeState,
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenPublicationRuntimeReadinessDataClasses,
    runtimeRef: `production-support-publication-runtime-readiness-${item}`,
    manualRequirement: `${item}-requires-real-public-runtime-publication-runner-and-manual-proof-before-product-claim`,
  } as const;
}
