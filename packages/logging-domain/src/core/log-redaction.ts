import { sanitizeStructuredLogValue } from './structured-log-sanitizer.js';
import { GeneratedSensitiveLogFieldNeedles } from '../generated-log-redaction-policy.js';

const RedactedLogFieldValue = '[REDACTED]';
const CircularLogFieldValue = '[CIRCULAR]';
const UnsupportedLogFieldValue = '[UNSUPPORTED_LOG_VALUE]';
const RedactedPathValue = '[REDACTED_PATH]';
const RedactedUrlValue = '[REDACTED_URL]';
const AssignmentPattern = /\b([A-Za-z][A-Za-z0-9_.-]{0,127})\s*([:=])\s*(?:"[^"]*"|'[^']*'|[^\s,;]+)/gu;
const AuthorizationValuePattern = /\b(bearer|basic)\s+[^\s,;]+/giu;
const UrlPattern = /\b(?:https?|wss?|file):\/\/[^\s"'<>]+/giu;
const WindowsPathPattern = /(^|[\s("'`=])(?:[A-Za-z]:(?=[^:\s"'`<>])|\\\\)[^\s"'`<>]*/gmu;
const UnixPathPattern = /(^|[\s("'`=])\/(?:Users|home|var|tmp|etc|opt|mnt|private|Volumes)\/[^\s"'`<>]*/gmu;
const CompactPathLogFields = new Set([
  'artifactfile',
  'artifactpath',
  'file',
  'filename',
  'filepath',
  'inputfile',
  'inputpath',
  'logfile',
  'logpath',
  'outputfile',
  'outputpath',
  'path',
  'sourcefile',
  'sourcepath',
]);

function isSensitiveLogField(key: string): boolean {
  const normalized = key.replace(/[^A-Za-z0-9]/gu, '').toLowerCase();
  return normalized.length > 0 && GeneratedSensitiveLogFieldNeedles.some((needle) => normalized.includes(needle));
}

function isPathLogField(key: string): boolean {
  const normalized = key.replace(/[^A-Za-z0-9]/gu, '').toLowerCase();
  if (CompactPathLogFields.has(normalized)) {
    return true;
  }
  const tokens = key
    .replace(/([a-z0-9])([A-Z])/gu, '$1 $2')
    .split(/[^A-Za-z0-9]+/gu)
    .filter((token) => token.length > 0);
  const finalToken = tokens.at(-1)?.toLowerCase();
  return finalToken === 'path' || finalToken === 'file' || finalToken === 'filename';
}

function shouldRedactStructuredLogField(key: string): boolean {
  return isSensitiveLogField(key) || isPathLogField(key);
}

export function redactUnstructuredLogText(value: string): string {
  return value
    .replace(AuthorizationValuePattern, (_match, scheme: string) => `${scheme} ${RedactedLogFieldValue}`)
    .replace(AssignmentPattern, (match, key: string, separator: string) =>
      isSensitiveLogField(key)
        ? `${key}${separator}${RedactedLogFieldValue}`
        : isPathLogField(key)
          ? `${key}${separator}${RedactedPathValue}`
          : match
    )
    .replace(UrlPattern, RedactedUrlValue)
    .replace(WindowsPathPattern, (_match, prefix: string) => `${prefix}${RedactedPathValue}`)
    .replace(UnixPathPattern, (_match, prefix: string) => `${prefix}${RedactedPathValue}`);
}

export function redactStructuredLogValue(value: unknown, fieldName?: string): unknown {
  return sanitizeStructuredLogValue(value, fieldName ?? '', {
    redactedValue: RedactedLogFieldValue,
    circularValue: CircularLogFieldValue,
    unsupportedValue: UnsupportedLogFieldValue,
    isSensitiveKey: shouldRedactStructuredLogField,
    sanitizeString: (entry) => redactUnstructuredLogText(entry),
  });
}
