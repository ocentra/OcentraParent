import { describe, expect, it } from 'vitest';
import {
  type ChildManagedServiceRespawnReadModel,
  ChildManagedServiceRespawnReadModelSchema,
} from '../../src/child-managed-service-respawn-proof';
import { managedServiceRespawnReadModelInput } from './child-managed-service-respawn-fixtures';

describe('child managed service respawn proof contracts', () => {
  acceptsExplicitDesktopRespawnRows();
  acceptsMobileRespawnLimits();
  rejectsMissingPlatformRows();
  rejectsHidingDesktopStopAsAutomaticRespawn();
  rejectsAndroidRespawnUpgrade();
  rejectsIosUnsupportedUpgrade();
  rejectsParentProofReuseBoundaryRemoval();
});

function acceptsExplicitDesktopRespawnRows(): void {
  it('ChildManagedServiceRespawnReadModelSchema: accepts explicit desktop respawn support', () => {
    const parsed = ChildManagedServiceRespawnReadModelSchema.parse(validReadModel());

    expect(platformState(parsed, 'windows')).toEqual({
      respawnState: 'proved',
      stopRecoveryState: 'manual-required',
      teardownState: 'proved',
    });
    expect(platformState(parsed, 'macos')).toEqual({
      respawnState: 'proved',
      stopRecoveryState: 'manual-required',
      teardownState: 'proved',
    });
    expect(platformState(parsed, 'linux')).toEqual({
      respawnState: 'proved',
      stopRecoveryState: 'manual-required',
      teardownState: 'proved',
    });
    expect(parsed.claimBoundaries.parentProofSeparation).toContain('Parent client update');
  });
}

function acceptsMobileRespawnLimits(): void {
  it('ChildManagedServiceRespawnReadModelSchema: keeps mobile respawn manual or unsupported', () => {
    const parsed = ChildManagedServiceRespawnReadModelSchema.parse(validReadModel());

    expect(platformState(parsed, 'android')).toEqual({
      respawnState: 'manual-required',
      stopRecoveryState: 'manual-required',
      teardownState: 'manual-required',
    });
    expect(platformState(parsed, 'ios')).toEqual({
      respawnState: 'unsupported',
      stopRecoveryState: 'unsupported',
      teardownState: 'unsupported',
    });
    expect(parsed.claimBoundaries.mobileNoReuse).toContain('Android stays manual-required');
  });
}

function rejectsMissingPlatformRows(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects missing platform rows', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        platformProofs: model.platformProofs.filter((entry) => entry.platform !== 'linux'),
      }).success
    ).toBe(false);
  });
}

function rejectsHidingDesktopStopAsAutomaticRespawn(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects treating deliberate desktop stop as automatic respawn', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        platformProofs: model.platformProofs.map((entry) =>
          entry.platform === 'windows' ? { ...entry, stopRecoveryState: 'proved' } : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsAndroidRespawnUpgrade(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects Android respawn upgrades without device proof', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        platformProofs: model.platformProofs.map((entry) =>
          entry.platform === 'android'
            ? {
                ...entry,
                proofState: 'ci-mechanical-proof',
                respawnState: 'proved',
                restartSurvivalState: 'proved',
                killRecoveryState: 'proved',
                rebootRecoveryState: 'proved',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsIosUnsupportedUpgrade(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects iOS managed-service support upgrades', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        platformProofs: model.platformProofs.map((entry) =>
          entry.platform === 'ios'
            ? {
                ...entry,
                proofState: 'manual-required',
                respawnState: 'manual-required',
                claimBoundary: 'iOS managed service respawn is available through desktop proof reuse',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsParentProofReuseBoundaryRemoval(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects parent-client proof reuse boundaries being removed', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        claimBoundaries: {
          ...model.claimBoundaries,
          parentProofSeparation: 'Parent client proof closes child respawn claims.',
        },
      }).success
    ).toBe(false);
  });
}

function validReadModel(): ChildManagedServiceRespawnReadModel {
  return ChildManagedServiceRespawnReadModelSchema.parse(managedServiceRespawnReadModelInput);
}

function platformState(
  model: ChildManagedServiceRespawnReadModel,
  platform: ChildManagedServiceRespawnReadModel['platformProofs'][number]['platform']
) {
  const entry = model.platformProofs.find((proof) => proof.platform === platform);
  return {
    respawnState: entry?.respawnState,
    stopRecoveryState: entry?.stopRecoveryState,
    teardownState: entry?.teardownState,
  };
}
