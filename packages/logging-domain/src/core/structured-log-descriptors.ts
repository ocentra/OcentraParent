export type SanitizeStructuredChild = (value: unknown, serializationKey: string) => unknown;

const MaximumStructuredContainerEntries = 256;
const MaximumStructuredKeyLength = 256;

export function sanitizeStructuredArray(
  value: unknown[],
  unsupportedValue: string,
  sanitizeChild: SanitizeStructuredChild
): unknown {
  let lengthDescriptor: PropertyDescriptor | undefined;
  try {
    lengthDescriptor = Object.getOwnPropertyDescriptor(value, 'length');
  } catch {
    return unsupportedValue;
  }
  const length = lengthDescriptor != null && 'value' in lengthDescriptor ? lengthDescriptor.value : null;
  if (!isValidArrayLength(length) || length > MaximumStructuredContainerEntries) {
    return unsupportedValue;
  }
  const entries: unknown[] = [];
  for (let index = 0; index < length; index += 1) {
    const descriptor = Object.getOwnPropertyDescriptor(value, String(index));
    if (descriptor == null || !('value' in descriptor)) {
      entries.push(unsupportedValue);
    } else {
      entries.push(sanitizeChild(descriptor.value, String(index)));
    }
  }
  return entries;
}

export function sanitizeStructuredRecord(
  value: Record<string, unknown>,
  unsupportedValue: string,
  redactedValue: string,
  isSensitiveKey: (key: string) => boolean,
  sanitizeChild: SanitizeStructuredChild
): unknown {
  const names = Object.getOwnPropertyNames(value);
  if (
    names.length > MaximumStructuredContainerEntries ||
    names.some((key) => key.length > MaximumStructuredKeyLength)
  ) {
    return unsupportedValue;
  }
  const entries: [string, unknown][] = [];
  for (const key of names) {
    const descriptor = Object.getOwnPropertyDescriptor(value, key);
    if (descriptor == null) {
      return unsupportedValue;
    }
    const sanitized = sanitizeRecordEntry(
      key,
      descriptor,
      unsupportedValue,
      redactedValue,
      isSensitiveKey,
      sanitizeChild
    );
    if (sanitized != null) {
      entries.push(sanitized);
    }
  }
  return Object.fromEntries(entries);
}

function sanitizeRecordEntry(
  key: string,
  descriptor: PropertyDescriptor,
  unsupportedValue: string,
  redactedValue: string,
  isSensitiveKey: (key: string) => boolean,
  sanitizeChild: SanitizeStructuredChild
): [string, unknown] | null {
  if (descriptor.enumerable !== true) {
    return null;
  }
  if (isSensitiveKey(key)) {
    return [key, redactedValue];
  }
  return 'value' in descriptor ? [key, sanitizeChild(descriptor.value, key)] : [key, unsupportedValue];
}

function isValidArrayLength(value: unknown): value is number {
  return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0;
}
