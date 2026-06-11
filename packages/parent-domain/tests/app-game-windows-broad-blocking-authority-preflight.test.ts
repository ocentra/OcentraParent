import { describe, expect, it } from 'vitest';
import {
  AppGameWindowsBroadBlockingAuthorityPreflightReadModelSchema,
  createAppGameWindowsBroadBlockingAuthorityPreflightReadModel,
  summarizeAppGameWindowsBroadBlockingAuthorityPreflightReadModel,
} from '../src/app-game-windows-broad-blocking-authority-preflight';

describe('app-game Windows broad blocking authority preflight', () => {
  keepsWindowsBroadLaunchBlockingBlocked();
  mapsExistingGateRefsIntoRows();
  rejectsDispatchAndPrivatePolicyOverclaims();
});

function keepsWindowsBroadLaunchBlockingBlocked() {
  it('keeps Windows broad launch blocking blocked until policy authority proof is attached', () => {
    const readModel = createAppGameWindowsBroadBlockingAuthorityPreflightReadModel({
      generatedAt: '2026-06-08T18:05:00.000Z',
    });
    const summary = summarizeAppGameWindowsBroadBlockingAuthorityPreflightReadModel(readModel);

    expect(summary.authorityState).toBe('host-visible-policy-proof-missing');
    expect(summary.windowsHostProbeAttached).toBe(true);
    expect(summary.dispatchableActionCount).toBe(0);
    expect(summary.blockedActionCount).toBe(5);
    expect(summary.openBlockerCount).toBe(6);
    expect(readModel.rows.map((row) => row.action)).toEqual([
      'block-launch-applocker',
      'block-launch-app-control',
      'system-app-allowlist',
      'policy-rollback',
      'audit-custody',
    ]);
    expect(readModel.rows.every((row) => row.canDispatchAdapter === false)).toBe(true);
    expect(readModel.openBlockers).toEqual(
      expect.arrayContaining([
        'windows-applocker-enforce-not-proved',
        'windows-app-control-not-proved',
        'windows-system-app-allowlist-not-proved',
        'windows-rollback-not-proved',
        'windows-audit-custody-not-proved',
        'windows-adapter-dispatch-blocked-before-authority',
      ])
    );
  });
}

function mapsExistingGateRefsIntoRows() {
  it('maps existing broad-blocking gate refs into parent-visible Windows action rows', () => {
    const readModel = createAppGameWindowsBroadBlockingAuthorityPreflightReadModel({
      generatedAt: '2026-06-08T18:05:00.000Z',
    });
    const appLocker = rowFor(readModel, 'block-launch-applocker');
    const appControl = rowFor(readModel, 'block-launch-app-control');
    const allowlist = rowFor(readModel, 'system-app-allowlist');
    const audit = rowFor(readModel, 'audit-custody');

    expect(appLocker.sourceGateIds).toEqual(
      expect.arrayContaining([
        'windows-block-launch-applocker-app-control-manual-required',
        'windows-applocker-audit-is-not-enforce-proof',
      ])
    );
    expect(appControl.sourceGateIds).toEqual(['windows-block-launch-applocker-app-control-manual-required']);
    expect(allowlist.requiredProofRefs).toEqual(['windows-system-app-allowlist-proof']);
    expect(audit.requiredProofRefs).toEqual(['windows-applocker-audit-proof', 'windows-audit-custody-proof']);
    expect(readModel.proofRefs).toEqual(['windows-host-local-probe-ref', 'windows-broad-blocking-gate-ref']);
  });
}

function rejectsDispatchAndPrivatePolicyOverclaims() {
  it('rejects dispatch enforcement and private Windows policy overclaims', () => {
    const readModel = createAppGameWindowsBroadBlockingAuthorityPreflightReadModel({
      generatedAt: '2026-06-08T18:05:00.000Z',
    });

    expect(
      AppGameWindowsBroadBlockingAuthorityPreflightReadModelSchema.safeParse({
        ...readModel,
        rows: [{ ...readModel.rows[0], canDispatchAdapter: true }],
      }).success
    ).toBe(false);
    expect(
      AppGameWindowsBroadBlockingAuthorityPreflightReadModelSchema.safeParse({
        ...readModel,
        broadBlockingClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameWindowsBroadBlockingAuthorityPreflightReadModelSchema.safeParse({
        ...readModel,
        rawPolicyXmlClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameWindowsBroadBlockingAuthorityPreflightReadModelSchema.safeParse({
        ...readModel,
        appLockerProofAttached: true,
      }).success
    ).toBe(false);
  });
}

function rowFor(
  readModel: ReturnType<typeof createAppGameWindowsBroadBlockingAuthorityPreflightReadModel>,
  action: string
) {
  const row = readModel.rows.find((candidate) => candidate.action === action);
  if (row === undefined) {
    throw new Error(`Missing Windows broad blocking preflight row: ${action}`);
  }
  return row;
}
