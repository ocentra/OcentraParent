import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dirname, '..', '..');
const proofDir = join(repoRoot, 'output', 'screen-plan-proof', '31-screen-intelligence-router');
const proofPath = join(proofDir, 'proof-summary.json');

run('npm', ['run', 'build', '--workspace', '@ocentra-parent/activity-domain']);

const screenEvidence = await import(
  pathToFileURL(join(repoRoot, 'packages', 'activity-domain', 'dist', 'screen-evidence.js')).href
);
const kinds = await import(pathToFileURL(join(repoRoot, 'packages', 'activity-domain', 'dist', 'kinds.js')).href);

const generatedAt = new Date().toISOString();
const scenarios = [
  {
    name: 'existing structured browser evidence skips screenshot',
    decision: screenEvidence.planScreenIntelligenceRoute({
      decisionId: 'proof-route-decision-no-screen-needed',
      decidedAt: generatedAt,
      request: request({
        routeRequestId: 'proof-route-request-no-screen-needed',
        managedBrowserStructuredExtractionAvailable: false,
        managedBrowserStructuredExtractionAttempted: true,
        availableEvidence: [evidence('proof-managed-browser-structured-evidence', true)],
      }),
    }),
  },
  {
    name: 'managed browser structured extraction is first route',
    decision: screenEvidence.planScreenIntelligenceRoute({
      decisionId: 'proof-route-decision-structured-first',
      decidedAt: generatedAt,
      request: request({
        routeRequestId: 'proof-route-request-structured-first',
      }),
    }),
  },
  {
    name: 'native game capture only after foreground evidence family check',
    decision: screenEvidence.planScreenIntelligenceRoute({
      decisionId: 'proof-route-decision-native-game',
      decidedAt: generatedAt,
      request: request({
        routeRequestId: 'proof-route-request-native-game',
        surfaceKind: 'nativeGame',
        allowedCaptureScope: 'activeWindow',
        managedBrowserStructuredExtractionAvailable: false,
        managedBrowserStructuredExtractionAttempted: false,
      }),
    }),
  },
  {
    name: 'launcher capture only after foreground evidence family check',
    decision: screenEvidence.planScreenIntelligenceRoute({
      decisionId: 'proof-route-decision-launcher',
      decidedAt: generatedAt,
      request: request({
        routeRequestId: 'proof-route-request-launcher',
        surfaceKind: 'launcher',
        allowedCaptureScope: 'activeWindow',
        managedBrowserStructuredExtractionAvailable: false,
        managedBrowserStructuredExtractionAttempted: false,
      }),
    }),
  },
  {
    name: 'unknown process capture only after foreground evidence family check',
    decision: screenEvidence.planScreenIntelligenceRoute({
      decisionId: 'proof-route-decision-unknown-process',
      decidedAt: generatedAt,
      request: request({
        routeRequestId: 'proof-route-request-unknown-process',
        surfaceKind: 'unknownProcess',
        allowedCaptureScope: 'selectedWindow',
        managedBrowserStructuredExtractionAvailable: false,
        managedBrowserStructuredExtractionAttempted: false,
      }),
    }),
  },
  {
    name: 'protected surface degrades without screenshot queue',
    decision: screenEvidence.planScreenIntelligenceRoute({
      decisionId: 'proof-route-decision-protected',
      decidedAt: generatedAt,
      request: request({
        routeRequestId: 'proof-route-request-protected',
        sensitivityFlags: ['protectedSurfaceLikely'],
      }),
    }),
  },
];

