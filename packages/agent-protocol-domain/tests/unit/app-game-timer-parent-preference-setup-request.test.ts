import { expect, it } from 'vitest';
import {
  AppGameTimerParentPreferenceSetupRequestResultSchema,
  AppGameTimerParentPreferenceSetupRequestSchema,
} from '@ocentra-parent/schema-domain/app-game-timer-parent-preference-setup-request';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { parseAgentAppGameTimerParentPreferenceSetupRequestEvent } from '../../src/app-game-timer-parent-preference-setup-request';

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
  childRuntimeDeliveryReceiptRequirementId: 'parent-preference-setup-ref-1-child-runtime-delivery-receipt-required',
  childRuntimeDeliveryReceiptRequirementIds: [
    'parent-preference-setup-ref-1-child-runtime-delivery-receipt-required',
    'parent-preference-setup-ref-1-child-runtime-delivery-dispatch',
    'parent-preference-setup-ref-1-child-runtime-delivery-queue',
    'parent-preference-setup-ref-1-child-runtime-delivery-handoff',
    'parent-preference-setup-ref-1-mutation-receipt',
    'parent-preference-setup-ref-1',
    'parent-surface-intent-ref-1',
  ],
  childRuntimeDeliveryReceiptRequirementStatus: 'receipt-required',
  childRuntimeDeliveryReceiptRequirementClaimed: true,
  childRuntimeDeliveryReceiptPendingId: 'parent-preference-setup-ref-1-child-runtime-delivery-receipt-pending',
  childRuntimeDeliveryReceiptPendingIds: [
    'parent-preference-setup-ref-1-child-runtime-delivery-receipt-pending',
    'parent-preference-setup-ref-1-child-runtime-delivery-receipt-required',
    'parent-preference-setup-ref-1-child-runtime-delivery-dispatch',
    'parent-preference-setup-ref-1-child-runtime-delivery-queue',
    'parent-preference-setup-ref-1-child-runtime-delivery-handoff',
    'parent-preference-setup-ref-1-mutation-receipt',
    'parent-preference-setup-ref-1',
    'parent-surface-intent-ref-1',
  ],
  childRuntimeDeliveryReceiptPendingStatus: 'receipt-pending',
  childRuntimeDeliveryReceiptPendingClaimed: true,
  childRuntimeDeliveryReceiptIngestedId: 'parent-preference-setup-ref-1-child-runtime-delivery-receipt-ingested',
  childRuntimeDeliveryReceiptIngestedIds: [
    'parent-preference-setup-ref-1-child-runtime-delivery-receipt-ingested',
    'parent-preference-setup-ref-1-child-runtime-delivery-receipt-pending',
    'parent-preference-setup-ref-1-child-runtime-delivery-receipt-required',
    'parent-preference-setup-ref-1-child-runtime-delivery-dispatch',
    'parent-preference-setup-ref-1-child-runtime-delivery-queue',
    'parent-preference-setup-ref-1-child-runtime-delivery-handoff',
    'parent-preference-setup-ref-1-mutation-receipt',
    'parent-preference-setup-ref-1',
    'parent-surface-intent-ref-1',
  ],
  childRuntimeDeliveryReceiptIngestedStatus: 'receipt-ingested',
  childRuntimeDeliveryReceiptIngestedClaimed: true,
  durableOutboxRecordId: 'parent-preference-setup-ref-1-durable-local-outbox',
  durableOutboxRecordIds: [
    'parent-preference-setup-ref-1-durable-local-outbox',
    'parent-preference-setup-ref-1-child-runtime-delivery-receipt-ingested',
    'parent-preference-setup-ref-1-child-runtime-delivery-receipt-pending',
    'parent-preference-setup-ref-1-child-runtime-delivery-dispatch',
    'parent-preference-setup-ref-1-mutation-receipt',
    'parent-preference-setup-ref-1',
    'parent-surface-intent-ref-1',
  ],
  durableOutboxStatus: 'outbox-recorded',
  providerDeliveryReadinessId: 'parent-preference-setup-ref-1-provider-delivery-readiness',
  providerDeliveryReadinessIds: [
    'parent-preference-setup-ref-1-provider-delivery-readiness',
    'parent-preference-setup-ref-1-durable-local-outbox',
  ],
  providerDeliveryReadinessStatus: 'provider-manual-required',
  providerDeliveryAttemptId: 'parent-preference-setup-ref-1-provider-delivery-attempt',
  providerDeliveryAttemptIds: [
    'parent-preference-setup-ref-1-provider-delivery-attempt',
    'parent-preference-setup-ref-1-provider-delivery-readiness',
  ],
  providerDeliveryAttemptStatus: 'provider-delivery-manual-required',
  providerDeliveryAdapterRequirementId: 'parent-preference-setup-ref-1-provider-adapter-required',
  providerDeliveryAdapterRequirementIds: [
    'parent-preference-setup-ref-1-provider-adapter-required',
    'parent-preference-setup-ref-1-provider-delivery-attempt',
  ],
  providerDeliveryAdapterRequirementStatus: 'provider-adapter-required',
  providerDeliveryCredentialRequirementId: 'parent-preference-setup-ref-1-provider-credential-proof-required',
  providerDeliveryCredentialRequirementIds: [
    'parent-preference-setup-ref-1-provider-credential-proof-required',
    'parent-preference-setup-ref-1-provider-adapter-required',
  ],
  providerDeliveryCredentialRequirementStatus: 'provider-credential-proof-required',
  providerDeliveryQueueId: 'parent-preference-setup-ref-1-provider-delivery-local-queue',
  providerDeliveryQueueIds: [
    'parent-preference-setup-ref-1-provider-delivery-local-queue',
    'parent-preference-setup-ref-1-provider-credential-proof-required',
  ],
  providerDeliveryQueueStatus: 'provider-delivery-queued',
  providerDeliveryReceiptRequirementId: 'parent-preference-setup-ref-1-provider-delivery-receipt-required',
  providerDeliveryReceiptRequirementIds: [
    'parent-preference-setup-ref-1-provider-delivery-receipt-required',
    'parent-preference-setup-ref-1-provider-delivery-local-queue',
  ],
  providerDeliveryReceiptRequirementStatus: 'provider-delivery-receipt-required',
  providerDeliveryReceiptPendingId: 'parent-preference-setup-ref-1-provider-delivery-receipt-pending',
  providerDeliveryReceiptPendingIds: [
    'parent-preference-setup-ref-1-provider-delivery-receipt-pending',
    'parent-preference-setup-ref-1-provider-delivery-receipt-required',
  ],
  providerDeliveryReceiptPendingStatus: 'provider-delivery-receipt-pending',
  providerDeliveryReceiptIngestedId: 'parent-preference-setup-ref-1-provider-delivery-receipt-ingested',
  providerDeliveryReceiptIngestedIds: [
    'parent-preference-setup-ref-1-provider-delivery-receipt-ingested',
    'parent-preference-setup-ref-1-provider-delivery-receipt-pending',
  ],
  providerDeliveryReceiptIngestedStatus: 'provider-delivery-receipt-ingested',
  commandBoundaryClaimed: true,
  actionResultHandoffClaimed: true,
  actionResultPersistenceClaimed: true,
  parentPreferenceMutationClaimed: false,
  notificationRuleMutationClaimed: false,
  providerDeliveryReadinessClaimed: true,
  providerDeliveryAttemptClaimed: true,
  providerDeliveryAdapterRequirementClaimed: true,
  providerDeliveryCredentialRequirementClaimed: true,
  providerDeliveryQueueClaimed: true,
  providerDeliveryReceiptRequirementClaimed: true,
  providerDeliveryReceiptPendingClaimed: true,
  providerDeliveryReceiptIngestedClaimed: true,
  providerDeliveryClaimed: false,
  providerReceiptIngestionClaimed: false,
  childRuntimeDeliveryClaimed: false,
  durableOutboxClaimed: true,
  adapterDispatchClaimed: false,
  broadBlockingClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsClaimed: false,
  rawTargetValuesClaimed: false,
  privateDiagnosticsClaimed: false,
} as const;
