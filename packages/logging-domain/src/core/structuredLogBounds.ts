import { assertBoundedText, MaximumMessageBytes } from './logTextCustody';

const MaximumStructuredDepth = 12;
const MaximumStructuredNodes = 2_048;
const MaximumStructuredEntries = 256;
const MaximumStructuredKeyBytes = 256;

interface StructuredBoundState {
  nodes: number;
  readonly active: WeakSet<object>;
}

export function assertStructuredLogBounds(value: unknown): void {
  visitStructuredLogValue(value, 0, { nodes: 0, active: new WeakSet<object>() });
}

function visitStructuredLogValue(value: unknown, depth: number, state: StructuredBoundState): void {
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
      visitStructuredLogArray(value, depth, state);
    } else {
      visitStructuredLogRecord(value, depth, state);
    }
  } finally {
    state.active.delete(value);
  }
}

function visitStructuredLogArray(value: unknown[], depth: number, state: StructuredBoundState): void {
  const lengthDescriptor = Object.getOwnPropertyDescriptor(value, 'length');
  const length = lengthDescriptor != null && 'value' in lengthDescriptor ? lengthDescriptor.value : null;
  if (!Number.isSafeInteger(length) || (length as number) < 0 || (length as number) > MaximumStructuredEntries) {
    throw new Error('structured log array exceeds its custody limit');
  }
  for (let index = 0; index < (length as number); index += 1) {
    const descriptor = Object.getOwnPropertyDescriptor(value, String(index));
    if (descriptor == null || !('value' in descriptor)) {
      throw new Error('structured log arrays must contain only owned data values');
    }
    visitStructuredLogValue(descriptor.value, depth + 1, state);
  }
}

function visitStructuredLogRecord(value: object, depth: number, state: StructuredBoundState): void {
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) {
    throw new Error('structured log objects must be plain data records');
  }
  const keys = Object.keys(value);
  if (keys.length > MaximumStructuredEntries) {
    throw new Error('structured log object exceeds its custody limit');
  }
  for (const key of keys) {
    const descriptor = Object.getOwnPropertyDescriptor(value, key);
    assertBoundedText(key, 'structured log key', MaximumStructuredKeyBytes);
    if (descriptor == null || !('value' in descriptor)) {
      throw new Error('structured log objects must not contain accessors');
    }
    visitStructuredLogValue(descriptor.value, depth + 1, state);
  }
}
