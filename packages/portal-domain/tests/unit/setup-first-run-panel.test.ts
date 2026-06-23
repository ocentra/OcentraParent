import { describe, expect, it } from 'vitest';
import { createSetupFirstRunPanelIntent, readableSetupValue } from '../../src/setup-first-run-panel';

describe('setup first-run panel intent', () => {
  registerSetupScreenProjectionTests();
  registerSetupRecoveryAndLegendTests();
  registerSetupRoleAndTrustTests();
  registerReadableSetupValueTests();
});

function registerSetupScreenProjectionTests(): void {
  it('maps the first-run setup screens, gates, and sibling handoffs into the start-route panel', () => {
    const intent = createSetupFirstRunPanelIntent();

    expect(intent.title).toBe('First-run family setup');
    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Screens mapped', value: '20' }),
        expect.objectContaining({
          label: 'Ready gate',
          value: 'setup-complete requires overall readiness = ready after data-custody',
        }),
      ])
    );

    expect(intent.cards).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          title: 'Parent entry and install',
          details: expect.arrayContaining([
            expect.objectContaining({
              label: 'Screens',
              value:
                'welcome-screen | sign-in-or-create-account-screen | create-or-join-household-screen | parent-install-screen | parent-bootstrap-agreement-screen | parent-bootstrap-code-screen | parent-install-progress-screen | parent-guided-setup-start-screen',
            }),
          ]),
        }),
        expect.objectContaining({
          title: 'Child device and readiness',
          details: expect.arrayContaining([
            expect.objectContaining({
              label: 'Screens',
              value:
                'child-profile-screen | child-pairing-screen | child-install-screen | waiting-for-child-device-screen | confirm-child-device-screen | permission-checklist-screen | policy-baseline-screen | data-custody-status-screen',
            }),
          ]),
        }),
      ])
    );
  });
}

function registerSetupRecoveryAndLegendTests(): void {
  it('keeps degraded, manual-required, blocked, and complete outcomes visible without premature ready claims', () => {
    const intent = createSetupFirstRunPanelIntent();
    const recoveryCard = intent.cards.find((card) => card.title === 'Recovery and completion gates');

    expect(recoveryCard).toMatchObject({
      title: 'Recovery and completion gates',
      summary:
        'Setup never skips past degraded, manual-required, or blocked outcomes, and setup-complete stays unavailable until the readiness gate resolves to ready.',
    });
    expect(recoveryCard?.details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          label: 'Degraded state',
          value: 'setup-degraded -> recovery-screen | degraded',
        }),
        expect.objectContaining({
          label: 'Manual-required state',
          value: 'manual-required -> manual-required-screen | recovery-required | blocked',
        }),
        expect.objectContaining({
          label: 'Blocked completion',
          value: 'setup-blocked -> setup-blocked-screen | setup-complete withheld',
        }),
        expect.objectContaining({
          label: 'Ready completion',
          value: 'setup-complete -> setup-complete-screen | ready',
        }),
      ])
    );
  });

  it('surfaces source/custody labels, required label legend, and adjacent owner handoffs explicitly', () => {
    const intent = createSetupFirstRunPanelIntent();
    const legendCard = intent.cards.find((card) => card.title === 'Checklist and label legend');
    const handoffCard = intent.cards.find((card) => card.title === 'Adjacent owner handoffs');

    expect(legendCard?.details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          label: 'Source/custody labels',
          value:
            'live-local | physical-household-lan | parent-cache | parent-owned-storage | stale | degraded | unavailable | manual-required',
        }),
        expect.objectContaining({
          label: 'Required labels',
          value:
            'notImplemented | previewOnly | manualRequired | readyForTest | productionReady | blocked | stale | degraded | unavailable',
        }),
      ])
    );

    expect(handoffCard?.details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          label: 'Account/session owner',
          value: 'account-identity-family-plan | account/provider/session implementation',
        }),
        expect.objectContaining({
          label: 'Runtime distribution owner',
          value: 'parent-desktop-runtime-package-plan | signed installers, updates, publishing',
        }),
        expect.objectContaining({
          label: 'LAN/device-trust owner',
          value: 'lan-plan + device-trust-bootstrap-plan | LAN and trusted-device proof',
        }),
      ])
    );
  });
}

function registerSetupRoleAndTrustTests(): void {
  it('surfaces invite-role-support distinctions and separates trust from login/session readiness', () => {
    const intent = createSetupFirstRunPanelIntent();
    const inviteRoleCard = intent.cards.find((card) => card.title === 'Invite, role, and support visibility');
    const trustCard = intent.cards.find((card) => card.title === 'Trust and session distinction');

    expect(inviteRoleCard?.details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          label: 'Signed-in without household',
          value: 'account-entry -> household-selection | signed-in account still lacks household authority',
        }),
        expect.objectContaining({
          label: 'Co-parent invite',
          value: 'pending invite -> co-parent role stays distinct from parent-owner and child-device trust',
        }),
        expect.objectContaining({
          label: 'Observer invite',
          value: 'pending invite -> observer stays read-only and cannot inherit owner controls',
        }),
        expect.objectContaining({
          label: 'Support access status',
          value: 'support-admin remains a separate audited support state | never parent-owner',
        }),
      ])
    );

    expect(trustCard?.details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          label: 'Trust status',
          value: 'Pairing:action-required:accepted | Pairing:complete:trusted',
        }),
        expect.objectContaining({
          label: 'Wrong-account state',
          value: 'Account:action-required:wrong-account',
        }),
        expect.objectContaining({
          label: 'Reauth/manual-required state',
          value: 'Account:action-required:recovery-required | manual-required-screen',
        }),
        expect.objectContaining({
          label: 'Revoked child state',
          value: 'Child service:action-required:revoked',
        }),
        expect.objectContaining({
          label: 'Stale parent state',
          value: 'Parent app:action-required:stale',
        }),
        expect.objectContaining({
          label: 'Direct-entry-required state',
          value: 'Network reachability:action-required:direct-entry-required',
        }),
      ])
    );
  });
}

function registerReadableSetupValueTests(): void {
  it('reuses shared readable portal labels where they already exist', () => {
    expect(readableSetupValue('manual-required')).toBe('Manual required');
    expect(readableSetupValue('live-local')).toBe('Live local');
    expect(readableSetupValue('productionReady')).toBe('ProductionReady');
  });
}
