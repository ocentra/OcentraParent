import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

import { DevLogField, DevLogMessage } from '@ocentra-parent/logging-domain/contracts';

import { setupDevLogMiddleware, writeDevServerLog } from '../../scripts/dev/dev-log-writer.mjs';

function resolveConfiguredPort(server) {
  return server.config.server.port ?? server.config.preview.port ?? 0;
}

export default defineConfig({
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
