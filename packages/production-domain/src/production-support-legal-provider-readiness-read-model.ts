import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ProductionSupportLegalProviderReadinessProofSchema,
  type ProductionSupportLegalProviderReadinessRow,
  type ProductionSupportLegalProviderReadinessSurface,
} from './production-support-legal-provider-readiness-proof';
import {
  ForbiddenProductionSupportLegalProviderReadinessDataClasses,
  RequiredProductionSupportLegalProviderReadinessNonClaims,
} from './production-support-legal-provider-readiness-values';

export const ProductionSupportLegalProviderReadinessReadModel =
  ProductionSupportLegalProviderReadinessProofSchema.parse({
    schemaVersion: 'production-support-legal-provider-readiness-proof',
    rows: [
      readinessRow('privacy-legal-review-readiness', 'production-release-public-docs-status-proof', [
        'privacy-policy-status-ref',
        'legal-disclosure-status-ref',
        'manual-proof-reference',
      ]),
      readinessRow('data-export-delete-runtime-readiness', 'production-incident-support-status-proof', [
        'export-delete-status-ref',
        'data-custody-reference',
        'manual-proof-reference',
      ]),
      readinessRow('provider-secret-custody-boundary', 'billing-expectation', [
        'provider-boundary-reference',
        'redaction-audit-reference',
        'manual-proof-reference',
      ]),
      readinessRow('billing-provider-contact-readiness', 'billing-support-admin-status-proof', [
        'billing-status-ref',
        'provider-boundary-reference',
        'manual-proof-reference',
      ]),
      readinessRow('remote-support-legal-session-boundary', 'production-support-case-resolution-status-proof', [
        'support-case-status-ref',
        'redaction-audit-reference',
        'support-runbook-reference',
      ]),
      readinessRow('production-sla-legal-boundary', 'release-installer-expectation', [
        'support-runbook-reference',
        'legal-disclosure-status-ref',
        'manual-proof-reference',
      ]),
    ],
    nonClaims: RequiredProductionSupportLegalProviderReadinessNonClaims,
    legalDisclosureExecutionState: 'manual-required',
    dataExportDeleteRuntimeState: 'manual-required',
    providerSecretCustodyState: 'not-implemented',
    billingProviderContactExecutionState: 'manual-required',
    accountLookupExecutionState: 'manual-required',
    remoteSupportSessionState: 'not-implemented',
    productionSlaState: 'not-implemented',
    supportBackendUploadExecutionState: 'manual-required',
    publicRuntimeExecutionState: 'not-implemented',
    childActivityCustodyState: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T02:36:00.000Z'),
  });

export const ProductionSupportLegalProviderReadinessKnownGaps = [
  'Legal disclosure execution remains manual-required; no legal publication or legal review execution is claimed.',
  'Data export/delete runtime remains manual-required and does not create, upload, retain, or delete hosted family data.',
  'Provider secret custody remains not implemented; no payment, support, storage, or account provider secrets are stored.',
  'Billing provider contact and account lookup remain manual-required and do not contact Stripe or any provider backend.',
  'Remote support sessions, production SLA commitments, support backend upload execution, public runtime execution, and child activity custody remain unclaimed.',
] as const;

function readinessRow(
  surface: ProductionSupportLegalProviderReadinessSurface,
  sourceProof: ProductionSupportLegalProviderReadinessRow['sourceProof'],
  readinessReferences: ProductionSupportLegalProviderReadinessRow['readinessReferences']
) {
  return {
    schemaVersion: 'production-support-legal-provider-readiness-proof',
    surface,
    sourceProof,
    sourceContractState: 'source-contract-ready',
    legalDisclosureState:
      surface === 'privacy-legal-review-readiness' || surface === 'production-sla-legal-boundary'
        ? 'manual-required'
        : 'source-contract-ready',
    dataExportDeleteState:
      surface === 'data-export-delete-runtime-readiness' ? 'manual-required' : 'source-contract-ready',
    providerSecretCustodyState:
      surface === 'provider-secret-custody-boundary' ? 'not-implemented' : 'source-contract-ready',
    billingProviderContactState:
      surface === 'billing-provider-contact-readiness' ? 'manual-required' : 'source-contract-ready',
    remoteSupportSessionState:
      surface === 'remote-support-legal-session-boundary' ? 'not-implemented' : 'source-contract-ready',
    productionSlaState: surface === 'production-sla-legal-boundary' ? 'not-implemented' : 'source-contract-ready',
    supportSafeDataClasses: supportSafeDataClassesFor(surface),
    forbiddenDataClasses: ForbiddenProductionSupportLegalProviderReadinessDataClasses,
    readinessReferences,
    manualRequirement: `${surface}-requires-legal-provider-runtime-proof`,
  } as const;
}

function supportSafeDataClassesFor(surface: ProductionSupportLegalProviderReadinessSurface) {
  switch (surface) {
    case 'privacy-legal-review-readiness':
      return ['public-policy-status', 'legal-disclosure-status', 'manual-proof-status'] as const;
    case 'data-export-delete-runtime-readiness':
      return ['export-delete-status', 'redaction-audit-status', 'manual-proof-status'] as const;
    case 'provider-secret-custody-boundary':
      return ['provider-boundary-status', 'redaction-audit-status', 'manual-proof-status'] as const;
    case 'billing-provider-contact-readiness':
      return ['billing-status', 'provider-boundary-status', 'manual-proof-status'] as const;
    case 'remote-support-legal-session-boundary':
      return ['support-case-status', 'redaction-audit-status', 'manual-proof-status'] as const;
    case 'production-sla-legal-boundary':
      return ['legal-disclosure-status', 'support-case-status', 'manual-proof-status'] as const;
  }
}
