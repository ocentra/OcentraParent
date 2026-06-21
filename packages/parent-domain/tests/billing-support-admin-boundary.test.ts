import { describe, expect, it } from 'vitest';
import {
  BillingSupportAdminBoundaryProofSchema,
  BillingSupportAdminBoundaryRowSchema,
} from '@ocentra-parent/schema-domain/billing-support-admin-boundary';
import { BillingSupportAdminBoundaryProofReadModel } from '@ocentra-parent/schema-domain/billing-support-admin-boundary-proof';

describe('billing support admin boundary canonical contracts', () => {
  acceptsBillingSupportAdminBoundaryProof();
  rejectsProviderContactExecution();
  rejectsManualActionsWithoutManualRequiredState();
  rejectsSupportRowsWithoutRedactionAuditRefs();
  rejectsProofsWithMissingAdminNonClaims();
});

function acceptsBillingSupportAdminBoundaryProof(): void {
  it('accepts billing support admin proof without provider contact portal UI or child activity custody', () => {
    const proof = BillingSupportAdminBoundaryProofSchema.parse(BillingSupportAdminBoundaryProofReadModel);

    expect(
      summarizeValues(
        proof.rows.map((entry) => entry.action),
        [
          'support-case-triage',
          'account-status-review',
          'billing-escalation-request',
          'provider-contact-manual-required',
          'entitlement-admin-override-manual-required',
          'refund-credit-manual-required',
        ]
      )
    ).toEqual({
      'support-case-triage': 1,
      'account-status-review': 1,
      'billing-escalation-request': 1,
      'provider-contact-manual-required': 1,
      'entitlement-admin-override-manual-required': 1,
      'refund-credit-manual-required': 1,
    });
    expect(
      summarizeValues(
        proof.rows.map((entry) => entry.runtimeState),
        ['read-only-local-proof', 'manual-required', 'not-implemented']
      )
    ).toEqual({
      'read-only-local-proof': 2,
      'manual-required': 2,
      'not-implemented': 2,
    });
    expect(proof.nonClaims).toEqual([
      'no-stripe-sdk',
      'no-provider-secrets',
      'no-billing-provider-contact',
      'no-account-backend-admin-runtime',
      'no-entitlement-admin-override-runtime',
      'no-refund-credit-runtime',
      'no-portal-admin-ui',
      'no-support-backend-upload',
      'no-child-activity-custody',
    ]);
    expect(proof.portalUiClaim).toBe('not-implemented');
    expect(proof.providerContactClaim).toBe('not-executed');
    expect(proof.backendUploadClaim).toBe('not-executed');
    expect(proof.childActivityCustodyClaim).toBe('not-included');
  });
}

function rejectsProviderContactExecution(): void {
  it('rejects billing support admin rows that execute provider contact or keep provider secrets', () => {
    const providerContact = requiredRow('provider-contact-manual-required');

    expect(
      BillingSupportAdminBoundaryRowSchema.safeParse({
        ...providerContact,
        providerContacted: true,
      }).success
    ).toBe(false);
    expect(
      BillingSupportAdminBoundaryRowSchema.safeParse({
        ...providerContact,
        providerSecretCustody: 'stored-in-portal',
      }).success
    ).toBe(false);
  });
}

function rejectsManualActionsWithoutManualRequiredState(): void {
  it('rejects provider contact admin rows without manual-required state and failure context', () => {
    const providerContact = requiredRow('provider-contact-manual-required');

    expect(
      BillingSupportAdminBoundaryRowSchema.safeParse({
        ...providerContact,
        manualRequired: false,
      }).success
    ).toBe(false);
    expect(
      BillingSupportAdminBoundaryRowSchema.safeParse({
        ...providerContact,
        failureState: null,
      }).success
    ).toBe(false);
  });
}

function rejectsSupportRowsWithoutRedactionAuditRefs(): void {
  it('rejects support admin rows without a redaction audit data-class reference', () => {
    const triage = requiredRow('support-case-triage');

    expect(
      BillingSupportAdminBoundaryRowSchema.safeParse({
        ...triage,
        disclosedDataClasses: ['support-case-status-ref', 'account-status-ref'],
      }).success
    ).toBe(false);
  });
}

function rejectsProofsWithMissingAdminNonClaims(): void {
  it('rejects proof rows when billing provider contact is not an explicit non-claim', () => {
    const proof = BillingSupportAdminBoundaryProofReadModel;

    expect(
      BillingSupportAdminBoundaryProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-billing-provider-contact'),
      }).success
    ).toBe(false);
  });
}

function requiredRow(action: 'support-case-triage' | 'provider-contact-manual-required') {
  const row = BillingSupportAdminBoundaryProofReadModel.rows.find((entry) => entry.action === action);
  if (row === undefined) {
    throw new Error(`missing billing support admin row: ${action}`);
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
