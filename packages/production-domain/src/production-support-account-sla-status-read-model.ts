import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ProductionSupportAccountSlaStatusProofSchema,
  type ProductionSupportAccountSlaStatusRow,
  type ProductionSupportAccountSlaStatusSurface,
} from './production-support-account-sla-status-proof';
import {
  ForbiddenProductionSupportAccountSlaStatusDataClasses,
  RequiredProductionSupportAccountSlaStatusNonClaims,
} from './production-support-account-sla-status-values';

export const ProductionSupportAccountSlaStatusReadModel = ProductionSupportAccountSlaStatusProofSchema.parse({
  schemaVersion: 'production-support-account-sla-status-proof',
  rows: [
    accountSlaStatus('account-lookup-request-status', 'production-incident-support-status-proof', [
      'support-case-status-ref',
      'account-status-ref',
      'parent-consent-reference',
      'manual-proof-reference',
    ]),
    accountSlaStatus('account-lookup-result-boundary', 'billing-support-admin-status-proof', [
      'account-status-ref',
      'redaction-audit-reference',
      'manual-proof-reference',
    ]),
    accountSlaStatus('billing-provider-contact-status', 'billing-support-admin-status-proof', [
      'subscription-status-ref',
      'billing-failure-state-ref',
      'manual-proof-reference',
    ]),
    accountSlaStatus('remote-support-request-status', 'public-support-contact-status-proof', [
      'support-case-status-ref',
      'support-runbook-reference',
      'parent-consent-reference',
    ]),
    accountSlaStatus('remote-support-session-boundary', 'production-support-case-resolution-status-proof', [
      'support-case-status-ref',
      'redaction-audit-reference',
      'manual-proof-reference',
    ]),
    accountSlaStatus('production-sla-status', 'release-installer-expectation', [
      'public-status-reference',
      'support-runbook-reference',
      'manual-proof-reference',
    ]),
  ],
  nonClaims: RequiredProductionSupportAccountSlaStatusNonClaims,
  accountLookupExecutionState: 'manual-required',
  billingProviderContactState: 'manual-required',
  remoteSupportSessionState: 'not-implemented',
  productionSlaState: 'not-implemented',
  supportBackendUploadExecutionState: 'manual-required',
  familyOcentraRuntimeState: 'not-implemented',
  childActivityCustodyState: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T00:15:00.000Z'),
});

export const ProductionSupportAccountSlaStatusKnownGaps = [
  'Account lookup remains parent-visible/manual-required source-contract proof; no account backend lookup is executed.',
  'Billing provider contact remains manual-required and does not contact Stripe or any payment provider.',
  'Remote support requests and sessions remain source-contract/manual-required only; no remote session or transcript is captured.',
  'Production SLA status remains not implemented and does not commit support response time, availability, or escalation guarantees.',
  'Support backend upload execution, family.ocentra.ca runtime, provider secrets, and child activity custody remain unclaimed.',
] as const;

function accountSlaStatus(
  surface: ProductionSupportAccountSlaStatusSurface,
  sourceProof: ProductionSupportAccountSlaStatusRow['sourceProof'],
  supportSafeDataClasses: ProductionSupportAccountSlaStatusRow['supportSafeDataClasses']
) {
  return {
    schemaVersion: 'production-support-account-sla-status-proof',
    surface,
    sourceProof,
    sourceContractState: 'source-contract-ready',
    accountLookupState:
      surface === 'account-lookup-request-status' || surface === 'account-lookup-result-boundary'
        ? 'manual-required'
        : 'source-contract-ready',
    billingProviderContactState:
      surface === 'billing-provider-contact-status' ? 'manual-required' : 'source-contract-ready',
    remoteSupportSessionState:
      surface === 'remote-support-request-status' || surface === 'remote-support-session-boundary'
        ? 'not-implemented'
        : 'source-contract-ready',
    productionSlaState: surface === 'production-sla-status' ? 'not-implemented' : 'source-contract-ready',
    parentVisibleState: surface === 'production-sla-status' ? 'manual-required' : 'parent-visible',
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenProductionSupportAccountSlaStatusDataClasses,
    accountReference: `production-support-account-sla-account-${surface}`,
    supportReference: `production-support-account-sla-support-${surface}`,
    manualRequirement: `${surface}-requires-provider-runtime-remote-support-and-sla-proof`,
  } as const;
}
