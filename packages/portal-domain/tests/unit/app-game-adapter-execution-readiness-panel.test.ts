import { AppGameSchemaVersion } from '@ocentra-parent/schema-domain/app-game-primitives';
import {
  AgentAppGameAdapterExecutionDecision,
  AgentAppGameAdapterExecutionState,
  type AppGameAdapterExecutionReadinessReadModel,
} from '@ocentra-parent/schema-domain/app-game-adapter-execution-readiness';
import { describe, expect, it } from 'vitest';
import { createAppGameAdapterExecutionReadinessPanelIntent } from '../../src/app-game-adapter-execution-readiness-panel';

const ReadModel: AppGameAdapterExecutionReadinessReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-adapter-execution-readiness',
  generatedAt: '2026-06-08T09:17:00.000Z',
  sourceReadModelIds: ['v0-8-supported-adapter-runtime-proof'],
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-execution-windows-app-game-owned-process-time-limit',
      sourceProofEntryId: 'windows-app-game-owned-process-time-limit',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'app-game-owned-process-time-limit',
      adapterExecutionState: AgentAppGameAdapterExecutionState.ProvedScopedExecution,
      executionDecision: AgentAppGameAdapterExecutionDecision.ExecutionAllowed,
      runtimeBoundary: 'windows-app-game-owned-process-time-limit',
      targetIdentityState: 'process-session-evidence-backed',
      rollbackReferenceState: 'timer-recovery-backed',
      auditReferenceState: 'audit-reference-backed',
      evidenceRefs: ['app-game-session-evidence-ref'],
      hostCapabilityState: 'available',
      hostCapabilityEvidenceRefs: ['adapter-capability-state-ref'],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      linkedProofArtifacts: ['test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json'],
      manualProofRequirements: [],
      claimBoundary: 'Scoped Windows owned-process app/game timer execution only.',
      fallbackBehavior: 'Targets without process/session identity stay manual-required.',
      adapterExecutionClaimed: true,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T09:17:00.000Z',
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-execution-windows-broad-installed-app-blocking-manual-gate',
      sourceProofEntryId: 'windows-broad-installed-app-blocking-manual-gate',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'broad-installed-app-blocking',
      adapterExecutionState: AgentAppGameAdapterExecutionState.ManualRequired,
      executionDecision: AgentAppGameAdapterExecutionDecision.BlockedBeforeExecution,
      runtimeBoundary: 'windows-broad-installed-app-blocking-manual-gate',
      targetIdentityState: 'insufficient-for-broad-target',
      rollbackReferenceState: 'manual-required',
      auditReferenceState: 'manual-required',
      evidenceRefs: [],
      hostCapabilityState: 'not-detected',
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      linkedProofArtifacts: [],
      manualProofRequirements: ['same app identity proof'],
      claimBoundary: 'Broad installed-app blocking remains manual-required.',
      fallbackBehavior: 'The runtime refuses broad app blocking claims until host apply artifacts exist.',
      adapterExecutionClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T09:17:00.000Z',
    },
  ],
};

describe('app-game adapter execution readiness panel intent', () => {
  it('renders adapter readiness rows without broad/platform/delivery claim upgrades', () => {
    const intent = createAppGameAdapterExecutionReadinessPanelIntent({
      ok: true,
      value: ReadModel,
    });

    expect(intent.loadState).toBe('Review');
    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Rows returned', value: '2' }),
        expect.objectContaining({ label: 'Read-model rows', value: '1' }),
        expect.objectContaining({ label: 'Manual review', value: '1' }),
        expect.objectContaining({ label: 'Platform state', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Child delivery', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Host available rows', value: '1' }),
        expect.objectContaining({ label: 'Host not-detected rows', value: '1' }),
        expect.objectContaining({ label: 'Host probe refs', value: '2' }),
      ])
    );
    expect(intent.rows).toHaveLength(2);
    expect(intent.rows[0].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Decision status', value: 'Execution allowed' }),
        expect.objectContaining({ label: 'Host capability state', value: 'available' }),
        expect.objectContaining({ label: 'Host capability evidence', value: 'adapter-capability-state-ref' }),
        expect.objectContaining({ label: 'Host capability probe', value: 'windows-host-local-probe-ref' }),
        expect.objectContaining({ label: 'Adapter dispatch', value: 'Ready' }),
      ])
    );
    expect(intent.rows[1].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Decision status', value: 'Blocked before execution' }),
        expect.objectContaining({ label: 'Adapter dispatch', value: 'Not claimed' }),
      ])
    );
  });

  it('renders parser failures as review state', () => {
    const intent = createAppGameAdapterExecutionReadinessPanelIntent({
      ok: false,
      reason: 'invalid-payload',
    });

    expect(intent.loadState).toBe('Review');
    expect(intent.rows).toHaveLength(0);
    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([expect.objectContaining({ label: 'Reason', value: 'invalid-payload' })])
    );
  });
});
