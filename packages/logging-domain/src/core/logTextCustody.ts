import { redactUnstructuredLogText } from './log-redaction';

export const MaximumMessageBytes = 16 * 1024;
export const MaximumStructuredDataBytes = 128 * 1024;
export const MaximumStackBytes = 64 * 1024;
export const MaximumMetadataBytes = 4 * 1024;

const MaximumTagBytes = 256;
const MaximumTags = 32;
const RedactedPathValue = '[REDACTED_PATH]';

export function utf8Bytes(value: string): number {
  return new TextEncoder().encode(value).byteLength;
}

export function assertBoundedText(value: string, label: string, maximumBytes: number): void {
  if (value.length > maximumBytes || utf8Bytes(value) > maximumBytes) {
    throw new Error(`${label} exceeds its custody limit`);
  }
}

export function sanitizeLogText(value: string, label: string, maximumBytes: number): string {
  assertBoundedText(value, label, maximumBytes);
  const sanitized = redactUnstructuredLogText(value);
  assertBoundedText(sanitized, label, maximumBytes);
  return sanitized;
}

export function sanitizeLogIdentity(value: string, label: string): string {
  if (value.length === 0) {
    throw new Error(`${label} must not be empty`);
  }
  const sanitized = sanitizeLogText(value, label, MaximumMetadataBytes);
  if (sanitized !== value) {
    throw new Error(`${label} contains restricted log data`);
  }
  return value;
}

export function sanitizeNullableLogText(
  value: string | null,
  label: string,
  maximumBytes = MaximumMetadataBytes
): string | null {
  return value == null ? null : sanitizeLogText(value, label, maximumBytes);
}

export function sanitizeLogPath(value: string | null, label: string): string | null {
  if (value == null) {
    return null;
  }
  assertBoundedText(value, label, MaximumMetadataBytes);
  const normalized = value.replace(/\\/gu, '/');
  const segments = normalized.split('/');
  if (/^(?:[A-Za-z]:|\/|\/\/)/u.test(normalized) || segments.includes('..')) {
    return RedactedPathValue;
  }
  return sanitizeLogText(normalized, label, MaximumMetadataBytes);
}

export function sanitizeLogTags(tags: readonly string[]): readonly string[] {
  if (tags.length > MaximumTags) {
    throw new Error('log tags exceed their custody limit');
  }
  return tags.map((tag) => sanitizeLogText(tag, 'log tag', MaximumTagBytes));
}
