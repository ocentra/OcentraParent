import { MaximumProviderFrameBytes } from './local-artifact-provider-protocol';

export const SharedHeaderBytes = 8;
export const StateIndex = 0;
export const LengthIndex = 1;
export const StateWaiting = 0;
export const StateReady = 1;
export const StateFailed = -1;

export interface SessionWorkerData {
  readonly pipeName: string;
  readonly sharedBuffer: SharedArrayBuffer;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value != null && !Array.isArray(value);
}

export function parseSessionWorkerData(value: unknown): SessionWorkerData {
  if (!isRecord(value)) throw new Error('provider worker data is invalid');
  const pipeName = value['pipeName'];
  const sharedBuffer = value['sharedBuffer'];
  if (
    typeof pipeName !== 'string' ||
    pipeName.length === 0 ||
    !(sharedBuffer instanceof SharedArrayBuffer) ||
    sharedBuffer.byteLength !== SharedHeaderBytes + MaximumProviderFrameBytes
  ) {
    throw new Error('provider worker data is invalid');
  }
  return { pipeName, sharedBuffer };
}

export function writeSharedFrame(
  control: Int32Array,
  payload: Uint8Array,
  state: typeof StateReady | typeof StateFailed,
  frame: Buffer
): void {
  if (frame.byteLength === 0 || frame.byteLength > payload.byteLength) {
    writeSharedError(control, payload, 'provider frame exceeded the shared transport bound');
    return;
  }
  payload.fill(0, 0, Math.min(64, payload.byteLength));
  payload.set(frame, 0);
  Atomics.store(control, LengthIndex, frame.byteLength);
  Atomics.store(control, StateIndex, state);
  Atomics.notify(control, StateIndex);
}

export function writeSharedError(control: Int32Array, payload: Uint8Array, message: string): void {
  const encoded = Buffer.from(message, 'utf8').subarray(0, Math.min(payload.byteLength, 2_048));
  payload.set(encoded, 0);
  Atomics.store(control, LengthIndex, encoded.byteLength);
  Atomics.store(control, StateIndex, StateFailed);
  Atomics.notify(control, StateIndex);
}

export function providerWorkerMessageFrame(value: unknown): Buffer {
  if (!isRecord(value) || !(value['frame'] instanceof Uint8Array)) {
    throw new Error('provider worker request is invalid');
  }
  const frame = Buffer.from(value['frame']);
  if (frame.byteLength < 5 || frame.byteLength > MaximumProviderFrameBytes) {
    throw new Error('provider worker request frame is invalid');
  }
  return frame;
}
