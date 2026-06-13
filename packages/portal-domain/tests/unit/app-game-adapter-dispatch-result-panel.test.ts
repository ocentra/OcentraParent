import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { AgentCommand, AgentEvent } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  AgentAppGameAdapterDispatchAdapterExecutionDecision,
  AgentAppGameAdapterDispatchAdapterExecutionState,
  AgentAppGameAdapterDispatchCommandResultDecision,
  AgentAppGameAdapterDispatchCommandResultState,
  AgentAppGameAdapterDispatchExecutionAuditDecision,
  AgentAppGameAdapterDispatchExecutionAuditState,
  type AgentAppGameAdapterDispatchExecuteResult,
  type AgentAppGameAdapterDispatchResultReadModel,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-dispatch-result';
import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightState,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-dispatch-preflight';
import { describe, expect, it } from 'vitest';
import { createAppGameAdapterDispatchResultPanelIntent } from '../../src/app-game-adapter-dispatch-result-panel';

const ReadModel: AgentAppGameAdapterDispatchResultReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-adapter-dispatch-result',
  generatedAt: '2026-06-08T10:44:00.000Z',
  sourceReadModelIds: ['app-game-adapter-dispatch-preflight', AgentCommand.EnforcementExecute],
  custodyLabel: 'adapter-dispatch-preflight-and-enforcement-command-result',
  capabilityStatus: 'app-game-adapter-dispatch-command-result-partial',
  returned: 2,
  commandAcceptedCount: 1,
  blockedBeforeCommandCount: 1,
  executionAuditRecordedCount: 1,
  blockedBeforeExecutionAuditCount: 1,
  adapterExecutionReportedCount: 1,
  adapterExecutionEvidenceMissingCount: 0,
  blockedBeforeAdapterExecutionCount: 1,
  adapterDispatchCommandResultClaimedCount: 1,
  serviceLocalExecutionAuditClaimedCount: 1,
  adapterDispatchExecutedClaimedCount: 1,
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
      enforcementCommandName: AgentCommand.EnforcementExecute,
      enforcementEventName: AgentEvent.EnforcementAuditReported,
      enforcementActionMode: 'terminate-process',
      dispatchCommandResultId: 'app-game-dispatch-command-result-owned-process-time-limit',
      dispatchCommandAuditRefs: ['audit-owned-process-dispatch-command-accepted'],
      dispatchCommandTimerRefs: ['timer-owned-process-active'],
      dispatchExecutionAuditState: AgentAppGameAdapterDispatchExecutionAuditState.ServiceLocalAuditRecorded,
      dispatchExecutionAuditDecision: AgentAppGameAdapterDispatchExecutionAuditDecision.ServiceLocalAuditRecorded,
      dispatchExecutionAuditId: 'app-game-adapter-dispatch-execution-audit-owned-process-time-limit',
      dispatchExecutionAuditRefs: ['audit-owned-process-dispatch-service-local-execution-recorded'],
      dispatchAdapterExecutionState: AgentAppGameAdapterDispatchAdapterExecutionState.AdapterExecutionReported,
      dispatchAdapterExecutionDecision: AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionReported,
      dispatchAdapterExecutionResultId: 'enforcement-result-app-game-owned-process',
      dispatchAdapterExecutionStatus: 'actually-enforced',
      dispatchAdapterExecutionAdapterResultCode: 'process-already-exited',
      dispatchAdapterExecutionAuditEventId: 'enforcement-audit-app-game-owned-process',
      dispatchAdapterExecutionRefs: ['adapter-execution-audit-enforcement-audit-app-game-owned-process'],
      manualProofRequirements: [],
      claimBoundary: `Dispatch command-result is limited to scoped Windows owned-process app/game time-limit rows and reuses ${AgentCommand.EnforcementExecute}.`,
      fallbackBehavior: 'Rows without scoped process/session identity stay blocked before dispatch command handoff.',
      adapterDispatchCommandResultClaimed: true,
      adapterDispatchExecutedClaimed: true,
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
      dispatchAdapterExecutionState: AgentAppGameAdapterDispatchAdapterExecutionState.BlockedBeforeAdapterExecution,
      dispatchAdapterExecutionDecision:
        AgentAppGameAdapterDispatchAdapterExecutionDecision.BlockedBeforeAdapterExecution,
      dispatchAdapterExecutionResultId: null,
      dispatchAdapterExecutionStatus: null,
      dispatchAdapterExecutionAdapterResultCode: null,
      dispatchAdapterExecutionAuditEventId: null,
      dispatchAdapterExecutionRefs: [],
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

const ExecuteResult: AgentAppGameAdapterDispatchExecuteResult = {
  schemaVersion: AppGameSchemaVersion,
  commandId: 'app-game-adapter-dispatch-execute-command',
  generatedAt: '2026-06-08T12:45:00.000Z',
  sourceReadModelId: 'app-game-adapter-dispatch-result',
  sourceDispatchRowId: 'app-game-adapter-dispatch-result-windows-app-game-owned-process-time-limit',
  sourceProofEntryId: 'windows-app-game-owned-process-time-limit',
  executionCommandName: AgentCommand.EnforcementExecute,
  executionEventName: AgentEvent.EnforcementAuditReported,
  executionResultId: 'enforcement-result-app-game-owned-process',
  executionStatus: 'actually-enforced',
  executionAdapterResultCode: 'process-already-exited',
  executionAuditEventId: 'enforcement-audit-app-game-owned-process',
  readbackCommandName: AgentCommand.ActivityAppGameAdapterDispatchResultReadModelGet,
  adapterDispatchExecutedClaimed: true,
  broadInstalledAppBlockingClaimed: false,
  childDeviceDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  providerDeliveryClaimed: false,
  privateDiagnosticsClaimed: false,
};

