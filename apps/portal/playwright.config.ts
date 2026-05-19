import { defineConfig, devices } from '@playwright/test';

const portalUrl = 'http://127.0.0.1:4490/#/commands';

export default defineConfig({
  expect: {
    timeout: 10000,
  },
  fullyParallel: false,
  outputDir: '../../test-results/portal-playwright',
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  reporter: process.env['CI']
    ? [['list'], ['html', { open: 'never', outputFolder: '../../playwright-report/portal' }]]
    : 'list',
  testDir: './e2e',
  timeout: 30000,
  use: {
    baseURL: 'http://127.0.0.1:4490',
    screenshot: 'only-on-failure',
    trace: 'retain-on-failure',
  },
  webServer: {
    command: 'node ../../scripts/test/portal-playwright-server.mjs',
    reuseExistingServer: false,
    timeout: 60000,
    url: portalUrl,
  },
});
