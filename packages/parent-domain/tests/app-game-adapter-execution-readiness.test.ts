import { expect, it } from 'vitest';
import {
  AppGameAdapterExecutionReadinessReadModel,
  AppGameAdapterExecutionReadinessRowSchema,
  buildAppGameAdapterExecutionReadinessReadModel,
  summarizeAppGameAdapterExecutionReadiness,
} from '../src/app-game-adapter-execution-readiness';
import { V08SupportedAdapterRuntimeProofReadModel } from '../src/v0-8-supported-adapter-runtime-proof';

it('projects app/game adapter execution readiness from the V0.8 supported adapter runtime proof', () => {
  const readModel = buildAppGameAdapterExecutionReadinessReadModel(V08SupportedAdapterRuntimeProofReadModel);
  const summary = summarizeAppGameAdapterExecutionReadiness(readModel);

  expect(readModel.readModelId).toBe('app-game-adapter-execution-readiness');
  expect(readModel.sourceReadModelIds).toEqual(['v0-8-supported-adapter-runtime-proof']);
  expect(summary).toEqual({
    rows: 8,
    executionAllowed: 1,
    blockedBeforeExecution: 7,
    adapterExecutionClaimed: 1,
    broadInstalledAppBlockingClaimed: 0,
    childDeviceDeliveryClaimed: 0,
    platformEnforcementClaimed: 0,
    providerDeliveryClaimed: 0,
    privateDiagnosticsClaimed: 0,
  });
});

it('allows only the scoped Windows owned-process time-limit adapter execution boundary', () => {
  const row = rowFor('windows-app-game-owned-process-time-limit');

  expect(row).toMatchObject({
    platform: 'windows',
    adapterCapability: 'app-game-owned-process-time-limit',
    adapterExecutionState: 'proved-scoped-execution',
    executionDecision: 'execution-allowed',
    targetIdentityState: 'process-session-evidence-backed',
    rollbackReferenceState: 'timer-recovery-backed',
    auditReferenceState: 'audit-reference-backed',
    adapterExecutionClaimed: true,
    broadInstalledAppBlockingClaimed: false,
    childDeviceDeliveryClaimed: false,
    platformEnforcementClaimed: false,
    providerDeliveryClaimed: false,
  });
  expect(row.productMeanings).toEqual(['native-app', 'native-game']);
  expect(row.evidenceRefs.length).toBeGreaterThan(0);
  expect(row.linkedProofArtifacts.length).toBeGreaterThan(0);
  expect(row.manualProofRequirements).toEqual([]);
  expect(row.claimBoundary).toContain('scoped owned-process time-limit');
});

it('blocks broad app and non-Windows platform rows before adapter execution', () => {
  expect(rowFor('windows-broad-installed-app-blocking-manual-gate')).toMatchObject({
    platform: 'windows',
    adapterCapability: 'broad-installed-app-blocking',
    adapterExecutionState: 'manual-required',
    executionDecision: 'blocked-before-execution',
    adapterExecutionClaimed: false,
  });
  expect(rowFor('linux-host-adapter-unavailable')).toMatchObject({
    platform: 'linux',
    adapterExecutionState: 'unavailable',
    executionDecision: 'blocked-before-execution',
    adapterExecutionClaimed: false,
  });
  expect(rowFor('macos-host-adapter-unsupported')).toMatchObject({
    platform: 'macos',
    adapterExecutionState: 'unsupported',
    executionDecision: 'blocked-before-execution',
    adapterExecutionClaimed: false,
  });
  expect(rowFor('android-mobile-control-manual-gate')).toMatchObject({
    platform: 'android',
    adapterExecutionState: 'manual-required',
    executionDecision: 'blocked-before-execution',
    adapterExecutionClaimed: false,
  });
  expect(rowFor('ios-mobile-control-manual-gate')).toMatchObject({
    platform: 'ios',
    adapterExecutionState: 'manual-required',
    executionDecision: 'blocked-before-execution',
    adapterExecutionClaimed: false,
  });
});

it('rejects claim upgrades and unsupported execution claims', () => {
  const broadApp = rowFor('windows-broad-installed-app-blocking-manual-gate');
  const supported = rowFor('windows-app-game-owned-process-time-limit');

  expect(() =>
    AppGameAdapterExecutionReadinessRowSchema.parse({
      ...broadApp,
      rowId: 'invalid-broad-app-execution',
      adapterExecutionState: 'proved-scoped-execution',
      executionDecision: 'execution-allowed',
      adapterExecutionClaimed: true,
    })
  ).toThrow();
  expect(() =>
    AppGameAdapterExecutionReadinessRowSchema.parse({
      ...supported,
      rowId: 'invalid-platform-enforcement',
      platformEnforcementClaimed: true,
    })
  ).toThrow();
  expect(() =>
    AppGameAdapterExecutionReadinessRowSchema.parse({
      ...supported,
      rowId: 'invalid-broad-blocking',
      broadInstalledAppBlockingClaimed: true,
    })
  ).toThrow();
});

function rowFor(sourceProofEntryId: string) {
  const row = AppGameAdapterExecutionReadinessReadModel.rows.find(
    (candidate) => candidate.sourceProofEntryId === sourceProofEntryId
  );
  if (row === undefined) {
    throw new Error(`Missing app/game adapter execution readiness row: ${sourceProofEntryId}`);
  }
  return row;
}
