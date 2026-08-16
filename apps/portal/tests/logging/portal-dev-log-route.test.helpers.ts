import { createServer, type IncomingMessage, type ServerResponse } from 'node:http';
import { expect } from 'vitest';
import { GeneratedDevLogEndpoint as DevLogEndpoint } from '@ocentra-parent/logging-domain/generated/logging-contracts';

export function createBridgeWriteServer(): {
  readonly server: ReturnType<typeof createServer>;
  readonly readBody: () => string;
  readonly healthChecks: () => number;
} {
  let healthChecks = 0;
  let receivedBody = '';
  const server = createServer((request: IncomingMessage, response: ServerResponse) => {
    if (request.url === '/__health__') {
      healthChecks += 1;
      response.statusCode = 200;
      response.end(JSON.stringify({ ok: true }));
      return;
    }
    if (request.url === '/__logs__') {
      appendRequestBody(
        request,
        (chunk) => {
          receivedBody += chunk;
        },
        () => {
          response.statusCode = 200;
          response.end(JSON.stringify({ ok: true }));
        }
      );
      return;
    }
    response.statusCode = 404;
    response.end();
  });
  return { server, readBody: () => receivedBody, healthChecks: () => healthChecks };
}

export function createFallbackWriteServer(): {
  readonly server: ReturnType<typeof createServer>;
  readonly readBody: () => string;
} {
  let receivedBody = '';
  const server = createServer((request: IncomingMessage, response: ServerResponse) => {
    if (request.url === DevLogEndpoint.Write) {
      appendRequestBody(
        request,
        (chunk) => {
          receivedBody += chunk;
        },
        () => {
          response.statusCode = 204;
          response.end();
        }
      );
      return;
    }
    response.statusCode = 404;
    response.end();
  });
  return { server, readBody: () => receivedBody };
}

export async function listenOnLoopback(server: ReturnType<typeof createServer>): Promise<void> {
  await new Promise<void>((resolve) => {
    server.listen(0, '127.0.0.1', () => resolve());
  });
}

export async function closeServers(servers: Array<ReturnType<typeof createServer>>): Promise<void> {
  const activeServers = servers.splice(0, servers.length);
  await Promise.all(
    activeServers.map(
      (server) =>
        new Promise<void>((resolve, reject) => {
          server.close((error) => {
            if (error != null) {
              reject(error);
              return;
            }
            resolve();
          });
        })
    )
  );
}

export function assertBridgeCompatiblePortalRows(
  payload: Array<{
    consumer: string;
    testName: string;
    log: {
      source: string;
      message: string;
      context: string;
      data: string | null;
      file: string | null;
      file_path: string | null;
    };
  }>
): void {
  expect(payload).toHaveLength(1);
  expect(payload[0]).toMatchObject({
    consumer: 'parent-portal',
    testName: 'portal-dev-runtime',
    log: {
      source: 'DevLogger',
      message: 'Portal dev runtime started.',
      context: 'DevLogger.sendPortalDevLog',
      file: 'dev-logger.ts',
    },
  });
  expect(payload[0]?.log.file_path).toContain('dev-logger.ts');
  expect(JSON.parse(payload[0]?.log.data ?? '{}')).toMatchObject({
    agentWebSocketUrl: 'ws://127.0.0.1:4477/api/dev/ws',
  });
}

function appendRequestBody(request: IncomingMessage, onChunk: (chunk: string) => void, onEnd: () => void): void {
  request.setEncoding('utf8');
  request.on('data', onChunk);
  request.on('end', onEnd);
}
