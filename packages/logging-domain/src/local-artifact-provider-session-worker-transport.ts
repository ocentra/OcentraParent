import net, { type Socket } from 'node:net';
import { type MessagePort } from 'node:worker_threads';
import { providerWorkerMessageFrame, type SessionWorkerData } from './local-artifact-provider-session-worker-frame';
import { ProviderWorkerState } from './local-artifact-provider-session-worker-state';

const ProviderConnectTimeoutMs = 10_000;
const ProviderConnectRetryMs = 5;

function errorMessage(error: unknown, fallback: string): string {
  return error instanceof Error ? error.message : fallback;
}

function isPendingPipe(error: NodeJS.ErrnoException): boolean {
  return error.code === 'ENOENT';
}

function attachConnectedTransport(socket: Socket, state: ProviderWorkerState, parentPort: MessagePort): void {
  socket.on('data', (chunk) => state.receive(chunk, socket));
  socket.on('error', (error) => state.fail(`provider pipe failed: ${error.message}`));
  socket.on('close', () => state.fail('provider pipe closed'));
  parentPort.on('message', (message: unknown) => {
    if (state.isTerminal()) return;
    try {
      const frame = providerWorkerMessageFrame(message);
      state.beginRequest();
      socket.write(frame, (error) => {
        if (error != null) state.fail(`provider pipe write failed: ${error.message}`);
      });
    } catch (error) {
      state.fail(errorMessage(error, 'provider worker request failed'));
      socket.destroy();
    }
  });
}

function connectUntilReady(
  pipeName: string,
  state: ProviderWorkerState,
  parentPort: MessagePort,
  deadline: number
): void {
  const socket = net.createConnection(pipeName);
  const handleConnected = (): void => {
    socket.removeListener('error', handleConnectionError);
    attachConnectedTransport(socket, state, parentPort);
  };
  const handleConnectionError = (error: NodeJS.ErrnoException): void => {
    socket.removeListener('connect', handleConnected);
    socket.destroy();
    if (isPendingPipe(error) && Date.now() < deadline) {
      setTimeout(() => connectUntilReady(pipeName, state, parentPort, deadline), ProviderConnectRetryMs);
      return;
    }
    state.fail(`provider pipe failed: ${error.message}`);
  };
  socket.once('connect', handleConnected);
  socket.once('error', handleConnectionError);
}

export function connectProviderWorkerTransport(data: SessionWorkerData, parentPort: MessagePort): void {
  const control = new Int32Array(data.sharedBuffer, 0, 2);
  const sharedPayload = new Uint8Array(data.sharedBuffer, 8);
  const state = new ProviderWorkerState(control, sharedPayload);
  connectUntilReady(data.pipeName, state, parentPort, Date.now() + ProviderConnectTimeoutMs);
}
