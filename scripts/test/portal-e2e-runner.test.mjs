import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

test('portal e2e owns agent and portal cleanup outside Playwright webServer', () => {
  const portalManifest = JSON.parse(readFileSync('apps/portal/package.json', 'utf8'));
  const configSource = readFileSync('apps/portal/playwright.config.ts', 'utf8');
  const runnerSource = readFileSync('scripts/test/portal-playwright-runner.mjs', 'utf8');

  assert.equal(portalManifest.scripts['test:e2e'], 'node ../../scripts/test/portal-playwright-runner.mjs');
  assert.equal(configSource.includes('webServer'), false);
  assert.equal(runnerSource.includes('stopProcessTree'), true);
  assert.equal(runnerSource.includes('SIGKILL'), true);
});
