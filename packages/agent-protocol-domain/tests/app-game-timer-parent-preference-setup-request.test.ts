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
  const result = AppGameTimerParentPreferenceSetupRequestResultSchema.parse({
    schemaVersion: 'app-game-timer-parent-preference-setup-request-proof',
    requestId: 'app-game-parent-preference-request-1',
    requestedAt: '2026-06-08T00:20:00Z',
    acceptedAt: '2026-06-08T00:20:01Z',
    requestStatus: 'accepted',
    parentSurfaceIntentReferenceId: 'parent-surface-intent-ref-1',
    parentPreferenceSetupReferenceId: 'parent-preference-setup-ref-1',
    requestReferenceIds: ['parent-surface-intent-ref-1', 'parent-preference-setup-ref-1'],
    commandBoundaryClaimed: true,
    parentPreferenceMutationClaimed: false,
    notificationRuleMutationClaimed: false,
    providerDeliveryClaimed: false,
    durableOutboxClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
  });
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
