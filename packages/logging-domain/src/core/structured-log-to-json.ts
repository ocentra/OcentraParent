export type StructuredLogToJsonResult =
  { status: 'absent' } | { status: 'failed' } | { status: 'serialized'; value: unknown };

export function serializeStructuredLogValue(
  value: object,
  serializationKey: string,
  enabled: boolean
): StructuredLogToJsonResult {
  if (!enabled) {
    return { status: 'absent' };
  }

  let toJson: unknown;
  try {
    toJson = Reflect.get(value, 'toJSON');
  } catch {
    return { status: 'failed' };
  }
  if (typeof toJson !== 'function') {
    return { status: 'absent' };
  }

  try {
    return { status: 'serialized', value: Reflect.apply(toJson, value, [serializationKey]) };
  } catch {
    return { status: 'failed' };
  }
}
