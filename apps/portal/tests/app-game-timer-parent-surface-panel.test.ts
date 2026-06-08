import { describe, expect, it } from 'vitest';
import {
  AgentCommand,
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  AgentProtocolSchemaVersion,
  AppGameTimerParentPreferenceSetupRequestSchema,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  AgentAppGameTimerParentSurfaceState,
  AgentAppGameTimerParentSurfaceTargetDomain,
} from '@ocentra-parent/agent-protocol-domain/app-game-timer-parent-surface-read-model';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { shouldRenderAppGameTimerParentSurfaceRoute } from '../src/AppGameTimerParentSurfaceRoutePanel';
import {
  createAppGameTimerParentPreferenceSetupCommandResultDetails,
  createAppGameTimerParentPreferenceSetupRequestPayload,
  createAppGameTimerParentSurfacePanelIntent,
} from '../src/app-game-timer-parent-surface-panel';
import { isCommandResultEvent } from '../src/event-results';
import { resolveLiveActivityState } from '../src/live-activity-state';

const AppGameSchemaVersion = 1;

const TimerParentSurfaceReadModel = {
  schemaVersion: AppGameSchemaVersion,
  generatedAt: '2026-06-07T17:55:00Z',
  custodyLabel: 'child-device-query-store',
  capabilityStatus: 'timer-parent-surface-partial',
  returned: 2,
  readyForParentSurfaceCount: 1,
  blockedBySourceFreshnessCount: 1,
  blockedByCompilerDecisionCount: 0,
  runtimeManualRequiredCount: 0,
  controlActionResultCount: 0,
  controlActionResultReferenceIds: [],
  controlActionResultStatuses: [],
  controlActionResultCapabilityStates: [],
  controlActionResultEnforcementStatuses: [],
  childFacingReasonReferenceIds: [],
  childFacingStatusReferenceIds: [],
  childUxHandoffReadyCount: 0,
  childUxHandoffBlockedCount: 0,
  childUxHandoffReferenceIds: [],
  childUxLocalHandoffArtifactRecordCount: 0,
  childUxLocalHandoffArtifactSkippedCount: 0,
  childUxLocalHandoffArtifactReferenceIds: [],
  childUxLocalHandoffArtifactRecords: [],
  childUxParentSurfaceIntentManualActionRequiredCount: 0,
  childUxParentSurfaceIntentUnavailableVisibleCount: 0,
  childUxParentSurfaceIntentHistoryVisibleCount: 0,
  childUxParentSurfaceIntentPreferenceSetupRequiredCount: 0,
  childUxParentSurfaceIntentReferenceIds: [],
  childUxParentSurfaceIntentRecords: [],
  childUxParentPreferenceSetupDraftReadyCount: 0,
  childUxParentPreferenceSetupUnavailableVisibleCount: 0,
  childUxParentPreferenceSetupReferenceIds: [],
  childUxParentPreferenceSetupRequestReadyCount: 0,
  childUxParentPreferenceSetupRequestUnavailableVisibleCount: 0,
  childUxParentPreferenceSetupRequestReferenceIds: [],
  childUxParentPreferenceSetupRecords: [],
  timerRuntimeClaimed: false,
  schedulerPersistenceClaimed: false,
  durableSchedulerStorageClaimed: false,
  auditRuntimeClaimed: false,
  rollbackRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
  rows: [
    timerParentSurfaceRow('identity-study-timer', {
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeApp,
      timerSurfaceState: AgentAppGameTimerParentSurfaceState.ReadyForParentSurface,
      evidenceReferenceIds: ['identity-study-timer', 'claim-study-timer'],
    }),
    timerParentSurfaceRow('identity-voxel-quest', {
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeGame,
      timerSurfaceState: AgentAppGameTimerParentSurfaceState.BlockedBySourceFreshness,
      evidenceReferenceIds: ['identity-voxel-quest', 'source-stale'],
    }),
  ],
} as const;

