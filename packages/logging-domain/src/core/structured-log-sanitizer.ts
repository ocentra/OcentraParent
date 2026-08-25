import { sanitizeStructuredContainer } from './structured-log-containers.js';

interface StructuredLogSanitizerPolicy {
  readonly redactedValue: string;
  readonly circularValue: string;
  readonly unsupportedValue: string;
  readonly isSensitiveKey: (key: string) => boolean;
  readonly sanitizeString?: (value: string, key: string) => string;
}

interface StructuredLogSanitizerState {
  nodes: number;
  readonly activeObjects: WeakSet<object>;
}

const MaximumSanitizedStructuredDepth = 12;
const MaximumSanitizedStructuredNodes = 2_048;
const MaximumSanitizedStringBytes = 16 * 1024;

export function sanitizeStructuredLogValue(
  value: unknown,
  serializationKey: string,
  policy: StructuredLogSanitizerPolicy
): unknown {
  try {
    return sanitizeValue(value, serializationKey, 0, { nodes: 0, activeObjects: new WeakSet<object>() }, policy);
  } catch {
    return policy.unsupportedValue;
  }
}

function sanitizeValue(
  value: unknown,
  serializationKey: string,
  depth: number,
  state: StructuredLogSanitizerState,
  policy: StructuredLogSanitizerPolicy
): unknown {
  if (policy.isSensitiveKey(serializationKey)) {
    return policy.redactedValue;
  }
  state.nodes += 1;
  if (state.nodes > MaximumSanitizedStructuredNodes || depth > MaximumSanitizedStructuredDepth) {
    return policy.unsupportedValue;
  }
  if (typeof value === 'string') {
    return sanitizeStringValue(value, serializationKey, policy);
  }
  if (isSupportedScalar(value)) {
    return value;
  }
  return sanitizeObjectValue(value, depth, state, policy);
}

function sanitizeStringValue(value: string, key: string, policy: StructuredLogSanitizerPolicy): unknown {
  if (
    value.length > MaximumSanitizedStringBytes ||
    new TextEncoder().encode(value).byteLength > MaximumSanitizedStringBytes
  ) {
    return policy.unsupportedValue;
  }
  return policy.sanitizeString?.(value, key) ?? value;
}

function isSupportedScalar(value: unknown): value is null | boolean | number {
  return value === null || typeof value === 'boolean' || (typeof value === 'number' && Number.isFinite(value));
}

function sanitizeObjectValue(
  value: unknown,
  depth: number,
  state: StructuredLogSanitizerState,
  policy: StructuredLogSanitizerPolicy
): unknown {
  if (value === null || typeof value !== 'object') {
    return policy.unsupportedValue;
  }
  if (state.activeObjects.has(value)) {
    return policy.circularValue;
  }

  state.activeObjects.add(value);
  try {
    return sanitizeStructuredContainer(
      value,
      policy.unsupportedValue,
      policy.redactedValue,
      policy.isSensitiveKey,
      (child, key) => sanitizeValue(child, key, depth + 1, state, policy)
    );
  } catch {
    return policy.unsupportedValue;
  } finally {
    state.activeObjects.delete(value);
  }
}
