#!/usr/bin/env node

import { createBridgeServer } from '../src/transport/bridgeServer';
import { createParentLogConfig } from '../src/core/logConfig';

function parsePort(value: string | undefined): number {
  const normalized = value?.trim() ?? '4479';
  if (!/^\d+$/u.test(normalized)) {
    throw new Error('invalid log bridge port');
  }
  const port = Number(normalized);
  if (!Number.isSafeInteger(port) || port < 0 || port > 65_535) {
    throw new Error('invalid log bridge port');
  }
  return port;
}

function parseHost(value: string | undefined, localOnly: boolean): string {
  const host = value?.trim() ?? '127.0.0.1';
  if (host.length === 0 || /[\s/\\]/u.test(host)) {
    throw new Error('invalid log bridge host');
  }
  if (localOnly && host !== 'localhost' && host !== '127.0.0.1' && host !== '::1') {
    throw new Error('local log bridge must bind to loopback');
  }
  return host;
}

function assertLocalEndpointMatchesPort(bridgeMode: string, bridgeUrl: string, port: number): void {
  if (bridgeMode !== 'local' || port === 0) {
    return;
  }
  const endpoint = new URL(bridgeUrl);
  const endpointPort = Number(endpoint.port.length > 0 ? endpoint.port : '80');
  if (endpoint.protocol !== 'http:' || endpointPort !== port) {
    throw new Error('local log bridge URL must match the listening port');
  }
}

const config = createParentLogConfig();
if (config.bridgeMode === 'disabled' || config.bridgeUrl == null) {
  throw new Error('log bridge is disabled or incompletely configured');
}
const port = parsePort(process.env.OCENTRA_PARENT_LOG_BRIDGE_PORT);
const host = parseHost(process.env.OCENTRA_PARENT_LOG_BRIDGE_HOST, config.bridgeMode === 'local');
assertLocalEndpointMatchesPort(config.bridgeMode, config.bridgeUrl, port);

const server = createBridgeServer({
  host,
  port,
  rootDir: process.env.OCENTRA_PARENT_LOG_DIR,
});

server.listen(port, host, () => {
  const address = server.address();
  if (address == null || typeof address === 'string') {
    throw new Error('log bridge did not expose a TCP listening address');
  }
  const displayHost = address.address.includes(':') ? `[${address.address}]` : address.address;
  process.stdout.write(`Logging bridge listening on http://${displayHost}:${address.port}\n`);
});
