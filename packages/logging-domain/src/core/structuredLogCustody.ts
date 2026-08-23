import { redactStructuredLogValue } from './log-redaction';
import { assertBoundedText, MaximumMessageBytes, MaximumStructuredDataBytes } from './logTextCustody';

const MaximumStructuredDepth = 12;
const MaximumStructuredNodes = 2_048;
const MaximumStructuredEntries = 256;
const MaximumStructuredKeyBytes = 256;

interface StructuredBoundState {
  nodes: number;
  readonly active: WeakSet<object>;
}

function assertStructuredBounds(value: unknown, depth: number, state: StructuredBoundState): void {
  state.nodes += 1;
  if (state.nodes > MaximumStructuredNodes || depth > MaximumStructuredDepth) {
    throw new Error('structured log data exceeds its custody shape limit');
  }
  if (typeof value === 'string') {
    assertBoundedText(value, 'structured log string', MaximumMessageBytes);
    return;
  }
  if (value == null || typeof value !== 'object' || state.active.has(value)) {
    return;
  }
  state.active.add(value);
  try {
    if (Array.isArray(value)) {
      if (value.length > MaximumStructuredEntries) {
        throw new Error('structured log array exceeds its custody limit');
      }
      for (const item of value) {
        assertStructuredBounds(item, depth + 1, state);
      }
      return;
    }
    const prototype = Object.getPrototypeOf(value);
    if (prototype !== Object.prototype && prototype !== null) {
      return;
    }
    const keys = Object.keys(value);
    if (keys.length > MaximumStructuredEntries) {
      throw new Error('structured log object exceeds its custody limit');
    }
    for (const key of keys) {
      assertBoundedText(key, 'structured log key', MaximumStructuredKeyBytes);
      assertStructuredBounds(Reflect.get(value, key), depth + 1, state);
    }
  } finally {
    state.active.delete(value);
  }
}

export function serializeStructuredLogDataForCustody(value: unknown): string | null {
  if (value == null) {
    return null;
  }
  assertStructuredBounds(value, 0, { nodes: 0, active: new WeakSet<object>() });
  const serialized = JSON.stringify(redactStructuredLogValue(value));
  if (serialized == null) {
    throw new Error('structured log data is not serializable');
  }
  assertBoundedText(serialized, 'structured log data', MaximumStructuredDataBytes);
  return serialized;
}
