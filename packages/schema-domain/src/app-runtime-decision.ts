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
  if (event.input.classification_state === AppClassificationState.InventoryOnly) {
    return (
      event.decision.observation_intent === AppObservationIntent.InventoryObservationOnly &&
      event.decision.runtime_action_state === AppRuntimeActionState.RecordInventory &&
      event.decision.ai_handoff_state === AppAiHandoffState.NotRequired &&
      event.decision.policy_handoff_state === AppPolicyHandoffState.DoNotPublish
    );
  }
  if (event.decision.runtime_action_state === AppRuntimeActionState.ManualRequired) {
    return (
      event.decision.observation_intent === AppObservationIntent.InventoryObservationOnly &&
      event.decision.ai_handoff_state === AppAiHandoffState.NotRequired &&
      event.decision.policy_handoff_state === AppPolicyHandoffState.DoNotPublish
    );
  }
  if (event.decision.ai_handoff_state === AppAiHandoffState.Required) {
    return event.decision.policy_handoff_state === AppPolicyHandoffState.DoNotPublish;
  }
  return true;
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
