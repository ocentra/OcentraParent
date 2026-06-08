import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import {
  AgentAppGameAdapterDispatchCommandResultDecision,
  AgentAppGameAdapterDispatchCommandResultState,
  AgentAppGameAdapterDispatchExecutionAuditDecision,
  AgentAppGameAdapterDispatchExecutionAuditState,
  type AgentAppGameAdapterDispatchResultReadModel,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-dispatch-result';
import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightState,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-dispatch-preflight';
import { describe, expect, it } from 'vitest';
import { createAppGameAdapterDispatchResultPanelIntent } from '../src/app-game-adapter-dispatch-result-panel';

const ReadModel: AgentAppGameAdapterDispatchResultReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-adapter-dispatch-result',
  generatedAt: '2026-06-08T10:44:00.000Z',
  sourceReadModelIds: ['app-game-adapter-dispatch-preflight', 'agent.enforcement.execute'],
  custodyLabel: 'adapter-dispatch-preflight-and-enforcement-command-result',
  capabilityStatus: 'app-game-adapter-dispatch-command-result-partial',
  returned: 2,
  commandAcceptedCount: 1,
  blockedBeforeCommandCount: 1,
  executionAuditRecordedCount: 1,
  blockedBeforeExecutionAuditCount: 1,
  adapterDispatchCommandResultClaimedCount: 1,
  serviceLocalExecutionAuditClaimedCount: 1,
  adapterDispatchExecutedClaimedCount: 0,
  broadInstalledAppBlockingClaimed: false,
  childDeviceDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  providerDeliveryClaimed: false,
  privateDiagnosticsClaimed: false,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-dispatch-result-windows-app-game-owned-process-time-limit',
      sourceDispatchPreflightRowId: 'app-game-adapter-dispatch-preflight-windows-app-game-owned-process-time-limit',
      sourceProofEntryId: 'windows-app-game-owned-process-time-limit',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'app-game-owned-process-time-limit',
      dispatchPreflightState: AgentAppGameAdapterDispatchPreflightState.DispatchEligible,
      dispatchDecision: AgentAppGameAdapterDispatchDecision.DispatchEligible,
      dispatchIntentId: 'dispatch-owned-process-time-limit',
      dispatchOutcomeState: AgentAppGameAdapterDispatchOutcomeState.DispatchReady,
      dispatchCommandResultState: AgentAppGameAdapterDispatchCommandResultState.CommandAccepted,
      dispatchCommandResultDecision: AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted,
      enforcementCommandName: 'agent.enforcement.execute',
      enforcementEventName: 'agent.enforcement.audit.reported',
      enforcementActionMode: 'terminate-process',
      dispatchCommandResultId: 'app-game-dispatch-command-result-owned-process-time-limit',
      dispatchCommandAuditRefs: ['audit-owned-process-dispatch-command-accepted'],
      dispatchCommandTimerRefs: ['timer-owned-process-active'],
      dispatchExecutionAuditState: AgentAppGameAdapterDispatchExecutionAuditState.ServiceLocalAuditRecorded,
      dispatchExecutionAuditDecision: AgentAppGameAdapterDispatchExecutionAuditDecision.ServiceLocalAuditRecorded,
      dispatchExecutionAuditId: 'app-game-adapter-dispatch-execution-audit-owned-process-time-limit',
      dispatchExecutionAuditRefs: ['audit-owned-process-dispatch-service-local-execution-recorded'],
      manualProofRequirements: [],
      claimBoundary:
        'Dispatch command-result is limited to scoped Windows owned-process app/game time-limit rows and reuses agent.enforcement.execute.',
      fallbackBehavior: 'Rows without scoped process/session identity stay blocked before dispatch command handoff.',
      adapterDispatchCommandResultClaimed: true,
      adapterDispatchExecutedClaimed: false,
      serviceLocalExecutionAuditClaimed: true,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T10:44:00.000Z',
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-dispatch-result-windows-broad-installed-app-blocking-manual-gate',
      sourceDispatchPreflightRowId:
        'app-game-adapter-dispatch-preflight-windows-broad-installed-app-blocking-manual-gate',
      sourceProofEntryId: 'windows-broad-installed-app-blocking-manual-gate',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'broad-installed-app-blocking',
      dispatchPreflightState: AgentAppGameAdapterDispatchPreflightState.ManualRequired,
      dispatchDecision: AgentAppGameAdapterDispatchDecision.BlockedBeforeDispatch,
      dispatchIntentId: null,
      dispatchOutcomeState: AgentAppGameAdapterDispatchOutcomeState.ManualRequired,
      dispatchCommandResultState: AgentAppGameAdapterDispatchCommandResultState.ManualRequired,
      dispatchCommandResultDecision: AgentAppGameAdapterDispatchCommandResultDecision.BlockedBeforeCommand,
      enforcementCommandName: null,
      enforcementEventName: null,
      enforcementActionMode: null,
      dispatchCommandResultId: null,
      dispatchCommandAuditRefs: [],
      dispatchCommandTimerRefs: [],
      dispatchExecutionAuditState: AgentAppGameAdapterDispatchExecutionAuditState.BlockedBeforeExecutionAudit,
      dispatchExecutionAuditDecision: AgentAppGameAdapterDispatchExecutionAuditDecision.BlockedBeforeExecutionAudit,
      dispatchExecutionAuditId: null,
      dispatchExecutionAuditRefs: [],
      manualProofRequirements: ['same app identity proof'],
      claimBoundary: 'Broad installed-app blocking stays blocked before dispatch command handoff.',
      fallbackBehavior: 'The parent surface must route this row to manual review instead of dispatch.',
      adapterDispatchCommandResultClaimed: false,
      adapterDispatchExecutedClaimed: false,
      serviceLocalExecutionAuditClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T10:44:00.000Z',
    },
  ],
};

describe('app-game adapter dispatch result panel', () => {
  it('renders accepted scoped dispatch command-result without execution claims', () => {
    const intent = createAppGameAdapterDispatchResultPanelIntent({
      ok: true,
      value: ReadModel,
    });

    expect(intent.loadState).toBe('Review');
    expect(intent.summaryDetails.map((detail) => [detail.label, detail.value])).toContainEqual([
      'Adapter dispatch',
      'Ready',
    ]);
    expect(intent.summaryDetails.map((detail) => [detail.label, detail.value])).toContainEqual([
      'Execution state',
      'Not claimed',
    ]);
    expect(intent.rows[0]?.details.map((detail) => [detail.label, detail.value])).toEqual(
      expect.arrayContaining([
        ['Dispatch command', 'agent.enforcement.execute'],
        ['Dispatch event', 'agent.enforcement.audit.reported'],
        ['Dispatch action', 'terminate-process'],
        ['Execution audit', 'Ready'],
        ['Execution audit refs', 'audit-owned-process-dispatch-service-local-execution-recorded'],
        ['Execution state', 'Not claimed'],
      ])
    );
    expect(intent.rows[1]?.details.map((detail) => [detail.label, detail.value])).toEqual(
      expect.arrayContaining([
        ['Dispatch command', 'Not reported'],
        ['Dispatch event', 'Not reported'],
        ['Manual review', 'same app identity proof'],
        ['Adapter dispatch', 'Not claimed'],
        ['Execution audit', 'Not claimed'],
      ])
    );
  });
});