const ActionResultReadModel = {
  ...TimerParentSurfaceReadModel,
  controlActionResultCount: 1,
  controlActionResultReferenceIds: ['action-result-app-game-1'],
  controlActionResultStatuses: ['enforced'],
  controlActionResultCapabilityStates: ['supported'],
  controlActionResultEnforcementStatuses: ['actually-enforced'],
  childFacingReasonReferenceIds: ['parent-approved'],
  childFacingStatusReferenceIds: ['child-status-limit-reached'],
  childUxHandoffReadyCount: 1,
  childUxHandoffBlockedCount: 0,
  childUxHandoffReferenceIds: ['action-result-app-game-1'],
  childUxLocalHandoffArtifactRecordCount: 1,
  childUxLocalHandoffArtifactSkippedCount: 0,
  childUxLocalHandoffArtifactReferenceIds: ['app-game-child-ux-local-handoff-action-result-app-game-1'],
  childUxLocalHandoffArtifactRecords: [
    {
      schemaVersion: AppGameSchemaVersion,
      artifactReferenceId: 'app-game-child-ux-local-handoff-action-result-app-game-1',
      sourceResultId: 'action-result-app-game-1',
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeGame,
      childReasonReferenceIds: ['parent-approved'],
      childStatusReferenceIds: ['child-status-limit-reached'],
      childDeliveryClaimed: false,
      notificationDeliveryClaimed: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      rawPrivateSourceRowsIncluded: false,
    },
  ],
  childUxParentSurfaceIntentManualActionRequiredCount: 1,
  childUxParentSurfaceIntentUnavailableVisibleCount: 0,
  childUxParentSurfaceIntentHistoryVisibleCount: 1,
  childUxParentSurfaceIntentPreferenceSetupRequiredCount: 1,
  childUxParentSurfaceIntentReferenceIds: ['app-game-child-ux-parent-surface-action-result-app-game-1'],
  childUxParentSurfaceIntentRecords: [
    {
      schemaVersion: AppGameSchemaVersion,
      parentSurfaceIntentReferenceId: 'app-game-child-ux-parent-surface-action-result-app-game-1',
      sourceResultId: 'action-result-app-game-1',
      sourceArtifactReferenceId: 'app-game-child-ux-local-handoff-action-result-app-game-1',
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeGame,
      historyVisibility: 'history-row-visible',
      parentSurfaceStatus: 'manual-action-required',
      preferenceVisibility: 'preference-setup-required',
      drillInReferenceIds: [
        'app-game-child-ux-local-handoff-action-result-app-game-1',
        'parent-approved',
        'child-status-limit-reached',
      ],
      manualProofReferenceIds: ['parent-approved', 'child-status-limit-reached'],
      sensitiveDetailIncluded: false,
      parentNotificationUiRendered: false,
      parentPreferenceMutationClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      rawPrivateSourceRowsIncluded: false,
    },
  ],
  childUxParentPreferenceSetupDraftReadyCount: 1,
  childUxParentPreferenceSetupUnavailableVisibleCount: 0,
  childUxParentPreferenceSetupReferenceIds: ['app-game-child-ux-parent-preference-setup-action-result-app-game-1'],
  childUxParentPreferenceSetupRequestReadyCount: 1,
  childUxParentPreferenceSetupRequestUnavailableVisibleCount: 0,
  childUxParentPreferenceSetupRequestReferenceIds: [
    'app-game-child-ux-parent-preference-setup-action-result-app-game-1',
  ],
  childUxParentPreferenceSetupRecords: [
    {
      schemaVersion: AppGameSchemaVersion,
      parentPreferenceSetupReferenceId: 'app-game-child-ux-parent-preference-setup-action-result-app-game-1',
      sourceParentSurfaceIntentReferenceId: 'app-game-child-ux-parent-surface-action-result-app-game-1',
      sourceResultId: 'action-result-app-game-1',
      sourceArtifactReferenceId: 'app-game-child-ux-local-handoff-action-result-app-game-1',
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeGame,
      draftStatus: 'draft-ready',
      parentPreferenceSetupRequestStatus: 'request-ready',
      parentPreferenceSetupRequestReferenceIds: [
        'app-game-child-ux-local-handoff-action-result-app-game-1',
        'parent-approved',
        'child-status-limit-reached',
      ],
      drillInReferenceIds: [
        'app-game-child-ux-local-handoff-action-result-app-game-1',
        'parent-approved',
        'child-status-limit-reached',
      ],
      manualProofReferenceIds: ['parent-approved', 'child-status-limit-reached'],
      parentPreferenceUiRendered: false,
      parentFrequencyControlUiRendered: false,
      parentPreferenceMutationClaimed: false,
      notificationRuleMutationClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      rawPrivateSourceRowsIncluded: false,
    },
  ],
} as const;

