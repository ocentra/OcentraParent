import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname } from 'node:path';
import { fileURLToPath, URL } from 'node:url';
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import {
  DevLogField,
  DevLogMessage,
  setupDevLogMiddleware,
  writeDevServerLog,
} from '../../scripts/dev/dev-log-writer.mjs';

function resolveConfiguredPort(server) {
  return server.config.server.port ?? server.config.preview.port ?? 0;
}

function vendorModule(relativePath) {
  return fileURLToPath(new URL(relativePath, import.meta.url));
}

const savedFrameLayoutFile = fileURLToPath(new URL('./public/portal-frame-layout.json', import.meta.url));
const draftFrameLayoutFile = fileURLToPath(new URL('../../.logs/portal-frame-layout-draft.json', import.meta.url));
const savedBackgroundConfigFile = fileURLToPath(new URL('./public/portal-background-config.json', import.meta.url));

function readRequestBody(request) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    request.on('data', (chunk) => chunks.push(Buffer.from(chunk)));
    request.on('error', reject);
    request.on('end', () => resolve(Buffer.concat(chunks).toString('utf8')));
  });
}

function setupFrameLayoutMiddleware(middlewares) {
  middlewares.use('/__ocentra-parent/frame-layout-saved', async (request, response) => {
    await handleJsonFileRequest(request, response, savedFrameLayoutFile, [savedFrameLayoutFile, draftFrameLayoutFile]);
  });
  middlewares.use('/__ocentra-parent/frame-layout', async (request, response) => {
    await handleJsonFileRequest(request, response, draftFrameLayoutFile, [draftFrameLayoutFile], savedFrameLayoutFile);
  });
}

function setupBackgroundConfigMiddleware(middlewares) {
  middlewares.use('/__ocentra-parent/background-config', async (request, response) => {
    await handleJsonFileRequest(request, response, savedBackgroundConfigFile, [savedBackgroundConfigFile]);
  });
}

async function handleJsonFileRequest(request, response, readPath, writePaths, defaultReadPath) {
  try {
    if (request.method === 'GET') {
      await handleJsonGetRequest(response, readPath, defaultReadPath);
      return;
    }
    if (request.method === 'PUT') {
      await handleJsonPutRequest(request, response, writePaths);
      return;
    }
    handleJsonMethodNotAllowed(response);
  } catch {
    handleJsonBadRequest(response);
  }
}

async function handleJsonGetRequest(response, readPath, defaultReadPath) {
  const json = await readFile(readPath, 'utf8').catch(async () => {
    if (defaultReadPath === undefined) {
      return '{}';
    }
    return readFile(defaultReadPath, 'utf8').catch(() => '{}');
  });
  response.setHeader('Content-Type', 'application/json');
  response.end(json);
}

async function handleJsonPutRequest(request, response, writePaths) {
  const body = await readRequestBody(request);
  JSON.parse(body);
  await Promise.all(
    writePaths.map(async (filePath) => {
      await mkdir(dirname(filePath), { recursive: true });
      await writeFile(filePath, `${body}\n`, 'utf8');
    })
  );
  response.setHeader('Content-Type', 'application/json');
  response.end(JSON.stringify({ ok: true }));
}

function handleJsonMethodNotAllowed(response) {
  response.statusCode = 405;
  response.end();
}

function handleJsonBadRequest(response) {
  response.statusCode = 400;
  response.end();
}

export default defineConfig({
  resolve: {
    alias: {
      '@ocentra-parent/portal-assets/auth': vendorModule(
        '../../vendor/ocentra-parent-core-ui/shims/parent-portal-assets-auth.ts'
      ),
      '@ocentra-parent/portal-assets/avatars': vendorModule(
        '../../vendor/ocentra-parent-core-ui/shims/parent-portal-assets-avatars.ts'
      ),
      '@ocentra-parent/portal-assets/common': vendorModule(
        '../../vendor/ocentra-parent-core-ui/shims/parent-portal-assets-common.ts'
      ),
      '@ocentra-parent/vendor-schema/effect-builder': vendorModule(
        '../../vendor/ocentra-parent-core-ui/shims/effect-builder.ts'
      ),
      '@ocentra-parent/portal-content-schema': vendorModule(
        '../../vendor/ocentra-parent-core-ui/parent-portal-content-domain/schemas/parent-portal-page-content-schema.ts'
      ),
    },
  },
  plugins: [
    react(),
    {
      name: 'ocentra-parent-dev-logs',
      async configureServer(server) {
        setupDevLogMiddleware(server.middlewares);
        setupFrameLayoutMiddleware(server.middlewares);
        setupBackgroundConfigMiddleware(server.middlewares);
        await writeDevServerLog(DevLogMessage.DevServerStarted, {
          [DevLogField.Port]: resolveConfiguredPort(server),
        });
      },
      async configurePreviewServer(server) {
        setupDevLogMiddleware(server.middlewares);
        setupFrameLayoutMiddleware(server.middlewares);
        setupBackgroundConfigMiddleware(server.middlewares);
        await writeDevServerLog(DevLogMessage.DevServerStarted, {
          [DevLogField.Port]: resolveConfiguredPort(server),
        });
      },
    },
  ],
});
