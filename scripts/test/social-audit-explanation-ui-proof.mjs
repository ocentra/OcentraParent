import { spawn } from 'node:child_process';
import { once } from 'node:events';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { fileURLToPath } from 'node:url';

import {
  ParentDevEnv,
  ParentDevHost,
  ParentDevPort,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  createHttpOrigin,
  createPortalCommandsUrl,
  isLikelyParentAgentOccupant,
  isLikelyParentPortalOccupant,
  resolveParentDevPort,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import { resolveDebugAgentServicePath, spawnVitePortal, stopProcessTreeAndWait } from './agent-service-process.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const portalRoot = path.join(repoRoot, 'apps', 'portal');
const proofRoot = path.join(repoRoot, 'output', 'browser-plan-proof', 'social-22-audit-explanation-read-model');
const screenshotDir = path.join(proofRoot, '06-ui-snapshots');
const proofResultDir = path.join(repoRoot, 'test-results', 'social-audit-explanation-ui-proof');
const proofPath = path.join(proofResultDir, 'proof.json');
const outputProofPath = path.join(proofRoot, '07-rendered-explanation-ui-proof.json');
const sourceSnapshotPath = path.join(proofRoot, '00-source-snapshot.md');
const contractProofPath = path.join(proofRoot, '01-contract-proof.log');
const validationLogPath = path.join(proofRoot, '10-validation-commands.log');
const securityLogPath = path.join(proofRoot, '08-security-negative-proof.log');
const playwrightLogPath = path.join(screenshotDir, 'social-audit-explanation-ui-playwright.log');
const desktopScreenshot = path.join(screenshotDir, 'social-audit-explanation-route.png');
const mobileScreenshot = path.join(screenshotDir, 'social-audit-explanation-route-mobile.png');
const accessibilitySummaryPath = path.join(proofResultDir, 'accessibility-summary.json');
const runRoot = await mkdtemp(path.join(tmpdir(), 'ocentra-parent-social-audit-explanation-'));
const devLogDir = path.join(runRoot, 'dev-log');
const activityDbPath = path.join(runRoot, 'activity.sqlite');
const commands = [];
const children = [];
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

let stopping = false;

try {
  await mkdir(devLogDir, { recursive: true });
  await mkdir(screenshotDir, { recursive: true });
  await mkdir(proofResultDir, { recursive: true });
  await runNpm(['run', 'build:contracts']);
  const proofBundle = await buildProofBundle();
  await runNpmWorkspace('@ocentra-parent/parent-domain', [
    'run',
    'test',
    '--',
    'social-audit-explanation-read-model.test.ts',
  ]);
  await runNpmWorkspace('@ocentra-parent/portal-domain', [
    'run',
    'test',
    '--',
    'social-audit-explanation-panel.test.ts',
  ]);
  await runNpmWorkspace('@ocentra-parent/portal', ['run', 'test', '--', 'social-audit-explanation-panel.test.ts']);
  await runNpmWorkspace('@ocentra-parent/portal-domain', ['run', 'type-check']);
  await runNpmWorkspace('@ocentra-parent/portal', ['run', 'type-check']);
  await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
  await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);

  const agent = spawnAgent();
  trackChild(agent, 'agent');
  await waitForHttp(createAgentHealthUrl(agentPort));

  const portal = spawnVitePortal(portalPort, portalEnv(proofBundle), repoRoot);
  trackChild(portal, 'portal');
  await waitForHttp(createPortalCommandsUrl(portalPort));

  const playwright = await runPlaywright(proofBundle);
  await writeProof(proofBundle, playwright);

  console.log('social-audit-explanation-ui-proof-ok=true');
  console.log(`evidence=${relativePath(proofPath)}`);
  console.log(`screenshots=${relativePath(screenshotDir)}`);
} finally {
  stopping = true;
  await Promise.all(children.map((child) => stopProcessTreeAndWait(child)));
  await rm(runRoot, { recursive: true, force: true });
}

async function buildProofBundle() {
  const { SocialAuditExplanationSnapshotSchema } =
    await import('@ocentra-parent/social-domain/social-audit-explanation-read-model');
  return SocialAuditExplanationSnapshotSchema.parse(validSnapshot());
}