describe('app-game timer parent-surface portal route panel', () => {
  it('attaches the renderer only to App/Game Sessions', expectRouteAttachment);
  it('uses the latest service-backed timer parent-surface event for the route intent', expectServiceBackedIntent);
  it(
    'shows active timer state-store visibility without upgrading audit or adapter claims',
    expectActiveStateVisibility
  );
  it('shows replayed control action-result visibility without adapter claims', expectControlActionResultVisibility);
  it(
    'shows accepted parent preference setup child-runtime handoff result without delivery claims',
    expectParentPreferenceSetupCommandResultVisibility
  );
  it('keeps absent or invalid service input explicit instead of inventing rows', expectAbsentServiceInput);
});

function expectRouteAttachment() {
  expect(shouldRenderAppGameTimerParentSurfaceRoute(PortalRoute.AppGameSessions)).toBe(true);
  expect(shouldRenderAppGameTimerParentSurfaceRoute(PortalRoute.Overview)).toBe(false);
}

function expectServiceBackedIntent() {
  const liveActivity = resolveLiveActivityState([timerParentSurfaceEvent(JSON.stringify(TimerParentSurfaceReadModel))]);

  expect(liveActivity.appGameTimerParentSurfaceReadModel).toMatchObject({
    ok: true,
    value: {
      returned: 2,
      timerRuntimeClaimed: false,
      adapterDispatchClaimed: false,
      childDeliveryClaimed: false,
      platformEnforcementClaimed: false,
    },
  });

  const intent = createAppGameTimerParentSurfacePanelIntent(liveActivity.appGameTimerParentSurfaceReadModel);
  expectNoRuntimeSummaryDetails(intent.summaryDetails);
  expectTimerParentSurfaceRows(intent.rows);
}

function expectActiveStateVisibility() {
  const activeStateModel = {
    ...TimerParentSurfaceReadModel,
    timerRuntimeClaimed: true,
    schedulerPersistenceClaimed: true,
    durableSchedulerStorageClaimed: true,
    auditRuntimeClaimed: true,
    rollbackRuntimeClaimed: true,
  };
  const liveActivity = resolveLiveActivityState([timerParentSurfaceEvent(JSON.stringify(activeStateModel))]);

  const intent = createAppGameTimerParentSurfacePanelIntent(liveActivity.appGameTimerParentSurfaceReadModel);

  expect(intent.summaryDetails).toContainEqual({
    label: 'Timer runtime',
    value: 'Ready',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Scheduler persistence',
    value: 'Ready',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Durable scheduler storage',
    value: 'Ready',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Audit runtime',
    value: 'Ready',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Rollback runtime',
    value: 'Ready',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Adapter dispatch',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Product claim',
    value:
      'Active timer state-store is visible; live scheduling execution, durable audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.',
  });
}

function expectControlActionResultVisibility() {
  const liveActivity = resolveLiveActivityState([timerParentSurfaceEvent(JSON.stringify(ActionResultReadModel))]);

  const intent = createAppGameTimerParentSurfacePanelIntent(liveActivity.appGameTimerParentSurfaceReadModel);

  expectActionResultSummaryDetails(intent.summaryDetails);
  expectParentActionRows(intent.parentActionRows);
  expectParentPreferenceSetupRows(intent.parentPreferenceSetupRows);
  expect(intent.summaryDetails).toContainEqual({
    label: 'Adapter dispatch',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Product claim',
    value:
      'Control action-result rows are visible from app/game SQLite replay; live scheduling automation, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.',
  });
}

