import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

export const AppRuntimeDecisionSchemaVersion = 1;
export const AppRuntimeDecisionRecordedEventType = 'app.runtime.decision-recorded' as const;

export const AppCapabilityState = {
  Supported: 'supported',
  Missing: 'missing',
} as const;

export const AppForegroundState = {
  Foreground: 'foreground',
  Background: 'background',
  Unknown: 'unknown',
} as const;

export const AppClassificationState = {
  KnownPolicyApp: 'known-policy-app',
  UnknownApp: 'unknown-app',
  InventoryOnly: 'inventory-only',
} as const;

export const AppRuntimeActionState = {
  RecordForeground: 'record-foreground',
  RecordInventory: 'record-inventory',
  ManualRequired: 'manual-required',
} as const;

export const AppAiHandoffState = {
  Required: 'required',
  NotRequired: 'not-required',
} as const;

export const AppPolicyHandoffState = {
  Publish: 'publish',
  DoNotPublish: 'do-not-publish',
} as const;

export const AppObservationIntent = {
  ForegroundAppRequiresPolicy: 'ForegroundAppRequiresPolicy',
  UnknownAppRequiresAi: 'UnknownAppRequiresAi',
  InventoryObservationOnly: 'InventoryObservationOnly',
} as const;

const AppRuntimeInputSchema = Schema.Struct({
  capability_state: Schema.Literal(...Object.values(AppCapabilityState)),
  foreground_state: Schema.Literal(...Object.values(AppForegroundState)),
  classification_state: Schema.Literal(...Object.values(AppClassificationState)),
});

const AppRuntimeDecisionSchema = Schema.Struct({
  observation_intent: Schema.Literal(...Object.values(AppObservationIntent)),
  runtime_action_state: Schema.Literal(...Object.values(AppRuntimeActionState)),
  ai_handoff_state: Schema.Literal(...Object.values(AppAiHandoffState)),
  policy_handoff_state: Schema.Literal(...Object.values(AppPolicyHandoffState)),
});

const AppRuntimeDecisionRecordedEventBaseSchema = Schema.Struct({
  aggregate_id: NonEmptyStringSchema,
  decision_id: NonEmptyStringSchema,
  input: AppRuntimeInputSchema,
  decision: AppRuntimeDecisionSchema,
});

type AppRuntimeDecisionRecordedEvent = Infer<typeof AppRuntimeDecisionRecordedEventBaseSchema>;

export const AppRuntimeDecisionRecordedEventSchema = withParser(
  AppRuntimeDecisionRecordedEventBaseSchema.pipe(
    Schema.filter((event) => appRuntimeDecisionHasSafeBoundary(event) || 'Invalid app runtime boundary')
  )
);

export function decodeAppRuntimeDecisionRecordedEvent(input: unknown): AppRuntimeDecisionRecordedEvent {
  assertExactKeys(input, ['aggregate_id', 'decision_id', 'input', 'decision']);
  const event = AppRuntimeDecisionRecordedEventSchema.parse(input);
  return event;
}

function appRuntimeDecisionHasSafeBoundary(event: AppRuntimeDecisionRecordedEvent): boolean {
  if (!hasOpaqueIdSuffix(event.aggregate_id, 'app.aggregate.')) {
    return false;
  }
  if (!hasOpaqueIdSuffix(event.decision_id, 'app.runtime-decision-')) {
    return false;
  }
  const expected = deriveRustRuntimeDecision(event.input);
  return (
    event.decision.observation_intent === expected.observation_intent &&
    event.decision.runtime_action_state === expected.runtime_action_state &&
    event.decision.ai_handoff_state === expected.ai_handoff_state &&
    event.decision.policy_handoff_state === expected.policy_handoff_state
  );
}

function deriveRustRuntimeDecision(input: Infer<typeof AppRuntimeInputSchema>): Infer<typeof AppRuntimeDecisionSchema> {
  if (input.capability_state === AppCapabilityState.Missing) {
    return inventoryDecision(AppRuntimeActionState.ManualRequired);
  }
  if (input.foreground_state !== AppForegroundState.Foreground) {
    return inventoryDecision(AppRuntimeActionState.RecordInventory);
  }
  if (input.classification_state === AppClassificationState.KnownPolicyApp) {
    return {
      observation_intent: AppObservationIntent.ForegroundAppRequiresPolicy,
      runtime_action_state: AppRuntimeActionState.RecordForeground,
      ai_handoff_state: AppAiHandoffState.NotRequired,
      policy_handoff_state: AppPolicyHandoffState.Publish,
    };
  }
  if (input.classification_state === AppClassificationState.UnknownApp) {
    return {
      observation_intent: AppObservationIntent.UnknownAppRequiresAi,
      runtime_action_state: AppRuntimeActionState.RecordForeground,
      ai_handoff_state: AppAiHandoffState.Required,
      policy_handoff_state: AppPolicyHandoffState.DoNotPublish,
    };
  }
  return inventoryDecision(AppRuntimeActionState.RecordInventory);
}

function inventoryDecision(
  runtime_action_state: Infer<typeof AppRuntimeDecisionSchema>['runtime_action_state']
): Infer<typeof AppRuntimeDecisionSchema> {
  return {
    observation_intent: AppObservationIntent.InventoryObservationOnly,
    runtime_action_state,
    ai_handoff_state: AppAiHandoffState.NotRequired,
    policy_handoff_state: AppPolicyHandoffState.DoNotPublish,
  };
}

function hasOpaqueIdSuffix(value: string, prefix: string): boolean {
  return value.startsWith(prefix) && value.slice(prefix.length).trim().length > 0;
}

function assertExactKeys(input: unknown, expectedKeys: readonly string[]): asserts input is Record<string, unknown> {
  if (input === null || typeof input !== 'object' || Array.isArray(input)) {
    throw new TypeError('Expected an app runtime decision event object');
  }
  const keys = Object.keys(input);
  if (keys.length !== expectedKeys.length || keys.some((key) => !expectedKeys.includes(key))) {
    throw new TypeError('App runtime decision events accept opaque identifiers only');
  }
}
