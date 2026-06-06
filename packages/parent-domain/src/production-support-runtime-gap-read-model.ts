import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ProductionSupportRuntimeGapProofSchema,
  type ProductionSupportRuntimeGapItem,
  type ProductionSupportRuntimeGapRow,
} from './production-support-runtime-gap-proof';
import { ForbiddenRuntimeGapDataClasses, RequiredRuntimeGapNonClaims } from './production-support-runtime-gap-values';

export const ProductionSupportRuntimeGapReadModel = ProductionSupportRuntimeGapProofSchema.parse({
  schemaVersion: 'production-support-runtime-gap-proof',
  rows: [
    runtimeGap('public-website-runtime-gap', 'production-release-public-runtime-handoff-proof', [
      'public-route-status',
      'manual-proof-reference',
      'source-contract-reference',
    ]),
    runtimeGap('support-publication-execution-gap', 'production-support-publication-workflow-proof', [
      'publication-status',
      'support-runbook-status',
      'manual-proof-reference',
      'source-contract-reference',
    ]),
    runtimeGap('support-backend-upload-execution-gap', 'production-support-backend-upload-execution-runtime-proof', [
      'support-upload-status-summary',
      'manual-proof-reference',
      'source-contract-reference',
    ]),
    runtimeGap('account-billing-provider-runtime-gap', 'production-support-account-sla-status-proof', [
      'account-status-summary',
      'billing-support-status-summary',
      'manual-proof-reference',
      'source-contract-reference',
    ]),
    runtimeGap('legal-export-delete-runtime-gap', 'production-incident-support-status-proof', [
      'legal-review-status',
      'export-delete-status-summary',
      'manual-proof-reference',
      'source-contract-reference',
    ]),
    runtimeGap('remote-support-sla-runtime-gap', 'production-support-account-sla-status-proof', [
      'remote-support-status-summary',
      'manual-proof-reference',
      'source-contract-reference',
    ]),
  ],
  nonClaims: RequiredRuntimeGapNonClaims,
  publicRuntimeClaim: 'not-implemented',
  supportPublicationExecutionClaim: 'manual-required',
  supportBackendUploadExecutionClaim: 'manual-required',
  accountBackendRuntimeClaim: 'backend-required',
  billingProviderRuntimeClaim: 'provider-required',
  legalExportDeleteRuntimeClaim: 'manual-required',
  remoteSupportSessionClaim: 'not-implemented',
  productionSlaClaim: 'not-implemented',
  childActivityCustodyClaim: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T03:15:00.000Z'),
});

export const ProductionSupportRuntimeGapKnownGaps = [
  'Public website, download, account, subscription, and support status surfaces remain source-contract handoffs until a live family.ocentra.ca runtime is implemented and proven.',
  'Support publication, legal disclosure, export/delete runtime, support backend upload execution, account lookup, billing provider contact, remote support sessions, and production SLA remain manual-required, backend-required, provider-required, or not implemented.',
  'The proof contains only support-safe status summaries and source/manual references; it excludes child activity evidence, raw support bundles, provider secrets, account lookup results, billing provider contact records, hosted family data, backend payloads, private paths, screenshots, journals, and SQLite snapshots.',
] as const;

function runtimeGap(
  item: ProductionSupportRuntimeGapItem,
  sourceProof: ProductionSupportRuntimeGapRow['sourceProof'],
  supportSafeDataClasses: ProductionSupportRuntimeGapRow['supportSafeDataClasses']
) {
  return {
    schemaVersion: 'production-support-runtime-gap-proof',
    item,
    sourceProof,
    sourceContractState: 'source-contract-ready',
    runtimeExecutionState: runtimeExecutionStateFor(item),
    backendRuntimeState: backendRuntimeStateFor(item),
    providerRuntimeState: providerRuntimeStateFor(item),
    publicationState:
      item === 'support-publication-execution-gap' || item === 'public-website-runtime-gap'
        ? 'publication-required'
        : 'manual-required',
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenRuntimeGapDataClasses,
    sourceReference: `production-support-runtime-gap-${item}`,
    manualRequirement: `${item}-requires-real-runtime-provider-or-publication-proof-before-product-claim`,
  } as const;
}

function runtimeExecutionStateFor(item: ProductionSupportRuntimeGapItem) {
  return item === 'public-website-runtime-gap' || item === 'remote-support-sla-runtime-gap'
    ? 'not-implemented'
    : 'manual-required';
}

function backendRuntimeStateFor(item: ProductionSupportRuntimeGapItem) {
  return item === 'account-billing-provider-runtime-gap' || item === 'support-backend-upload-execution-gap'
    ? 'backend-required'
    : 'not-implemented';
}

function providerRuntimeStateFor(item: ProductionSupportRuntimeGapItem) {
  return item === 'account-billing-provider-runtime-gap' ? 'provider-required' : 'not-implemented';
}
