import { sanitizeStructuredLogValue } from './structured-log-sanitizer.js';

const SensitiveLogFieldPattern =
  /(authorization|clipboard|cookie|keystroke|password|screenshot|secret|token|url|child.?name|account.?name|full.?name|command.?line)/iu;
const RedactedLogFieldValue = '[REDACTED]';
const CircularLogFieldValue = '[CIRCULAR]';
const UnsupportedLogFieldValue = '[UNSUPPORTED_LOG_VALUE]';

export function redactStructuredLogValue(value: unknown, fieldName?: string): unknown {
  return sanitizeStructuredLogValue(value, fieldName ?? '', {
    redactedValue: RedactedLogFieldValue,
    circularValue: CircularLogFieldValue,
    unsupportedValue: UnsupportedLogFieldValue,
    isSensitiveKey: (key) => key.length > 0 && SensitiveLogFieldPattern.test(key),
  });
}
