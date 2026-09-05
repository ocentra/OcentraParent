export const PortalAgentHealthPreflightMode = Object.freeze({
  Authenticated: 'authenticated',
  DegradedUnavailable: 'degraded-unavailable',
});

export function resolvePortalAgentHealthPreflight(health) {
  if (!isRecord(health)) {
    throw new Error('Portal E2E Agent health must be an object.');
  }

  if (
    health.state === 'ready' &&
    health.transport === 'websocket' &&
    health.authenticationState === 'authenticated' &&
    health.reason === 'ready'
  ) {
    return PortalAgentHealthPreflightMode.Authenticated;
  }

  if (
    health.state === 'degraded' &&
    health.transport === null &&
    health.authenticationState === 'unavailable' &&
    health.reason === 'route-dependency-unavailable'
  ) {
    return PortalAgentHealthPreflightMode.DegradedUnavailable;
  }

  throw new Error(
    `Portal E2E Agent health is neither authenticated-ready nor the known fail-closed degraded state: ${JSON.stringify(health)}`
  );
}

function isRecord(value) {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
