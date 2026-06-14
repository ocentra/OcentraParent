import { describe, expect, it } from 'vitest';
import {
  PolicyControlDeliveryReadModelEventEnvelopeSchema,
  PolicyControlDeliveryReadModelPayloadField,
  PolicyControlDeliveryReadModelReportedEventName,
  PolicyControlDeliveryReadModelSchemaVersion,
  PolicyControlDeliveryReadModelSnapshotSchema,
  parseAgentPolicyControlDeliveryReadModelEvent,
} from '../../src/policy-control-delivery-read-model';

const Timestamp = '2026-06-13T21:05:00Z';

describe('policy control delivery read-model adapter', () => {
  it('parses parent-visible ack degraded manual-required and partial delivery states', () => {
    const result = parseAgentPolicyControlDeliveryReadModelEvent(eventWithSnapshot(snapshot()));

    expect(result.ok).toBe(true);
    if (!result.ok) {
      return;
    }

    expect(result.value.rows).toHaveLength(4);
    expect(result.value.acknowledgedCount).toBe(1);
    expect(result.value.degradedCount).toBe(1);
    expect(result.value.manualRequiredCount).toBe(1);
    expect(result.value.partiallyAppliedCount).toBe(1);
    expect(result.value.parentVisibleState).toBe('manual-required');
    expect(result.value.activationBlocked).toBe(true);
    expect(result.value.rows[1]?.transportState).toBe('offline');
    expect(result.value.rows[2]?.manualProofRequirements).toEqual(['child-device-admin-permission-regrant']);
    expect(result.value.rows[3]?.domainStates.map((state) => state.deliveryState)).toEqual([
      'applied',
      'degraded',
    ]);
  });

  it('rejects wrong events missing json invalid json and unsafe delivery state claims', () => {
    expect(
      parseAgentPolicyControlDeliveryReadModelEvent({
        ...eventWithSnapshot(snapshot()),
        event: 'agent.health.reported',
      }).ok
    ).toBe(false);
    expect(
      parseAgentPolicyControlDeliveryReadModelEvent({
        ...eventWithSnapshot(snapshot()),
        payload: {},
      })
    ).toEqual({
      ok: false,
      reason: 'missing-json-field',
    });
    expect(
      parseAgentPolicyControlDeliveryReadModelEvent({
        ...eventWithSnapshot(snapshot()),
        payload: {
          [PolicyControlDeliveryReadModelPayloadField]: '{',
        },
      })
    ).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentPolicyControlDeliveryReadModelEvent(
        eventWithSnapshot({
          ...snapshot(),
          rows: [
            {
              ...appliedRow(),
              ackState: 'pending',
            },
          ],
          pendingCount: 0,
          acknowledgedCount: 0,
          degradedCount: 0,
          manualRequiredCount: 0,
          appliedCount: 1,
          partiallyAppliedCount: 0,
          rejectedCount: 0,
          rolledBackCount: 0,
          supersededCount: 0,
          expiredBeforeDeliveryCount: 0,
          parentVisibleState: 'applied',
          activationBlocked: false,
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
    expect(
      PolicyControlDeliveryReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        rows: [
          {
            ...manualRequiredRow(),
            manualProofRequirements: [],
            ackState: 'pending',
            applyState: 'pending',
          },
        ],
        pendingCount: 0,
        acknowledgedCount: 0,
        degradedCount: 0,
        manualRequiredCount: 1,
        appliedCount: 0,
        partiallyAppliedCount: 0,
        rejectedCount: 0,
        rolledBackCount: 0,
        supersededCount: 0,
        expiredBeforeDeliveryCount: 0,
        parentVisibleState: 'manual-required',
        activationBlocked: true,
      }).success
    ).toBe(false);
    expect(
      PolicyControlDeliveryReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        rows: [
          {
            ...partialRow(),
            domainStates: [
              {
                ...partialRow().domainStates[0],
                deliveryState: 'applied',
              },
              {
                ...partialRow().domainStates[1],
                deliveryState: 'applied',
                lastAckEventId: 'partial-screen-ack-1',
                lastAppliedEventId: 'partial-screen-apply-1',
              },
            ],
          },
        ],
        pendingCount: 0,
        acknowledgedCount: 0,
        degradedCount: 0,
        manualRequiredCount: 0,
        appliedCount: 0,
        partiallyAppliedCount: 1,
        rejectedCount: 0,
        rolledBackCount: 0,
        supersededCount: 0,
        expiredBeforeDeliveryCount: 0,
        parentVisibleState: 'partially-applied',
        activationBlocked: true,
      }).success
    ).toBe(false);
  });
});

function eventWithSnapshot(snapshot: unknown) {
  return PolicyControlDeliveryReadModelEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'policy-control-delivery-event-1',
    correlationId: 'policy-control-delivery-command-1',
    sentAt: Timestamp,
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: PolicyControlDeliveryReadModelReportedEventName,
    severity: 'info',
    payload: {
      [PolicyControlDeliveryReadModelPayloadField]: JSON.stringify(snapshot),
    },
    snapshot: null,
  });
}

function snapshot() {
  return {
    schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
    readModelId: 'policy-control-delivery-parent-surface',
    generatedAt: Timestamp,
    rows: [acknowledgedRow(), degradedRow(), manualRequiredRow(), partialRow()],
    pendingCount: 0,
    acknowledgedCount: 1,
    degradedCount: 1,
    manualRequiredCount: 1,
    appliedCount: 0,
    partiallyAppliedCount: 1,
    rejectedCount: 0,
    rolledBackCount: 0,
    supersededCount: 0,
    expiredBeforeDeliveryCount: 0,
    parentVisibleState: 'manual-required',
    activationBlocked: true,
    nonClaims: [
      'no-global-active-policy-claim-before-child-acknowledgement',
      'no-success-ui-while-manual-proof-remains-required',
    ],
  };
}

