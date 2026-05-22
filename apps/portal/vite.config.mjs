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
      '@ocentra/endpoint-domain/constants/local': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-local-endpoints.ts'
      ),
      '@ocentra/endpoint-domain/constants/public-routes': vendorModule(
        '../../vendor/ocentra-games-core-ui/shims/ocentra-public-routes.ts'
      ),
      '@ocentra/schema-domain/effect-builder': vendorModule(
        '../../vendor/ocentra-games-core-ui/schema-domain/effect-builder.ts'
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
        await writeDevServerLog(DevLogMessage.DevServerStarted, {
          [DevLogField.Port]: resolveConfiguredPort(server),
        });
      },
      async configurePreviewServer(server) {
        setupDevLogMiddleware(server.middlewares);
        await writeDevServerLog(DevLogMessage.DevServerStarted, {
          [DevLogField.Port]: resolveConfiguredPort(server),
        });
      },
    },
  ],
});
