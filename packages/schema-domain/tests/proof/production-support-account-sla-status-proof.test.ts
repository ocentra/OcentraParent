import { describe, expect, it } from 'vitest';
import {
  ProductionSupportAccountSlaStatusProofSchema,
  ProductionSupportAccountSlaStatusRowSchema,
  summarizeProductionSupportAccountSlaStatusRows,
} from '@ocentra-parent/schema-domain/production-support-account-sla-status-proof';
import { ProductionSupportAccountSlaStatusReadModel } from '@ocentra-parent/schema-domain/production-support-account-sla-status-read-model';

describe('production support account SLA status proof', () => {
  acceptsAccountSlaStatusRows();
  rejectsAccountBillingRemoteAndSlaOverclaims();
  rejectsProviderAndCustodyData();
  rejectsIncompleteAccountSlaCoverage();
});

function acceptsAccountSlaStatusRows(): void {
  it('accepts account lookup, billing contact, remote support, and SLA status rows as non-executing proof', () => {
    const proof = ProductionSupportAccountSlaStatusProofSchema.parse(ProductionSupportAccountSlaStatusReadModel);

    expect(summarizeProductionSupportAccountSlaStatusRows(proof.rows)).toEqual({
      'account-lookup-request-status': 1,
      'account-lookup-result-boundary': 1,
      'billing-provider-contact-status': 1,
      'remote-support-request-status': 1,
      'remote-support-session-boundary': 1,
      'production-sla-status': 1,
    });
    expect(proof.accountLookupExecutionState).toBe('manual-required');
    expect(proof.billingProviderContactState).toBe('manual-required');
    expect(proof.remoteSupportSessionState).toBe('not-implemented');
    expect(proof.productionSlaState).toBe('not-implemented');
    expect(proof.supportBackendUploadExecutionState).toBe('manual-required');
    expect(proof.familyOcentraRuntimeState).toBe('not-implemented');
    expect(proof.childActivityCustodyState).toBe('not-implemented');
  });
}

function rejectsAccountBillingRemoteAndSlaOverclaims(): void {
  it('rejects account lookup, billing contact, remote session, and SLA execution claims', () => {
    const accountLookup = requiredStatus('account-lookup-result-boundary');
    const billingContact = requiredStatus('billing-provider-contact-status');
    const remoteSession = requiredStatus('remote-support-session-boundary');

    expect(
      ProductionSupportAccountSlaStatusRowSchema.safeParse({
        ...accountLookup,
        accountLookupState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportAccountSlaStatusRowSchema.safeParse({
        ...billingContact,
        billingProviderContactState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportAccountSlaStatusRowSchema.safeParse({
        ...remoteSession,
        remoteSupportSessionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportAccountSlaStatusProofSchema.safeParse({
        ...ProductionSupportAccountSlaStatusReadModel,
        productionSlaState: 'implemented',
      }).success
    ).toBe(false);
  });
}

function rejectsProviderAndCustodyData(): void {
  it('rejects provider secrets, account lookup results, remote transcripts, and child activity custody', () => {
    const accountLookup = requiredStatus('account-lookup-request-status');

    expect(
      ProductionSupportAccountSlaStatusRowSchema.safeParse({
        ...accountLookup,
        supportSafeDataClasses: [...accountLookup.supportSafeDataClasses, 'account-lookup-result'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportAccountSlaStatusRowSchema.safeParse({
        ...accountLookup,
        forbiddenDataClasses: accountLookup.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secret'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteAccountSlaCoverage(): void {
  it('rejects missing remote support rows or provider-secret non-claims', () => {
    expect(
      ProductionSupportAccountSlaStatusProofSchema.safeParse({
        ...ProductionSupportAccountSlaStatusReadModel,
        rows: ProductionSupportAccountSlaStatusReadModel.rows.filter(
          (row) => row.surface !== 'remote-support-session-boundary'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportAccountSlaStatusProofSchema.safeParse({
        ...ProductionSupportAccountSlaStatusReadModel,
        nonClaims: ProductionSupportAccountSlaStatusReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-provider-secrets'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredStatus(
  surface:
    | 'account-lookup-request-status'
    | 'account-lookup-result-boundary'
    | 'billing-provider-contact-status'
    | 'remote-support-session-boundary'
) {
  const row = ProductionSupportAccountSlaStatusReadModel.rows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing production support account/SLA status row: ${surface}`);
  }
  return row;
}
