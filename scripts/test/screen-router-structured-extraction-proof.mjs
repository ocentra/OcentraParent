import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const proofOutputDir = join(process.cwd(), 'output', 'screen-plan-proof', '31-32-screen-router-structured-extraction');
const proofOutputPath = join(proofOutputDir, 'proof-summary.json');
const validationLogPath = join(proofOutputDir, 'validation-commands.log');
const successfulCommands = [];

await runPackageCommand([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/activity-domain',
  '--',
  'screen-intelligence-router.test.ts',
]);
await runPackageCommand(['run', 'build', '--workspace', '@ocentra-parent/activity-domain']);
await writeProof();

console.log(`screen-router-structured-extraction-proof-ok: ${proofOutputPath}`);

async function writeProof() {
  const {
    ScreenIntelligenceRouterSchemaVersion,
    ScreenManagedBrowserStructuredTextLimit,
    planScreenIntelligenceRoute,
  } = await import('@ocentra-parent/screen-domain/screen-evidence');

  const enoughStructured = structuredExtraction({
    extractionId: 'managed-browser-structured-youtube-lesson',
    extractionState: 'enoughForPolicy',
    noScreenNeeded: true,
    screenshotRequired: false,
    enoughForPolicy: true,
    policyQuestionAnswered: true,
    categoryCandidate: 'school',
    confidence: 0.91,
    reason: null,
  });
  const ambiguousStructured = structuredExtraction({
    extractionId: 'managed-browser-structured-ambiguous-video',
    extractionState: 'needsScreenshot',
    visibleTextSummary: 'Managed browser URL and title are ambiguous after bounded structured extraction.',
    visibleTextCharacterCount: 78,
    domOverflowRedacted: true,
    redactionState: 'overflowRedacted',
    noScreenNeeded: false,
    screenshotRequired: true,
    enoughForPolicy: false,
    policyQuestionAnswered: false,
    categoryCandidate: null,
    confidence: 0.42,
    reason: 'structured browser evidence is not enough to classify visible activity',
  });
  const decisions = [
    planScreenIntelligenceRoute(
      routeRequest({ structuredExtraction: enoughStructured }),
      'screen-route-no-image-needed'
    ),
    planScreenIntelligenceRoute(
      routeRequest({ structuredExtraction: ambiguousStructured }),
      'screen-route-structured-first'
    ),
    planScreenIntelligenceRoute(
      routeRequest({
        sourceKind: 'nativeGame',
        structuredExtraction: null,
        existingEvidenceRefs: [evidenceRef('native-game-window-ref')],
        allowedCaptureScopes: ['activeWindow', 'selectedWindow'],
      }),
      'screen-route-native-game-active-window'
    ),
    planScreenIntelligenceRoute(
      routeRequest({
        sourceKind: 'unknownProcess',
        structuredExtraction: null,
        allowedCaptureScopes: ['managedBrowserWindow'],
      }),
      'screen-route-manual-required'
    ),
    planScreenIntelligenceRoute(
      routeRequest({
        policySensitivity: 'protectedSurface',
        protectedSurfaceSuspected: true,
      }),
      'screen-route-protected-surface-unavailable'
    ),
  ];

  await mkdir(proofOutputDir, { recursive: true });
  await writeFile(
    proofOutputPath,
    `${JSON.stringify(
      {
        proofGeneratedAt: new Date().toISOString(),
        proofTopic: 'screen-router-structured-extraction',
        workpacks: [
          'docs/plans/screen-plan/workpacks/31-screen-intelligence-router.md',
          'docs/plans/screen-plan/workpacks/32-browser-structured-extraction-before-screenshot.md',
        ],
        claimsProven: [
          'screen router checks existing typed evidence before selecting screenshot capture',
          'managed browser URL/title/metadata/visible-text structured extraction is selected before screenshots',
          'screenshot is skipped when bounded managed-browser structured evidence answers the policy question',
          'native game and app-like foreground cases route only to parent-allowed active-window or selected-window scopes',
          'protected surfaces and credential-risk surfaces return unavailable instead of capture or AI routes',
          'route contracts reject raw DOM inclusion, remote AI defaults, and raw screenshot retention',
        ],
        nonClaims: [
          'this proof does not capture live browser DOM, accessibility trees, or screenshots',
          'this proof does not claim real managed-browser producer integration, portal UI, policy execution, or enforcement',
          'this proof does not update docs/product-capability-checklist.md while PR321 central checklist conflict is sequenced',
        ],
        validationCommands: successfulCommands,
        limits: {
          schemaVersion: ScreenIntelligenceRouterSchemaVersion,
          maxStructuredVisibleTextCharacters: ScreenManagedBrowserStructuredTextLimit,
        },
        routeSummary: summarize(decisions),
        decisions,
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(validationLogPath, `${successfulCommands.join('\n')}\n`, 'utf8');
}

function routeRequest(overrides = {}) {
  return {
    schemaVersion: 1,
    requestId: 'screen-route-request-youtube-lesson',
    requestedAt: '2026-06-05T04:45:00.000Z',
    deviceRef: 'windows-child-device',
    sourceKind: 'managedBrowser',
    captureReason: 'managedBrowserUrlChange',
    policyQuestion: 'Can typed browser evidence answer before taking a screenshot?',
    policySensitivity: 'ordinary',
    existingEvidenceRefs: [evidenceRef('managed-browser-url-ref')],
    structuredExtraction: null,
    parentAllowsManagedBrowserStructuredExtraction: true,
    parentAllowsScreenCapture: true,
    allowedCaptureScopes: ['managedBrowserWindow', 'activeWindow'],
    protectedSurfaceSuspected: false,
    credentialPromptSuspected: false,
    ...overrides,
  };
}

function structuredExtraction(overrides) {
  return {
    schemaVersion: 1,
    extractionId: 'managed-browser-structured-evidence',
    capturedAt: '2026-06-05T04:44:58.000Z',
    evidenceRefs: [evidenceRef('managed-browser-url-ref'), evidenceRef('managed-browser-title-ref')],
    extractionState: 'enoughForPolicy',
    urlTitleMetadataCaptured: true,
    visibleTextSummary: 'YouTube lesson page with math title and education metadata only.',
    visibleTextCharacterCount: 64,
    domOverflowRedacted: false,
    privateContentRedacted: false,
    rawDomIncluded: false,
    redactionState: 'none',
    enoughForPolicy: true,
    policyQuestionAnswered: true,
    noScreenNeeded: true,
    screenshotRequired: false,
    categoryCandidate: 'school',
    riskSignals: [],
    confidence: 0.91,
    custodyState: 'child-device-query-store',
    reason: null,
    ...overrides,
  };
}

function evidenceRef(evidenceId) {
  return {
    evidenceId,
    kind: 'local-db-row',
    digest: `${evidenceId}-digest`,
    uri: null,
  };
}

function summarize(decisions) {
  return {
    totalRoutes: decisions.length,
    noScreenNeeded: decisions.filter((decision) => decision.routeKind === 'noScreenNeeded').length,
    structuredFirst: decisions.filter((decision) => decision.routeKind === 'managedBrowserStructuredExtraction').length,
    screenshotRoutes: decisions.filter((decision) => decision.routeKind.startsWith('screenCapture')).length,
    manualRequired: decisions.filter((decision) => decision.routeKind === 'manualRequired').length,
    unavailable: decisions.filter((decision) => decision.routeKind === 'unavailable').length,
    remoteAiAllowed: decisions.some((decision) => decision.remoteAiAllowed),
    rawScreenshotRetained: decisions.some((decision) => decision.rawScreenshotRetained),
  };
}

function runPackageCommand(args) {
  if (process.platform === 'win32') {
    return runCommand(...npmCommand([...args]));
  }

  return runCommand('npm', args);
}

function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: process.cwd(),
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    const output = collectOutput(child);
    child.on('error', reject);
    child.on('exit', (code) => {
      const commandLine = `${command} ${args.join(' ')}`;
      if (code === 0) {
        successfulCommands.push(commandLine);
        resolve();
        return;
      }
      reject(new Error(`${commandLine} failed with ${code}\n${output()}`));
    });
  });
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
