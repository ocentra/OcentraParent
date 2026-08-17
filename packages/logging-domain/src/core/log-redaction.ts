import { sanitizeStructuredLogValue } from './structured-log-sanitizer.js';
import { GeneratedSensitiveLogFieldNeedles } from '../generated-log-redaction-policy.js';

const RedactedLogFieldValue = '[REDACTED]';
const CircularLogFieldValue = '[CIRCULAR]';
const UnsupportedLogFieldValue = '[UNSUPPORTED_LOG_VALUE]';

function isSensitiveLogField(key: string): boolean {
  const normalized = key.replace(/[^A-Za-z0-9]/gu, '').toLowerCase();
  return normalized.length > 0 && GeneratedSensitiveLogFieldNeedles.some((needle) => normalized.includes(needle));
}

export function redactStructuredLogValue(value: unknown, fieldName?: string): unknown {
  return sanitizeStructuredLogValue(value, fieldName ?? '', {
    redactedValue: RedactedLogFieldValue,
    circularValue: CircularLogFieldValue,
    unsupportedValue: UnsupportedLogFieldValue,
    isSensitiveKey: isSensitiveLogField,
  });
}
