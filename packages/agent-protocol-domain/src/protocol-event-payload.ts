import {
  isAgentProtocolLogText,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';

export type JsonPayloadEventFailureReason = 'wrong-event' | 'missing-json-field' | 'invalid-json' | 'invalid-payload';

export type JsonPayloadEventResult<TValue> =
  | {
      readonly ok: true;
      readonly value: TValue;
    }
  | {
      readonly ok: false;
      readonly reason: JsonPayloadEventFailureReason;
    };

type SafeParseSchema<TValue> = {
  readonly safeParse: (input: unknown) => { readonly success: boolean; readonly data?: TValue };
};

type ProtocolFieldTransform = (value: unknown) => unknown;
type ProtocolFieldMapping<TTargetKey extends string> = readonly [
  targetKey: TTargetKey,
  sourceField: string,
  transform?: ProtocolFieldTransform,
];

type EventPayloadInput = {
  readonly event: string;
  readonly payload: Record<string, unknown>;
};

export function parseJsonPayloadFieldEvent<TValue>(
  event: EventPayloadInput,
  expectedEvent: string,
  field: string,
  schema: SafeParseSchema<TValue>
): JsonPayloadEventResult<TValue> {
  if (event.event !== expectedEvent) {
    return payloadFailure('wrong-event');
  }

  return parseJsonPayloadField(event.payload, field, schema);
}

export function parseJsonPayloadField<TValue>(
  payload: Record<string, unknown>,
  field: string,
  schema: SafeParseSchema<TValue>
): JsonPayloadEventResult<TValue> {
  const decoded = parseJsonPayloadFieldValue(payload, field);
  if (!decoded.ok) {
    return decoded;
  }

  const parsed = schema.safeParse(decoded.value);
  if (!parsed.success || parsed.data === undefined) {
    return payloadFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

export function parseJsonPayloadFieldObjectEvent(
  event: EventPayloadInput,
  expectedEvent: string,
  field: string
): JsonPayloadEventResult<Record<string, unknown>> {
  if (event.event !== expectedEvent) {
    return payloadFailure('wrong-event');
  }

  return parseJsonPayloadFieldObject(event.payload, field);
}

export function parseJsonPayloadFieldObject(
  payload: Record<string, unknown>,
  field: string
): JsonPayloadEventResult<Record<string, unknown>> {
  const decoded = parseJsonPayloadFieldValue(payload, field);
  if (!decoded.ok) {
    return decoded;
  }

  if (decoded.value === null || typeof decoded.value !== 'object' || Array.isArray(decoded.value)) {
    return payloadFailure('invalid-payload');
  }

  return {
    ok: true,
    value: decoded.value as Record<string, unknown>,
  };
}

export function parseJsonPayloadFieldArray(
  payload: Record<string, unknown>,
  field: string
): JsonPayloadEventResult<readonly unknown[]> {
  const decoded = parseJsonPayloadFieldValue(payload, field);
  if (!decoded.ok) {
    return decoded;
  }

  if (!Array.isArray(decoded.value)) {
    return payloadFailure('invalid-payload');
  }

  return {
    ok: true,
    value: decoded.value,
  };
}

export function parseJsonStringArrayField(payload: Record<string, unknown>, field: string): readonly string[] | null {
  const decoded = parseJsonPayloadFieldArray(payload, field);
  if (!decoded.ok) {
    return null;
  }

  if (decoded.value.some((entry) => typeof entry !== 'string' || entry.length === 0)) {
    return null;
  }

  return decoded.value as readonly string[];
}

export function mapProtocolFields<TTargetKey extends string>(
  payload: Record<string, unknown>,
  mappings: readonly ProtocolFieldMapping<TTargetKey>[]
): Record<TTargetKey, unknown> {
  const result = {} as Record<TTargetKey, unknown>;
  for (const [targetKey, sourceField, transform] of mappings) {
    const value = payload[sourceField];
    result[targetKey] = transform === undefined ? value : transform(value);
  }
  return result;
}

export function undefinedToNull(value: unknown): unknown {
  return value === undefined ? null : value;
}

export function splitDelimitedStringField(value: unknown, delimiter: string): readonly string[] {
  if (!isAgentProtocolLogText(value)) {
    return [];
  }

  return value.split(delimiter).filter((entry) => entry.length > 0);
}

export function mapJsonPayloadEventToStatus<TValue, TReason extends string>(
  parsed: JsonPayloadEventResult<TValue>,
  failureReasons: {
    readonly 'missing-json-field': TReason;
    readonly 'invalid-json': TReason;
    readonly 'invalid-payload': TReason;
  }
): { readonly ok: true; readonly status: TValue } | { readonly ok: false; readonly reason: 'wrong-event' | TReason } {
  if (!parsed.ok) {
    if (parsed.reason === 'wrong-event') {
      return {
        ok: false,
        reason: 'wrong-event',
      };
    }

    return {
      ok: false,
      reason: failureReasons[parsed.reason],
    };
  }

  return {
    ok: true,
    status: parsed.value,
  };
}

function parseJsonPayloadFieldValue(
  payload: Record<string, unknown>,
  field: string
): JsonPayloadEventResult<unknown> {
  const raw = payload[field];
  if (!isAgentProtocolLogText(raw)) {
    return payloadFailure('missing-json-field');
  }

  try {
    return {
      ok: true,
      value: JSON.parse(raw) as unknown,
    };
  } catch {
    return payloadFailure('invalid-json');
  }
}

function payloadFailure(reason: JsonPayloadEventFailureReason): JsonPayloadEventResult<never> {
  return {
    ok: false,
    reason,
  };
}
