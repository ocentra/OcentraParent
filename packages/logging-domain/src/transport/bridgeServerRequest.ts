import type http from 'node:http';
import {
  RunType,
  TestLogScope,
  parseRunTypeOrDefault,
  parseSuiteTypeOrNull,
  parseTestLogScopeOrDefault,
} from '../test-log/types';
import type { BridgeRunStartState } from './bridgeLifecycleStateCodec';
import { readBridgeRequestBody } from './bridgeHttp';

function requestObject(rawBody: string): Record<string, unknown> {
  const value = rawBody.trim().length === 0 ? {} : (JSON.parse(rawBody) as unknown);
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid bridge request');
  }
  return value as Record<string, unknown>;
}

function requiredRunId(value: unknown): string {
  if (typeof value !== 'string' || value.trim().length === 0 || value.length > 256) {
    throw new Error('invalid bridge run id');
  }
  return value;
}

function optionalString(value: unknown): string | null {
  if (value == null) {
    return null;
  }
  if (typeof value !== 'string') {
    throw new Error('invalid bridge value');
  }
  return value;
}

function optionalFilePath(value: unknown): string | null {
  const filePath = optionalString(value);
  if (filePath != null && filePath.length > 4_096) {
    throw new Error('invalid bridge file selector');
  }
  return filePath;
}

export async function parseRunStartedRequest(request: http.IncomingMessage): Promise<BridgeRunStartState> {
  const payload = requestObject(await readBridgeRequestBody(request));
  if (payload['wipeAll'] != null && typeof payload['wipeAll'] !== 'boolean') {
    throw new Error('invalid bridge wipe mode');
  }
  return {
    runId: requiredRunId(payload['runId']),
    runType: parseRunTypeOrDefault(optionalString(payload['runType']), RunType.Single),
    suiteType: parseSuiteTypeOrNull(optionalString(payload['suiteType'])),
    scope: parseTestLogScopeOrDefault(optionalString(payload['scope']), TestLogScope.ParentTest),
    filePath: optionalFilePath(payload['filePath']),
    wipeAll: payload['wipeAll'] === true,
  };
}

export async function parseFlushRunId(request: http.IncomingMessage, currentRunId: string | null): Promise<string> {
  if (request.method === 'GET') {
    const requestUrl = new URL(request.url ?? '/', 'http://127.0.0.1');
    return requiredRunId(requestUrl.searchParams.get('runId') ?? currentRunId);
  }
  return requiredRunId(requestObject(await readBridgeRequestBody(request))['runId']);
}
