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
} from '../../src/app-runtime-decision';

describe('schema-domain app runtime decision edge decoder', () => {
  it('decodes the Rust-serialized event golden with matching type and version', () => {
    const golden = appRuntimeDecisionGolden();

    expect(golden.event_type).toBe(AppRuntimeDecisionRecordedEventType);
    expect(golden.schema_version).toBe(AppRuntimeDecisionSchemaVersion);
    expect(decodeAppRuntimeDecisionRecordedEvent(golden.payload)).toEqual(golden.payload);
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
});

function appRuntimeDecisionGolden(): {
  readonly event_type: string;
  readonly schema_version: number;
  readonly payload: unknown;
} {
  const parsed: unknown = JSON.parse(
    readFileSync(new URL('../fixtures/app-runtime-decision-recorded-event.json', import.meta.url), 'utf8')
  );
  if (
    parsed === null ||
    typeof parsed !== 'object' ||
    !('event_type' in parsed) ||
    !('schema_version' in parsed) ||
    !('payload' in parsed) ||
    typeof parsed.event_type !== 'string' ||
    typeof parsed.schema_version !== 'number'
  ) {
    throw new TypeError('Invalid Rust app runtime decision golden');
  }
  return parsed;
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