function acknowledgedRow() {
  return {
    schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
    deliveryRowId: 'policy-delivery-acknowledged-browser-row',
    policyVersionRef: 'policy-version-2026-06-13-browser',
    childDeviceId: 'child-device-browser-1',
    generatedAt: Timestamp,
    parentVisibleState: 'acknowledged',
    intentState: 'confirmed',
    transportState: 'delivered',
    acknowledgementRequired: true,
    ackState: 'acknowledged',
    applyState: 'pending',
    blockedReason: null,
    latestAuditEventId: 'audit-policy-delivery-acknowledged-browser-row',
    auditRefs: ['audit-ref-policy-delivery-acknowledged-browser-row'],
    retryScheduleRefs: [],
    manualProofRequirements: [],
    domainStates: [
      {
        schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
        domainId: 'browser',
        deliveryState: 'acknowledged',
        auditRefs: ['audit-ref-policy-delivery-domain-browser-ack'],
        lastAckEventId: 'browser-ack-event-1',
        lastAppliedEventId: null,
      },
    ],
  };
}

function degradedRow() {
  return {
    schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
    deliveryRowId: 'policy-delivery-degraded-network-row',
    policyVersionRef: 'policy-version-2026-06-13-network',
    childDeviceId: 'child-device-network-1',
    generatedAt: Timestamp,
    parentVisibleState: 'degraded',
    intentState: 'confirmed',
    transportState: 'offline',
    acknowledgementRequired: true,
    ackState: 'pending',
    applyState: 'degraded',
    blockedReason: 'offline-child',
    latestAuditEventId: 'audit-policy-delivery-degraded-network-row',
    auditRefs: ['audit-ref-policy-delivery-degraded-network-row'],
    retryScheduleRefs: ['retry-ref-policy-delivery-degraded-network-row'],
    manualProofRequirements: [],
    domainStates: [
      {
        schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
        domainId: 'network',
        deliveryState: 'degraded',
        auditRefs: ['audit-ref-policy-delivery-domain-network-degraded'],
        lastAckEventId: null,
        lastAppliedEventId: null,
      },
    ],
  };
}

function manualRequiredRow() {
  return {
    schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
    deliveryRowId: 'policy-delivery-manual-required-screen-row',
    policyVersionRef: 'policy-version-2026-06-13-screen',
    childDeviceId: 'child-device-screen-1',
    generatedAt: Timestamp,
    parentVisibleState: 'manual-required',
    intentState: 'confirmed',
    transportState: 'permission-blocked',
    acknowledgementRequired: true,
    ackState: 'manual-required',
    applyState: 'manual-required',
    blockedReason: 'permission-loss',
    latestAuditEventId: 'audit-policy-delivery-manual-required-screen-row',
    auditRefs: ['audit-ref-policy-delivery-manual-required-screen-row'],
    retryScheduleRefs: [],
    manualProofRequirements: ['child-device-admin-permission-regrant'],
    domainStates: [
      {
        schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
        domainId: 'screen',
        deliveryState: 'manual-required',
        auditRefs: ['audit-ref-policy-delivery-domain-screen-manual-required'],
        lastAckEventId: null,
        lastAppliedEventId: null,
      },
    ],
  };
}

function partialRow() {
  return {
    schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
    deliveryRowId: 'policy-delivery-partial-tracking-row',
    policyVersionRef: 'policy-version-2026-06-13-tracking',
    childDeviceId: 'child-device-tracking-1',
    generatedAt: Timestamp,
    parentVisibleState: 'partially-applied',
    intentState: 'confirmed',
    transportState: 'delivered',
    acknowledgementRequired: true,
    ackState: 'acknowledged',
    applyState: 'partially-applied',
    blockedReason: null,
    latestAuditEventId: 'audit-policy-delivery-partial-tracking-row',
    auditRefs: ['audit-ref-policy-delivery-partial-tracking-row'],
    retryScheduleRefs: ['retry-ref-policy-delivery-partial-tracking-row'],
    manualProofRequirements: [],
    domainStates: [
      {
        schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
        domainId: 'tracking',
        deliveryState: 'applied',
        auditRefs: ['audit-ref-policy-delivery-domain-tracking-applied'],
        lastAckEventId: 'tracking-ack-event-1',
        lastAppliedEventId: 'tracking-apply-event-1',
      },
      {
        schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
        domainId: 'screen',
        deliveryState: 'degraded',
        auditRefs: ['audit-ref-policy-delivery-domain-screen-degraded'],
        lastAckEventId: null,
        lastAppliedEventId: null,
      },
    ],
  };
}

function appliedRow() {
  return {
    schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
    deliveryRowId: 'policy-delivery-applied-browser-row',
    policyVersionRef: 'policy-version-2026-06-13-browser-applied',
    childDeviceId: 'child-device-browser-2',
    generatedAt: Timestamp,
    parentVisibleState: 'applied',
    intentState: 'confirmed',
    transportState: 'delivered',
    acknowledgementRequired: true,
    ackState: 'acknowledged',
    applyState: 'applied',
    blockedReason: null,
    latestAuditEventId: 'audit-policy-delivery-applied-browser-row',
    auditRefs: ['audit-ref-policy-delivery-applied-browser-row'],
    retryScheduleRefs: [],
    manualProofRequirements: [],
    domainStates: [
      {
        schemaVersion: PolicyControlDeliveryReadModelSchemaVersion,
        domainId: 'browser',
        deliveryState: 'applied',
        auditRefs: ['audit-ref-policy-delivery-domain-browser-applied'],
        lastAckEventId: 'browser-ack-applied-event-1',
        lastAppliedEventId: 'browser-apply-event-1',
      },
    ],
  };
}
