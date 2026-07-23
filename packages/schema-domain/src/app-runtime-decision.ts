import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { RustOwnedAppRuntimeDecisionContracts } from './app-runtime-decision-contract-data';

export const AppRuntimeDecisionSchemaVersion = RustOwnedAppRuntimeDecisionContracts.currentSchemaVersion;
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

const AppRuntimeDecisionRecordedEventEnvelopeBaseSchema = Schema.Struct({
  event_type: Schema.Literal(AppRuntimeDecisionRecordedEventType),
  schema_version: Schema.Literal(1, AppRuntimeDecisionSchemaVersion),
  payload: AppRuntimeDecisionRecordedEventBaseSchema,
});

type AppRuntimeDecisionRecordedEventEnvelope = Infer<typeof AppRuntimeDecisionRecordedEventEnvelopeBaseSchema>;

export const AppRuntimeDecisionRecordedEventSchema = withParser(
  AppRuntimeDecisionRecordedEventBaseSchema.pipe(
    Schema.filter(
      (event) =>
        appRuntimeDecisionHasSafeBoundary(event, AppRuntimeDecisionSchemaVersion) || 'Invalid app runtime boundary'
    )
  )
);

export const AppRuntimeDecisionRecordedEventEnvelopeSchema = withParser(
  AppRuntimeDecisionRecordedEventEnvelopeBaseSchema.pipe(
    Schema.filter(
      (envelope) =>
        appRuntimeDecisionHasSafeBoundary(envelope.payload, envelope.schema_version) || 'Invalid app runtime boundary'
    )
  )
);

export function decodeAppRuntimeDecisionRecordedEvent(input: unknown): AppRuntimeDecisionRecordedEvent {
  assertExactKeys(input, ['aggregate_id', 'decision_id', 'input', 'decision']);
  const event = AppRuntimeDecisionRecordedEventSchema.parse(input);
  return event;
}

export function decodeAppRuntimeDecisionRecordedEventEnvelope(input: unknown): AppRuntimeDecisionRecordedEventEnvelope {
  assertExactKeys(input, ['event_type', 'schema_version', 'payload']);
  return AppRuntimeDecisionRecordedEventEnvelopeSchema.parse(input);
}

function appRuntimeDecisionHasSafeBoundary(event: AppRuntimeDecisionRecordedEvent, schemaVersion: number): boolean {
  if (!hasOpaqueIdSuffix(event.aggregate_id, 'app.aggregate.')) {
    return false;
  }
  if (!hasOpaqueIdSuffix(event.decision_id, 'app.runtime-decision-')) {
    return false;
  }
  const expected = expectedRustRuntimeDecision(event.input, schemaVersion);
  if (expected === undefined) {
    return false;
  }
  return (
    event.decision.observation_intent === expected.observation_intent &&
    event.decision.runtime_action_state === expected.runtime_action_state &&
    event.decision.ai_handoff_state === expected.ai_handoff_state &&
    event.decision.policy_handoff_state === expected.policy_handoff_state
  );
}

function expectedRustRuntimeDecision(
  input: Infer<typeof AppRuntimeInputSchema>,
  schemaVersion: number
): Infer<typeof AppRuntimeDecisionSchema> | undefined {
  const decisions =
    schemaVersion === 1
      ? [
          ...RustOwnedAppRuntimeDecisionContracts.currentDecisions,
          ...RustOwnedAppRuntimeDecisionContracts.legacyV1DecisionDeltas,
        ]
      : RustOwnedAppRuntimeDecisionContracts.currentDecisions;
  const matching = decisions.find(
    ([capabilityState, foregroundState, classificationState]) =>
      input.capability_state === capabilityState &&
      input.foreground_state === foregroundState &&
      input.classification_state === classificationState
  );
  if (matching === undefined) {
    return undefined;
  }
  const [, , , observation_intent, runtime_action_state, ai_handoff_state, policy_handoff_state] = matching;
  if (schemaVersion === 1) {
    const legacy = RustOwnedAppRuntimeDecisionContracts.legacyV1DecisionDeltas.find(
      ([capabilityState, foregroundState, classificationState]) =>
        input.capability_state === capabilityState &&
        input.foreground_state === foregroundState &&
        input.classification_state === classificationState
    );
    if (legacy !== undefined) {
      const [, , , legacyIntent, legacyAction, legacyAiHandoff, legacyPolicyHandoff] = legacy;
      return decisionFromContract(legacyIntent, legacyAction, legacyAiHandoff, legacyPolicyHandoff);
    }
  }
  return decisionFromContract(observation_intent, runtime_action_state, ai_handoff_state, policy_handoff_state);
}

function decisionFromContract(
  observation_intent: string,
  runtime_action_state: string,
  ai_handoff_state: string,
  policy_handoff_state: string
): Infer<typeof AppRuntimeDecisionSchema> {
  return {
    observation_intent: observation_intent as Infer<typeof AppRuntimeDecisionSchema>['observation_intent'],
    runtime_action_state: runtime_action_state as Infer<typeof AppRuntimeDecisionSchema>['runtime_action_state'],
    ai_handoff_state: ai_handoff_state as Infer<typeof AppRuntimeDecisionSchema>['ai_handoff_state'],
    policy_handoff_state: policy_handoff_state as Infer<typeof AppRuntimeDecisionSchema>['policy_handoff_state'],
  };
}

function hasOpaqueIdSuffix(value: string, prefix: string): boolean {
  return value.startsWith(prefix) && /^[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$/.test(value.slice(prefix.length));
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
