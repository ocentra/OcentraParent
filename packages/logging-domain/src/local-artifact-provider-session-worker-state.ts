import { type Socket } from 'node:net';
import { MaximumProviderFrameBytes } from './local-artifact-provider-protocol';
import {
  StateIndex,
  StateReady,
  StateWaiting,
  writeSharedError,
  writeSharedFrame,
} from './local-artifact-provider-session-worker-frame';

export class ProviderWorkerState {
  private received = Buffer.alloc(0);
  private expectedBodyBytes: number | null = null;
  private expectingFrame = true;
  private terminal = false;

  constructor(
    private readonly control: Int32Array,
    private readonly sharedPayload: Uint8Array
  ) {}

  isTerminal(): boolean {
    return this.terminal;
  }

  fail(message: string): void {
    const publishError = !this.terminal && Atomics.load(this.control, StateIndex) === StateWaiting;
    this.terminal = true;
    if (publishError) writeSharedError(this.control, this.sharedPayload, message);
  }

  beginRequest(): void {
    if (this.terminal || this.expectingFrame || Atomics.load(this.control, StateIndex) !== StateWaiting) {
      throw new Error('provider worker already has an in-flight frame');
    }
    this.expectingFrame = true;
  }

  receive(chunk: Buffer, socket: Socket): void {
    if (this.terminal) return;
    this.received = Buffer.concat([this.received, chunk]);
    if (!this.expectingFrame) {
      this.reject(socket, 'provider sent an unsolicited response frame');
      return;
    }
    if (!this.loadExpectedLength(socket)) return;
    const expectedBodyBytes = this.expectedBodyBytes ?? 0;
    if (this.received.byteLength < expectedBodyBytes) return;
    if (Atomics.load(this.control, StateIndex) !== StateWaiting) {
      this.reject(socket, 'provider sent an unsolicited response frame');
      return;
    }
    const frame = this.received.subarray(0, expectedBodyBytes);
    this.received = this.received.subarray(expectedBodyBytes);
    this.expectedBodyBytes = null;
    this.expectingFrame = false;
    writeSharedFrame(this.control, this.sharedPayload, StateReady, frame);
    if (this.received.byteLength > 0) {
      this.reject(socket, 'provider sent multiple response frames for one request');
    }
  }

  private loadExpectedLength(socket: Socket): boolean {
    if (this.expectedBodyBytes != null) return true;
    if (this.received.byteLength < 4) return false;
    this.expectedBodyBytes = this.received.readUInt32BE(0);
    this.received = this.received.subarray(4);
    if (this.expectedBodyBytes === 0 || this.expectedBodyBytes > MaximumProviderFrameBytes - 4) {
      this.reject(socket, 'provider response frame length is invalid');
      return false;
    }
    return true;
  }

  private reject(socket: Socket, message: string): void {
    this.fail(message);
    socket.destroy();
  }
}
