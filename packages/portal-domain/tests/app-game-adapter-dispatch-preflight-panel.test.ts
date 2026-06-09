import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightState,
  type AgentAppGameAdapterDispatchPreflightReadModel,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-dispatch-preflight';
import {
  AgentAppGameAdapterExecutionDecision,
  AgentAppGameAdapterExecutionState,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-execution-readiness';
import { describe, expect, it } from 'vitest';
import { createAppGameAdapterDispatchPreflightPanelIntent } from '../src/app-game-adapter-dispatch-preflight-panel';

const ReadModel: AgentAppGameAdapterDispatchPreflightReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-adapter-dispatch-preflight',
  generatedAt: '2026-06-08T10:16:00.000Z',
  sourceReadModelIds: ['app-game-adapter-execution-readiness', 'v0-8-enforcement-policy-dispatch'],
  custodyLabel: 'adapter-execution-readiness-and-policy-dispatch',
  capabilityStatus: 'app-game-adapter-dispatch-preflight-partial',
  returned: 2,
  dispatchEligibleCount: 1,
  blockedBeforeDispatchCount: 1,
  adapterDispatchEligibleCount: 1,
  adapterDispatchExecutedClaimedCount: 0,
  hostCapabilityAvailableCount: 1,
  hostCapabilityNotDetectedCount: 1,
  hostCapabilityNotApplicableCount: 0,
  hostCapabilityProbeRefCount: 2,
  broadInstalledAppBlockingClaimed: false,
  childDeviceDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  providerDeliveryClaimed: false,
  privateDiagnosticsClaimed: false,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-dispatch-preflight-windows-app-game-owned-process-time-limit',
      sourceExecutionReadinessRowId: 'app-game-adapter-execution-windows-app-game-owned-process-time-limit',
      sourceProofEntryId: 'windows-app-game-owned-process-time-limit',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'app-game-owned-process-time-limit',
      adapterExecutionState: AgentAppGameAdapterExecutionState.ProvedScopedExecution,
      executionDecision: AgentAppGameAdapterExecutionDecision.ExecutionAllowed,
      dispatchPreflightState: AgentAppGameAdapterDispatchPreflightState.DispatchEligible,
      dispatchDecision: AgentAppGameAdapterDispatchDecision.DispatchEligible,
      dispatchIntentId: 'dispatch-owned-process-time-limit',
      dispatchOutcomeState: AgentAppGameAdapterDispatchOutcomeState.DispatchReady,
      dispatchEvidenceRefs: ['evidence-app-session-owned-process'],
      hostCapabilityState: 'available',
      hostCapabilityEvidenceRefs: ['adapter-capability-state-ref'],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      dispatchAuditRefs: ['audit-owned-process-dispatch-accepted'],
      dispatchTimerRefs: ['timer-owned-process-active'],
      manualProofRequirements: [],
      claimBoundary: 'Dispatch eligibility is limited to scoped Windows owned-process app/game time-limit rows.',
      fallbackBehavior: 'Rows without scoped process/session identity stay blocked before adapter dispatch.',
      adapterDispatchEligible: true,
      adapterDispatchExecutedClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T10:16:00.000Z',
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-dispatch-preflight-windows-broad-installed-app-blocking-manual-gate',
      sourceExecutionReadinessRowId: 'app-game-adapter-execution-windows-broad-installed-app-blocking-manual-gate',
      sourceProofEntryId: 'windows-broad-installed-app-blocking-manual-gate',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'broad-installed-app-blocking',
      adapterExecutionState: AgentAppGameAdapterExecutionState.ManualRequired,
      executionDecision: AgentAppGameAdapterExecutionDecision.BlockedBeforeExecution,
      dispatchPreflightState: AgentAppGameAdapterDispatchPreflightState.ManualRequired,
      dispatchDecision: AgentAppGameAdapterDispatchDecision.BlockedBeforeDispatch,
      dispatchIntentId: null,
      dispatchOutcomeState: AgentAppGameAdapterDispatchOutcomeState.ManualRequired,
      dispatchEvidenceRefs: [],
      hostCapabilityState: 'not-detected',
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      dispatchAuditRefs: [],
      dispatchTimerRefs: [],
      manualProofRequirements: ['same app identity proof'],
      claimBoundary: 'Broad installed-app blocking stays blocked before adapter dispatch.',
      fallbackBehavior: 'The parent surface must route this row to manual review instead of dispatch.',
      adapterDispatchEligible: false,
      adapterDispatchExecutedClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T10:16:00.000Z',
    },
  ],
};

describe('app-game adapter dispatch preflight panel intent', () => {
  it('renders dispatch eligibility without claiming adapter execution', () => {
    const intent = createAppGameAdapterDispatchPreflightPanelIntent({
      ok: true,
      value: ReadModel,
    });

    expect(intent.loadState).toBe('Review');
    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Rows returned', value: '2' }),
        expect.objectContaining({ label: 'Read-model rows', value: '1' }),
        expect.objectContaining({ label: 'Manual review', value: '1' }),
        expect.objectContaining({ label: 'Adapter dispatch', value: 'Ready' }),
        expect.objectContaining({ label: 'Execution state', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Platform state', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Host available rows', value: '1' }),
        expect.objectContaining({ label: 'Host not-detected rows', value: '1' }),
        expect.objectContaining({ label: 'Host probe refs', value: '2' }),
      ])
    );
    expect(intent.rows).toHaveLength(2);
    expect(intent.rows[0].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Decision status', value: 'Dispatch eligible' }),
        expect.objectContaining({ label: 'Dispatch intent', value: 'dispatch-owned-process-time-limit' }),
        expect.objectContaining({ label: 'Dispatch outcome', value: 'Dispatch ready' }),
        expect.objectContaining({ label: 'Host capability state', value: 'available' }),
        expect.objectContaining({ label: 'Host capability evidence', value: 'adapter-capability-state-ref' }),
        expect.objectContaining({ label: 'Host capability probe', value: 'windows-host-local-probe-ref' }),
        expect.objectContaining({ label: 'Execution state', value: 'Not claimed' }),
      ])
    );
    expect(intent.rows[1].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Decision status', value: 'Blocked before dispatch' }),
        expect.objectContaining({ label: 'Dispatch intent', value: 'Not reported' }),
        expect.objectContaining({ label: 'Adapter dispatch', value: 'Not claimed' }),
      ])
    );
  });

  it('renders parser failures as review state', () => {
    const intent = createAppGameAdapterDispatchPreflightPanelIntent({
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
