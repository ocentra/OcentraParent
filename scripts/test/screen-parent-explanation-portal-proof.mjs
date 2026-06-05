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
const outputRoot = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'parent-explanation-portal');
const artifactScreenshotPath = join(outputRoot, 'parent-explanation-activity-route.png');
const artifactSummaryPath = join(outputRoot, 'proof-summary.json');
const validationLogPath = join(outputRoot, '10-validation-commands.log');
const failureScreenshotPath = join(outputRoot, 'failure.png');
const failureSummaryPath = join(outputRoot, 'failure.json');
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-screen-explanation-portal-'));
const activityDbPath = join(devLogDir, 'activity.sqlite');
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

const expected = {
  rowId: 'screen-parent-explanation-portal-row',
  queueJobId: 'screen-parent-explanation-portal-queue',
  policyDecisionRef: 'screen-parent-explanation-portal-policy-decision',
  policyAction: 'allow',
  parentRuleRef: 'screen-parent-explanation-portal-parent-rule',
  localModelRuntimeRef: 'screen-parent-explanation-portal-local-runtime',
  parentExplanationRef: 'screen-parent-explanation-portal-explanation',
  explanationReason: 'screen-summary-cited',
  deletionReason: 'screen-image-deleted',
};

await mkdir(outputRoot, { recursive: true });
await Promise.all([rm(failureScreenshotPath, { force: true }), rm(failureSummaryPath, { force: true })]);
await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);
await seedActivityStore(activityDbPath);

