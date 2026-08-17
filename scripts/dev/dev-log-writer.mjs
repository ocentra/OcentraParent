import { appendFile, mkdir } from 'node:fs/promises';
import { randomUUID } from 'node:crypto';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { DevLogEntrySchema } from '@ocentra-parent/logging-domain/logging-contracts';
import {
  GeneratedDevLogEndpoint as DevLogEndpoint,
  GeneratedDevLogEnvironment as DevLogEnvironment,
  GeneratedDevLogFile as DevLogFile,
  GeneratedDevLogIdPrefix as DevLogIdPrefix,
  GeneratedLogLevel as LogLevel,
  GeneratedLogSource as LogSource,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';

export const DevLogField = {
  Port: 'port',
};

export const DevLogMessage = {
  DevServerStarted: 'Vite dev server started.',
};

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const requestBodyLimitBytes = 1024 * 256;
const sensitiveFieldPattern =
  /(authorization|clipboard|cookie|keystroke|password|screenshot|secret|token|url|child.?name|account.?name|full.?name|command.?line)/iu;
const redactedFieldValue = '[REDACTED]';

export async function appendDevLog(entry) {
  const parsed = DevLogEntrySchema.parse(entry);
  const sanitized = {
    ...parsed,
    fields: redactDevLogFields(parsed.fields),
  };
  const filePath = resolveDevLogFile(parsed.source);
  await mkdir(dirname(filePath), { recursive: true });
  await appendFile(filePath, `${JSON.stringify(sanitized)}\n`, 'utf8');
}

function redactDevLogFields(fields) {
  return Object.fromEntries(
    Object.entries(fields).map(([key, value]) => [key, sensitiveFieldPattern.test(key) ? redactedFieldValue : value])
  );
}

export async function writeDevServerLog(message, fields = {}) {
  await appendDevLog({
    schemaVersion: 1,
    id: `${DevLogIdPrefix.DevServer}${randomUUID()}`,
    timestamp: new Date().toISOString(),
    level: LogLevel.Info,
    source: LogSource.DevServer,
    message,
    fields,
  });
}

export function setupDevLogMiddleware(middlewares) {
  middlewares.use(DevLogEndpoint.Write, async (request, response) => {
    if (request.method !== 'POST') {
      response.statusCode = 405;
      response.end();
      return;
    }

    try {
      const body = await readBody(request);
      await appendDevLog(JSON.parse(body));
      response.statusCode = 204;
      response.end();
    } catch {
      response.statusCode = 400;
      response.end();
    }
  });
}

function resolveDevLogFile(source) {
  const directory = process.env[DevLogEnvironment.Directory] ?? join(repoRoot, '.logs', DevLogFile.DirectoryName);
  const day = new Date().toISOString().slice(0, 10);
  return join(directory, `${source}-${day}.${DevLogFile.Extension}`);
}

function readBody(request) {
  return new Promise((resolveBody, rejectBody) => {
    const chunks = [];
    let size = 0;

    request.on('data', (chunk) => {
      size += chunk.length;
      if (size > requestBodyLimitBytes) {
        rejectBody(new Error('dev log request body too large'));
        request.destroy();
        return;
      }
      chunks.push(chunk);
    });

    request.on('end', () => {
      resolveBody(Buffer.concat(chunks).toString('utf8'));
    });
    request.on('error', rejectBody);
  });
}
