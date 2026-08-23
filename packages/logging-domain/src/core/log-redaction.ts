import { sanitizeStructuredLogValue } from './structured-log-sanitizer.js';
import { GeneratedSensitiveLogFieldNeedles } from '../generated-log-redaction-policy.js';

const RedactedLogFieldValue = '[REDACTED]';
const CircularLogFieldValue = '[CIRCULAR]';
const UnsupportedLogFieldValue = '[UNSUPPORTED_LOG_VALUE]';
const RedactedPathValue = '[REDACTED_PATH]';
const RedactedUrlValue = '[REDACTED_URL]';
const SensitiveAssignmentPattern = new RegExp(
  `\\b(${GeneratedSensitiveLogFieldNeedles.join('|')})\\b\\s*([:=])\\s*(?:"[^"]*"|'[^']*'|[^\\s,;]+)`,
  'giu'
);
const AuthorizationValuePattern = /\b(bearer|basic)\s+[A-Za-z0-9+/_=-]{8,}/giu;
const UrlPattern = /\b(?:https?|wss?|file):\/\/[^\s"'<>]+/giu;
const WindowsPathPattern = /(^|[\s("'`])(?:[A-Za-z]:[\\/])[^\s"'`<>]*/gmu;
const UnixPathPattern = /(^|[\s("'`])\/(?:Users|home|var|tmp|etc|opt|mnt|private|Volumes)\/[^\s"'`<>]*/gmu;

function isSensitiveLogField(key: string): boolean {
  const normalized = key.replace(/[^A-Za-z0-9]/gu, '').toLowerCase();
  return normalized.length > 0 && GeneratedSensitiveLogFieldNeedles.some((needle) => normalized.includes(needle));
}

export function redactUnstructuredLogText(value: string): string {
  return value
    .replace(
      SensitiveAssignmentPattern,
      (_match, key: string, separator: string) => `${key}${separator}${RedactedLogFieldValue}`
    )
    .replace(AuthorizationValuePattern, (_match, scheme: string) => `${scheme} ${RedactedLogFieldValue}`)
    .replace(UrlPattern, RedactedUrlValue)
    .replace(WindowsPathPattern, (_match, prefix: string) => `${prefix}${RedactedPathValue}`)
    .replace(UnixPathPattern, (_match, prefix: string) => `${prefix}${RedactedPathValue}`);
}

export function redactStructuredLogValue(value: unknown, fieldName?: string): unknown {
  return sanitizeStructuredLogValue(value, fieldName ?? '', {
    redactedValue: RedactedLogFieldValue,
    circularValue: CircularLogFieldValue,
    unsupportedValue: UnsupportedLogFieldValue,
    isSensitiveKey: isSensitiveLogField,
    sanitizeString: (entry) => redactUnstructuredLogText(entry),
  });
}