function expectParentPreferenceSetupCommandResultVisibility() {
  const details = createAppGameTimerParentPreferenceSetupCommandResultDetails(parentPreferenceSetupRequestedEvent());

  expect(details).toContainEqual({
    label: 'Status',
    value: 'Ready',
  });
  expect(details).toContainEqual({
    label: 'Parent preference setup action-result refs',
    value: 'app-game-parent-preference-setup-action-result::request-1',
  });
  expect(details).toContainEqual({
    label: 'Parent preference setup action-result status',
    value: 'Persisted',
  });
  expect(details).toContainEqual({
    label: 'Parent preference setup mutation receipt refs',
    value: 'app-game-parent-preference-setup-mutation-receipt::request-1',
  });
  expect(details).toContainEqual({
    label: 'Parent preference setup mutation receipt status',
    value: 'Persisted',
  });
  expect(details).toContainEqual({
    label: 'Child runtime handoff refs',
    value: 'app-game-parent-preference-setup-child-runtime-handoff::request-1',
  });
  expect(details).toContainEqual({
    label: 'Child runtime handoff status',
    value: 'Handoff ready',
  });
  expect(details).toContainEqual({
    label: 'Child runtime queue refs',
    value: 'app-game-parent-preference-setup-child-runtime-queue::request-1',
  });
  expect(details).toContainEqual({
    label: 'Child runtime queue status',
    value: 'Ready',
  });
  expect(details).toContainEqual({
    label: 'Parent preference setup mutation',
    value: 'Not claimed',
  });
  expect(details).toContainEqual({
    label: 'Notification rule mutation',
    value: 'Not claimed',
  });
  expect(details).toContainEqual({
    label: 'Child delivery',
    value: 'Not claimed',
  });
  expect(details).toContainEqual({
    label: 'Adapter dispatch',
    value: 'Not claimed',
  });
  expect(details).toContainEqual({
    label: 'Platform state',
    value: 'Not claimed',
  });
}

function expectNoRuntimeSummaryDetails(
  summaryDetails: ReturnType<typeof createAppGameTimerParentSurfacePanelIntent>['summaryDetails']
) {
  for (const detail of [
    ['Timer runtime', 'Not claimed'],
    ['Scheduler persistence', 'Not claimed'],
    ['Durable scheduler storage', 'Not claimed'],
    ['Control action results', '0'],
    ['Control action result refs', 'Not reported'],
    ['Control action result statuses', 'Not reported'],
    ['Child-facing reason refs', 'Not reported'],
    ['Child-facing status refs', 'Not reported'],
    ['Child UX handoff ready', '0'],
    ['Child UX handoff blocked', '0'],
    ['Child UX handoff refs', 'Not reported'],
    ['Child UX local artifact records', '0'],
    ['Child UX local artifact skipped', '0'],
    ['Child UX local artifact refs', 'Not reported'],
    ['Child UX local artifact sources', 'Not reported'],
    ['Child UX local artifact targets', 'Not reported'],
    ['Child UX parent-surface manual required', '0'],
    ['Child UX parent-surface unavailable', '0'],
    ['Child UX parent-surface history visible', '0'],
    ['Child UX parent-surface preference setup', '0'],
    ['Child UX parent-surface refs', 'Not reported'],
    ['Child UX parent-surface sources', 'Not reported'],
    ['Child UX parent-surface artifact refs', 'Not reported'],
    ['Child UX parent-surface targets', 'Not reported'],
    ['Child UX parent-surface drill-in refs', 'Not reported'],
    ['Child UX parent-surface manual proof refs', 'Not reported'],
    ['Adapter dispatch', 'Not claimed'],
    ['Child delivery', 'Not claimed'],
    ['Platform state', 'Not claimed'],
  ] as const) {
    expect(summaryDetails).toContainEqual({ label: detail[0], value: detail[1] });
  }
}

