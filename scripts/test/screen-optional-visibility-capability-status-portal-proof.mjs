import { spawn, spawnSync } from 'node:child_process';
import { mkdir, mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { chromium } from 'playwright';

import {
  ParentDevEnv,
  ParentDevHost,
  ParentDevPort,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  createHttpOrigin,
  isLikelyParentAgentOccupant,
  isLikelyParentPortalOccupant,
  resolveParentDevPort,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import {
  removeDirectoryWithRetry,
  resolveDebugAgentServicePath,
  spawnVitePortal,
  stopProcessTreeAndWait,
} from './agent-service-process.mjs';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', 'optional-visibility-capability-status-portal');
const artifactScreenshotPath = join(outputRoot, 'parent-optional-visibility-capability-status.png');
const artifactSummaryPath = join(outputRoot, 'proof-summary.json');
const failureScreenshotPath = join(outputRoot, 'failure.png');
const failureSummaryPath = join(outputRoot, 'failure.json');
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-optional-visibility-capability-status-'));
const agentPort = resolveParentDevPort(
  process.env[ParentDevEnv.AgentPort],
  ParentDevPort.PortalSmokeAgent,
  ParentDevEnv.AgentPort
);
const portalPort = resolveParentDevPort(
  process.env[ParentDevEnv.PortalPort],
  ParentDevPort.PortalSmokePortal,
  ParentDevEnv.PortalPort
);

await mkdir(outputRoot, { recursive: true });
await Promise.all([rm(failureScreenshotPath, { force: true }), rm(failureSummaryPath, { force: true })]);
runCommand('cmd', ['/c', 'npm run build --workspace @ocentra-parent/activity-domain']);
runCommand('cmd', ['/c', 'npm run build --workspace @ocentra-parent/portal-domain']);
runCommand('cmd', ['/c', 'npm run type-check --workspace @ocentra-parent/portal']);
runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);

const agent = spawn(resolveDebugAgentServicePath(), [], {
  cwd: repoRoot,
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
    [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
    [ParentDevEnv.ActivityDbPath]: join(devLogDir, 'activity.sqlite'),
    [ParentDevEnv.DevLogDir]: devLogDir,
  },
  stdio: ['ignore', 'pipe', 'pipe'],
});
const agentOutput = collectOutput(agent);
const portal = spawnVitePortal(portalPort, {
  ...process.env,
  [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
  [ParentDevEnv.DevLogDir]: devLogDir,
});
const portalOutput = collectOutput(portal);

let browser;
let page;

try {
  await waitForHttp(createAgentHealthUrl(agentPort));
  await waitForHttp(`http://127.0.0.1:${portalPort}/`);
  browser = await chromium.launch({ headless: true });
  page = await browser.newPage({ viewport: { width: 1600, height: 1200 } });
  await page.goto(`http://127.0.0.1:${portalPort}/#/settings-rules`, { waitUntil: 'domcontentloaded' });
  await expectText('screen-optional-visibility-capability-status-proof');
  await expectText('screen-parent-retention-capability-disabled');
  await expectText('screen-parent-retention-capability-local-ttl');
  await expectText('screen-parent-live-capability-disabled');
  await expectText('screen-parent-live-capability-lan');
  await expectText('manualRequired');
  await expectText('blocked');
  await expectText('screen-capture-only');
  await page.screenshot({ path: artifactScreenshotPath, fullPage: true });
  await writeFile(artifactSummaryPath, `${JSON.stringify(proofSummary(), null, 2)}\n`);
  console.log(`screen-optional-visibility-capability-status-portal-proof-ok ${artifactSummaryPath}`);
} catch (error) {
  await writeFailureLog(error);
  throw error;
} finally {
  if (browser !== undefined) {
    await browser.close();
  }
  await Promise.all([stopProcessTreeAndWait(portal), stopProcessTreeAndWait(agent)]);
  await removeDirectoryWithRetry(devLogDir);
}

function proofSummary() {
  return {
    status: 'ok',
    proof: 'screen-optional-visibility-capability-status-portal-proof',
    proofTier: 'P3_REAL_PORTAL_RENDERING',
    route: '#/settings-rules',
    artifacts: {
      screenshot: artifactScreenshotPath,
      summary: artifactSummaryPath,
    },
    renderedAssertions: [
      'screen-optional-visibility-capability-status-proof',
      'screen-parent-retention-capability-disabled',
      'screen-parent-retention-capability-local-ttl',
      'screen-parent-live-capability-disabled',
      'screen-parent-live-capability-lan',
      'manualRequired',
      'blocked',
      'screen-capture-only',
    ],
    nonClaims: [
      'This proves the real parent Settings route renders optional visibility readiness rows from the domain contract.',
      'It does not enable raw screenshot retention, live view, live transport, relay/cache, remote input, or privacy/legal approval.',
      'It does not prove platform prompt screenshots or physical-device live-view parity.',
    ],
  };
}

function runCommand(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', stdio: 'inherit' });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${result.status}`);
  }
}

function collectOutput(child) {
  const chunks = [];
  child.stdout?.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr?.on('data', (chunk) => chunks.push(String(chunk)));
  return chunks;
}

async function waitForHttp(url) {
  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return response;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for ${url}`);
}

async function expectText(assertion) {
  const deadline = Date.now() + 20_000;
  while (Date.now() < deadline) {
    const pageText = await page.locator('body').innerText();
    if (pageText.includes(assertion)) {
      return;
    }
    await delay(250);
  }
  throw new Error(`Optional visibility capability portal proof did not render expected text: ${assertion}`);
}

async function writeFailureLog(error) {
  if (page !== undefined) {
    await page.screenshot({ path: failureScreenshotPath, fullPage: true }).catch(() => undefined);
  }
  await writeFile(
    failureSummaryPath,
    `${JSON.stringify(
      {
        status: 'failed',
        error: error instanceof Error ? error.message : String(error),
        agentOutput: agentOutput.slice(-80),
        portalOutput: portalOutput.slice(-80),
      },
      null,
      2
    )}\n`
  );
}
