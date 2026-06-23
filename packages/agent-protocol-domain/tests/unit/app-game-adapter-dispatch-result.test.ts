import { AppGameSchemaVersion } from '@ocentra-parent/schema-domain/app-game-primitives';
import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightState,
} from '@ocentra-parent/schema-domain/app-game-adapter-dispatch-preflight';
import {
  AgentAppGameAdapterDispatchAdapterExecutionDecision,
  AgentAppGameAdapterDispatchAdapterExecutionState,
  AgentAppGameAdapterDispatchCommandResultDecision,
  AgentAppGameAdapterDispatchCommandResultState,
  AgentAppGameAdapterDispatchExecuteResultPayloadField,
  AgentAppGameAdapterDispatchExecutionAuditDecision,
  AgentAppGameAdapterDispatchExecutionAuditState,
  AgentAppGameAdapterDispatchResultPayloadField,
} from '@ocentra-parent/schema-domain/app-game-adapter-dispatch-result';
import { describe, expect, it } from 'vitest';
import {
  AgentCommand,
  AgentEvent,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import {
  parseAgentAppGameAdapterDispatchExecuteEvent,
  parseAgentAppGameAdapterDispatchResultEvent,
} from '../../src/app-game-adapter-dispatch-result';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const DispatchResultReadModel = {
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
} as const;

const DispatchExecuteResult = {
  schemaVersion: AppGameSchemaVersion,
  commandId: 'app-game-adapter-dispatch-execute-command',
  generatedAt: '2026-06-08T10:44:02.000Z',
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
} as const;

function unsafeBroadDispatchResultReadModel() {
  return {
    ...DispatchResultReadModel,
    rows: [
      {
        ...DispatchResultReadModel.rows[1],
        dispatchCommandResultState: AgentAppGameAdapterDispatchCommandResultState.CommandAccepted,
        dispatchCommandResultDecision: AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted,
        enforcementCommandName: AgentCommand.EnforcementExecute,
        enforcementEventName: AgentEvent.EnforcementAuditReported,
        enforcementActionMode: 'terminate-process',
        dispatchCommandResultId: 'unsafe-broad-app-command-result',
        dispatchCommandAuditRefs: ['unsafe-audit-ref'],
        dispatchCommandTimerRefs: ['unsafe-timer-ref'],
        dispatchExecutionAuditState: AgentAppGameAdapterDispatchExecutionAuditState.ServiceLocalAuditRecorded,
        dispatchExecutionAuditDecision: AgentAppGameAdapterDispatchExecutionAuditDecision.ServiceLocalAuditRecorded,
        dispatchExecutionAuditId: 'unsafe-broad-app-execution-audit',
        dispatchExecutionAuditRefs: ['unsafe-execution-audit-ref'],
        dispatchAdapterExecutionState: AgentAppGameAdapterDispatchAdapterExecutionState.AdapterExecutionReported,
        dispatchAdapterExecutionDecision: AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionReported,
        dispatchAdapterExecutionResultId: 'unsafe-broad-app-execution-result',
        dispatchAdapterExecutionStatus: 'actually-enforced',
        dispatchAdapterExecutionAdapterResultCode: 'process-exited',
        dispatchAdapterExecutionAuditEventId: 'unsafe-broad-app-audit-event',
        dispatchAdapterExecutionRefs: ['unsafe-broad-app-adapter-execution-ref'],
        manualProofRequirements: [],
        adapterDispatchCommandResultClaimed: true,
        adapterDispatchExecutedClaimed: true,
        serviceLocalExecutionAuditClaimed: true,
      },
    ],
  };
}

describe('agent app-game adapter dispatch result parser', () => {
  it('parses scoped dispatch command-result rows without claiming adapter execution', () => {
    const parsed = parseAgentAppGameAdapterDispatchResultEvent(
      dispatchResultEvent(JSON.stringify(DispatchResultReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: DispatchResultReadModel,
    });
  });

  it('rejects invalid result payloads and unsafe command-result claim upgrades', () => {
    expect(
      parseAgentAppGameAdapterDispatchResultEvent({
        ...dispatchResultEvent(JSON.stringify(DispatchResultReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGameAdapterDispatchResultEvent(dispatchResultEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGameAdapterDispatchResultEvent(
        dispatchResultEvent(JSON.stringify(unsafeBroadDispatchResultReadModel()))
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });

  it('parses scoped adapter dispatch execute results without broad platform claims', () => {
    expect(
      parseAgentAppGameAdapterDispatchExecuteEvent(dispatchExecuteEvent(JSON.stringify(DispatchExecuteResult)))
    ).toEqual({
      ok: true,
      value: DispatchExecuteResult,
    });

    expect(
      parseAgentAppGameAdapterDispatchExecuteEvent({
        ...dispatchExecuteEvent(JSON.stringify(DispatchExecuteResult)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGameAdapterDispatchExecuteEvent(dispatchExecuteEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGameAdapterDispatchExecuteEvent(
        dispatchExecuteEvent(
          JSON.stringify({
            ...DispatchExecuteResult,
            sourceProofEntryId: 'windows-broad-installed-app-blocking-manual-gate',
            broadInstalledAppBlockingClaimed: true,
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function dispatchResultEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-adapter-dispatch-result-event',
    correlationId: 'app-game-adapter-dispatch-result-command',
    sentAt: '2026-06-08T10:44:01.000Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGameAdapterDispatchResultReadModelReported,
    severity: 'info',
    payload: {
      [AgentAppGameAdapterDispatchResultPayloadField]: serializedReadModel,
    },
    snapshot: null,
  };
}

function dispatchExecuteEvent(serializedResult: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-adapter-dispatch-execute-event',
    correlationId: 'app-game-adapter-dispatch-execute-command',
    sentAt: '2026-06-08T10:44:03.000Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGameAdapterDispatchExecuted,
    severity: 'info',
    payload: {
      [AgentAppGameAdapterDispatchExecuteResultPayloadField]: serializedResult,
    },
    snapshot: null,
  };
}
