import { sanitizeStructuredContainer } from './structured-log-containers.js';
import { serializeStructuredLogValue } from './structured-log-to-json.js';

interface StructuredLogSanitizerPolicy {
  readonly redactedValue: string;
  readonly circularValue: string;
  readonly unsupportedValue: string;
  readonly isSensitiveKey: (key: string) => boolean;
  readonly sanitizeString?: (value: string, key: string) => string;
}

export function sanitizeStructuredLogValue(
  value: unknown,
  serializationKey: string,
  policy: StructuredLogSanitizerPolicy
): unknown {
  try {
    return sanitizeValue(value, serializationKey, new WeakSet<object>(), policy, true);
  } catch {
    return policy.unsupportedValue;
  }
}

function sanitizeValue(
  value: unknown,
  serializationKey: string,
  activeObjects: WeakSet<object>,
  policy: StructuredLogSanitizerPolicy,
  invokeToJson: boolean
): unknown {
  if (policy.isSensitiveKey(serializationKey)) {
    return policy.redactedValue;
  }
  if (typeof value === 'string') {
    return policy.sanitizeString?.(value, serializationKey) ?? value;
  }
  if (value === null || typeof value === 'boolean' || (typeof value === 'number' && Number.isFinite(value))) {
    return value;
  }
  if (typeof value !== 'object') {
    return policy.unsupportedValue;
  }
  if (activeObjects.has(value)) {
    return policy.circularValue;
  }

  activeObjects.add(value);
  try {
    const serialized = serializeStructuredLogValue(value, serializationKey, invokeToJson);
    if (serialized.status === 'failed') {
      return policy.unsupportedValue;
    }
    if (serialized.status === 'serialized') {
      if (serialized.value === value) {
        return policy.unsupportedValue;
      }
      return sanitizeValue(serialized.value, serializationKey, activeObjects, policy, false);
    }

    return sanitizeStructuredContainer(
      value,
      policy.unsupportedValue,
      policy.redactedValue,
      policy.isSensitiveKey,
      (child, key) => sanitizeValue(child, key, activeObjects, policy, true)
    );
  } catch {
    return policy.unsupportedValue;
  } finally {
    activeObjects.delete(value);
  }
}