function expectActionResultSummaryDetails(
  summaryDetails: ReturnType<typeof createAppGameTimerParentSurfacePanelIntent>['summaryDetails']
) {
  for (const detail of [
    ['Control action results', '1'],
    ['Control action result refs', 'action-result-app-game-1'],
    ['Control action result statuses', 'enforced'],
    ['Control action capabilities', 'supported'],
    ['Control action enforcement statuses', 'actually-enforced'],
    ['Child-facing reason refs', 'parent-approved'],
    ['Child-facing status refs', 'child-status-limit-reached'],
    ['Child UX handoff ready', '1'],
    ['Child UX handoff blocked', '0'],
    ['Child UX handoff refs', 'action-result-app-game-1'],
    ['Child UX local artifact records', '1'],
    ['Child UX local artifact skipped', '0'],
    ['Child UX local artifact refs', 'app-game-child-ux-local-handoff-action-result-app-game-1'],
    ['Child UX local artifact sources', 'action-result-app-game-1'],
    ['Child UX local artifact targets', AgentAppGameTimerParentSurfaceTargetDomain.NativeGame],
    ['Child UX parent-surface manual required', '1'],
    ['Child UX parent-surface unavailable', '0'],
    ['Child UX parent-surface history visible', '1'],
    ['Child UX parent-surface preference setup', '1'],
    ['Child UX parent-surface refs', 'app-game-child-ux-parent-surface-action-result-app-game-1'],
    ['Child UX parent-surface sources', 'action-result-app-game-1'],
    ['Child UX parent-surface artifact refs', 'app-game-child-ux-local-handoff-action-result-app-game-1'],
    ['Child UX parent-surface targets', AgentAppGameTimerParentSurfaceTargetDomain.NativeGame],
    [
      'Child UX parent-surface drill-in refs',
      'app-game-child-ux-local-handoff-action-result-app-game-1 | parent-approved | child-status-limit-reached',
    ],
    ['Child UX parent-surface manual proof refs', 'parent-approved | child-status-limit-reached'],
    ['Parent preference setup request status', '1'],
    ['Parent preference setup request unavailable', '0'],
    ['Parent preference setup request refs', 'app-game-child-ux-parent-preference-setup-action-result-app-game-1'],
  ] as const) {
    expect(summaryDetails).toContainEqual({ label: detail[0], value: detail[1] });
  }
}

function expectTimerParentSurfaceRows(rows: ReturnType<typeof createAppGameTimerParentSurfacePanelIntent>['rows']) {
  expect(rows.map((row) => row.title)).toEqual(['identity-study-timer', 'identity-voxel-quest']);
  const firstRow = rows[0];
  const secondRow = rows[1];
  expect(firstRow).toBeDefined();
  expect(secondRow).toBeDefined();
  if (firstRow === undefined || secondRow === undefined) {
    return;
  }
  expect(rowPairs(firstRow)).toContainEqual(['Target type', 'Native app']);
  expect(rowPairs(firstRow)).toContainEqual(['Status', 'Ready for parent surface']);
  expect(rowPairs(secondRow)).toContainEqual(['Target type', 'Native game']);
  expect(rowPairs(secondRow)).toContainEqual(['Status', 'Blocked by source freshness']);
}

function expectParentActionRows(
  rows: ReturnType<typeof createAppGameTimerParentSurfacePanelIntent>['parentActionRows']
) {
  expect(rows.map((row) => row.title)).toEqual(['app-game-child-ux-parent-surface-action-result-app-game-1']);
  const row = rows[0];
  expect(row).toBeDefined();
  if (row === undefined) {
    return;
  }
  expect(rowPairs(row)).toContainEqual(['Target type', 'Native game']);
  expect(rowPairs(row)).toContainEqual(['Status', 'Manual action required']);
  expect(rowPairs(row)).toContainEqual([
    'Child UX parent-surface artifact refs',
    'app-game-child-ux-local-handoff-action-result-app-game-1',
  ]);
  expect(rowPairs(row)).toContainEqual(['Child UX parent-surface sources', 'action-result-app-game-1']);
  expect(rowPairs(row)).toContainEqual(['Child UX parent-surface history visible', 'History row visible']);
  expect(rowPairs(row)).toContainEqual(['Child UX parent-surface preference setup', 'Preference setup required']);
  expect(rowPairs(row)).toContainEqual([
    'Child UX parent-surface drill-in refs',
    'app-game-child-ux-local-handoff-action-result-app-game-1 | parent-approved | child-status-limit-reached',
  ]);
  expect(rowPairs(row)).toContainEqual([
    'Child UX parent-surface manual proof refs',
    'parent-approved | child-status-limit-reached',
  ]);
  expect(rowPairs(row)).toContainEqual(['Adapter dispatch', 'Not claimed']);
  expect(rowPairs(row)).toContainEqual(['Child delivery', 'Not claimed']);
  expect(rowPairs(row)).toContainEqual(['Platform state', 'Not claimed']);
}

