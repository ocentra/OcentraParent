import { describe, expect, it } from 'vitest';
import {
  BillingSupportAdminStatusProofReadModel,
  BillingSupportAdminStatusProofRowSchema,
  BillingSupportAdminStatusProofSchema,
} from '@ocentra-parent/schema-domain/billing-support-admin-status-proof';

describe('billing support admin status canonical proof', () => {
  acceptsBillingSupportAdminStatusProof();
  rejectsProviderOrBackendExecution();
  rejectsManualRowsWithoutManualProofRefs();
  rejectsRowsWithoutBoundaryProofRefs();
  rejectsProofsWithMissingNonClaims();
});

function acceptsBillingSupportAdminStatusProof(): void {
  it('accepts support billing admin status rows without provider account lookup upload or child custody claims', () => {
    const proof = BillingSupportAdminStatusProofSchema.parse(BillingSupportAdminStatusProofReadModel);

    expect(
      summarizeValues(
        proof.rows.map((entry) => entry.statusRow),
        [
          'case-triage-visible',
          'account-review-visible',
          'billing-escalation-visible',
          'provider-contact-manual-required',
          'entitlement-override-manual-required',
          'refund-credit-manual-required',
          'resolution-update-ready',
        ]
      )
    ).toEqual({
      'case-triage-visible': 1,
      'account-review-visible': 1,
      'billing-escalation-visible': 1,
      'provider-contact-manual-required': 1,
      'entitlement-override-manual-required': 1,
      'refund-credit-manual-required': 1,
      'resolution-update-ready': 1,
    });
    expect(
      summarizeValues(
        proof.rows.map((entry) => entry.runtimeState),
        ['source-contract-ready', 'manual-required', 'not-implemented']
      )
    ).toEqual({
      'source-contract-ready': 3,
      'manual-required': 2,
      'not-implemented': 2,
    });
    expect(proof.nonClaims).toEqual([
      'no-stripe-sdk',
      'no-provider-secrets',
      'no-billing-provider-contact-execution',
      'no-account-lookup-execution',
      'no-entitlement-admin-override-runtime',
      'no-refund-credit-runtime',
      'no-portal-admin-ui',
      'no-support-backend-upload',
      'no-child-activity-custody',
    ]);
    expect(proof.providerClaim).toBe('not-executed');
    expect(proof.portalAdminUiClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-included');
  });
}

function rejectsProviderOrBackendExecution(): void {
  it('rejects status rows that execute provider contact account lookup support upload or refund runtime', () => {
    const providerContact = requiredRow('provider-contact-manual-required');

    expect(
      BillingSupportAdminStatusProofRowSchema.safeParse({
        ...providerContact,
        providerContactExecuted: true,
      }).success
    ).toBe(false);
    expect(
      BillingSupportAdminStatusProofRowSchema.safeParse({
        ...providerContact,
        accountLookupExecuted: true,
      }).success
    ).toBe(false);
    expect(
      BillingSupportAdminStatusProofRowSchema.safeParse({
        ...providerContact,
        supportBackendUploadExecuted: true,
      }).success
    ).toBe(false);
    expect(
      BillingSupportAdminStatusProofRowSchema.safeParse({
        ...providerContact,
        refundCreditIssued: true,
      }).success
    ).toBe(false);
  });
}

function rejectsManualRowsWithoutManualProofRefs(): void {
  it('rejects manual-required rows without manual proof and manual-required status', () => {
    const providerContact = requiredRow('provider-contact-manual-required');

    expect(
      BillingSupportAdminStatusProofRowSchema.safeParse({
        ...providerContact,
        manualRequired: false,
      }).success
    ).toBe(false);
    expect(
      BillingSupportAdminStatusProofRowSchema.safeParse({
        ...providerContact,
        disclosedDataClasses: ['account-status-ref', 'billing-failure-state-ref', 'redaction-audit-ref'],
      }).success
    ).toBe(false);
  });
}

function rejectsRowsWithoutBoundaryProofRefs(): void {
  it('rejects status rows that do not retain the billing support admin boundary proof reference', () => {
    const resolution = requiredRow('resolution-update-ready');

    expect(
      BillingSupportAdminStatusProofRowSchema.safeParse({
        ...resolution,
        proofRefs: ['billing-entitlement-runtime-proof'],
      }).success
    ).toBe(false);
  });
}

function rejectsProofsWithMissingNonClaims(): void {
  it('rejects proofs that omit provider contact execution as a non-claim', () => {
    const proof = BillingSupportAdminStatusProofReadModel;

    expect(
      BillingSupportAdminStatusProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-billing-provider-contact-execution'),
      }).success
    ).toBe(false);
  });
}

function requiredRow(statusRow: 'provider-contact-manual-required' | 'resolution-update-ready') {
  const row = BillingSupportAdminStatusProofReadModel.rows.find((entry) => entry.statusRow === statusRow);
  if (row === undefined) {
    throw new Error(`missing billing support admin status row: ${statusRow}`);
  }
  return row;
}

function summarizeValues<T extends string>(
  values: ReadonlyArray<T>,
  expectedValues: ReadonlyArray<T>
): Record<T, number> {
  const counts = new Map<T, number>(expectedValues.map((value) => [value, 0]));
  for (const value of values) {
    counts.set(value, (counts.get(value) ?? 0) + 1);
  }
  return Object.fromEntries(counts) as Record<T, number>;
}
