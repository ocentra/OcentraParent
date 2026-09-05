import { parentPort, workerData } from 'node:worker_threads';
import {
  parseSessionWorkerData,
  SharedHeaderBytes,
  writeSharedError,
} from './local-artifact-provider-session-worker-frame';
import { connectProviderWorkerTransport } from './local-artifact-provider-session-worker-transport';

function run(): void {
  const data = parseSessionWorkerData(workerData);
  if (parentPort == null) throw new Error('provider worker parent port is unavailable');
  connectProviderWorkerTransport(data, parentPort);
}

try {
  run();
} catch (error) {
  const data = parseSessionWorkerData(workerData);
  const control = new Int32Array(data.sharedBuffer, 0, 2);
  const payload = new Uint8Array(data.sharedBuffer, SharedHeaderBytes);
  writeSharedError(control, payload, error instanceof Error ? error.message : 'provider worker failed to start');
}
