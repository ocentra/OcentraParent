import { spawn, spawnSync } from 'node:child_process';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
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
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', 'settings-service-command');
const artifactScreenshotPath = join(outputRoot, 'parent-settings-service-command.png');
const artifactSummaryPath = join(outputRoot, 'proof-summary.json');
const failureScreenshotPath = join(outputRoot, 'failure.png');
const failureSummaryPath = join(outputRoot, 'failure.json');
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-screen-settings-service-command-'));
const screenSettingsStorePath = join(devLogDir, 'screen-settings.json');
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
runCommand('cmd', ['/c', 'npm run build --workspace @ocentra-parent/screen-domain']);
runCommand('cmd', ['/c', 'npm run build --workspace @ocentra-parent/agent-protocol-domain']);
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
    OCENTRA_PARENT_SCREEN_SETTINGS_STORE_PATH: screenSettingsStorePath,
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
  await page.getByText('Writable screen settings proof').waitFor({ timeout: 20000 });
  await page.getByRole('button', { name: 'Approve local short-TTL retention' }).click();
  await expectText('parentApprovedLocalShortTtl');
  await expectText('120');
  await page.getByRole('button', { name: 'Save selected screen setting' }).click();
  await expectText('service accepted persisted setting');
  await expectText('screen-settings-request-1');
  await expectText('screen-setting-audit-1');
  await expectText('Screen settings update accepted.');
  await page.getByRole('button', { name: 'Refresh persisted screen setting' }).click();
  await expectText('screen-settings-request-2');
  await expectText('Screen settings state reported.');
  const store = JSON.parse(await readFile(screenSettingsStorePath, 'utf8'));
  if (
    store.active_setting_version !== 4 ||
    store.settings?.[0]?.setting?.retainRawImage !== true ||
    store.settings?.[0]?.setting?.temporaryImageTtlSeconds !== 120
  ) {
    throw new Error('Screen settings store did not persist approved local short-TTL raw-retention setting.');
  }
  await page.screenshot({ path: artifactScreenshotPath, fullPage: true });
  await writeFile(artifactSummaryPath, `${JSON.stringify(proofSummary(store), null, 2)}\n`);
  console.log(`screen-settings-portal-service-command-proof-ok ${artifactSummaryPath}`);
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

function proofSummary(store) {
  return {
    status: 'ok',
    proof: 'screen-settings-portal-service-command-proof',
    proofTier: 'P3_REAL_PORTAL_SERVICE_COMMAND',
    route: '#/settings-rules',
    artifacts: {
      screenshot: artifactScreenshotPath,
      summary: artifactSummaryPath,
    },
    ports: {
      agent: agentPort,
      portal: portalPort,
    },
    persistedStore: {
      activeSettingVersion: store.active_setting_version,
      revisionCount: store.settings.length,
      auditCount: store.audit_events.length,
      retainRawImage: store.settings[0].setting.retainRawImage,
      temporaryImageTtlSeconds: store.settings[0].setting.temporaryImageTtlSeconds,
      policyUseEnabled: store.settings[0].setting.policyUseEnabled,
      auditEventId: store.audit_events[0].audit_event_id,
    },
    renderedAssertions: [
      'Writable screen settings proof',
      'Save selected screen setting',
      'Refresh persisted screen setting',
      'Approve local short-TTL retention',
      'parentApprovedLocalShortTtl',
      'service accepted persisted setting',
      'screen-settings-request-1',
      'screen-setting-audit-1',
      'Screen settings update accepted.',
      'screen-settings-request-2',
      'Screen settings state reported.',
    ],
    nonClaims: [
      'This proves the real parent Settings route can submit schema-valid screen settings through the Rust service WebSocket command path.',
      'It proves local JSON persistence and portal-visible service acknowledgement for parent-approved local short-TTL raw screenshot retention.',
      'It does not enable raw screenshot retention by default, live view, raw remote screenshot upload, broad platform parity, privacy/legal approval, or production OCR/VLM quality.',
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
  throw new Error(`Settings service command proof did not render expected text: ${assertion}`);
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
