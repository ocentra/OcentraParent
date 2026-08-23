import { normalizeGeneratedBridgeEndpoint } from '../parent-log-runtime';

const DefaultBridgeTimeoutMs = 5_000;
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

export async function fetchBridge(
  input: string,
  init: RequestInit,
  timeoutMs = DefaultBridgeTimeoutMs
): Promise<Response> {
  const resolvedTimeout = Number.isSafeInteger(timeoutMs) && timeoutMs > 0 ? timeoutMs : DefaultBridgeTimeoutMs;
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), resolvedTimeout);
  try {
    return await fetch(input, { ...init, signal: controller.signal });
  } finally {
    clearTimeout(timeout);
  }
}

export async function parseBridgeObject(response: Response): Promise<Record<string, unknown> | null> {
  try {
    const body = (await response.json()) as unknown;
    return typeof body === 'object' && body != null && !Array.isArray(body) ? (body as Record<string, unknown>) : null;
  } catch {
    return null;
  }
}
