import { describe, expect, it } from 'vitest';
import {
  V08BrowserEnforcementTimerRecoveryProofReadModelSchema,
  V08TimerRecoveryProofEntrySchema,
  V08TimerRecoverySurface,
  V08UnmanagedBrowserFallbackProofEntrySchema,
  V08UnmanagedBrowserFallbackSurface,
} from '@ocentra-parent/schema-domain/v0-8-browser-enforcement-timer-recovery-proof';
import { V08BrowserEnforcementTimerRecoveryProofReadModel } from '../../src/v0-8-browser-enforcement-timer-recovery-proof-values';

describe('V0.8 browser enforcement timer recovery proof', () => {
  capturesTimerRecoveryLifecycleStates();
  capturesUnmanagedFallbackStates();
  recordsParentVisibleTimerFailureAndRollbackState();
  keepsUnmanagedBrowserEvidenceProcessScoped();
  rejectsTimerLifecycleStateDrift();
  rejectsUnmanagedFallbackClaimUpgrades();
});

function capturesTimerRecoveryLifecycleStates() {
  it('captures create, extend, expire, cancel, restart recovery, recovery-needed, and rollback states', () => {
    const readModel = V08BrowserEnforcementTimerRecoveryProofReadModelSchema.parse(
      V08BrowserEnforcementTimerRecoveryProofReadModel
    );
    const lifecycleCounts = countBy(readModel.timerEntries.map((entry) => entry.lifecycleState));
    const persistenceCounts = countBy(readModel.timerEntries.map((entry) => entry.statePersistence));

    expect(readModel.timerEntries).toHaveLength(8);
    expect(lifecycleCounts).toEqual({
      created: 1,
      extended: 1,
      expired: 1,
      cancelled: 1,
      'restart-recovered': 1,
      'recovery-needed': 1,
      'rollback-completed': 1,
      'rollback-unavailable': 1,
    });
    expect(persistenceCounts).toEqual({
      'active-state-persisted': 3,
      'active-state-cleared': 4,
      'active-state-missing': 1,
    });
    expect(new Set(readModel.timerEntries.map((entry) => entry.proofEntryId)).size).toBe(readModel.timerEntries.length);
  });
}

function capturesUnmanagedFallbackStates() {
  it('keeps unmanaged fallback process, report, warn, review, terminate, relaunch, manual, degraded, and unavailable states separate', () => {
    const readModel = V08BrowserEnforcementTimerRecoveryProofReadModelSchema.parse(
      V08BrowserEnforcementTimerRecoveryProofReadModel
    );
    const fallbackCounts = countBy(readModel.unmanagedFallbackEntries.map((entry) => entry.fallbackState));
    const executionCounts = countBy(readModel.unmanagedFallbackEntries.map((entry) => entry.adapterExecutionState));

    expect(readModel.unmanagedFallbackEntries).toHaveLength(9);
    expect(fallbackCounts).toEqual({
      'process-identity-required': 1,
      'report-only': 1,
      'warn-child': 1,
      'parent-review': 1,
      'terminate-process': 1,
      'relaunch-managed-browser': 1,
      'manual-required': 1,
      degraded: 1,
      unavailable: 1,
    });
    expect(executionCounts).toEqual({
      'process-identity-rejected': 1,
      'returns-report-only': 1,
      'returns-degraded-noop': 2,
      'returns-parent-review': 1,
      'executes-real-service': 1,
      'returns-manual-required': 2,
      'returns-unavailable': 1,
    });
    expect(new Set(readModel.unmanagedFallbackEntries.map((entry) => entry.proofEntryId)).size).toBe(
      readModel.unmanagedFallbackEntries.length
    );
  });
}

function recordsParentVisibleTimerFailureAndRollbackState() {
  it('records parent-visible next-check, failure, and rollback state without changing claim scope', () => {
    const created = timerEntryFor(V08TimerRecoverySurface.CreatedActive);
    const recovered = timerEntryFor(V08TimerRecoverySurface.RestartRecoveredActive);
    const recoveryNeeded = timerEntryFor(V08TimerRecoverySurface.RecoveryNeededMissingState);
    const rollbackCompleted = timerEntryFor(V08TimerRecoverySurface.RollbackCompletedCleared);
    const rollbackUnavailable = timerEntryFor(V08TimerRecoverySurface.RollbackUnavailableVisible);

    expect(created.nextCheckAtVisible).toBe(true);
    expect(created.parentVisibleStates).toEqual(['next-check-visible']);
    expect(recovered).toMatchObject({
      timerEventKind: 'restart-recovered',
      statePersistence: 'active-state-persisted',
      nextCheckAtVisible: true,
    });
    expect(recoveryNeeded).toMatchObject({
      timerEventKind: 'recovery-needed',
      resultStatus: 'unavailable',
      rollbackState: 'unavailable',
      failureStateVisible: true,
    });
    expect(rollbackCompleted.parentVisibleStates).toContain('rollback-state-visible');
    expect(rollbackCompleted).toMatchObject({
      resultStatus: 'rolled-back',
      rollbackState: 'completed',
      statePersistence: 'active-state-cleared',
    });
    expect(rollbackUnavailable.parentVisibleStates).toEqual(['failure-state-visible', 'rollback-state-visible']);
    expect(rollbackUnavailable.manualProofRequirements).toContain('operator-visible rollback unavailable state');
  });
}

