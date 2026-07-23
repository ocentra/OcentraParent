import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  AppAiHandoffState,
  AppClassificationState,
  AppObservationIntent,
  AppPolicyHandoffState,
  AppRuntimeActionState,
  AppRuntimeDecisionRecordedEventType,
  AppRuntimeDecisionSchemaVersion,
  decodeAppRuntimeDecisionRecordedEvent,
  decodeAppRuntimeDecisionRecordedEventEnvelope,
} from '../../src/app-runtime-decision';

describe('schema-domain app runtime decision edge decoder', () => {
  it('decodes every Rust-owned current decision matrix entry with matching type and version', () => {
    const contracts = appRuntimeDecisionContracts();

    expect(contracts.event_type).toBe(AppRuntimeDecisionRecordedEventType);
    expect(contracts.current_schema_version).toBe(AppRuntimeDecisionSchemaVersion);
    contracts.current_decisions.forEach(({ input, decision }, index) => {
      const envelope = {
        event_type: AppRuntimeDecisionRecordedEventType,
        schema_version: AppRuntimeDecisionSchemaVersion,
        payload: runtimeDecision(input, decision, index + 1),
      } as const;
      expect(decodeAppRuntimeDecisionRecordedEventEnvelope(envelope)).toEqual(envelope);
    });
  });

  it('accepts the Rust app-core inventory decision contract', () => {
    expect(decodeAppRuntimeDecisionRecordedEvent(inventoryDecision())).toEqual(inventoryDecision());
  });

  it('rejects display-name-only identity fields', () => {
    expect(() =>
      decodeAppRuntimeDecisionRecordedEvent({ ...inventoryDecision(), display_name: 'Chat Client' })
    ).toThrow(/opaque identifiers/i);
  });

  it('rejects runtime identifiers without the Rust-owned prefix and opaque suffix', () => {
    expect(() =>
      decodeAppRuntimeDecisionRecordedEvent({
        ...inventoryDecision(),
        aggregate_id: 'child-device-1',
      })
    ).toThrow(/invalid app runtime boundary/i);
    expect(() =>
      decodeAppRuntimeDecisionRecordedEvent({
        ...inventoryDecision(),
        aggregate_id: 'app.aggregate.Chat Client',
      })
    ).toThrow(/invalid app runtime boundary/i);
    expect(() =>
      decodeAppRuntimeDecisionRecordedEvent({
        ...inventoryDecision(),
        decision_id: 'app.runtime-decision-',
      })
    ).toThrow(/invalid app runtime boundary/i);
  });

  it('rejects inventory input that claims foreground runtime evidence', () => {
    expect(() =>
      decodeAppRuntimeDecisionRecordedEvent({
        ...inventoryDecision(),
        decision: {
          ...inventoryDecision().decision,
          runtime_action_state: AppRuntimeActionState.RecordForeground,
        },
      })
    ).toThrow(/invalid app runtime boundary/i);
  });

  it('rejects AI and manual-required decisions that publish policy', () => {
    expect(() =>
      decodeAppRuntimeDecisionRecordedEvent({
        ...unknownAppDecision(),
        decision: { ...unknownAppDecision().decision, policy_handoff_state: AppPolicyHandoffState.Publish },
      })
    ).toThrow(/invalid app runtime boundary/i);
    expect(() =>
      decodeAppRuntimeDecisionRecordedEvent({
        ...inventoryDecision(),
        input: { ...inventoryDecision().input, capability_state: 'missing' },
        decision: {
          ...inventoryDecision().decision,
          runtime_action_state: AppRuntimeActionState.ManualRequired,
          policy_handoff_state: AppPolicyHandoffState.Publish,
        },
      })
    ).toThrow(/invalid app runtime boundary/i);
  });

  it('rejects a known-policy foreground input downgraded to inventory-only', () => {
    expect(() =>
      decodeAppRuntimeDecisionRecordedEvent({
        ...knownPolicyAppDecision(),
        decision: inventoryDecision().decision,
      })
    ).toThrow(/invalid app runtime boundary/i);
  });

  it('rejects an unknown foreground app that omits its required AI handoff', () => {
    expect(() =>
      decodeAppRuntimeDecisionRecordedEvent({
        ...unknownAppDecision(),
        decision: {
          ...unknownAppDecision().decision,
          ai_handoff_state: AppAiHandoffState.NotRequired,
        },
      })
    ).toThrow(/invalid app runtime boundary/i);
  });

  it('keeps v1 replay compatibility only for the recorded mapping delta', () => {
    const legacy = appRuntimeDecisionContracts().legacy_v1_decision_deltas[0];
    if (legacy === undefined) {
      throw new TypeError('Missing Rust-owned v1 compatibility delta');
    }
    const legacyEnvelope = {
      event_type: AppRuntimeDecisionRecordedEventType,
      schema_version: 1,
      payload: runtimeDecision(legacy.input, legacy.decision, 99),
    } as const;
    expect(decodeAppRuntimeDecisionRecordedEventEnvelope(legacyEnvelope)).toEqual(legacyEnvelope);

    const current = appRuntimeDecisionContracts().current_decisions.find(
      ({ input }) =>
        input.capability_state === legacy.input.capability_state &&
        input.foreground_state === legacy.input.foreground_state &&
        input.classification_state === legacy.input.classification_state
    );
    if (current === undefined) {
      throw new TypeError('Missing current Rust-owned decision matrix entry');
    }
    expect(() =>
      decodeAppRuntimeDecisionRecordedEventEnvelope({
        event_type: AppRuntimeDecisionRecordedEventType,
        schema_version: 1,
        payload: runtimeDecision(current.input, current.decision, 100),
      })
    ).toThrow(/invalid app runtime boundary/i);
  });
});

