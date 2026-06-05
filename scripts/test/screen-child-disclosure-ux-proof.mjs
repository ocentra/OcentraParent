import { spawn } from 'node:child_process';
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
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', '26-child-disclosure-ux');
const snapshotPath = join(outputRoot, '00-source-snapshot.md');
const contractLogPath = join(outputRoot, '01-contract-proof.log');
const uiSnapshotDir = join(outputRoot, '10-ui-snapshots');
const artifactScreenshotPath = join(uiSnapshotDir, 'screen-child-disclosure-active-card.png');
const routeScreenshotPath = join(uiSnapshotDir, 'screen-child-disclosure-settings-route.png');
const playwrightLogPath = join(outputRoot, '11-playwright-proof.log');
const validationLogPath = join(outputRoot, '14-validation-commands.log');
const artifactSummaryPath = join(outputRoot, 'proof-summary.json');
const failureScreenshotPath = join(outputRoot, 'failure.png');
const failureSummaryPath = join(outputRoot, 'failure.json');
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-screen-child-disclosure-'));
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

await mkdir(uiSnapshotDir, { recursive: true });
await Promise.all([
  rm(failureScreenshotPath, { force: true }),
  rm(failureSummaryPath, { force: true }),
  rm(contractLogPath, { force: true }),
  rm(playwrightLogPath, { force: true }),
  rm(validationLogPath, { force: true }),
]);

const validationCommands = [
  ['cmd', ['/c', 'npm exec --workspace @ocentra-parent/activity-domain -- vitest run tests/screen-child-disclosure-ux.test.ts']],
  ['cmd', ['/c', 'npm exec --workspace @ocentra-parent/text-domain -- vitest run tests/screen-child-disclosure-ux-text.test.ts']],
  ['cmd', ['/c', 'npm run build --workspace @ocentra-parent/activity-domain']],
  ['cmd', ['/c', 'npm run build --workspace @ocentra-parent/text-domain']],
  ['cmd', ['/c', 'npm run type-check --workspace @ocentra-parent/portal']],
];

const validationResults = [];
for (const [command, args] of validationCommands) {
  validationResults.push(await runCommand(command, args));
}
await writeFile(
  contractLogPath,
  validationResults.map((result) => formatCommandResult(result)).join('\n\n'),
  'utf8'
);
await writeFile(
  validationLogPath,
  validationCommands.map(([command, args]) => `${command} ${args.join(' ')}`).join('\n'),
  'utf8'
);
await writeFile(
  snapshotPath,
  [
    '# Screen Child Disclosure UX Source Snapshot',
    '',
    '- Branch: codex/screen-child-disclosure-ux-proof',
    '- Workpack: docs/plans/screen-plan/workpacks/26-child-disclosure-ux.md',
    '- Feature: docs/features/screen-evidence-analysis.md',
    '- Contract: packages/activity-domain/src/screen-child-disclosure-ux.ts',
    '- Copy: packages/text-domain/src/screen-child-disclosure-ux-text.ts',
    '- Portal proof surface: apps/portal/src/ScreenChildDisclosureProofPanel.tsx',
    '',
  ].join('\n'),
  'utf8'
);

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
const browserOutput = [];

try {
  await waitForHttp(createAgentHealthUrl(agentPort));
  await waitForHttp(`http://127.0.0.1:${portalPort}/`);
  browser = await chromium.launch({ headless: true });
  page = await browser.newPage({ viewport: { width: 1600, height: 1300 } });
  page.on('console', (message) => browserOutput.push(`${message.type()}: ${message.text()}`));
  page.on('pageerror', (error) => browserOutput.push(`pageerror: ${error.message}`));
  await page.goto(`http://127.0.0.1:${portalPort}/#/settings-rules`, { waitUntil: 'domcontentloaded' });
  await page.getByText('Screen check status').waitFor({ timeout: 20_000 });

  const renderedAssertions = [
    'Screen check status',
    'Screen checks are off',
    'Screen checks are paused',
    'Screen checks are ready',
    'Screen check is running',
    'Screen check skipped',
    'Running locally',
    'Skipped protected screen',
    'childDeviceLocal',
    'live-local-child-agent',
  ];
  for (const assertion of renderedAssertions) {
    await expectText(assertion);
  }

  const activeDisclosureCard = page
    .locator('article')
    .filter({ has: page.getByRole('heading', { name: 'Screen check is running' }) })
    .first();
  await activeDisclosureCard.scrollIntoViewIfNeeded();
  await activeDisclosureCard.screenshot({ path: artifactScreenshotPath });
  await page.screenshot({ path: routeScreenshotPath, fullPage: true });
  await writeFile(playwrightLogPath, renderedAssertions.join('\n'), 'utf8');

  const summary = {
    status: 'ok',
    proof: 'screen-child-disclosure-ux-proof',
    proofTier: 'P3_LOCAL_DEV_PORTAL',
    route: '#/settings-rules',
    generatedAt: new Date().toISOString(),
    artifacts: {
      sourceSnapshot: snapshotPath,
      contractLog: contractLogPath,
      screenshot: artifactScreenshotPath,
      routeScreenshot: routeScreenshotPath,
      playwrightLog: playwrightLogPath,
      validationLog: validationLogPath,
      summary: artifactSummaryPath,
    },
    ports: {
      agent: agentPort,
      portal: portalPort,
    },
    renderedAssertions,
    claimsProven: [
      'child-visible screen disclosure rows exist for disabled, paused, ready, capture-active, and protected-surface states',
      'the parent Settings route renders the disclosure copy from shared text-domain tokens',
      'the disclosure contract rejects hidden capture, invisible active checks, raw screenshot path exposure, and remote screenshot upload',
      'the proof runs against the real Rust agent health endpoint and real Vite portal route',
    ],
    nonClaims: [
      'This is not a production child app, OS notification, tray integration, or foreground overlay claim.',
      'This does not persist child disclosure state through the Rust service settings store.',
      'This does not enable screenshot retention, live view transport, remote screenshot upload, or raw image display.',
    ],
  };
  await writeFile(artifactSummaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`screen-child-disclosure-ux-proof-ok:${artifactSummaryPath}`);
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
  const pageText = await page.locator('body').innerText();
  if (!pageText.includes(assertion)) {
    throw new Error(`Screen child disclosure proof did not render expected text: ${assertion}`);
  }
}

function runCommand(command, args) {
  return new Promise((resolveCommand, rejectCommand) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    const stdout = [];
    const stderr = [];
    child.stdout?.on('data', (chunk) => stdout.push(String(chunk)));
    child.stderr?.on('data', (chunk) => stderr.push(String(chunk)));
    child.on('error', rejectCommand);
    child.on('exit', (code) => {
      const result = {
        command: `${command} ${args.join(' ')}`,
        exitCode: code,
        stdout: stdout.join(''),
        stderr: stderr.join(''),
      };
      if (code === 0) {
        resolveCommand(result);
        return;
      }
      rejectCommand(new Error(formatCommandResult(result)));
    });
  });
}

function formatCommandResult(result) {
  return [
    `$ ${result.command}`,
    `exit=${result.exitCode}`,
    result.stdout.trim(),
    result.stderr.trim(),
  ]
    .filter((line) => line.length > 0)
    .join('\n');
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
        browserOutput: browserOutput.slice(-80),
        agentOutput: agentOutput.slice(-80),
        portalOutput: portalOutput.slice(-80),
      },
      null,
      2
    )}\n`
  );
}
