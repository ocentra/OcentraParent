import { expect, it } from 'vitest';
import {
  AgentEvent,
  AgentProtocolDefaults,
  AppGameTimerParentPreferenceSetupRequestResultSchema,
  AppGameTimerParentPreferenceSetupRequestSchema,
  parseAgentAppGameTimerParentPreferenceSetupRequestEvent,
  type AgentEventEnvelope,
} from '../src/contracts';

it('AppGameTimerParentPreferenceSetupRequestSchema: accepts parent-safe request references', () => {
  const parsed = AppGameTimerParentPreferenceSetupRequestSchema.safeParse({
    requestId: 'app-game-parent-preference-request-1',
    requestedAt: '2026-06-08T00:20:00Z',
    parentSurfaceIntentReferenceId: 'parent-surface-intent-ref-1',
    parentPreferenceSetupReferenceId: 'parent-preference-setup-ref-1',
    requestReferenceIds: ['parent-surface-intent-ref-1', 'parent-preference-setup-ref-1'],
  });

  expect(parsed.success).toBe(true);
});

it('parseAgentAppGameTimerParentPreferenceSetupRequestEvent: keeps mutation and delivery unclaimed', () => {
  const result = AppGameTimerParentPreferenceSetupRequestResultSchema.parse(setupRequestResultFixture);
  const event = {
    event: AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested,
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGameTimerParentPreferenceSetupRequest]: JSON.stringify(result),
    },
  } as AgentEventEnvelope;

  const parsed = parseAgentAppGameTimerParentPreferenceSetupRequestEvent(event);

  expect(parsed).toEqual({
    ok: true,
    value: result,
  });
});

const setupRequestResultFixture = {
  schemaVersion: 'app-game-timer-parent-preference-setup-request-proof',
  requestId: 'app-game-parent-preference-request-1',
  requestedAt: '2026-06-08T00:20:00Z',
  acceptedAt: '2026-06-08T00:20:01Z',
  requestStatus: 'accepted',
  parentSurfaceIntentReferenceId: 'parent-surface-intent-ref-1',
  parentPreferenceSetupReferenceId: 'parent-preference-setup-ref-1',
  requestReferenceIds: ['parent-surface-intent-ref-1', 'parent-preference-setup-ref-1'],
  actionResultReferenceId: 'parent-preference-setup-ref-1',
  actionResultReferenceIds: ['parent-preference-setup-ref-1', 'parent-surface-intent-ref-1'],
  actionResultPersistenceStatus: 'persisted',
  parentPreferenceMutationReceiptId: 'parent-preference-setup-ref-1-mutation-receipt',
  parentPreferenceMutationReceiptIds: [
    'parent-preference-setup-ref-1-mutation-receipt',
    'parent-preference-setup-ref-1',
    'parent-surface-intent-ref-1',
  ],
  parentPreferenceMutationReceiptStatus: 'persisted',
  parentPreferenceMutationReceiptClaimed: true,
  childRuntimeDeliveryHandoffId: 'parent-preference-setup-ref-1-child-runtime-delivery-handoff',
  childRuntimeDeliveryHandoffIds: [
    'parent-preference-setup-ref-1-child-runtime-delivery-handoff',
    'parent-preference-setup-ref-1-mutation-receipt',
    'parent-preference-setup-ref-1',
    'parent-surface-intent-ref-1',
  ],
  childRuntimeDeliveryHandoffStatus: 'handoff-ready',
  childRuntimeDeliveryHandoffClaimed: true,
  childRuntimeDeliveryQueueId: 'parent-preference-setup-ref-1-child-runtime-delivery-queue',
  childRuntimeDeliveryQueueIds: [
    'parent-preference-setup-ref-1-child-runtime-delivery-queue',
    'parent-preference-setup-ref-1-child-runtime-delivery-handoff',
    'parent-preference-setup-ref-1-mutation-receipt',
    'parent-preference-setup-ref-1',
    'parent-surface-intent-ref-1',
  ],
  childRuntimeDeliveryQueueStatus: 'queued',
  childRuntimeDeliveryQueueClaimed: true,
  childRuntimeDeliveryDispatchId: 'parent-preference-setup-ref-1-child-runtime-delivery-dispatch',
  childRuntimeDeliveryDispatchIds: [
    'parent-preference-setup-ref-1-child-runtime-delivery-dispatch',
    'parent-preference-setup-ref-1-child-runtime-delivery-queue',
    'parent-preference-setup-ref-1-child-runtime-delivery-handoff',
    'parent-preference-setup-ref-1-mutation-receipt',
    'parent-preference-setup-ref-1',
    'parent-surface-intent-ref-1',
  ],
  childRuntimeDeliveryDispatchStatus: 'dispatch-ready',
  childRuntimeDeliveryDispatchClaimed: true,
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
} as const;
