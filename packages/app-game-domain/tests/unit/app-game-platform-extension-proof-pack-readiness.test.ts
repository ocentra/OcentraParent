import { expect, it } from 'vitest';
import {
  AppGamePlatformExtensionProofPackReadinessReadModel,
  AppGamePlatformExtensionProofPackReadinessReadModelSchema,
  AppGamePlatformExtensionProofPackReadinessRowSchema,
  summarizeAppGamePlatformExtensionProofPackReadiness,
} from '../../src/app-game-platform-extension-proof-pack-readiness';

it('covers non-Windows app/game platform extension proof packs without adapter claims', () => {
  const readModel = AppGamePlatformExtensionProofPackReadinessReadModelSchema.parse(
    AppGamePlatformExtensionProofPackReadinessReadModel
  );
  const summary = summarizeAppGamePlatformExtensionProofPackReadiness(readModel);

  expect(summary).toEqual({
    rows: 4,
    platforms: 4,
    nativeAppRows: 4,
    nativeGameRows: 4,
    manualRequiredRows: 4,
    adapterExecutedRows: 0,
    broadBlockingClaimedRows: 0,
    privilegedMobileClaimedRows: 0,
  });
  expect(readModel.rows.map((row) => row.platform).sort()).toEqual(['android', 'ios', 'linux', 'macos']);
});

it('keeps mobile rows privileged-manual instead of claimed', () => {
  const readModel = AppGamePlatformExtensionProofPackReadinessReadModel;
  const android = rowFor('android');
  const ios = rowFor('ios');

  expect(android.proofPackState).toBe('privileged-mobile-proof-required');
  expect(android.requiredProofRefs).toContain('device-owner-or-profile-owner-artifact');
  expect(ios.proofPackState).toBe('privileged-mobile-proof-required');
  expect(ios.requiredProofRefs).toContain('familycontrols-entitlement-artifact');
  expect(readModel.nonClaims).toContain('no-mobile-privileged-control');
});

it('rejects platform rows that upgrade adapter or broad blocking claims', () => {
  const macos = rowFor('macos');
  const linux = rowFor('linux');

  expect(() =>
    AppGamePlatformExtensionProofPackReadinessRowSchema.parse({
      ...macos,
      rowId: 'invalid-adapter-claim',
      adapterExecutionClaim: 'executed',
    })
  ).toThrow();
  expect(() =>
    AppGamePlatformExtensionProofPackReadinessRowSchema.parse({
      ...linux,
      rowId: 'invalid-broad-blocking-claim',
      broadBlockingClaimed: true,
    })
  ).toThrow();
});

function rowFor(platform: string) {
  const row = AppGamePlatformExtensionProofPackReadinessReadModel.rows.find(
    (candidate) => candidate.platform === platform
  );
  if (row === undefined) {
    throw new Error(`Missing app/game platform extension proof-pack row: ${platform}`);
  }
  return row;
}
