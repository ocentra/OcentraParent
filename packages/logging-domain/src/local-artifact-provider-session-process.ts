import type childProcess from 'node:child_process';
import crypto from 'node:crypto';
import { Worker } from 'node:worker_threads';

const ProviderGracefulExitTimeoutMs = 250;
const ProviderTerminationTimeoutMs = 2_000;

export function localArtifactProviderPipeName(): string {
  return `\\\\.\\pipe\\ocentra-parent-local-artifact-${process.pid}-${crypto.randomBytes(16).toString('hex')}`;
}

function sourceWorkerBootstrap(): URL {
  const loaderUrl = import.meta.resolve('tsx/esm/api');
  const workerUrl = new URL('./local-artifact-provider-session-worker.ts', import.meta.url).href;
  const source = `import { register } from ${JSON.stringify(loaderUrl)}; register(); await import(${JSON.stringify(workerUrl)});`;
  return new URL(`data:text/javascript,${encodeURIComponent(source)}`);
}

export function createLocalArtifactProviderWorker(pipeName: string, sharedBuffer: SharedArrayBuffer): Worker {
  const sourceMode = import.meta.url.endsWith('.ts');
  return new Worker(
    sourceMode ? sourceWorkerBootstrap() : new URL('./local-artifact-provider-session-worker.js', import.meta.url),
    { workerData: { pipeName, sharedBuffer } }
  );
}

export async function terminateLocalArtifactProviderWorker(worker: Worker): Promise<void> {
  await worker.terminate();
}

export function observeLocalArtifactProviderProcessExit(provider: childProcess.ChildProcess): Promise<void> {
  return new Promise((resolve) => {
    provider.once('close', () => resolve());
  });
}

function processExitWithin(processExit: Promise<void>, timeoutMs: number): Promise<boolean> {
  return new Promise((resolve) => {
    const timeout = setTimeout(() => resolve(false), timeoutMs);
    void processExit.then(() => {
      clearTimeout(timeout);
      resolve(true);
    });
  });
}

export async function terminateLocalArtifactProviderProcess(
  provider: childProcess.ChildProcess,
  processExit: Promise<void>
): Promise<void> {
  if (await processExitWithin(processExit, ProviderGracefulExitTimeoutMs)) return;
  if (provider.exitCode == null) {
    provider.kill();
  }
  if (!(await processExitWithin(processExit, ProviderTerminationTimeoutMs))) {
    throw new Error('local artifact provider process did not terminate within the bound');
  }
}