const agent = spawn(resolveDebugAgentServicePath(), [], {
  cwd: repoRoot,
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
    [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
    [ParentDevEnv.ActivityDbPath]: activityDbPath,
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
  page = await browser.newPage({ viewport: { width: 1920, height: 1280 } });
  await page.goto(`http://127.0.0.1:${portalPort}/#/commands`, { waitUntil: 'domcontentloaded' });
  await page.getByRole('button', { exact: true, name: 'Refresh activity screen' }).click();
  await page.getByText('agent.activity.screen.read-model.reported').waitFor({ timeout: 20000 });
  await page.goto(`http://127.0.0.1:${portalPort}/#/activity`, { waitUntil: 'domcontentloaded' });
  await assertRenderedParentExplanation(page);
  await page.screenshot({ path: artifactScreenshotPath, fullPage: true });

  const summary = {
    status: 'ok',
    proofKind: 'screen-parent-explanation-portal-proof',
    artifacts: {
      screenshot: artifactScreenshotPath,
      summary: artifactSummaryPath,
      validationLog: validationLogPath,
    },
    ports: {
      agent: agentPort,
      portal: portalPort,
    },
    renderedAssertions: [
      expected.rowId,
      expected.queueJobId,
      expected.policyDecisionRef,
      expected.policyAction,
      expected.parentRuleRef,
      expected.localModelRuntimeRef,
      expected.parentExplanationRef,
      expected.explanationReason,
      expected.deletionReason,
      'child-device-journal',
      'localOcr',
      'windows-winrt-ocr-local-proof',
      'screen-parent-explanation-portal-v1',
    ],
    closure: {
      realRustServiceReadModel: true,
      realPortalCommand: true,
      realPortalRouteRendering: true,
      parentExplanationRefsVisible: true,
      rawScreenshotsRendered: false,
      rawScreenshotsRetainedByDefault: false,
      remoteAiUsedForChildSafety: false,
    },
    nonClaims: [
      'This proof starts the real Rust service and Vite portal, then clicks the real Activity Screen command.',
      'It proves parent explanation refs render from a service-backed Activity Screen read model.',
      'It does not create a new capture, rerun model inference, upload raw screenshots, or claim remote/API AI.',
    ],
  };
  await writeFile(artifactSummaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  await writeFile(
    validationLogPath,
    [
      'node --check scripts/test/screen-parent-explanation-portal-proof.mjs',
      'node scripts/test/screen-parent-explanation-portal-proof.mjs',
    ].join('\n') + '\n'
  );
  console.log(`screen-parent-explanation-portal-proof-ok ${artifactSummaryPath}`);
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

async function seedActivityStore(dbPath) {
  const sqlite = resolveSqlite();
  const sqlPath = join(devLogDir, 'seed-screen-parent-explanation-portal.sql');
  const fields = {
    screenAnalysisResultId: expected.rowId,
    queueJobId: expected.queueJobId,
    summary: 'Screen summary parent explanation is rendered in the parent portal.',
    primaryCategory: 'school',
    confidence: 0.94,
    imageDeletionState: 'deleted',
    policyEligible: true,
    modelRuntimeRef: expected.localModelRuntimeRef,
    localModelRuntimeRefs: expected.localModelRuntimeRef,
    modelId: 'windows-winrt-ocr-local-proof',
    providerKind: 'localOcr',
    promptOrTemplateVersion: 'screen-parent-explanation-portal-v1',
    captureReason: 'managedBrowserUrlChange',
    captureScope: 'selectedWindow',
    capabilityStatus: 'ready',
    imageDigest: 'sha256:screen-parent-explanation-portal-digest',
    custodyState: 'child-device-journal',
    policyDecisionId: expected.policyDecisionRef,
    policyAction: expected.policyAction,
    reasonCodes: 'screen-summary-linked,parent-rule-linked,deleted-image-linked',
    ruleIds: expected.parentRuleRef,
    parentExplanationRefs: expected.parentExplanationRef,
    explanationReasons: `${expected.explanationReason},policy-decision-cited,parent-rule-cited`,
    deletionReasons: expected.deletionReason,
  };
  const evidence = [
    {
      evidenceId: 'screen-parent-explanation-portal-evidence',
      kind: 'journal-entry',
      digest: 'sha256:screen-parent-explanation-portal-digest',
      uri: null,
    },
  ];
  const sql = `
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
CREATE TABLE IF NOT EXISTS activity_events (
  event_id TEXT PRIMARY KEY,
  observed_at TEXT NOT NULL,
  device_id TEXT NOT NULL,
  platform TEXT NOT NULL,
  observer TEXT NOT NULL,
  kind TEXT NOT NULL,
  subject_kind TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  subject_display_name TEXT,
  fields_json TEXT NOT NULL,
  evidence_json TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS activity_events_recent_idx
  ON activity_events (observed_at DESC, event_id DESC);
DELETE FROM activity_events;
INSERT INTO activity_events (
  event_id,
  observed_at,
  device_id,
  platform,
  observer,
  kind,
  subject_kind,
  subject_id,
  subject_display_name,
  fields_json,
  evidence_json
) VALUES (
  'screen-parent-explanation-portal-event',
  '2026-06-05T12:00:00.000Z',
  'local-dev-agent',
  'windows',
  'local-ai',
  'activity.screen.analysis.summarized',
  'device',
  'local-dev-agent',
  NULL,
  ${sqlString(JSON.stringify(fields))},
  ${sqlString(JSON.stringify(evidence))}
);
`;
  await writeFile(sqlPath, sql);
  const seeded = spawnSync(sqlite, [dbPath, `.read ${sqlPath}`], {
    cwd: repoRoot,
    encoding: 'utf8',
  });
  if (seeded.status !== 0) {
    throw new Error(`sqlite seed failed: ${seeded.stderr || seeded.stdout}`);
  }
}

async function assertRenderedParentExplanation(targetPage) {
  const expectedText = [
    expected.rowId,
    expected.queueJobId,
    expected.policyDecisionRef,
    expected.policyAction,
    expected.parentRuleRef,
    expected.localModelRuntimeRef,
    expected.parentExplanationRef,
    expected.explanationReason,
    expected.deletionReason,
    'child-device-journal',
    'localOcr',
    'windows-winrt-ocr-local-proof',
    'screen-parent-explanation-portal-v1',
  ];
  for (const text of expectedText) {
    await targetPage.getByText(text, { exact: false }).first().waitFor({ timeout: 20000 });
  }
}

async function waitForHttp(url, timeoutMs = 30000) {
  const deadline = Date.now() + timeoutMs;
  let lastError;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return response;
      }
      lastError = new Error(`${url} returned ${response.status}`);
    } catch (error) {
      lastError = error;
    }
    await delay(250);
  }
  throw lastError instanceof Error ? lastError : new Error(`Timed out waiting for ${url}`);
}

function resolveSqlite() {
  const command = process.platform === 'win32' ? 'where' : 'which';
  const result = spawnSync(command, ['sqlite3'], { encoding: 'utf8' });
  if (result.status !== 0) {
    throw new Error('sqlite3 is required for screen parent explanation portal proof.');
  }
  return result.stdout.split(/\r?\n/u).find(Boolean);
}

function sqlString(value) {
  return `'${value.replaceAll("'", "''")}'`;
}

function collectOutput(child) {
  const chunks = [];
  child.stdout?.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr?.on('data', (chunk) => chunks.push(String(chunk)));
  return chunks;
}

async function writeFailureLog(error) {
  let pageText = '';
  if (page !== undefined) {
    try {
      await page.screenshot({ path: failureScreenshotPath, fullPage: true });
      pageText = await page.locator('body').innerText({ timeout: 2000 });
    } catch {
      pageText = '';
    }
  }
  const failure = {
    status: 'failed',
    message: error instanceof Error ? error.message : String(error),
    screenshot: failureScreenshotPath,
    pageText: pageText.slice(0, 8000),
    agentOutput: agentOutput.join('').slice(-8000),
    portalOutput: portalOutput.join('').slice(-8000),
  };
  await writeFile(failureSummaryPath, `${JSON.stringify(failure, null, 2)}\n`);
}
