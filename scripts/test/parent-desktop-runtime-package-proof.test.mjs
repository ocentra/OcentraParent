import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { describe, it } from 'node:test';

import { main as runParentDesktopDev } from '../../scripts/dev/dev-parent-desktop.mjs';
import { ParentDevEnv, ParentDevNetworkMode } from '../../scripts/dev/local-dev-config.mjs';

const repoRoot = process.cwd();
const generatedConfigDirectory = join(repoRoot, 'apps', 'parent-desktop', 'src-tauri', '.generated');

describe('parent desktop runtime package', () => {
  it('builds a loopback Tauri launch config from the real desktop dev entrypoint', async () => {
    const config = await runDryRun({
      agentPort: 5177,
      portalPort: 5178,
      mode: ParentDevNetworkMode.Loopback,
    });

    assert.equal(config.build.frontendDist, '../../portal/dist');
    assert.equal(config.build.devUrl, 'http://127.0.0.1:5178');
    assert.equal(config.build.beforeDevCommand, 'npm --prefix ../.. run dev:desktop:stack');
    assert.match(config.app.security.csp, /connect-src[^;]*http:\/\/127\.0\.0\.1:5178/u);
    assert.match(config.app.security.csp, /connect-src[^;]*ws:\/\/127\.0\.0\.1:5178/u);
    assert.equal(config.app.security.csp.includes(':5177'), false);
  });

  it('builds a LAN Tauri launch config while keeping the service connection loopback-owned', async () => {
    const config = await runDryRun({
      agentPort: 5277,
      portalPort: 5278,
      mode: ParentDevNetworkMode.Lan,
      lanHost: '192.168.50.25',
    });

    assert.equal(config.build.frontendDist, '../../portal/dist');
    assert.equal(config.build.devUrl, 'http://192.168.50.25:5278');
    assert.equal(config.build.beforeDevCommand, 'npm --prefix ../.. run dev:desktop:stack:lan');
    assert.match(config.app.security.csp, /connect-src[^;]*http:\/\/192\.168\.50\.25:5278/u);
    assert.match(config.app.security.csp, /connect-src[^;]*ws:\/\/192\.168\.50\.25:5278/u);
    assert.equal(config.app.security.csp.includes(':5277'), false);
  });
});

async function runDryRun({ agentPort, portalPort, mode, lanHost }) {
  const environment = {
    [ParentDevEnv.AgentPort]: String(agentPort),
    [ParentDevEnv.PortalPort]: String(portalPort),
    [ParentDevEnv.DevNetworkMode]: mode,
    ...(lanHost === undefined ? {} : { [ParentDevEnv.LanHost]: lanHost }),
  };
  const argv = ['node', 'scripts/dev/dev-parent-desktop.mjs', '--dry-run'];
  if (mode === ParentDevNetworkMode.Lan) {
    argv.push('--lan');
  }

  await runParentDesktopDev(argv, environment);
  const generatedPath = join(generatedConfigDirectory, `tauri.dev.portal-${portalPort}.agent-${agentPort}.conf.json`);
  return JSON.parse(await readFile(generatedPath, 'utf8'));
}
