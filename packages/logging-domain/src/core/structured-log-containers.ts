import {
  sanitizeStructuredArray,
  sanitizeStructuredRecord,
  type SanitizeStructuredChild,
} from './structured-log-descriptors.js';

export function sanitizeStructuredContainer(
  value: object,
  unsupportedValue: string,
  redactedValue: string,
  isSensitiveKey: (key: string) => boolean,
  sanitizeChild: SanitizeStructuredChild
): unknown {
  try {
    if (Array.isArray(value)) {
      return sanitizeStructuredArray(value, unsupportedValue, sanitizeChild);
    }
    return isPlainRecord(value)
      ? sanitizeStructuredRecord(value, unsupportedValue, redactedValue, isSensitiveKey, sanitizeChild)
      : unsupportedValue;
  } catch {
    return unsupportedValue;
  }
}

function isPlainRecord(value: object): value is Record<string, unknown> {
  try {
    const prototype = Object.getPrototypeOf(value);
    return prototype === Object.prototype || prototype === null;
  } catch {
    return false;
  }
}
