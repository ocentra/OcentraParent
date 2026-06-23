import { describe, expect, it } from 'vitest';
import {
  AppGameWindowsLocalPolicyEvidenceProofSchema,
  createAppGameWindowsLocalPolicyEvidenceProof,
  summarizeAppGameWindowsLocalPolicyEvidenceProof,
} from '@ocentra-parent/schema-domain/app-game-windows-local-policy-evidence-proof';

describe('app-game Windows local policy evidence proof', () => {
  recordsUnavailableLocalPolicyWithoutBroadBlockingClaim();
  recordsVisiblePolicyEvidenceWithoutDispatchClaim();
  rejectsRawPolicyAndEnforcementOverclaims();
});

function recordsUnavailableLocalPolicyWithoutBroadBlockingClaim() {
  it('records unavailable local policy state as a non-promoting proof', () => {
    const proof = createAppGameWindowsLocalPolicyEvidenceProof({
      serviceState: 'appidsvc-stopped',
      appLockerPolicyState: 'policy-empty',
      appControlPolicyState: 'app-control-not-present',
      appLockerRuleCount: 0,
      appControlPolicyCount: 0,
      policyReadable: true,
      enforceModeObserved: false,
      auditModeObserved: false,
      appControlEnforcementObserved: false,
      checkedAt: '2026-06-08T21:35:00.000Z',
    });
    const summary = summarizeAppGameWindowsLocalPolicyEvidenceProof(proof);

    expect(summary.serviceState).toBe('appidsvc-stopped');
    expect(summary.appLockerPolicyState).toBe('policy-empty');
    expect(summary.enforceModeObserved).toBe(false);
    expect(summary.openGapCount).toBeGreaterThan(0);
    expect(proof.openGaps).toEqual(
      expect.arrayContaining([
        'windows-applocker-service-not-running',
        'windows-applocker-enforce-policy-not-observed',
        'windows-app-control-enforcement-not-observed',
        'windows-system-app-allowlist-not-proved',
        'windows-rollback-not-proved',
        'windows-audit-custody-not-proved',
        'windows-broad-blocking-adapter-dispatch-not-proved',
      ])
    );
  });
}

function recordsVisiblePolicyEvidenceWithoutDispatchClaim() {
  it('records visible local policy counts without storing raw policy XML or claiming dispatch', () => {
    const proof = createAppGameWindowsLocalPolicyEvidenceProof({
      serviceState: 'appidsvc-running',
      appLockerPolicyState: 'policy-readable',
      appControlPolicyState: 'app-control-present',
      appLockerRuleCount: 3,
      appControlPolicyCount: 1,
      policyReadable: true,
      enforceModeObserved: true,
      auditModeObserved: true,
      appControlEnforcementObserved: true,
      checkedAt: '2026-06-08T21:35:00.000Z',
    });

    expect(proof.proofRefs).toEqual(
      expect.arrayContaining([
        'windows-applocker-service-state-ref',
        'windows-applocker-local-policy-state-ref',
        'windows-device-guard-policy-state-ref',
      ])
    );
    expect(proof.openGaps).not.toContain('windows-applocker-service-not-running');
    expect(proof.openGaps).not.toContain('windows-applocker-enforce-policy-not-observed');
    expect(proof.openGaps).toContain('windows-system-app-allowlist-not-proved');
    expect(proof.broadBlockingClaimed).toBe(false);
    expect(proof.adapterDispatchClaimed).toBe(false);
    expect(proof.platformEnforcementClaimed).toBe(false);
  });
}

function rejectsRawPolicyAndEnforcementOverclaims() {
  it('rejects raw policy custody, broad blocking, dispatch, and child delivery claims', () => {
    const proof = createAppGameWindowsLocalPolicyEvidenceProof({
      serviceState: 'appidsvc-running',
      appLockerPolicyState: 'policy-readable',
      appControlPolicyState: 'app-control-present',
      appLockerRuleCount: 3,
      appControlPolicyCount: 1,
      policyReadable: true,
      enforceModeObserved: true,
      auditModeObserved: true,
      appControlEnforcementObserved: true,
      checkedAt: '2026-06-08T21:35:00.000Z',
    });

    expect(
      AppGameWindowsLocalPolicyEvidenceProofSchema.safeParse({
        ...proof,
        rawPolicyXmlStored: true,
      }).success
    ).toBe(false);
    expect(
      AppGameWindowsLocalPolicyEvidenceProofSchema.safeParse({
        ...proof,
        broadBlockingClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameWindowsLocalPolicyEvidenceProofSchema.safeParse({
        ...proof,
        childDeviceDeliveryClaimed: true,
      }).success
    ).toBe(false);
  });
}