type RuntimeInput = {
  readonly capability_state: string;
  readonly foreground_state: string;
  readonly classification_state: string;
};

type RuntimeDecision = {
  readonly observation_intent: string;
  readonly runtime_action_state: string;
  readonly ai_handoff_state: string;
  readonly policy_handoff_state: string;
};

type AppRuntimeDecisionContracts = {
  readonly event_type: string;
  readonly current_schema_version: number;
  readonly current_decisions: readonly { readonly input: RuntimeInput; readonly decision: RuntimeDecision }[];
  readonly legacy_v1_decision_deltas: readonly { readonly input: RuntimeInput; readonly decision: RuntimeDecision }[];
};

function appRuntimeDecisionContracts(): AppRuntimeDecisionContracts {
  const parsed: unknown = JSON.parse(
    readFileSync(
      new URL(
        '../../../../crates/app-core/tests/contract/fixtures/app-runtime-decision-contracts.json',
        import.meta.url
      ),
      'utf8'
    )
  );
  if (parsed === null || typeof parsed !== 'object') {
    throw new TypeError('Invalid Rust-owned app runtime decision contracts');
  }
  return parsed as AppRuntimeDecisionContracts;
}

function runtimeDecision(input: RuntimeInput, decision: RuntimeDecision, id: number) {
  return {
    aggregate_id: 'app.aggregate.child-device-1',
    decision_id: `app.runtime-decision-${id}`,
    input,
    decision,
  };
}

function inventoryDecision() {
  return {
    aggregate_id: 'app.aggregate.child-device-1',
    decision_id: 'app.runtime-decision-1',
    input: {
      capability_state: 'supported',
      foreground_state: 'background',
      classification_state: AppClassificationState.InventoryOnly,
    },
    decision: {
      observation_intent: AppObservationIntent.InventoryObservationOnly,
      runtime_action_state: AppRuntimeActionState.RecordInventory,
      ai_handoff_state: AppAiHandoffState.NotRequired,
      policy_handoff_state: AppPolicyHandoffState.DoNotPublish,
    },
  } as const;
}

function unknownAppDecision() {
  return {
    aggregate_id: 'app.aggregate.child-device-1',
    decision_id: 'app.runtime-decision-2',
    input: {
      capability_state: 'supported',
      foreground_state: 'foreground',
      classification_state: AppClassificationState.UnknownApp,
    },
    decision: {
      observation_intent: AppObservationIntent.UnknownAppRequiresAi,
      runtime_action_state: AppRuntimeActionState.RecordForeground,
      ai_handoff_state: AppAiHandoffState.Required,
      policy_handoff_state: AppPolicyHandoffState.DoNotPublish,
    },
  } as const;
}

function knownPolicyAppDecision() {
  return {
    aggregate_id: 'app.aggregate.child-device-1',
    decision_id: 'app.runtime-decision-3',
    input: {
      capability_state: 'supported',
      foreground_state: 'foreground',
      classification_state: AppClassificationState.KnownPolicyApp,
    },
    decision: {
      observation_intent: AppObservationIntent.ForegroundAppRequiresPolicy,
      runtime_action_state: AppRuntimeActionState.RecordForeground,
      ai_handoff_state: AppAiHandoffState.NotRequired,
      policy_handoff_state: AppPolicyHandoffState.Publish,
    },
  } as const;
}
