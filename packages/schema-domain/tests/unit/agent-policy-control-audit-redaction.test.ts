import { describe, expect, it } from 'vitest';
import {
  PolicyControlAuditPayloadSchema,
  PolicyControlAuditPayloadSchemaVersion,
  PolicyControlAuditRedactionPlaceholder,
  PolicyControlRedactedAuditPayloadSchema,
  PolicyControlRedactedAuditPayloadSchemaVersion,
  redactPolicyControlAuditPayload,
} from '../../src/agent-policy-control-audit-redaction';

describe('agent policy control audit redaction', () => {
  it('redacts sensitive audit payload fields while preserving portal-safe state', () => {
    const redacted = redactPolicyControlAuditPayload(rawAuditPayload());

    expect(redacted.schemaVersion).toBe(PolicyControlRedactedAuditPayloadSchemaVersion);
    expect(redacted.policyDomain).toBe('network');
    expect(redacted.parentVisibleState).toBe('manual-required');
    expect(redacted.reasonCodes).toEqual(['permission-loss', 'manual-portal-follow-up']);
    expect(redacted.auditRefs).toEqual(['audit-ref-policy-control-network-manual-required']);
    expect(redacted.manualProofRequirements).toEqual([
      'child-device-admin-permission-regrant',
    ]);
    expect(redacted.childDisplayName).toBe(
      PolicyControlAuditRedactionPlaceholder.ChildIdentity
    );
    expect(redacted.accountLocator).toBe(
      PolicyControlAuditRedactionPlaceholder.AccountIdentity
    );
    expect(redacted.policyTargetValue).toBe(
      PolicyControlAuditRedactionPlaceholder.PolicyTarget
    );
    expect(redacted.rawUrl).toBe(PolicyControlAuditRedactionPlaceholder.RawUrl);
    expect(redacted.secretToken).toBe(PolicyControlAuditRedactionPlaceholder.Secret);
    expect(redacted.providerDetail).toBe(
      PolicyControlAuditRedactionPlaceholder.ProviderDetail
    );
    expect(redacted.protectedFieldKinds).toEqual([
      'child-display-name',
      'account-locator',
      'policy-target-value',
      'raw-url',
      'secret-token',
      'provider-detail',
    ]);
    expect(redacted.redactedSensitiveFieldCount).toBe(6);
    expect(redacted.redactionApplied).toBe(true);
  });

  it('rejects leaked raw fields and inconsistent redaction accounting', () => {
    expect(
      PolicyControlAuditPayloadSchema.safeParse({
        ...rawAuditPayload(),
        auditRefs: [],
      }).success
    ).toBe(false);

    const redacted = redactPolicyControlAuditPayload({
      ...rawAuditPayload(),
      childDisplayName: null,
      accountLocator: null,
      policyTargetValue: null,
      rawUrl: null,
      secretToken: null,
      providerDetail: null,
    });

    expect(redacted.redactionApplied).toBe(false);
    expect(redacted.redactedSensitiveFieldCount).toBe(0);
    expect(redacted.protectedFieldKinds).toEqual([]);
    expect(redacted.childDisplayName).toBe(null);
    expect(
      PolicyControlRedactedAuditPayloadSchema.safeParse({
        ...redacted,
        rawUrl: 'https://family.example/private-target',
      }).success
    ).toBe(false);
    expect(
      PolicyControlRedactedAuditPayloadSchema.safeParse({
        ...redacted,
        schemaVersion: PolicyControlRedactedAuditPayloadSchemaVersion,
        protectedFieldKinds: ['raw-url'],
        redactedSensitiveFieldCount: 0,
        redactionApplied: false,
      }).success
    ).toBe(false);
  });
});

function rawAuditPayload() {
  return PolicyControlAuditPayloadSchema.parse({
    schemaVersion: PolicyControlAuditPayloadSchemaVersion,
    auditEventId: 'policy-control-audit-network-manual-required',
    deliveryRowId: 'policy-control-delivery-network-manual-required',
    policyVersionRef: 'policy-version-2026-06-13-network-manual-required',
    policyDomain: 'network',
    eventKind: 'manual-required',
    parentVisibleState: 'manual-required',
    childDeviceId: 'child-device-network-1',
    actorRole: 'policy-control-plane',
    reasonCodes: ['permission-loss', 'manual-portal-follow-up'],
    auditRefs: ['audit-ref-policy-control-network-manual-required'],
    manualProofRequirements: ['child-device-admin-permission-regrant'],
    retryScheduleRefs: ['retry-ref-policy-control-network-manual-required'],
    childDisplayName: 'Alice Example',
    accountLocator: 'parent@example.com',
    policyTargetValue: 'example-video-service',
    rawUrl: 'https://family.example/private-target',
    secretToken: 'secret-token-value',
    providerDetail: 'HTTP 403 from provider with raw adapter context',
  });
}
