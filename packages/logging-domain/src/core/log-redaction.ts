const SensitiveLogFieldPattern =
  /(authorization|clipboard|cookie|keystroke|password|screenshot|secret|token|url|child.?name|account.?name|full.?name|command.?line)/iu;
const RedactedLogFieldValue = '[REDACTED]';

export function redactStructuredLogValue(value: unknown, fieldName?: string): unknown {
  if (fieldName != null && SensitiveLogFieldPattern.test(fieldName)) {
    return RedactedLogFieldValue;
  }
  if (Array.isArray(value)) {
    return value.map((item) => redactStructuredLogValue(item));
  }
  if (value != null && typeof value === 'object') {
    return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, redactStructuredLogValue(item, key)]));
  }
  return value;
}
