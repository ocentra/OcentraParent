import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { MaximumProviderFrameBytes } from './local-artifact-provider-protocol';

const FramePrefixBytes = 4;
const SharedHeaderBytes = 8;
const StateIndex = 0;
const LengthIndex = 1;
const StateWaiting = 0;
const StateReady = 1;
const StateFailed = -1;

function sharedViews(sharedBuffer: SharedArrayBuffer): {
  readonly control: Int32Array;
  readonly payload: Uint8Array;
} {
  return {
    control: new Int32Array(sharedBuffer, 0, 2),
    payload: new Uint8Array(sharedBuffer, SharedHeaderBytes),
  };
}

export function encodeProviderFrame(value: unknown): Buffer {
  const body = Buffer.from(JSON.stringify(value), 'utf8');
  if (body.byteLength === 0 || body.byteLength > MaximumProviderFrameBytes - FramePrefixBytes) {
    throw new LocalArtifactProviderError('protocol-limit', 'provider frame exceeded its bound');
  }
  const frame = Buffer.allocUnsafe(FramePrefixBytes + body.byteLength);
  frame.writeUInt32BE(body.byteLength, 0);
  body.copy(frame, FramePrefixBytes);
  return frame;
}

export function parseProviderJsonFrame(frame: Buffer): unknown {
  try {
    return JSON.parse(frame.toString('utf8'));
  } catch {
    throw new LocalArtifactProviderError('protocol-frame', 'provider frame is not valid JSON');
  }
}

export function waitForProviderWorkerFrame(
  sharedBuffer: SharedArrayBuffer,
  timeoutMs: number,
  timeoutMessage: string
): Buffer {
  const { control, payload } = sharedViews(sharedBuffer);
  const waitResult = Atomics.wait(control, StateIndex, StateWaiting, timeoutMs);
  if (waitResult === 'timed-out') {
    throw new LocalArtifactProviderError('provider-timeout', timeoutMessage);
  }
  const state = Atomics.load(control, StateIndex);
  const length = Atomics.load(control, LengthIndex);
  if (length <= 0 || length > payload.byteLength) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider worker frame is invalid');
  }
  const frame = Buffer.from(payload.slice(0, length));
  if (state === StateFailed) {
    throw new LocalArtifactProviderError('provider-unavailable', frame.toString('utf8'));
  }
  if (state !== StateReady) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider worker state is invalid');
  }
  return frame;
}

export function prepareProviderSharedRequest(sharedBuffer: SharedArrayBuffer): void {
  const { control } = sharedViews(sharedBuffer);
  Atomics.store(control, LengthIndex, 0);
  Atomics.store(control, StateIndex, StateWaiting);
}
