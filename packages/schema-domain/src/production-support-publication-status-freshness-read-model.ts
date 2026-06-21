import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ProductionSupportPublicationStatusFreshnessProofSchema,
  type ProductionSupportPublicationStatusFreshnessRow,
  type ProductionSupportPublicationStatusFreshnessSurface,
} from './production-support-publication-status-freshness-proof';
import {
  ForbiddenPublicationStatusFreshnessDataClasses,
  RequiredPublicationStatusFreshnessNonClaims,
} from './production-support-publication-status-freshness-values';

export const ProductionSupportPublicationStatusFreshnessReadModel =
  ProductionSupportPublicationStatusFreshnessProofSchema.parse({
    schemaVersion: 'production-support-publication-status-freshness-proof',
    rows: [
      freshnessRow('support-runbook-publication-freshness', 'production-support-publication-workflow-proof', [
        'support-runbook-status',
        'freshness-policy-reference',
        'manual-proof-reference',
        'publication-reference',
      ]),
      freshnessRow('incident-status-publication-freshness', 'production-support-publication-workflow-proof', [
        'incident-status',
        'support-runbook-status',
        'freshness-policy-reference',
        'publication-reference',
      ]),
      freshnessRow('public-support-contact-publication-freshness', 'public-support-contact-status-proof', [
        'public-support-contact-status',
        'support-runbook-status',
        'freshness-policy-reference',
        'manual-proof-reference',
      ]),
      freshnessRow('support-backend-upload-publication-freshness', 'production-support-publication-workflow-proof', [
        'support-upload-status-summary',
        'support-runbook-status',
        'freshness-policy-reference',
        'manual-proof-reference',
      ]),
      freshnessRow('privacy-legal-publication-freshness', 'production-release-public-docs-freshness-proof', [
        'privacy-policy-status',
        'legal-review-status',
        'freshness-policy-reference',
        'publication-reference',
      ]),
      freshnessRow(
        'account-billing-support-publication-freshness',
        'production-release-public-status-freshness-proof',
        ['account-status-summary', 'billing-support-status', 'freshness-policy-reference', 'manual-proof-reference']
      ),
    ],
    nonClaims: RequiredPublicationStatusFreshnessNonClaims,
    publicRuntimeClaim: 'not-implemented',
    supportPublicationExecutionClaim: 'manual-required',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountLookupExecutionClaim: 'manual-required',
    billingProviderContactClaim: 'manual-required',
    productionSlaClaim: 'not-implemented',
    legalDisclosureExecutionClaim: 'manual-required',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T01:34:33.000Z'),
  });

export const ProductionSupportPublicationStatusFreshnessKnownGaps = [
  'Support publication/status freshness rows are source-contract proof only; family.ocentra.ca publication remains unimplemented.',
  'Support publication execution, support backend upload execution, account lookup, billing provider contact, legal disclosure execution, and production SLA remain manual-required or not implemented.',
  'No child activity evidence, raw support bundle payload, provider secret, account lookup result, billing contact record, parent rule source, or remote support transcript is included.',
] as const;

function freshnessRow(
  surface: ProductionSupportPublicationStatusFreshnessSurface,
  sourceProof: ProductionSupportPublicationStatusFreshnessRow['sourceProof'],
  supportSafeDataClasses: ProductionSupportPublicationStatusFreshnessRow['supportSafeDataClasses']
) {
  return {
    schemaVersion: 'production-support-publication-status-freshness-proof',
    surface,
    sourceProof,
    sourceContractState: 'source-contract-ready',
    freshnessPolicyState: 'freshness-policy-ready',
    publicPublicationState:
      surface === 'public-support-contact-publication-freshness' ? 'publication-required' : 'manual-required',
    publicRuntimeState: 'not-implemented',
    supportBackendUploadState:
      surface === 'support-backend-upload-publication-freshness' ? 'manual-required' : 'not-implemented',
    legalExecutionState: surface === 'privacy-legal-publication-freshness' ? 'manual-required' : 'not-implemented',
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenPublicationStatusFreshnessDataClasses,
    freshnessReference: `production-support-publication-status-freshness-${surface}`,
    manualRequirement: `${surface}-requires-public-runtime-publication-smoke-and-freshness-proof-before-product-claim`,
  } as const;
}
