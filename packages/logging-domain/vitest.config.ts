import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vitest/config';

const packageRoot = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root: packageRoot,
  test: {
    include: ['tests/unit/**/*.test.ts'],
    exclude: ['../../test-results/**', 'dist/**', 'node_modules/**'],
    testTimeout: 15000,
    hookTimeout: 15000,
  },
});
