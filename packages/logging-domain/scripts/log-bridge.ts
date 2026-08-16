#!/usr/bin/env node

import { createBridgeServer } from '../src/transport/bridgeServer';

const port = Number.parseInt(process.env.OCENTRA_PARENT_LOG_BRIDGE_PORT ?? '4479', 10);
const host = process.env.OCENTRA_PARENT_LOG_BRIDGE_HOST ?? '127.0.0.1';

const server = createBridgeServer({
  host,
  port,
  rootDir: process.env.OCENTRA_PARENT_LOG_DIR,
});

server.listen(port, host, () => {
  process.stdout.write(`Logging bridge listening on http://${host}:${port}\n`);
});
