import type http from 'node:http';

const MaximumBridgeRequestBytes = 1024 * 1024;
const LoopbackHosts = new Set(['localhost', '127.0.0.1', '[::1]']);

export function isBridgeLoopbackAddress(value: string | undefined): boolean {
  if (value == null) {
    return false;
  }
  const normalized = value.toLowerCase();
  return (
    normalized === 'localhost' ||
    normalized === '127.0.0.1' ||
    normalized === '::1' ||
    normalized === '::ffff:127.0.0.1'
  );
}

export function isBridgeLoopbackRequest(request: http.IncomingMessage): boolean {
  return isBridgeLoopbackAddress(request.socket.localAddress) && isBridgeLoopbackAddress(request.socket.remoteAddress);
}

function loopbackOrigin(value: string): string | null {
  try {
    const origin = new URL(value);
    return LoopbackHosts.has(origin.hostname.toLowerCase()) ? origin.origin : null;
  } catch {
    return null;
  }
}

export function applyBridgeCorsHeaders(request: http.IncomingMessage, response: http.ServerResponse): void {
  const origin = typeof request.headers.origin === 'string' ? loopbackOrigin(request.headers.origin) : null;
  if (origin != null) {
    response.setHeader('Access-Control-Allow-Origin', origin);
    response.setHeader('Vary', 'Origin');
  }
  response.setHeader('Access-Control-Allow-Methods', 'GET,POST,OPTIONS');
  response.setHeader('Access-Control-Allow-Headers', 'Content-Type');
}

export function hasBridgeJsonContentType(request: http.IncomingMessage): boolean {
  return request.headers['content-type']?.split(';', 1)[0]?.trim().toLowerCase() === 'application/json';
}

export function sendBridgeJson(response: http.ServerResponse, statusCode: number, body: object): void {
  response.statusCode = statusCode;
  response.setHeader('Content-Type', 'application/json');
  response.end(JSON.stringify(body));
}

export function readBridgeRequestBody(request: http.IncomingMessage): Promise<string> {
  return new Promise((resolve, reject) => {
    let body = '';
    let byteLength = 0;
    let oversized = false;
    request.setEncoding('utf8');
    request.on('data', (chunk: string) => {
      byteLength += Buffer.byteLength(chunk, 'utf8');
      if (byteLength > MaximumBridgeRequestBytes) {
        oversized = true;
        body = '';
        return;
      }
      if (!oversized) {
        body += chunk;
      }
    });
    request.on('end', () => {
      if (oversized) {
        reject(new Error('bridge request body is too large'));
        return;
      }
      resolve(body);
    });
    request.on('error', reject);
  });
}
