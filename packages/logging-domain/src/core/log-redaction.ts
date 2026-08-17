const SensitiveLogFieldPattern =
  /(authorization|clipboard|cookie|keystroke|password|screenshot|secret|token|url|child.?name|account.?name|full.?name|command.?line)/iu;
const RedactedLogFieldValue = '[REDACTED]';
const CircularLogFieldValue = '[CIRCULAR]';
const UnsupportedLogFieldValue = '[UNSUPPORTED_LOG_VALUE]';

export function redactStructuredLogValue(value: unknown, fieldName?: string): unknown {
  return redactValue(value, fieldName, new WeakSet<object>());
}

function redactValue(value: unknown, fieldName: string | undefined, activeObjects: WeakSet<object>): unknown {
  if (fieldName != null && SensitiveLogFieldPattern.test(fieldName)) {
    return RedactedLogFieldValue;
  }
  if (value == null || typeof value !== 'object') {
    return value;
  }
  if (activeObjects.has(value)) {
    return CircularLogFieldValue;
  }

  activeObjects.add(value);
  try {
    const toJson = Reflect.get(value, 'toJSON');
    if (typeof toJson === 'function') {
      try {
        const serialized = Reflect.apply(toJson, value, []);
        return serialized === value ? UnsupportedLogFieldValue : redactValue(serialized, undefined, activeObjects);
      } catch {
        return UnsupportedLogFieldValue;
      }
    }

    if (!isPlainRecord(value) && !Array.isArray(value)) {
      return UnsupportedLogFieldValue;
    }

    if (Array.isArray(value)) {
      return value.map((item) => redactValue(item, undefined, activeObjects));
    }

    return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, redactValue(item, key, activeObjects)]));
  } finally {
    activeObjects.delete(value);
  }
}

function isPlainRecord(value: object): value is Record<string, unknown> {
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}