describe('app-game adapter dispatch result panel', () => {
  it('renders accepted scoped dispatch command-result with scoped execution evidence', () => {
    const intent = createAppGameAdapterDispatchResultPanelIntent({
      ok: true,
      value: ReadModel,
    });

    expect(intent.loadState).toBe('Review');
    expectScopedDispatchSummary(intent.summaryDetails);
    expectAcceptedScopedDispatchRow(intent.rows[0]);
    expectBlockedBroadDispatchRow(intent.rows[1]);
  });

  it('renders latest manual execute result without platform or child-delivery claim upgrades', () => {
    const intent = createAppGameAdapterDispatchResultPanelIntent(
      {
        ok: true,
        value: ReadModel,
      },
      {
        ok: true,
        value: ExecuteResult,
      }
    );

    expect(intent.summaryDetails.map((detail) => [detail.label, detail.value])).toEqual(
      expect.arrayContaining([
        ['Execute command', 'app-game-adapter-dispatch-execute-command'],
        ['Execute status', 'actually-enforced'],
        ['Execute result', 'enforcement-result-app-game-owned-process'],
        ['Adapter execution status', 'process-already-exited'],
        ['Execute audit', 'enforcement-audit-app-game-owned-process'],
        ['Execute readback', AgentCommand.ActivityAppGameAdapterDispatchResultReadModelGet],
        ['Adapter dispatch', 'Ready'],
        ['Platform state', 'Not claimed'],
        ['Child delivery', 'Not claimed'],
      ])
    );
  });

  it('exposes the manual execute action only when the scoped command row is accepted', () => {
    const acceptedIntent = createAppGameAdapterDispatchResultPanelIntent({
      ok: true,
      value: ReadModel,
    });
    const blockedIntent = createAppGameAdapterDispatchResultPanelIntent({
      ok: true,
      value: {
        ...ReadModel,
        commandAcceptedCount: 0,
        blockedBeforeCommandCount: 1,
        executionAuditRecordedCount: 0,
        blockedBeforeExecutionAuditCount: 1,
        adapterExecutionReportedCount: 0,
        blockedBeforeAdapterExecutionCount: 1,
        adapterDispatchCommandResultClaimedCount: 0,
        serviceLocalExecutionAuditClaimedCount: 0,
        adapterDispatchExecutedClaimedCount: 0,
        rows: [ReadModel.rows[1]],
        returned: 1,
      },
    });

    expect(acceptedIntent.executeAction).toEqual({
      label: 'Execute scoped adapter dispatch',
      command: AgentCommand.ActivityAppGameAdapterDispatchExecute,
      resultEvent: AgentEvent.ActivityAppGameAdapterDispatchExecuted,
    });
    expect(blockedIntent.executeAction).toBeNull();
  });
});

function expectScopedDispatchSummary(
  details: ReturnType<typeof createAppGameAdapterDispatchResultPanelIntent>['summaryDetails']
) {
  expect(details.map((detail) => [detail.label, detail.value])).toContainEqual(['Adapter dispatch', 'Ready']);
  expect(details.map((detail) => [detail.label, detail.value])).toContainEqual(['Execution state', 'Ready']);
}

function expectAcceptedScopedDispatchRow(
  row: ReturnType<typeof createAppGameAdapterDispatchResultPanelIntent>['rows'][number] | undefined
) {
  expect(row?.details.map((detail) => [detail.label, detail.value])).toEqual(
    expect.arrayContaining([
      ['Dispatch command', AgentCommand.EnforcementExecute],
      ['Dispatch event', AgentEvent.EnforcementAuditReported],
      ['Dispatch action', 'terminate-process'],
      ['Execution audit', 'Ready'],
      ['Execution audit refs', 'audit-owned-process-dispatch-service-local-execution-recorded'],
      ['Adapter execution', 'Adapter execution reported'],
      ['Adapter execution result', 'enforcement-result-app-game-owned-process'],
      ['Adapter execution status', 'actually-enforced'],
      ['Execution state', 'Ready'],
    ])
  );
}

function expectBlockedBroadDispatchRow(
  row: ReturnType<typeof createAppGameAdapterDispatchResultPanelIntent>['rows'][number] | undefined
) {
  expect(row?.details.map((detail) => [detail.label, detail.value])).toEqual(
    expect.arrayContaining([
      ['Dispatch command', 'Not reported'],
      ['Dispatch event', 'Not reported'],
      ['Manual review', 'same app identity proof'],
      ['Adapter dispatch', 'Not claimed'],
      ['Execution audit', 'Not claimed'],
      ['Adapter execution', 'Blocked before adapter execution'],
    ])
  );
}