function keepsUnmanagedBrowserEvidenceProcessScoped() {
  it('keeps unmanaged browser fallback process-scoped and excludes exact browser content claims', () => {
    const processIdentity = unmanagedEntryFor(V08UnmanagedBrowserFallbackSurface.ProcessIdentityRequired);
    const terminate = unmanagedEntryFor(V08UnmanagedBrowserFallbackSurface.TerminateProcess);
    const warn = unmanagedEntryFor(V08UnmanagedBrowserFallbackSurface.WarnChild);
    const relaunch = unmanagedEntryFor(V08UnmanagedBrowserFallbackSurface.RelaunchManagedBrowser);
    const unavailable = unmanagedEntryFor(V08UnmanagedBrowserFallbackSurface.Unavailable);

    expect(processIdentity).toMatchObject({
      processIdentityRequired: true,
      processIdentityState: 'missing-process-identity-rejected',
    });
    expect(terminate).toMatchObject({
      processIdentityRequired: true,
      processIdentityState: 'pid-name-validated',
      exactUrlClaimed: false,
      activeTabClaimed: false,
      titleClaimed: false,
      contentClaimed: false,
      broadBrowserBlockingClaimed: false,
    });
    expect(warn.manualProofRequirements).toContain('notification delivery artifact');
    expect(relaunch.processIdentityState).toBe('pid-name-required');
    expect(unavailable.fallbackBehavior).toContain('unavailable');
  });
}

function rejectsTimerLifecycleStateDrift() {
  it('rejects timer proof entries that drift away from their lifecycle state boundary', () => {
    const created = timerEntryFor(V08TimerRecoverySurface.CreatedActive);
    const rollbackCompleted = timerEntryFor(V08TimerRecoverySurface.RollbackCompletedCleared);

    expect(() =>
      V08TimerRecoveryProofEntrySchema.parse({
        ...created,
        proofEntryId: 'invalid-created-not-persisted',
        statePersistence: 'active-state-cleared',
      })
    ).toThrow();
    expect(() =>
      V08TimerRecoveryProofEntrySchema.parse({
        ...rollbackCompleted,
        proofEntryId: 'invalid-rollback-upgrade-next-check',
        nextCheckAtVisible: true,
        parentVisibleStates: ['next-check-visible', 'rollback-state-visible'],
      })
    ).toThrow();
  });
}

function rejectsUnmanagedFallbackClaimUpgrades() {
  it('rejects exact URL, active tab, title, content, notification delivery, and broad browser claim upgrades', () => {
    const terminate = unmanagedEntryFor(V08UnmanagedBrowserFallbackSurface.TerminateProcess);
    const warn = unmanagedEntryFor(V08UnmanagedBrowserFallbackSurface.WarnChild);
    const reportOnly = unmanagedEntryFor(V08UnmanagedBrowserFallbackSurface.ReportOnly);

    expect(() =>
      V08UnmanagedBrowserFallbackProofEntrySchema.parse({
        ...terminate,
        proofEntryId: 'invalid-unmanaged-exact-url-upgrade',
        exactUrlClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08UnmanagedBrowserFallbackProofEntrySchema.parse({
        ...terminate,
        proofEntryId: 'invalid-unmanaged-active-tab-upgrade',
        activeTabClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08UnmanagedBrowserFallbackProofEntrySchema.parse({
        ...terminate,
        proofEntryId: 'invalid-unmanaged-title-upgrade',
        titleClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08UnmanagedBrowserFallbackProofEntrySchema.parse({
        ...terminate,
        proofEntryId: 'invalid-unmanaged-content-upgrade',
        contentClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08UnmanagedBrowserFallbackProofEntrySchema.parse({
        ...warn,
        proofEntryId: 'invalid-unmanaged-notification-upgrade',
        notificationDeliveryClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08UnmanagedBrowserFallbackProofEntrySchema.parse({
        ...reportOnly,
        proofEntryId: 'invalid-unmanaged-broad-browser-upgrade',
        broadBrowserBlockingClaimed: true,
      })
    ).toThrow();
  });
}

function timerEntryFor(surface: string) {
  const entry = V08BrowserEnforcementTimerRecoveryProofReadModel.timerEntries.find(
    (candidate) => candidate.surface === surface
  );
  if (entry === undefined) {
    throw new Error(`Missing V0.8 timer recovery proof entry: ${surface}`);
  }
  return entry;
}

function unmanagedEntryFor(surface: string) {
  const entry = V08BrowserEnforcementTimerRecoveryProofReadModel.unmanagedFallbackEntries.find(
    (candidate) => candidate.surface === surface
  );
  if (entry === undefined) {
    throw new Error(`Missing V0.8 unmanaged fallback proof entry: ${surface}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
