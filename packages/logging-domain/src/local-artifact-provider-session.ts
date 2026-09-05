import childProcess from 'node:child_process';
import crypto from 'node:crypto';
import path from 'node:path';
import { type Worker } from 'node:worker_threads';
import { LocalArtifactProviderError } from './local-artifact-provider-error';
import {
  LocalArtifactProviderProtocolVersion,
  MaximumProviderFrameBytes,
  operationName,
  type LocalArtifactProviderIdentity,
  type LocalArtifactProviderOperation,
  type LocalArtifactProviderRequest,
  type LocalArtifactProviderResponse,
} from './local-artifact-provider-protocol';
import { parseProviderReady, parseProviderResponse } from './local-artifact-provider-protocol-parse';
import { parseBooleanResult } from './local-artifact-provider-protocol-results';
import {
  encodeProviderFrame,
  parseProviderJsonFrame,
  prepareProviderSharedRequest,
  waitForProviderWorkerFrame,
} from './local-artifact-provider-session-client-frame';
import {
  createLocalArtifactProviderWorker,
  localArtifactProviderPipeName,
  observeLocalArtifactProviderProcessExit,
} from './local-artifact-provider-session-process';
import { terminateLocalArtifactProviderSessionResources } from './local-artifact-provider-session-termination';

const ProviderStartupTimeoutMs = 10_000;
const ProviderRequestTimeoutMs = 15_000;
const SharedHeaderBytes = 8;

export interface LocalArtifactProviderSession {
  readonly rootDir: string;
  readonly pipeName: string;
  readonly worker: Worker;
  readonly sharedBuffer: SharedArrayBuffer;
  readonly process: childProcess.ChildProcess;
  readonly processExit: Promise<void>;
  readonly providerInstanceId: string;
  readonly binarySha256: string;
  readonly rootIdentity: LocalArtifactProviderIdentity;
  leaseId: string | null;
  closed: boolean;
  termination: Promise<void> | null;
}

export interface LocalArtifactProviderPreparedRequest {
  readonly requestId: string;
  readonly nonce: string;
  readonly operation: LocalArtifactProviderOperation;
}

export function startLocalArtifactProviderSession(
  rootDir: string,
  binary: string,
  workingDirectory: string
): LocalArtifactProviderSession {
  if (process.platform !== 'win32') {
    throw new LocalArtifactProviderError(
      'unsupported-provider',
      'local artifact provider is available only on Windows'
    );
  }
  const resolvedRoot = path.resolve(rootDir);
  const name = localArtifactProviderPipeName();
  const provider = childProcess.spawn(
    binary,
    ['--pipe-name', name, '--root', resolvedRoot, '--parent-pid', String(process.pid)],
    { cwd: workingDirectory, stdio: 'ignore', windowsHide: true }
  );
  const processExit = observeLocalArtifactProviderProcessExit(provider);
  provider.on('error', () => {
    // The worker-owned startup deadline turns spawn failure into a bounded terminal error.
  });
  provider.unref();
  const sharedBuffer = new SharedArrayBuffer(SharedHeaderBytes + MaximumProviderFrameBytes);
  const worker = createLocalArtifactProviderWorker(name, sharedBuffer);
  worker.on('error', () => {
    // The worker writes its typed terminal state into shared memory before exiting.
  });
  worker.unref();
  try {
    const ready = parseProviderReady(
      parseProviderJsonFrame(
        waitForProviderWorkerFrame(
          sharedBuffer,
          ProviderStartupTimeoutMs,
          'provider did not publish its authenticated named pipe within the bound'
        )
      )
    );
    return {
      rootDir: resolvedRoot,
      pipeName: name,
      worker,
      sharedBuffer,
      process: provider,
      processExit,
      providerInstanceId: ready.providerInstanceId,
      binarySha256: ready.binarySha256,
      rootIdentity: ready.rootIdentity,
      leaseId: null,
      closed: false,
      termination: null,
    };
  } catch (error) {
    void terminateLocalArtifactProviderSessionResources(worker, provider, processExit).catch(() => undefined);
    throw error;
  }
}

export function prepareLocalArtifactProviderRequest(
  operation: LocalArtifactProviderOperation
): LocalArtifactProviderPreparedRequest {
  return {
    requestId: crypto.randomBytes(24).toString('hex'),
    nonce: crypto.randomBytes(24).toString('hex'),
    operation,
  };
}

export function sendLocalArtifactProviderRequest(
  session: LocalArtifactProviderSession,
  prepared: LocalArtifactProviderPreparedRequest
): LocalArtifactProviderResponse {
  if (session.closed || session.process.exitCode != null) {
    throw new LocalArtifactProviderError('provider-unavailable', 'provider process is not running');
  }
  const request: LocalArtifactProviderRequest = {
    protocol_version: LocalArtifactProviderProtocolVersion,
    request_id: prepared.requestId,
    nonce: prepared.nonce,
    lease_id: session.leaseId,
    operation: prepared.operation,
  };
  const operation = operationName(prepared.operation);
  try {
    prepareProviderSharedRequest(session.sharedBuffer);
    session.worker.postMessage({ frame: encodeProviderFrame(request) });
    const response = parseProviderJsonFrame(
      waitForProviderWorkerFrame(
        session.sharedBuffer,
        ProviderRequestTimeoutMs,
        'provider did not complete its response within the bound'
      )
    );
    return parseProviderResponse(response, {
      requestId: prepared.requestId,
      operation,
      nonce: prepared.nonce,
    });
  } catch (error) {
    void closeLocalArtifactProviderSession(session).catch(() => undefined);
    throw error;
  }
}

export function shutdownLocalArtifactProviderSession(session: LocalArtifactProviderSession): void {
  const prepared = prepareLocalArtifactProviderRequest({ kind: 'shutdown' });
  const response = sendLocalArtifactProviderRequest(session, prepared);
  if (!response.ok) {
    throw new LocalArtifactProviderError(response.error.code, response.error.message);
  }
  if (!parseBooleanResult(response.result, 'shutdown', 'shutdown')) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider shutdown result is invalid');
  }
}

export function closeLocalArtifactProviderSession(session: LocalArtifactProviderSession): Promise<void> {
  if (session.termination != null) return session.termination;
  session.closed = true;
  session.termination = terminateLocalArtifactProviderSessionResources(
    session.worker,
    session.process,
    session.processExit
  );
  return session.termination;
}
