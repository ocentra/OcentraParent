import type { BridgeMode } from './logConfig';

const BridgeModes = new Set<BridgeMode>(['local', 'tunnel', 'disabled']);
const BridgeProtocols = new Set(['http:', 'https:']);
const LoopbackHosts = new Set(['localhost', '127.0.0.1', '[::1]']);

export function resolveBridgeMode(value: string | undefined, generatedMode: BridgeMode): BridgeMode {
  if (value == null || value.trim().length === 0) {
    return generatedMode;
  }
  const normalized = value.trim().toLowerCase() as BridgeMode;
  return BridgeModes.has(normalized) ? normalized : 'disabled';
}

function validBridgeUrl(value: string, loopbackOnly: boolean): string | null {
  try {
    const url = new URL(value);
    const valid = [
      BridgeProtocols.has(url.protocol),
      url.username.length === 0,
      url.password.length === 0,
      url.pathname === '/',
      url.search.length === 0,
      url.hash.length === 0,
      !loopbackOnly || LoopbackHosts.has(url.hostname.toLowerCase()),
    ].every(Boolean);
    return valid ? value.trim() : null;
  } catch {
    return null;
  }
}

export function resolveBridgeUrl(
  mode: BridgeMode,
  configuredValue: string | undefined,
  generatedUrl: string | null
): string | null {
  if (mode === 'disabled') {
    return null;
  }
  const configured = configuredValue?.trim();
  if (mode === 'tunnel') {
    return configured == null || configured.length === 0 ? null : validBridgeUrl(configured, false);
  }
  const candidate = configured != null && configured.length > 0 ? configured : generatedUrl;
  return candidate == null ? null : validBridgeUrl(candidate, true);
}
