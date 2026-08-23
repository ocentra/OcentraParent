import { normalizeGeneratedBridgeEndpoint } from '../parent-log-runtime';

export interface BridgeHttpResult {
  readonly ok: boolean;
  readonly status: number;
  readonly body: Record<string, unknown> | null;
}

const DefaultBridgeTimeoutMs = 5_000;
const MaximumBridgeResponseBytes = 64 * 1024;
const BridgeProtocols = new Set(['http:', 'https:']);

export function bridgeEndpoint(endpoint: string): string {
  const normalized = normalizeGeneratedBridgeEndpoint(endpoint.trim());
  try {
    const url = new URL(normalized);
    const valid = [
      BridgeProtocols.has(url.protocol),
      url.username.length === 0,
      url.password.length === 0,
      url.pathname === '/',
      url.search.length === 0,
      url.hash.length === 0,
    ].every(Boolean);
    if (!valid) {
      throw new Error('invalid log bridge endpoint');
    }
  } catch {
    throw new Error('invalid log bridge endpoint');
  }
  return normalized;
}

async function readBoundedResponseBody(response: Response): Promise<string> {
  if (response.body == null) {
    return '';
  }
  const reader = response.body.getReader();
  const decoder = new TextDecoder();
  let body = '';
  let bytes = 0;
  try {
    while (true) {
      const chunk = await reader.read();
      if (chunk.done) {
        return body + decoder.decode();
      }
      bytes += chunk.value.byteLength;
      if (bytes > MaximumBridgeResponseBytes) {
        throw new Error('log bridge response exceeds its custody limit');
      }
      body += decoder.decode(chunk.value, { stream: true });
    }
  } catch (error) {
    await reader.cancel().catch(() => undefined);
    throw error;
  } finally {
    reader.releaseLock();
  }
}

function parseBridgeObject(body: string): Record<string, unknown> | null {
  try {
    const value = JSON.parse(body) as unknown;
    return typeof value === 'object' && value != null && !Array.isArray(value)
      ? (value as Record<string, unknown>)
      : null;
  } catch {
    return null;
  }
}

export async function requestBridgeObject(
  input: string,
  init: RequestInit,
  timeoutMs = DefaultBridgeTimeoutMs
): Promise<BridgeHttpResult> {
  const resolvedTimeout = Number.isSafeInteger(timeoutMs) && timeoutMs > 0 ? timeoutMs : DefaultBridgeTimeoutMs;
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), resolvedTimeout);
  try {
    const response = await fetch(input, { ...init, signal: controller.signal });
    const body = parseBridgeObject(await readBoundedResponseBody(response));
    return { ok: response.ok, status: response.status, body };
  } finally {
    clearTimeout(timeout);
  }
}
