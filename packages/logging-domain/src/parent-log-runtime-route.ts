/* generated from crates/logging-core/src/parent_log_runtime.rs */

export function normalizeGeneratedBridgeEndpoint(endpoint: string): string {
  return endpoint.endsWith('/') ? endpoint.slice(0, -1) : endpoint;
}

export function resolveGeneratedBridgeRoute(
  method: string,
  pathname: string
): 'health' | 'run-info' | 'run-started' | 'logs' | 'flush' | 'not-found' {
  switch (pathname) {
    case '/__health__':
      return method === 'GET' ? 'health' : 'not-found';
    case '/__run_info__':
      return method === 'GET' ? 'run-info' : 'not-found';
    case '/__run_started__':
      return method === 'POST' ? 'run-started' : 'not-found';
    case '/__logs__':
      return method === 'POST' ? 'logs' : 'not-found';
    case '/__flush__':
      return method === 'GET' || method === 'POST' ? 'flush' : 'not-found';
    default:
      return 'not-found';
  }
}