const negativeChecks = [
  {
    name: 'raw screenshot evidence rejected as existing structured input',
    rejected: !screenEvidence.ScreenIntelligenceExistingEvidenceSchema.safeParse({
      ...evidence('proof-raw-screenshot-evidence', true),
      rawScreenshotEvidence: true,
    }).success,
  },
  {
    name: 'browser screenshot rejected before structured extraction',
    rejected: !screenEvidence.ScreenIntelligenceRouteDecisionSchema.safeParse({
      schemaVersion: 1,
      decisionId: 'proof-route-decision-unsafe-browser-screenshot',
      requestId: 'proof-route-request-unsafe-browser-screenshot',
      decidedAt: generatedAt,
      selectedRoute: 'managedBrowserScreenshot',
      nextStep: 'encryptedImageQueue',
      reason: 'managedBrowserStructuredExhausted',
      policyQuestion: 'categoryReview',
      surfaceKind: 'managedBrowser',
      existingEvidenceChecked: true,
      checkedEvidenceKinds: ['managedBrowserStructured'],
      sourceEvidenceRefs: [],
      structuredExtractionAttemptedBeforeScreenshot: false,
      screenshotQueued: true,
      captureScope: 'managedBrowserWindow',
      rawScreenshotRetainedByDefault: false,
      remoteRawScreenshotUploadAllowed: false,
      parentVisibleSummary: 'Unsafe route queues screenshot before structured extraction.',
      sensitivityFlags: ['lowSensitivity'],
      degradedStates: [],
    }).success,
  },
  {
    name: 'route decision rejected without evidence family checks',
    rejected: !screenEvidence.ScreenIntelligenceRouteDecisionSchema.safeParse({
      schemaVersion: 1,
      decisionId: 'proof-route-decision-no-check',
      requestId: 'proof-route-request-no-check',
      decidedAt: generatedAt,
      selectedRoute: 'nativeActiveWindowCapture',
      nextStep: 'encryptedImageQueue',
      reason: 'nativeSurfaceCaptureAllowed',
      policyQuestion: 'categoryReview',
      surfaceKind: 'nativeApp',
      existingEvidenceChecked: true,
      checkedEvidenceKinds: [],
      sourceEvidenceRefs: [],
      structuredExtractionAttemptedBeforeScreenshot: false,
      screenshotQueued: true,
      captureScope: 'activeWindow',
      rawScreenshotRetainedByDefault: false,
      remoteRawScreenshotUploadAllowed: false,
      parentVisibleSummary: 'Unsafe route did not record existing evidence checks.',
      sensitivityFlags: ['lowSensitivity'],
      degradedStates: [],
    }).success,
  },
];

if (negativeChecks.some((check) => !check.rejected)) {
  throw new Error('Expected every negative screen intelligence router proof row to reject');
}

mkdirSync(proofDir, { recursive: true });
writeFileSync(
  proofPath,
  `${JSON.stringify(
    {
      schemaVersion: 1,
      generatedAt,
      claim:
        'Screen intelligence routing checks existing structured evidence before screenshot capture and routes managed browser surfaces to structured extraction first.',
      scenarios: scenarios.map((scenario) => ({
        name: scenario.name,
        selectedRoute: scenario.decision.selectedRoute,
        nextStep: scenario.decision.nextStep,
        checkedEvidenceKinds: scenario.decision.checkedEvidenceKinds,
        screenshotQueued: scenario.decision.screenshotQueued,
        sourceEvidenceCount: scenario.decision.sourceEvidenceRefs.length,
        degradedStates: scenario.decision.degradedStates,
      })),
      negativeChecks,
      nonClaims: [
        'No live browser CDP capture is performed by this proof.',
        'No network runtime, browser provider fallback, OCR quality, VLM quality, policy action, or enforcement action is claimed.',
        'No raw screenshot retention or remote screenshot upload is permitted by these contracts.',
      ],
    },
    null,
    2
  )}\n`
);

console.log(`screen-intelligence-router-proof-ok:${proofPath}`);

function request(overrides = {}) {
  return {
    schemaVersion: screenEvidence.ScreenIntelligenceRouterSchemaVersion,
    routeRequestId: 'proof-route-request',
    requestedAt: generatedAt,
    policyQuestion: 'categoryReview',
    surfaceKind: 'managedBrowser',
    capabilityStatus: 'ready',
    parentScreenAnalysisEnabled: true,
    captureAllowedByParent: true,
    allowedCaptureScope: 'managedBrowserWindow',
    managedBrowserStructuredExtractionAvailable: true,
    managedBrowserStructuredExtractionAttempted: false,
    sensitivityFlags: ['lowSensitivity'],
    availableEvidence: [],
    routeReason: 'Proof route request checks existing evidence before screenshot capture.',
    ...overrides,
  };
}

function evidence(evidenceId, canAnswerPolicyQuestion) {
  return {
    evidenceRef: {
      evidenceId,
      kind: kinds.ActivityEvidenceKind.JournalEntry,
      digest: `sha256:${evidenceId}`,
      uri: null,
    },
    evidenceKind: 'managedBrowserStructured',
    observedAt: generatedAt,
    category: canAnswerPolicyQuestion ? 'school' : null,
    confidence: canAnswerPolicyQuestion ? 0.93 : 0.2,
    canAnswerPolicyQuestion,
    privacySafeForPolicy: true,
    rawScreenshotEvidence: false,
  };
}

function run(command, args) {
  const runner = process.platform === 'win32' ? 'cmd' : command;
  const runnerArgs = process.platform === 'win32' ? ['/c', command, ...args] : args;
  execFileSync(runner, runnerArgs, { cwd: repoRoot, stdio: 'inherit' });
}