function validSnapshot() {
  return {
    schemaVersion: 'social-audit-explanation-read-model',
    snapshotId: 'social-audit-explanation-snapshot-rendered',
    familyId: 'family-social-audit-rendered',
    childProfileId: 'child-social-audit-rendered',
    capturedAt: '2026-06-06T05:20:00.000Z',
    entries: [
      accountApprovalEntry(),
      feedVideoGateEntry(),
      nativeAppGapEntry(),
      connectorBoundaryEntry(),
      decisionMemoryEntry(),
      manualRequiredGapEntry(),
    ],
    claimBoundaries: {
      runtimeAuditStore: 'not-claimed',
      renderedExplanationUi: 'not-claimed',
      notificationDelivery: 'not-claimed',
      rawAccountVideoMessageContent: 'not-claimed',
      connectorAuthorization: 'not-claimed',
      nativeAppControl: 'not-claimed',
      finalPolicyDecision: 'not-claimed',
      enforcement: 'not-claimed',
    },
  };
}

function accountApprovalEntry() {
  return auditEntry('account-approval', {
    eventId: 'social-audit-account-approval-rendered',
    decisionState: 'parent-recorded',
    actionCandidate: 'parent-review-candidate',
    evidenceLinks: evidenceLinks('policy-candidate', 'parent-approval'),
    explanationReasons: ['evidence-linked', 'policy-candidate-linked', 'parent-decision-linked'],
    parentApprovalRequestRef: 'parent-evidence-approval-request-rendered',
    parentApprovalDecisionRef: 'parent-evidence-approval-decision-rendered',
  });
}

function feedVideoGateEntry() {
  return auditEntry('feed-video-gate', {
    eventId: 'social-audit-feed-video-gate-rendered',
    actionCandidate: 'warn-candidate',
    policyReasonCodes: ['social-risk-high', 'video-safety-risk'],
    evidenceLinks: evidenceLinks('route-evidence', 'policy-candidate'),
  });
}

function nativeAppGapEntry() {
  return manualEntry('native-app-gap', {
    eventId: 'social-audit-native-gap-rendered',
    evidenceLinks: evidenceLinks('native-capability'),
    nativeCapabilityRef: 'parent-evidence-native-capability-rendered',
    explanationReasons: ['native-app-manual-required', 'missing-runtime-proof'],
  });
}

function connectorBoundaryEntry() {
  return manualEntry('connector-boundary', {
    eventId: 'social-audit-connector-boundary-rendered',
    evidenceLinks: evidenceLinks('connector-boundary'),
    connectorBoundaryRef: 'parent-evidence-connector-boundary-rendered',
    explanationReasons: ['connector-boundary-linked', 'manual-review-required'],
  });
}

function decisionMemoryEntry() {
  return auditEntry('decision-memory', {
    eventId: 'social-audit-decision-memory-rendered',
    status: 'contract-only',
    evidenceLinks: evidenceLinks('decision-memory'),
    explanationReasons: ['memory-linked', 'evidence-linked'],
    decisionMemoryRef: 'parent-evidence-decision-memory-rendered',
  });
}

function manualRequiredGapEntry() {
  return manualEntry('manual-required-gap', {
    eventId: 'social-audit-manual-required-gap-rendered',
    evidenceLinks: evidenceLinks('manual-gap'),
    manualRequiredRef: 'parent-evidence-manual-gap-rendered',
  });
}

