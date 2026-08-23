import { BridgeEntryArraySchema, type BridgeEntry } from './bridgeLogPayload';
import { RunType } from '../test-log/types';
import { createParentLogConfig } from '../core/logConfig';
import { buildGeneratedRunStartedPayload } from '../parent-log-runtime';
import { bridgeEndpoint, fetchBridge, parseBridgeObject } from './bridgeTransportHttp';

export interface BridgeRunInfo {
  readonly runId: string;
  readonly runType: string;
  readonly suiteType: string | null;
  readonly scope: string | null;
  readonly startedAt: number | null;
}

export interface BridgeRunStartedPayload {
  readonly runId: string;
  readonly runType?: string;
  readonly suiteType?: string;
  readonly scope?: string;
  readonly filePath?: string;
  readonly wipeAll?: boolean;
}

export interface BridgeSendOptions {
  readonly skipHealthCheck?: boolean;
  readonly timeoutMs?: number;
}

export async function sendToBridge(
  entries: readonly BridgeEntry[],
  endpoint: string,
  options: BridgeSendOptions = {}
): Promise<void> {
  if (entries.length === 0) {
    return;
  }

  BridgeEntryArraySchema.parse(entries);

  const normalized = bridgeEndpoint(endpoint);
  if (options.skipHealthCheck !== true) {
    const healthResponse = await fetchBridge(`${normalized}/__health__`, { method: 'GET' }, options.timeoutMs);
    if (!healthResponse.ok) {
      throw new Error(`Log bridge health check failed: ${healthResponse.status}`);
    }
    if ((await parseBridgeObject(healthResponse))?.['ok'] !== true) {
      throw new Error('Log bridge health check returned an invalid response');
    }
  }

  const response = await fetchBridge(
    `${normalized}/__logs__`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(entries),
    },
    options.timeoutMs
  );

  if (!response.ok) {
    throw new Error(`Log bridge POST failed: ${response.status}`);
  }
  const body = await parseBridgeObject(response);
  if (body?.['ok'] !== true || body['stored'] !== entries.length) {
    throw new Error('Log bridge did not confirm the complete batch');
  }
}

export async function fetchRunInfoFromBridge(endpoint: string): Promise<BridgeRunInfo | null> {
  const normalized = bridgeEndpoint(endpoint);
  const response = await fetchBridge(`${normalized}/__run_info__`, { method: 'GET' });
  if (!response.ok) {
    return null;
  }

  const body = (await response.json()) as {
    ok?: boolean;
    runId?: string;
    runType?: string;
    suiteType?: string | null;
    scope?: string | null;
    startedAt?: number | null;
  };

  if (body.ok !== true || typeof body.runId !== 'string' || typeof body.runType !== 'string') {
    return null;
  }

  return {
    runId: body.runId,
    runType: body.runType,
    suiteType: body.suiteType ?? null,
    scope: body.scope ?? null,
    startedAt: body.startedAt ?? null,
  };
}

export async function notifyBridgeRunStarted(endpoint: string, payload: BridgeRunStartedPayload): Promise<boolean> {
  const normalized = bridgeEndpoint(endpoint);
  const response = await fetchBridge(`${normalized}/__run_started__`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(buildGeneratedRunStartedPayload({ ...payload, runType: payload.runType ?? RunType.Single })),
  });

  const body = response.ok ? await parseBridgeObject(response) : null;
  return body?.['ok'] === true;
}

export async function flushBridgeRun(endpoint: string, runId: string): Promise<boolean> {
  const normalized = bridgeEndpoint(endpoint);
  const response = await fetchBridge(`${normalized}/__flush__`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ runId }),
  });

  const body = response.ok ? await parseBridgeObject(response) : null;
  return (
    body?.['ok'] === true &&
    body['runId'] === runId &&
    Number.isSafeInteger(body['flushed']) &&
    (body['flushed'] as number) >= 0 &&
    Number.isSafeInteger(body['stored']) &&
    (body['stored'] as number) >= (body['flushed'] as number)
  );
}

export function resolveBridgeEndpoint(env?: NodeJS.ProcessEnv | Record<string, string | undefined>): string | null {
  return createParentLogConfig(env).bridgeUrl;
}

export class BridgeTransport {
  readonly name = 'bridge';
  private readonly defaultEndpoint: string | undefined;
  private readonly skipHealthCheck: boolean;

  constructor(defaultEndpoint?: string, skipHealthCheck = false) {
    this.defaultEndpoint = defaultEndpoint;
    this.skipHealthCheck = skipHealthCheck;
  }

  async emit(entries: readonly BridgeEntry[], endpoint?: string): Promise<void> {
    const target = endpoint ?? this.defaultEndpoint;
    if (target == null || target.trim().length === 0) {
      throw new Error('log bridge endpoint is required');
    }
    await sendToBridge(entries, target, { skipHealthCheck: this.skipHealthCheck });
  }
}
