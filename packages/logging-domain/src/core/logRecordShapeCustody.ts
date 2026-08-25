const MaximumLogRecordFields = 64;

export function ownedLogRecord(input: unknown, label: string): Record<string, unknown> {
  if (typeof input !== 'object' || input == null || Array.isArray(input)) {
    throw new Error(`${label} must be a plain data record`);
  }
  const prototype = Object.getPrototypeOf(input);
  if (prototype !== Object.prototype && prototype !== null) {
    throw new Error(`${label} must be a plain data record`);
  }
  const names = Object.getOwnPropertyNames(input);
  if (names.length > MaximumLogRecordFields || Object.getOwnPropertySymbols(input).length > 0) {
    throw new Error(`${label} exceeds its custody shape limit`);
  }
  return Object.fromEntries(
    names.flatMap((key) => {
      const descriptor = Object.getOwnPropertyDescriptor(input, key);
      if (descriptor?.enumerable !== true) {
        return [];
      }
      if (!('value' in descriptor)) {
        throw new Error(`${label} must not contain accessors`);
      }
      return [[key, descriptor.value]];
    })
  );
}

export function ownedLogArray(input: unknown, maximumEntries: number, label: string): unknown[] {
  if (!Array.isArray(input) || Object.getPrototypeOf(input) !== Array.prototype) {
    throw new Error(`${label} must be a plain data array`);
  }
  const lengthDescriptor = Object.getOwnPropertyDescriptor(input, 'length');
  const length = lengthDescriptor != null && 'value' in lengthDescriptor ? lengthDescriptor.value : null;
  if (!Number.isSafeInteger(length) || (length as number) < 0 || (length as number) > maximumEntries) {
    throw new Error(`${label} exceeds its custody limit`);
  }
  const entries: unknown[] = [];
  for (let index = 0; index < (length as number); index += 1) {
    const descriptor = Object.getOwnPropertyDescriptor(input, String(index));
    if (descriptor == null || !('value' in descriptor)) {
      throw new Error(`${label} must contain only owned data values`);
    }
    entries.push(descriptor.value);
  }
  return entries;
}
