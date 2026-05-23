import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname } from 'node:path';
import { fileURLToPath, URL } from 'node:url';
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

import { DevLogField, DevLogMessage } from '@ocentra-parent/logging-domain/contracts';

import { setupDevLogMiddleware, writeDevServerLog } from '../../scripts/dev/dev-log-writer.mjs';

function resolveConfiguredPort(server) {
  return server.config.server.port ?? server.config.preview.port ?? 0;
}

function vendorModule(relativePath) {
  return fileURLToPath(new URL(relativePath, import.meta.url));
}

const savedFrameLayoutFile = fileURLToPath(new URL('./public/portal-frame-layout.json', import.meta.url));
const draftFrameLayoutFile = fileURLToPath(new URL('../../.logs/portal-frame-layout-draft.json', import.meta.url));

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
    await handleFrameLayoutRequest(request, response, savedFrameLayoutFile, [
      savedFrameLayoutFile,
      draftFrameLayoutFile,
    ]);
  });
  middlewares.use('/__ocentra-parent/frame-layout', async (request, response) => {
    await handleFrameLayoutRequest(
      request,
      response,
      draftFrameLayoutFile,
      [draftFrameLayoutFile],
      savedFrameLayoutFile
    );
  });
}

async function handleFrameLayoutRequest(request, response, readPath, writePaths, fallbackReadPath) {
  try {
    if (request.method === 'GET') {
      const json = await readFile(readPath, 'utf8').catch(() =>
        fallbackReadPath === undefined ? '{}' : readFile(fallbackReadPath, 'utf8').catch(() => '{}')
      );
      response.setHeader('Content-Type', 'application/json');
      response.end(json);
      return;
    }
    if (request.method === 'PUT') {
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
      return;
    }
    response.statusCode = 405;
    response.end();
  } catch {
    response.statusCode = 400;
    response.end();
  }
}

export default defineConfig({
  resolve: {
    alias: {
      '@ocentra/app-assets/auth': vendorModule('../../vendor/ocentra-games-core-ui/shims/ocentra-app-assets-auth.ts'),
      '@ocentra/app-assets/avatars': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-app-assets-avatars.ts'
      ),
      '@ocentra/app-assets/commons': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-app-assets-commons.ts'
      ),
      '@ocentra/app-assets/banners': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-app-assets-banners.ts'
      ),
      '@ocentra/app-assets/cardgame': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-app-assets-cardgame.ts'
      ),
      '@ocentra/app-assets/placeholders': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-app-assets-placeholders.ts'
      ),
      '@ocentra/app-assets/shop-page': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-app-assets-shop-page.ts'
      ),
      '@ocentra/endpoint-domain/constants/local': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-local-endpoints.ts'
      ),
      '@ocentra/endpoint-domain/constants/public-routes': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-public-routes.ts'
      ),
      '@ocentra/schema-domain/effect-builder': vendorModule(
        '../../vendor/ocentra-games-core-ui/schema-domain/effect-builder.ts'
      ),
      '@ocentra/game-asset-domain/schemas/leaderboard-page-content-schema': vendorModule(
        '../../vendor/ocentra-games-core-ui/game-asset-domain/schemas/leaderboard-page-content-schema.ts'
      ),
      '@tauri-apps/api/core': vendorModule('../../vendor/ocentra-games-core-ui/shims/tauri-core.ts'),
      'react-router-dom': vendorModule('../../vendor/ocentra-games-core-ui/shims/react-router-dom.ts'),
    },
  },
  plugins: [
    react(),
    {
      name: 'ocentra-parent-dev-logs',
      async configureServer(server) {
        setupDevLogMiddleware(server.middlewares);
        setupFrameLayoutMiddleware(server.middlewares);
        await writeDevServerLog(DevLogMessage.DevServerStarted, {
          [DevLogField.Port]: resolveConfiguredPort(server),
        });
      },
      async configurePreviewServer(server) {
        setupDevLogMiddleware(server.middlewares);
        setupFrameLayoutMiddleware(server.middlewares);
        await writeDevServerLog(DevLogMessage.DevServerStarted, {
          [DevLogField.Port]: resolveConfiguredPort(server),
        });
      },
    },
  ],
});
