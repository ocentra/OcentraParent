import { describe, expect, it } from 'vitest';
import {
  BillingAbuseProtectionRowSchema,
  BillingAlertRowSchema,
  BillingLogRedactionRowSchema,
  BillingMetadataDenylistRowSchema,
  BillingSecurityPrivacyObservabilityProofReadModel,
  BillingSecurityPrivacyObservabilityProofSchema,
  BillingWebhookSecurityRowSchema,
} from '@ocentra-parent/schema-domain/billing-security-privacy-observability';

describe('billing security privacy observability', () => {
  acceptsBillingSecurityBoundaryProof();
  rejectsForbiddenMetadataThatIsNotBlocked();
  rejectsUnredactedBillingLogs();
  rejectsAbuseProtectionDrift();
  rejectsWebhookReplayGaps();
  rejectsProofWithoutRequiredAlertCoverage();
});

function acceptsBillingSecurityBoundaryProof(): void {
  it('accepts metadata, logging, abuse control, webhook, and alert proof without child data custody claims', () => {
    const proof = BillingSecurityPrivacyObservabilityProofSchema.parse(
      BillingSecurityPrivacyObservabilityProofReadModel
    );

    const metadataSurfaceCounts = proof.metadataAllowlistRows.reduce(
      (counts, row) => {
        counts[row.metadataSurface] += 1;
        return counts;
      },
      {
        'checkout-session': 0,
        'billing-portal-session': 0,
        'provider-webhook-event': 0,
        'support-audit-export': 0,
      } as Record<(typeof proof.metadataAllowlistRows)[number]['metadataSurface'], number>
    );

    expect(metadataSurfaceCounts).toEqual({
      'checkout-session': 3,
      'billing-portal-session': 1,
      'provider-webhook-event': 1,
      'support-audit-export': 0,
    });
    const alertKindCounts = proof.alertRows.reduce(
      (counts, row) => {
        counts[row.alertKind] += 1;
        return counts;
      },
      {
        'webhook-failure': 0,
        'payment-drift': 0,
        'checkout-abuse': 0,
        'fraud-signal': 0,
        'secret-exposure': 0,
      } as Record<(typeof proof.alertRows)[number]['alertKind'], number>
    );

    expect(alertKindCounts).toEqual({
      'webhook-failure': 1,
      'payment-drift': 1,
      'checkout-abuse': 1,
      'fraud-signal': 1,
      'secret-exposure': 1,
    });
    expect(proof.nonClaims).toEqual([
      'no-child-data-in-metadata',
      'no-raw-payment-identifiers-in-logs',
      'no-provider-secret-logs',
      'no-child-activity-custody',
      'no-pci-pan-custody',
    ]);
  });
}

function rejectsForbiddenMetadataThatIsNotBlocked(): void {
  it('rejects metadata denylist rows that stop blocking child or safety data', () => {
    const row = requiredDenylistRow('child-name');

    expect(
      BillingMetadataDenylistRowSchema.safeParse({
        ...row,
        blocked: false,
      }).success
    ).toBe(false);
  });
}

function rejectsUnredactedBillingLogs(): void {
  it('rejects log rows that expose identifiers or child safety data', () => {
    const row = requiredLogRow('provider-webhook');

    expect(
      BillingLogRedactionRowSchema.safeParse({
        ...row,
        paymentIdentifiersRedacted: false,
      }).success
    ).toBe(false);
    expect(
      BillingLogRedactionRowSchema.safeParse({
        ...row,
        childSafetyDataAbsent: false,
      }).success
    ).toBe(false);
  });
}

function rejectsAbuseProtectionDrift(): void {
  it('rejects abuse control rows that disable rate limiting or interactive bot protection', () => {
    expect(
      BillingAbuseProtectionRowSchema.safeParse({
        ...requiredAbuseRow('checkout-session-create'),
        rateLimitEnabled: false,
      }).success
    ).toBe(false);
    expect(
      BillingAbuseProtectionRowSchema.safeParse({
        ...requiredAbuseRow('billing-portal-session-create'),
        botProtectionMode: 'not-applicable',
      }).success
    ).toBe(false);
  });
}

function rejectsWebhookReplayGaps(): void {
  it('rejects webhook security rows that skip timestamp or replay verification', () => {
    const row = BillingSecurityPrivacyObservabilityProofReadModel.webhookSecurityRows[0];
    if (row === undefined) {
      throw new Error('missing webhook security row');
    }

    expect(
      BillingWebhookSecurityRowSchema.safeParse({
        ...row,
        timestampWithinTolerance: false,
      }).success
    ).toBe(false);
    expect(
      BillingWebhookSecurityRowSchema.safeParse({
        ...row,
        replayCacheChecked: false,
      }).success
    ).toBe(false);
  });
}

function rejectsProofWithoutRequiredAlertCoverage(): void {
  it('rejects proof that omits secret exposure alert coverage', () => {
    expect(
      BillingSecurityPrivacyObservabilityProofSchema.safeParse({
        ...BillingSecurityPrivacyObservabilityProofReadModel,
        alertRows: BillingSecurityPrivacyObservabilityProofReadModel.alertRows.filter(
          (row) => row.alertKind !== 'secret-exposure'
        ),
      }).success
    ).toBe(false);

    expect(
      BillingAlertRowSchema.safeParse({
        ...requiredAlertRow('webhook-failure'),
        redactedPayloadOnly: false,
      }).success
    ).toBe(false);
  });
}

function requiredDenylistRow(forbiddenClass: 'child-name') {
  const row = BillingSecurityPrivacyObservabilityProofReadModel.metadataDenylistRows.find(
    (entry) => entry.forbiddenClass === forbiddenClass
  );
  if (row === undefined) {
    throw new Error(`missing metadata denylist row: ${forbiddenClass}`);
  }
  return row;
}

function requiredLogRow(logSignal: 'provider-webhook') {
  const row = BillingSecurityPrivacyObservabilityProofReadModel.logRedactionRows.find(
    (entry) => entry.logSignal === logSignal
  );
  if (row === undefined) {
    throw new Error(`missing log redaction row: ${logSignal}`);
  }
  return row;
}

function requiredAbuseRow(operation: 'checkout-session-create' | 'billing-portal-session-create') {
  const row = BillingSecurityPrivacyObservabilityProofReadModel.abuseProtectionRows.find(
    (entry) => entry.operation === operation
  );
  if (row === undefined) {
    throw new Error(`missing abuse protection row: ${operation}`);
  }
  return row;
}

function requiredAlertRow(alertKind: 'webhook-failure') {
  const row = BillingSecurityPrivacyObservabilityProofReadModel.alertRows.find(
    (entry) => entry.alertKind === alertKind
  );
  if (row === undefined) {
    throw new Error(`missing alert row: ${alertKind}`);
  }
  return row;
}
