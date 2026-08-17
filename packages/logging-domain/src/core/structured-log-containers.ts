type SanitizeChild = (value: unknown, serializationKey: string) => unknown;

export function sanitizeStructuredContainer(
  value: object,
  unsupportedValue: string,
  redactedValue: string,
  isSensitiveKey: (key: string) => boolean,
  sanitizeChild: SanitizeChild
): unknown {
  try {
    if (Array.isArray(value)) {
      return sanitizeArray(value, unsupportedValue, sanitizeChild);
    }
    return isPlainRecord(value)
      ? sanitizeRecord(value, unsupportedValue, redactedValue, isSensitiveKey, sanitizeChild)
      : unsupportedValue;
  } catch {
    return unsupportedValue;
  }
}

function sanitizeArray(value: unknown[], unsupportedValue: string, sanitizeChild: SanitizeChild): unknown {
  let length: unknown;
  try {
    length = Reflect.get(value, 'length');
  } catch {
    return unsupportedValue;
  }
  if (!isValidArrayLength(length)) {
    return unsupportedValue;
  }

  const entries: unknown[] = [];
  for (let index = 0; index < length; index += 1) {
    const key = String(index);
    try {
      entries.push(sanitizeChild(Reflect.get(value, key), key));
    } catch {
      entries.push(unsupportedValue);
    }
  }
  return entries;
}

function sanitizeRecord(
  value: Record<string, unknown>,
  unsupportedValue: string,
  redactedValue: string,
  isSensitiveKey: (key: string) => boolean,
  sanitizeChild: SanitizeChild
): unknown {
  let keys: string[];
  try {
    keys = Object.keys(value);
  } catch {
    return unsupportedValue;
  }

  const entries: [string, unknown][] = [];
  for (const key of keys) {
    if (isSensitiveKey(key)) {
      entries.push([key, redactedValue]);
      continue;
    }
    try {
      entries.push([key, sanitizeChild(Reflect.get(value, key), key)]);
    } catch {
      entries.push([key, unsupportedValue]);
    }
  }
  return Object.fromEntries(entries);
}

function isValidArrayLength(value: unknown): value is number {
  return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0;
}

function isPlainRecord(value: object): value is Record<string, unknown> {
  try {
    const prototype = Object.getPrototypeOf(value);
    return prototype === Object.prototype || prototype === null;
  } catch {
    return false;
  }
}