function auditEntry(subjectKind, overrides) {
  return {
    eventId: 'social-audit-event-rendered',
    subjectKind,
    status: 'ready-for-parent',
    decisionState: 'candidate-only',
    audience: 'parent',
    policyVersionRef: 'policy-version-social-audit-rendered',
    actionCandidate: 'allow-candidate',
    policyReasonCodes: ['parent-rule-match'],
    explanationReasons: ['evidence-linked', 'policy-candidate-linked'],
    evidenceLinks: evidenceLinks('policy-candidate'),
    auditRefs: ['parent-evidence-audit-ref-rendered'],
    parentApprovalRequestRef: null,
    parentApprovalDecisionRef: null,
    decisionMemoryRef: null,
    connectorBoundaryRef: null,
    nativeCapabilityRef: null,
    manualRequiredRef: null,
    runtimeAuditStoreClaimed: false,
    renderedExplanationUiClaimed: false,
    notificationDeliveredClaimed: false,
    rawAccountDataIncluded: false,
    rawVideoContentIncluded: false,
    rawMessageContentIncluded: false,
    connectorAuthorizationClaimed: false,
    nativeAppControlClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function manualEntry(subjectKind, overrides) {
  return auditEntry(subjectKind, {
    status: 'manual-required',
    decisionState: 'manual-required',
    actionCandidate: 'manual-review-candidate',
    policyVersionRef: null,
    policyReasonCodes: ['manual-required'],
    explanationReasons: ['manual-review-required'],
    ...overrides,
  });
}

function evidenceLinks(...kinds) {
  return kinds.map((evidenceKind) => ({
    evidenceKind,
    evidenceRef: `parent-evidence-${evidenceKind}`,
  }));
}

function spawnAgent() {
  return spawn(resolveDebugAgentServicePath(repoRoot), [], {
    cwd: repoRoot,
    detached: process.platform !== 'win32',
    env: {
      ...process.env,
      [ParentDevEnv.ActivityDbPath]: activityDbPath,
      [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
      [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
      [ParentDevEnv.DevLogDir]: devLogDir,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
}

function portalEnv(proofBundle) {
  return {
    ...process.env,
    [ParentDevEnv.ActivityDbPath]: activityDbPath,
    [ParentDevEnv.DevLogDir]: devLogDir,
    [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
    [ParentDevEnv.PortalPort]: String(portalPort),
    SOCIAL_AUDIT_EXPLANATION_UI_PROOF: '1',
    VITE_SOCIAL_AUDIT_EXPLANATION_PROOF_BUNDLE: JSON.stringify(proofBundle),
  };
}

function trackChild(child, label) {
  children.push(child);
  child.stdout?.on('data', (chunk) => process.stdout.write(chunk));
  child.stderr?.on('data', (chunk) => process.stderr.write(chunk));
  child.once('exit', (code, signal) => {
    if (!stopping && code !== 0) {
      console.error(`${label} exited early: code=${code ?? 'null'} signal=${signal ?? 'null'}`);
    }
  });
}

async function runPlaywright(proofBundle) {
  const cliPath = path.join(repoRoot, 'node_modules', '@playwright', 'test', 'cli.js');
  const args = [
    cliPath,
    'test',
    '--config',
    path.join(portalRoot, 'playwright.config.ts'),
    'social-audit-explanation-ui-proof.spec.ts',
    '--workers=1',
  ];
  const result = await runCommand(process.execPath, args, {
    cwd: portalRoot,
    env: portalEnv(proofBundle),
    capture: true,
  });
  await writeFile(playwrightLogPath, `${result.output.trimEnd()}\n`);
  return {
    command: [process.execPath, ...args].join(' '),
    exitCode: result.exitCode,
    log: relativePath(playwrightLogPath),
  };
}

async function runNpmWorkspace(workspaceName, args) {
  await runNpm(['--workspace', workspaceName, ...args]);
}

async function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  await runCommand(command, commandArgs, ...rest);
}

async function runCommand(command, args, options = {}) {
  const commandLine = [command, ...args].join(' ');
  const chunks = [];
  const child = spawn(command, args, {
    cwd: options.cwd ?? repoRoot,
    env: options.env ?? process.env,
    stdio: ['ignore', options.capture ? 'pipe' : 'inherit', options.capture ? 'pipe' : 'inherit'],
    windowsHide: true,
  });
  if (options.capture) {
    child.stdout?.on('data', (chunk) => {
      chunks.push(String(chunk));
      process.stdout.write(chunk);
    });
    child.stderr?.on('data', (chunk) => {
      chunks.push(String(chunk));
      process.stderr.write(chunk);
    });
  }
  const [code, signal] = await once(child, 'exit');
  const exitCode = signal === null ? (code ?? 1) : 1;
  commands.push({ command: commandLine, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${commandLine} exited with ${exitCode}`);
  }
  return { exitCode, output: chunks.join('') };
}

async function waitForHttp(url) {
  const deadline = Date.now() + 90_000;
  let lastError;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return;
      }
      lastError = new Error(`${url} returned ${response.status}`);
    } catch (error) {
      lastError = error;
    }
    await delay(500);
  }
  throw lastError instanceof Error ? lastError : new Error(`Timed out waiting for ${url}`);
}

async function writeProof(proofBundle, playwright) {
  const checkedAt = new Date().toISOString();
  const accessibilitySummary = JSON.parse(await readFile(accessibilitySummaryPath, 'utf8'));
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit: await gitHead(),
    workpackIds: ['social-22-audit-explanation-read-model'],
    proofMode: 'real-portal-browser-route-social-audit-explanation-proof-bundle',
    route: '#/browser',
    currentStatus: 'rendered-social-explanation-proof-only',
    productClaimReady: false,
    artifacts: {
      proof: relativePath(proofPath),
      outputProof: relativePath(outputProofPath),
      sourceSnapshot: relativePath(sourceSnapshotPath),
      contractProof: relativePath(contractProofPath),
      playwrightLog: playwright.log,
      securityNegativeLog: relativePath(securityLogPath),
      validationCommands: relativePath(validationLogPath),
      desktopScreenshot: relativePath(desktopScreenshot),
      mobileScreenshot: relativePath(mobileScreenshot),
      accessibilitySummary: relativePath(accessibilitySummaryPath),
    },
    sourceBoundary: {
      source: 'schema-decoded SOCIAL-22 parent-domain explanation snapshot passed through dedicated portal proof env',
      route: '#/browser',
      rawSocialContentRendered: false,
      serviceBackedReadModelClaimed: false,
    },
    renderedSnapshot: {
      snapshotId: proofBundle.snapshotId,
      entries: proofBundle.entries.map((entry) => ({
        eventId: entry.eventId,
        subjectKind: entry.subjectKind,
        status: entry.status,
        actionCandidate: entry.actionCandidate,
        evidenceKinds: entry.evidenceLinks.map((link) => link.evidenceKind),
      })),
    },
    assertions: [
      'Portal browser route renders the Social explanations region from the real app shell.',
      'The rendered snapshot is parsed by SocialAuditExplanationSnapshotSchema before reaching the portal panel intent.',
      'The route shows all six SOCIAL-22 subject rows: account approval, feed/video gate, native app gap, connector boundary, decision memory, and manual proof gap.',
      'The rendered copy explicitly keeps runtime audit-store delivery, notification delivery, connector authorization, native app control, final policy execution, and enforcement unclaimed.',
      'Desktop and mobile screenshots were captured from the real portal route.',
    ],
    nonClaims: [
      'This proof does not claim production service delivery of social explanation bundles.',
      'This proof does not claim notification delivery, connector authorization, native app control, final policy execution, or enforcement.',
      'This proof does not claim product capability checklist completion.',
    ],
    remainingGapsBeforeProductReady: [
      'Service-backed social explanation read-model/event delivery remains pending until protocol locks are available.',
      'Parent notification/report delivery remains pending.',
      'Connector/native runtime, final policy execution, and enforcement remain separate proof gates.',
    ],
    accessibilitySummary,
    commands,
  };
  const proofContent = `${JSON.stringify(proof, null, 2)}\n`;
  await writeFile(proofPath, proofContent);
  await writeFile(outputProofPath, proofContent);
  await writeFile(
    sourceSnapshotPath,
    [
      '# SOCIAL-22 Source Snapshot',
      '',
      `checkedAt=${checkedAt}`,
      `branch=${await gitBranch()}`,
      `commit=${await gitHead()}`,
      '',
      'SOCIAL-22 renders the existing parent-domain social audit/explanation snapshot contract in the Browser route.',
      'The portal receives only a schema-decoded explanation proof bundle through the dedicated proof env var.',
      'No raw account, video, message, connector token, native app, final policy, or enforcement data is rendered.',
      '',
    ].join('\n')
  );
  await writeFile(
    contractProofPath,
    [
      'SOCIAL-22 contract proof commands:',
      ...commands
        .filter((entry) => entry.command.includes('social-audit-explanation'))
        .map((entry) => `${entry.command} # exit ${entry.exitCode}`),
      '',
    ].join('\n')
  );
  await writeFile(
    validationLogPath,
    commands.map((entry) => `${entry.command} # exit ${entry.exitCode}`).join('\n') + '\n'
  );
  await writeFile(
    securityLogPath,
    [
      `checkedAt=${checkedAt}`,
      'asserted=no connector authorized claim',
      'asserted=no native app controlled claim',
      'asserted=no notification delivered claim',
      'asserted=no final policy execution proved claim',
      'asserted=no enforcement active claim',
      'asserted=productClaimReady=false',
      '',
    ].join('\n')
  );
}

async function gitHead() {
  const result = await runCommand('git', ['rev-parse', 'HEAD'], { capture: true });
  return result.output.trim();
}

async function gitBranch() {
  const result = await runCommand('git', ['rev-parse', '--abbrev-ref', 'HEAD'], { capture: true });
  return result.output.trim();
}

function relativePath(value) {
  return path.relative(repoRoot, value).replace(/\\/gu, '/');
}
