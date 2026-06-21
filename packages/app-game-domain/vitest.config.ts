import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globalSetup: ['./tests/unit/app-game-timer-service-proof-artifacts.setup.ts'],
  },
});
