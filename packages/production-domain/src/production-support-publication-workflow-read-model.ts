import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ProductionSupportPublicationWorkflowProofSchema,
  type ProductionSupportPublicationWorkflowItem,
  type ProductionSupportPublicationWorkflowRow,
} from './production-support-publication-workflow';
import {
  ForbiddenPublicationWorkflowDataClasses,
  RequiredPublicationWorkflowNonClaims,
} from './production-support-publication-workflow-values';

export const ProductionSupportPublicationWorkflowReadModel = ProductionSupportPublicationWorkflowProofSchema.parse({
  schemaVersion: 'production-support-publication-workflow-proof',
  rows: [
    publicationWorkflow('public-privacy-policy-publication', 'production-release-public-docs-status-proof', [
      'public-policy-text',
      'retention-summary',
      'export-delete-summary',
      'manual-proof-reference',
      'publication-reference',
    ]),
    publicationWorkflow('privacy-legal-disclosure-execution', 'production-release-public-docs-status-proof', [
      'public-policy-text',
      'legal-review-status',
      'manual-proof-reference',
      'publication-reference',
    ]),
    publicationWorkflow('support-runbook-publication', 'production-release-public-docs-status-proof', [
      'support-runbook-status',
      'manual-proof-reference',
      'publication-reference',
    ]),
    publicationWorkflow('support-incident-status-publication', 'production-release-public-surface-publication-proof', [
      'incident-status',
      'support-runbook-status',
      'manual-proof-reference',
      'publication-reference',
    ]),
    publicationWorkflow(
      'support-backend-upload-publication-handoff',
      'production-support-backend-upload-status-proof',
      ['support-upload-status-summary', 'support-runbook-status', 'manual-proof-reference']
    ),
    publicationWorkflow('public-support-contact-publication', 'documentation-expectation', [
      'support-runbook-status',
      'legal-review-status',
      'manual-proof-reference',
      'publication-reference',
    ]),
  ],
  nonClaims: RequiredPublicationWorkflowNonClaims,
  publicRuntimeClaim: 'not-implemented',
  legalExecutionClaim: 'manual-required',
  supportBackendUploadExecutionClaim: 'manual-required',
  accountLookupExecutionClaim: 'manual-required',
  billingProviderContactClaim: 'manual-required',
  productionSlaClaim: 'not-implemented',
  childActivityCustodyClaim: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-05T10:45:11.117Z'),
});

export const ProductionSupportPublicationWorkflowKnownGaps = [
  'Public publication workflow rows are source-contract proof only; family.ocentra.ca runtime publication remains unimplemented.',
  'Privacy/legal disclosure execution, support backend upload execution, account lookup execution, billing provider contact, and production SLA remain manual-required or not implemented.',
  'No child activity evidence, raw support bundle payload, provider secret, account lookup result, billing contact record, parent rule source, or remote support transcript is included.',
] as const;

function publicationWorkflow(
  item: ProductionSupportPublicationWorkflowItem,
  sourceProof: ProductionSupportPublicationWorkflowRow['sourceProof'],
  supportSafeDataClasses: ProductionSupportPublicationWorkflowRow['supportSafeDataClasses']
) {
  return {
    schemaVersion: 'production-support-publication-workflow-proof',
    item,
    sourceProof,
    sourceContractState: 'source-contract-ready',
    publicPublicationState: item === 'public-support-contact-publication' ? 'publication-required' : 'manual-required',
    legalExecutionState: item === 'privacy-legal-disclosure-execution' ? 'manual-required' : 'legal-review-required',
    supportBackendUploadState:
      item === 'support-backend-upload-publication-handoff' ? 'manual-required' : 'not-implemented',
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenPublicationWorkflowDataClasses,
    publicationReference: `production-support-publication-workflow-${item}`,
    manualRequirement: `${item}-requires-public-publication-legal-review-and-runtime-proof-before-product-claim`,
  } as const;
}
