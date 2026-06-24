import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { AppGameSchemaVersion } from '@ocentra-parent/schema-domain/app-game-primitives';
import {
  AgentAppGameAdapterDispatchAdapterExecutionDecision,
  AgentAppGameAdapterDispatchAdapterExecutionState,
  AgentAppGameAdapterDispatchCommandResultDecision,
  AgentAppGameAdapterDispatchCommandResultState,
  AgentAppGameAdapterDispatchExecutionAuditDecision,
  AgentAppGameAdapterDispatchExecutionAuditState,
  type AgentAppGameAdapterDispatchResultReadModel,
} from '@ocentra-parent/schema-domain/app-game-adapter-dispatch-result';
import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightState,
} from '@ocentra-parent/schema-domain/app-game-adapter-dispatch-preflight';
import { createAppGameAdapterDispatchResultPanelIntent } from '@ocentra-parent/portal-domain/app-game-adapter-dispatch-result-panel';
import { PortalRoute } from '@ocentra-parent/schema-domain/portal-contracts';
import type { PortalRenderActions } from '../src/portal-actions';
import {
  AppGameAdapterDispatchRoutePanel,
  sendAppGameAdapterDispatchExecuteAction,
  shouldRenderAppGameAdapterDispatchRoute,
} from '../src/AppGameAdapterDispatchRoutePanel';

const AcceptedReadModel: AgentAppGameAdapterDispatchResultReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-adapter-dispatch-result',
  generatedAt: '2026-06-08T13:20:00.000Z',
  sourceReadModelIds: ['app-game-adapter-dispatch-preflight', 'agent.enforcement.execute'],
  custodyLabel: 'adapter-dispatch-preflight-and-enforcement-command-result',
  capabilityStatus: 'app-game-adapter-dispatch-command-result-partial',
  returned: 1,
  commandAcceptedCount: 1,
  blockedBeforeCommandCount: 0,
  executionAuditRecordedCount: 1,
  blockedBeforeExecutionAuditCount: 0,
  adapterExecutionReportedCount: 1,
  adapterExecutionEvidenceMissingCount: 0,
  blockedBeforeAdapterExecutionCount: 0,
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
      dispatchAdapterExecutionState: AgentAppGameAdapterDispatchAdapterExecutionState.AdapterExecutionReported,
      dispatchAdapterExecutionDecision: AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionReported,
      dispatchAdapterExecutionResultId: 'enforcement-result-app-game-owned-process',
      dispatchAdapterExecutionStatus: 'actually-enforced',
      dispatchAdapterExecutionAdapterResultCode: 'process-already-exited',
      dispatchAdapterExecutionAuditEventId: 'enforcement-audit-app-game-owned-process',
      dispatchAdapterExecutionRefs: ['adapter-execution-audit-enforcement-audit-app-game-owned-process'],
      manualProofRequirements: [],
      claimBoundary:
        'Dispatch command-result is limited to scoped Windows owned-process app/game time-limit rows and reuses agent.enforcement.execute.',
      fallbackBehavior: 'Rows without scoped process/session identity stay blocked before dispatch command handoff.',
      adapterDispatchCommandResultClaimed: true,
      adapterDispatchExecutedClaimed: true,
      serviceLocalExecutionAuditClaimed: true,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T13:20:00.000Z',
    },
  ],
};

describe('app-game adapter dispatch route panel', () => {
  it('attaches only to App/Game Sessions', () => {
    expect(shouldRenderAppGameAdapterDispatchRoute(PortalRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGameAdapterDispatchRoute(PortalRoute.Overview)).toBe(false);
  });

  it('sends the explicit scoped execute action through the typed command path', () => {
    const resultIntent = createAppGameAdapterDispatchResultPanelIntent({
      ok: true,
      value: AcceptedReadModel,
    });
    let requested = 0;
    const actions: PortalRenderActions = {
      reconnect() {},
      selectCommandResult() {},
      async sendCommand() {
        return null;
      },
      async requestAppGameAdapterDispatchExecute() {
        requested += 1;
        return null;
      },
    };

    expect(resultIntent.executeAction).not.toBeNull();
    if (resultIntent.executeAction === null) {
      return;
    }
    sendAppGameAdapterDispatchExecuteAction(actions, resultIntent.executeAction);

    expect(requested).toBe(1);
  });

  it('renders refresh controls and the scoped execute control without broad/platform claim upgrades', () => {
    const html = renderToStaticMarkup(
      <AppGameAdapterDispatchRoutePanel
        actions={{
          reconnect() {},
          selectCommandResult() {},
          async sendCommand() {
            return null;
          },
        }}
        commandEnabled={true}
        executeResult={null}
        preflightResult={null}
        resultReadModel={{
          ok: true,
          value: AcceptedReadModel,
        }}
      />
    );

    expect(html).toContain('Refresh adapter dispatch preflight');
    expect(html).toContain('Refresh adapter dispatch result');
    expect(html).toContain('Execute scoped adapter dispatch');
    expect(html).toContain('windows-app-game-owned-process-time-limit');
    expect(html).toContain('Broad installed-app blocking, platform enforcement, provider delivery');
    expect(html).toContain('Platform state</dt><dd>Not claimed');
    expect(html).toContain('Child delivery</dt><dd>Not claimed');
  });
});