function expectParentPreferenceSetupRows(
  rows: ReturnType<typeof createAppGameTimerParentSurfacePanelIntent>['parentPreferenceSetupRows']
) {
  expect(rows.map((row) => row.title)).toEqual(['app-game-child-ux-parent-preference-setup-action-result-app-game-1']);
  const row = rows[0];
  expect(row).toBeDefined();
  if (row === undefined) {
    return;
  }
  expect(rowPairs(row)).toContainEqual(['Target type', 'Native game']);
  expect(rowPairs(row)).toContainEqual(['Parent preference setup draft status', 'Preference setup required']);
  expect(rowPairs(row)).toContainEqual([
    'Parent preference setup draft refs',
    'app-game-child-ux-parent-preference-setup-action-result-app-game-1',
  ]);
  expect(rowPairs(row)).toContainEqual(['Parent preference setup request status', 'Ready']);
  expect(rowPairs(row)).toContainEqual([
    'Parent preference setup request refs',
    'app-game-child-ux-local-handoff-action-result-app-game-1 | parent-approved | child-status-limit-reached',
  ]);
  expect(rowPairs(row)).toContainEqual([
    'Child UX parent-surface refs',
    'app-game-child-ux-parent-surface-action-result-app-game-1',
  ]);
  expect(rowPairs(row)).toContainEqual(['Parent preference setup UI', 'Ready']);
  expect(rowPairs(row)).toContainEqual(['Parent preference setup mutation', 'Not claimed']);
  expect(rowPairs(row)).toContainEqual(['Notification rule mutation', 'Not claimed']);
  expect(rowPairs(row)).toContainEqual(['Adapter dispatch', 'Not claimed']);
  expect(rowPairs(row)).toContainEqual(['Child delivery', 'Not claimed']);
  expect(rowPairs(row)).toContainEqual(['Platform state', 'Not claimed']);
  expect(row.preferenceSetupRequestAction).toMatchObject({
    label: 'Request parent setup',
    command: AgentCommand.ActivityAppGameTimerParentPreferenceSetupRequest,
    resultEvent: AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested,
    parentSurfaceIntentReferenceId: 'app-game-child-ux-parent-surface-action-result-app-game-1',
    parentPreferenceSetupReferenceId: 'app-game-child-ux-parent-preference-setup-action-result-app-game-1',
    requestReferenceIds: [
      'app-game-child-ux-local-handoff-action-result-app-game-1',
      'parent-approved',
      'child-status-limit-reached',
    ],
  });
  expect(isCommandResultEvent(AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested)).toBe(true);
  const action = row.preferenceSetupRequestAction;
  expect(action).not.toBeNull();
  if (action === null || action === undefined) {
    return;
  }
  const payload = createAppGameTimerParentPreferenceSetupRequestPayload(action, '2026-06-08T00:55:00.000Z');
  const serializedRequest = payload[AgentProtocolDefaults.Field.ActivityAppGameTimerParentPreferenceSetupRequest];
  expect(typeof serializedRequest).toBe('string');
  const request = AppGameTimerParentPreferenceSetupRequestSchema.parse(JSON.parse(String(serializedRequest)));
  expect(request).toEqual({
    requestId:
      'app-game-parent-preference-setup-request::app-game-child-ux-parent-preference-setup-action-result-app-game-1::2026-06-08T00:55:00.000Z',
    requestedAt: '2026-06-08T00:55:00.000Z',
    parentSurfaceIntentReferenceId: 'app-game-child-ux-parent-surface-action-result-app-game-1',
    parentPreferenceSetupReferenceId: 'app-game-child-ux-parent-preference-setup-action-result-app-game-1',
    requestReferenceIds: [
      'app-game-child-ux-local-handoff-action-result-app-game-1',
      'parent-approved',
      'child-status-limit-reached',
    ],
  });
}

function expectAbsentServiceInput() {
  const intent = createAppGameTimerParentSurfacePanelIntent(null);

  expect(intent.loadState).toBe('Unavailable');
  expect(intent.parentActionRows).toEqual([]);
  expect(intent.parentPreferenceSetupRows).toEqual([]);
  expect(intent.rows).toEqual([]);
  expect(intent.emptyMessage).toBe('No app/game timer parent-surface read model has been reported yet.');
  expect(intent.summaryDetails).toContainEqual({
    label: 'Product claim',
    value:
      'Parent-surface rendering only; active timer state-store is shown only when reported by the service. Live scheduling execution, durable audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.',
  });
}

