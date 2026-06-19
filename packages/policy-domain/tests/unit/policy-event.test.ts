import { describe, expect, it } from 'vitest';
import { EventingEventPriority } from '@ocentra-parent/schema-domain/eventing';
import { type PolicyAuditReferenceId } from '../../src/authority';
import {
  PolicyEventDeadLetterReason,
  PolicyEventFamilyNamespace,
  PolicyEventKind,
  PolicyEventSchema,
  PolicyEventScope,
  applyPolicyEventReplay,
  parsePolicyEvent,
  parsePolicyEventEnvelope,
  policyEventAggregateKey,
  policyEventContractForKind,
  policyEventFamilyRegistry,
  policyEventFamilyVariants,
  policyEventIdempotencyKey,
  policyEventRedactedSummary,
  policyEventSchemaVersion,
  type PolicyEvent,
} from '../../src/policy-event';
import { type PolicyReasonCode } from '../../src/policy';

function sampleEvent(overrides: Partial<PolicyEvent> = {}): PolicyEvent {
  return parsePolicyEvent({
    schemaVersion: policyEventSchemaVersion(),
    kind: PolicyEventKind.DeliverySent,
    sequence: 7,
    scope: PolicyEventScope.Delivery,
    auditReferenceIds: ['audit-delivery-1' as PolicyAuditReferenceId],
    reasonCode: null,
    deadLetterReason: null,
    ...overrides,
  });
}

function sampleEnvelope(payload: PolicyEvent) {
  return parsePolicyEventEnvelope({
    contract: policyEventContractForKind(payload.kind),
    metadata: {
      eventId: 'policy-event-1',
      correlationId: 'correlation-policy-event-1',
      causationId: 'causation-policy-event-1',
      aggregateKey: policyEventAggregateKey(payload),
      idempotencyKey: policyEventIdempotencyKey(payload),
      source: {
        custody: 'local',
        role: 'policy-control-plane',
        service: 'policy-control-plane',
        component: 'policy-event-test',
        instanceId: 'instance-1',
      },
      observedAt: '2026-06-13T20:05:00.000Z',
      targetHandler: 'policy-event-handler',
      priority: EventingEventPriority.Normal,
      deadline: null,
    },
    payload,
  });
}

describe('policy event contracts', () => {
  it('policyEventFamilyRegistry: keeps the registry and variants aligned with the explicit policy event kinds', () => {
    const expectedKinds = Object.values(PolicyEventKind) as PolicyEvent['kind'][];

    expect(policyEventFamilyRegistry()).toEqual(
      expectedKinds.map((kind) => policyEventContractForKind(kind))
    );
    expect(policyEventFamilyVariants()).toEqual(
      expectedKinds.map((kind) => ({
        family: PolicyEventFamilyNamespace.Policy,
        eventType: kind,
      }))
    );
  });

  it('policyEventAggregateKey: keeps delivery keys stable and redacted summaries free of private identifiers', () => {
    const event = sampleEvent();

    expect(policyEventAggregateKey(event)).toBe(
      'policy-delivery:household-default:policy-delivery-default:child-primary:device-laptop:tracking:5'
    );
    expect(policyEventIdempotencyKey(event)).toBe(
      'policy-event|policy.delivery.sent|policy-delivery:household-default:policy-delivery-default:child-primary:device-laptop:tracking:5|7|delivery|audit-delivery-1|none|none'
    );

    const summary = policyEventRedactedSummary(event);
    expect(summary).toBe('policy-event kind=policy.delivery.sent scope=delivery sequence=7');
    expect(summary).not.toContain('device-laptop');
    expect(summary).not.toContain('policy-delivery-default');
  });

  it('parsePolicyEventEnvelope: keeps causation, correlation, and deterministic metadata aligned', () => {
    const payload = sampleEvent();
    const envelope = sampleEnvelope(payload);

    expect(envelope.contract).toEqual(policyEventContractForKind(payload.kind));
    expect(envelope.metadata.aggregateKey).toBe(policyEventAggregateKey(payload));
    expect(envelope.metadata.idempotencyKey).toBe(policyEventIdempotencyKey(payload));
    expect(envelope.metadata.correlationId).toBe('correlation-policy-event-1');
    expect(envelope.metadata.causationId).toBe('causation-policy-event-1');
  });

  it('applyPolicyEventReplay: accepts duplicate and stale replays but rejects conflicting same-sequence writes', () => {
    const currentEvent = sampleEvent();
    const current = {
      aggregateKey: policyEventAggregateKey(currentEvent),
      lastSequence: currentEvent.sequence,
      lastEventType: currentEvent.kind,
      lastIdempotencyKey: policyEventIdempotencyKey(currentEvent),
    };

    expect(applyPolicyEventReplay(current, sampleEvent()).state).toBe('duplicate');
    expect(applyPolicyEventReplay(current, sampleEvent({ sequence: currentEvent.sequence - 1 })).state).toBe(
      'stale'
    );
    expect(() =>
      applyPolicyEventReplay(current, sampleEvent({ kind: PolicyEventKind.DeliveryAcknowledged }))
    ).toThrow('conflicting replay for sequence 7 on policy.delivery.sent');
  });

  it('PolicyEventSchema: keeps rollback linkage and dead-letter/manual-required visibility explicit', () => {
    const rollback = sampleEvent({
      kind: PolicyEventKind.RollbackApplied,
      scope: PolicyEventScope.Rollback,
      reasonCode: 'rollback-applied' as PolicyReasonCode,
      deadLetterReason: null,
    });

    const rollbackScope = rollback.scope;
    if (rollbackScope.scope !== PolicyEventScope.Rollback.scope) {
      throw new Error('expected rollback scope');
    }

    expect(rollbackScope.rollbackRef.rolledBackPolicyVersion).toBe('5');
    expect(rollbackScope.rollbackRef.restoredPolicyVersion).toBe('4');
    expect(policyEventAggregateKey(rollback)).toBe(
      'policy-rollback:household-default:policy-source-default:5:policy-source-previous:4'
    );

    const manualRequired = sampleEvent({
      kind: PolicyEventKind.ManualRequired,
      scope: PolicyEventScope.SourceDocument,
      reasonCode: 'manual-required' as PolicyReasonCode,
      deadLetterReason: null,
      auditReferenceIds: ['audit-manual-1' as PolicyAuditReferenceId],
    });
    expect(policyEventRedactedSummary(manualRequired)).toContain('manual-required');

    const deadLetter = sampleEvent({
      kind: PolicyEventKind.DeadLetterRecorded,
      scope: PolicyEventScope.SourceDocument,
      reasonCode: null,
      deadLetterReason: PolicyEventDeadLetterReason.UnsupportedTarget,
      auditReferenceIds: ['audit-dead-letter-1' as PolicyAuditReferenceId],
    });
    expect(policyEventRedactedSummary(deadLetter)).toContain('dead-lettered');
    expect(PolicyEventSchema.safeParse({ ...deadLetter, deadLetterReason: null }).success).toBe(false);
  });
});
