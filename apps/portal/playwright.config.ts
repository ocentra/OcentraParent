import { defineConfig, devices } from '@playwright/test';

const portalPort = resolvePortalPort(process.env['OCENTRA_PARENT_PORTAL_PORT']);
const runTrackingHostedUiProof = process.env['TRACKING_PLAN_HOSTED_UI_PROOF'] === '1';

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
  testDir: './tests/e2e',
  testIgnore: runTrackingHostedUiProof ? [] : ['tracking-hosted-ui-proof.spec.ts'],
  timeout: 30000,
  use: {
    baseURL: `http://127.0.0.1:${portalPort}`,
    permissions: ['clipboard-read', 'clipboard-write'],
    screenshot: 'only-on-failure',
    trace: 'retain-on-failure',
  },
});

function resolvePortalPort(value: string | undefined): number {
  if (value === undefined || value.trim().length === 0) {
    return 4490;
  }

  const port = Number(value.trim());
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    throw new Error('OCENTRA_PARENT_PORTAL_PORT must be an integer TCP port between 1 and 65535.');
  }

  return port;
}