function timerParentSurfaceEvent(serializedReadModel: string): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-timer-parent-surface-event',
    correlationId: 'app-game-timer-parent-surface-command',
    sentAt: '2026-06-07T17:55:01Z',
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityAppGameTimerParentSurfaceReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGameTimerParentSurfaceReadModel]: serializedReadModel,
    },
    snapshot: null,
  });
}

function parentPreferenceSetupRequestedEvent(): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-parent-preference-setup-requested-event',
    correlationId: 'app-game-parent-preference-setup-request-command',
    sentAt: '2026-06-08T02:18:01Z',
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGameTimerParentPreferenceSetupRequest]: JSON.stringify({
        schemaVersion: 'app-game-timer-parent-preference-setup-request-proof',
        requestId: 'request-1',
        requestedAt: '2026-06-08T02:18:00Z',
        acceptedAt: '2026-06-08T02:18:01Z',
        requestStatus: 'accepted',
        parentSurfaceIntentReferenceId: 'app-game-child-ux-parent-surface-action-result-app-game-1',
        parentPreferenceSetupReferenceId: 'app-game-child-ux-parent-preference-setup-action-result-app-game-1',
        requestReferenceIds: [
          'app-game-child-ux-local-handoff-action-result-app-game-1',
          'parent-approved',
          'child-status-limit-reached',
        ],
        actionResultReferenceId: 'app-game-parent-preference-setup-action-result::request-1',
        actionResultReferenceIds: ['app-game-parent-preference-setup-action-result::request-1'],
        actionResultPersistenceStatus: 'persisted',
        parentPreferenceMutationReceiptId: 'app-game-parent-preference-setup-mutation-receipt::request-1',
        parentPreferenceMutationReceiptIds: ['app-game-parent-preference-setup-mutation-receipt::request-1'],
        parentPreferenceMutationReceiptStatus: 'persisted',
        parentPreferenceMutationReceiptClaimed: false,
        childRuntimeDeliveryHandoffId: 'app-game-parent-preference-setup-child-runtime-handoff::request-1',
        childRuntimeDeliveryHandoffIds: ['app-game-parent-preference-setup-child-runtime-handoff::request-1'],
        childRuntimeDeliveryHandoffStatus: 'handoff-ready',
        childRuntimeDeliveryHandoffClaimed: false,
        childRuntimeDeliveryQueueId: 'app-game-parent-preference-setup-child-runtime-queue::request-1',
        childRuntimeDeliveryQueueIds: ['app-game-parent-preference-setup-child-runtime-queue::request-1'],
        childRuntimeDeliveryQueueStatus: 'queued',
        childRuntimeDeliveryQueueClaimed: true,
        commandBoundaryClaimed: true,
        actionResultHandoffClaimed: true,
        actionResultPersistenceClaimed: true,
        parentPreferenceMutationClaimed: false,
        notificationRuleMutationClaimed: false,
        providerDeliveryClaimed: false,
        providerReceiptIngestionClaimed: false,
        childRuntimeDeliveryClaimed: false,
        durableOutboxClaimed: false,
        adapterDispatchClaimed: false,
        broadBlockingClaimed: false,
        platformEnforcementClaimed: false,
        rawPrivateSourceRowsClaimed: false,
        rawTargetValuesClaimed: false,
        privateDiagnosticsClaimed: false,
      }),
    },
    snapshot: null,
  });
}

function timerParentSurfaceRow(
  rowId: string,
  input: {
    readonly targetDomain: AgentAppGameTimerParentSurfaceTargetDomain;
    readonly timerSurfaceState: AgentAppGameTimerParentSurfaceState;
    readonly evidenceReferenceIds: readonly string[];
  }
) {
  return {
    schemaVersion: AppGameSchemaVersion,
    rowId,
    targetDomain: input.targetDomain,
    timerSurfaceState: input.timerSurfaceState,
    rowCount: input.evidenceReferenceIds.length,
    evidenceReferenceIds: input.evidenceReferenceIds,
    evidence: input.evidenceReferenceIds.map((evidenceId) => ({
      evidenceId,
      kind: 'local-db-row',
      digest: null,
      uri: null,
    })),
  };
}

function rowPairs(row: {
  readonly details: ReturnType<typeof createAppGameTimerParentSurfacePanelIntent>['rows'][number]['details'];
}) {
  return row.details.map((detail) => [detail.label, detail.value]);
}
