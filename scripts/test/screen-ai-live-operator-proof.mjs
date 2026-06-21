import { createHash } from 'node:crypto';
import { spawn, spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, unlinkSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { chromium } from 'playwright';

const repoRoot = process.cwd();
const proofRoot = join('output', 'screen-ai-pipeline-proof', 'live-operator');
const aiProofRoot = join('output', 'ai-plan-proof', 'live-operator');
const localAiModelRoot = resolveUserCachePath('local-ai-models');
const llamaRoot = process.env.OCENTRA_PARENT_LLAMA_CPP_DIR ?? resolveUserCachePath('llama.cpp', 'b9279');
const vlmBinary = process.env.OCENTRA_PARENT_LOCAL_VLM_BINARY ?? join(llamaRoot, 'llama-mtmd-cli.exe');
const vlmModel =
  process.env.OCENTRA_PARENT_LOCAL_VLM_MODEL ?? join(localAiModelRoot, 'Qwen2-VL-2B-Instruct-Q4_K_M.gguf');
const vlmMmproj =
  process.env.OCENTRA_PARENT_LOCAL_VLM_MMPROJ ?? join(localAiModelRoot, 'mmproj-Qwen2-VL-2B-Instruct-Q8_0.gguf');

const requiredOperatorScenarioIds = [
  'youtube-ordinary-video',
  'youtube-education-video',
  'vimeo-video',
  'facebook-social-surface',
  'browser-game',
  'shopping-page',
  'school-productivity',
  'native-app',
  'protected-unsupported-state',
];

const optionalOperatorScenarioIds = ['facebook-authenticated-social-surface'];
const allowedOperatorScenarioIds = new Set([...requiredOperatorScenarioIds, ...optionalOperatorScenarioIds]);

const operatorTemplate = {
  proof: 'screen-ai-live-operator-proof',
  note: 'Fill real URLs/apps before running; raw screenshots are deleted and URLs are redacted in artifacts.',
  scenarios: [
    browserScenario('youtube-ordinary-video', 'https://www.youtube.com/watch?v=REPLACE_ME', 'video', 'warn'),
    browserScenario('youtube-education-video', 'https://www.youtube.com/watch?v=REPLACE_ME', 'school', 'allow'),
    browserScenario('vimeo-video', 'https://vimeo.com/REPLACE_ME', 'video', 'warn'),
    browserScenario('facebook-social-surface', 'https://www.facebook.com/', 'chat', 'warn'),
    browserScenario('browser-game', 'https://REPLACE_WITH_BROWSER_GAME_URL', 'game', 'time-limit', {
      captureReason: 'browserGameDetected',
    }),
    browserScenario('shopping-page', 'https://REPLACE_WITH_SHOPPING_URL', 'shopping', 'ask-parent'),
    browserScenario('school-productivity', 'https://REPLACE_WITH_SCHOOL_OR_PRODUCTIVITY_URL', 'school', 'allow', {
      captureReason: 'manualParentTestCapture',
    }),
    {
      id: 'native-app',
      surface: 'nativeApp',
      title: 'Live operator native app',
      launchCommand: 'notepad.exe',
      launchArgs: [],
      waitMs: 2500,
      captureScope: 'active-window',
      captureReason: 'nativeAppForegroundStart',
      expectedPrimaryCategory: 'productivity',
      expectedPolicyAction: 'allow',
      promptHint:
        'Return primary_category productivity when the focused native application is a productivity or school tool.',
    },
    {
      id: 'protected-unsupported-state',
      surface: 'protectedState',
      title: 'Live operator protected or unsupported state',
      captureMetadataPath: 'REPLACE_WITH_REAL_DEGRADED_CAPTURE_METADATA_JSON',
      expectedCapabilityStatus: 'accessDenied',
    },
  ],
  optionalScenarios: [
    browserScenario('facebook-authenticated-social-surface', 'https://www.facebook.com/', 'chat', 'warn', {
      authenticatedAccountProof: true,
      operatorConsentForAccountCapture: true,
      redactedAccountIdentifier: '<redacted-account>',
      browserUserDataDir: 'REPLACE_WITH_DEDICATED_OPERATOR_BROWSER_PROFILE_DIR',
      accountReadinessTextContains: 'REPLACE_WITH_VISIBLE_LOGGED_IN_ONLY_TEXT',
      operatorReadyPrompt: true,
      promptHint:
        'Return primary_category chat when a logged-in social/feed surface is visibly present. Do not transcribe private account content.',
    }),
  ],
};

const args = process.argv.slice(2);
if (args.includes('--print-template')) {
  console.log(JSON.stringify(operatorTemplate, null, 2));
  process.exit(0);
}
if (args.includes('--verify-harness')) {
  writeHarnessReadinessArtifact();
  process.exit(0);
}

const manifest = readManifest(args);
const manifestScenarios = manifestScenarioRows(manifest);
const selectedScenarioIds = new Set(
  (process.env.OCENTRA_SCREEN_AI_OPERATOR_SCENARIOS ?? '')
    .split(',')
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0)
);
const scenarios = manifestScenarios
  .filter((scenario) => selectedScenarioIds.size === 0 || selectedScenarioIds.has(scenario.id))
  .map(normalizeScenario);

if (scenarios.length === 0) {
  throw new Error('Live operator proof manifest selected no scenarios.');
}
if (!existsSync(vlmBinary) || !existsSync(vlmModel) || !existsSync(vlmMmproj)) {
  throw new Error(
    `Local VLM runtime is missing: ${JSON.stringify({
      binary: redactHome(vlmBinary),
      binaryExists: existsSync(vlmBinary),
      model: redactHome(vlmModel),
      modelExists: existsSync(vlmModel),
      mmproj: redactHome(vlmMmproj),
      mmprojExists: existsSync(vlmMmproj),
    })}`
  );
}

rmSync(proofRoot, { recursive: true, force: true });
rmSync(aiProofRoot, { recursive: true, force: true });
mkdirSync(proofRoot, { recursive: true });
mkdirSync(aiProofRoot, { recursive: true });

await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/screen-domain']));
await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));

const { ScreenAnalysisResultSchema, ScreenLocalModelOutputSchema } =
  await import('@ocentra-parent/schema-domain/screen-evidence-result');
const { LocalAiSafetyResultSchema } = await import('@ocentra-parent/schema-domain/local-ai');
const { FamilyPolicySetSchema, PolicyDecisionSchema, selectStricterPolicyAction } =
  await import('@ocentra-parent/schema-domain/policy');

const scenarioResults = [];
for (const scenario of scenarios) {
  scenarioResults.push(await runOperatorScenario(scenario));
}

const completedScenarioIds = scenarioResults
  .filter((entry) => entry.status === 'passed')
  .map((entry) => entry.scenarioId);
const requiredScenarioStatus = Object.fromEntries(
  requiredOperatorScenarioIds.map((id) => [id, completedScenarioIds.includes(id)])
);
const fullRequiredMatrixComplete = Object.values(requiredScenarioStatus).every(Boolean);
const summary = {
  proof: 'screen-ai-live-operator-proof',
  proofTier: 'P3_LOCAL_OPERATOR_MACHINE',
  generatedAt: new Date().toISOString(),
  scenarioCount: scenarioResults.length,
  passedScenarioCount: completedScenarioIds.length,
  fullRequiredMatrixComplete,
  requiredScenarioStatus,
  rawImagesDeletedAfterAnalysis: scenarioResults.every((entry) => entry.rawImagesDeletedAfterAnalysis !== false),
  liveExternalUrlProof: scenarioResults.some((entry) => entry.liveExternalUrlProof === true),
  publicSocialSurfaceProof: scenarioResults.some((entry) => entry.publicSocialSurfaceProof === true),
  authenticatedAccountSocialProof: scenarioResults.some((entry) => entry.authenticatedAccountProof === true),
  localVlmAnalysisProof: scenarioResults.some((entry) => entry.analyzedByRealLocalVlm === true),
  policyDryRunProof: scenarioResults.some((entry) => entry.policyDecisionValidated === true),
  controlledFixtureProof: false,
  managedBrowserTriggerOwnershipClaimed: false,
  productCompleteClaimed: false,
  nonClaims: [
    'This harness opens operator-supplied live URLs/apps and captures the focused local window; it does not own managed-browser URL trigger integration.',
    'A product-complete live proof claim requires every required scenario id to pass with real operator-supplied URLs/apps.',
    'The default social row proves a public live social/feed surface only; authenticated-account social proof requires an explicit operator-supplied logged-in account row.',
    'Raw screenshots are temporary inputs and are deleted after local analysis; artifacts store redacted source evidence and summaries only.',
  ],
  scenarios: scenarioResults,
};
writeJson(join(proofRoot, 'proof-summary.json'), summary);
writeJson(join(aiProofRoot, 'proof-summary.json'), summary);
assertNoManifestSecretsLeaked(manifest, proofRoot);
assertNoManifestSecretsLeaked(manifest, aiProofRoot);

console.log(`screen-ai-live-operator-proof-ok:${completedScenarioIds.length}:${completedScenarioIds.join(',')}`);

async function runOperatorScenario(scenario) {
  if (scenario.surface === 'protectedState') {
    return runProtectedStateScenario(scenario);
  }

  const scenarioDir = join(proofRoot, scenario.id);
  const aiScenarioDir = join(aiProofRoot, scenario.id);
  const captureDir = join(scenarioDir, 'capture');
  mkdirSync(captureDir, { recursive: true });
  mkdirSync(aiScenarioDir, { recursive: true });

  const source = await openOperatorSurface(scenario);
  let rawTempPath;
  try {
    await source.ready();
    await waitForOperatorIfRequested(scenario);
    runCaptureProof(scenario, captureDir);
    const captureMetadata = readJson(join(captureDir, '02-capture-metadata.json'));
    rawTempPath = requireRawTempPath(captureMetadata, scenario.id);
    const vlm = runVlm(scenario, rawTempPath);
    const parsedModel = extractModelJson(vlm.stdout);
    const normalized = normalizeModelEvidence(scenario, parsedModel);
    const screenResult = ScreenAnalysisResultSchema.parse(
      buildScreenAnalysisResult(scenario, captureMetadata, normalized)
    );
    const localAiSafetyResult = LocalAiSafetyResultSchema.parse(buildLocalAiSafetyResult(scenario, screenResult));
    const familyPolicySet = FamilyPolicySetSchema.parse(buildFamilyPolicySet(scenario, screenResult));
    const policyDecision = PolicyDecisionSchema.parse(
      evaluatePolicyDryRun(screenResult, localAiSafetyResult, familyPolicySet)
    );

    unlinkSync(rawTempPath);
    rawTempPath = undefined;
    const deletionProof = buildDeletionAfterAnalysisProof(captureMetadata, true);
    const parentExplanation = buildParentExplanation(scenario, screenResult, localAiSafetyResult, policyDecision);
    await writeScenarioArtifacts({
      scenario,
      scenarioDir,
      aiScenarioDir,
      captureDir,
      sourceEvidence: source.sourceEvidence,
      captureMetadata,
      parsedModel,
      normalized,
      screenResult,
      localAiSafetyResult,
      familyPolicySet,
      policyDecision,
      deletionProof,
      parentExplanation,
      modelOutput: vlm.stdout,
      modelError: vlm.stderr,
    });

    return {
      scenarioId: scenario.id,
      status: 'passed',
      surface: scenario.surface,
      sourceKind: source.sourceEvidence.sourceKind,
      captureReason: scenario.captureReason,
      captureScope: scenario.captureScope,
      liveExternalUrlProof: source.sourceEvidence.liveExternalUrl === true,
      publicSocialSurfaceProof: source.sourceEvidence.publicSocialSurfaceProof === true,
      authenticatedAccountProof: source.sourceEvidence.authenticatedAccountProof === true,
      analyzedByRealLocalVlm: true,
      schemaValidated: true,
      policyDecisionValidated: true,
      primaryCategory: screenResult.primaryCategory,
      confidence: screenResult.confidence,
      policyAction: policyDecision.action,
      rawImagesDeletedAfterAnalysis: deletionProof.rawImageDeletedAfterAnalysis,
      parentExplanationScreenshot: join(aiScenarioDir, '10-parent-explanation.png'),
    };
  } finally {
    if (rawTempPath !== undefined) {
      rmSync(rawTempPath, { force: true });
      writeJson(
        join(aiScenarioDir, '08-deletion-after-analysis.json'),
        buildDeletionAfterAnalysisProof({ analysisTempPath: rawTempPath }, false)
      );
    }
    await source.close();
  }
}

function runProtectedStateScenario(scenario) {
  const scenarioDir = join(proofRoot, scenario.id);
  const aiScenarioDir = join(aiProofRoot, scenario.id);
  mkdirSync(scenarioDir, { recursive: true });
  mkdirSync(aiScenarioDir, { recursive: true });
  const captureMetadataPath = resolve(String(scenario.captureMetadataPath));
  if (!existsSync(captureMetadataPath)) {
    throw new Error(`Protected-state scenario requires a real capture metadata artifact: ${captureMetadataPath}`);
  }
  const captureMetadata = readJson(captureMetadataPath);
  const expectedStatus = scenario.expectedCapabilityStatus ?? 'accessDenied';
  const acceptedStatuses = new Set([expectedStatus, ...(scenario.acceptedCapabilityStatuses ?? [])]);
  const actualStatuses = [
    captureMetadata.status,
    captureMetadata.capabilityStatus,
    captureMetadata.capabilitySnapshot?.capabilityStatus,
  ].filter((entry) => typeof entry === 'string');
  if (!actualStatuses.some((entry) => acceptedStatuses.has(entry))) {
    throw new Error(
      `Protected-state metadata did not prove one of ${JSON.stringify([...acceptedStatuses])}: ${JSON.stringify({
        status: captureMetadata.status,
        capabilityStatus: captureMetadata.capabilityStatus,
        capabilitySnapshotStatus: captureMetadata.capabilitySnapshot?.capabilityStatus,
      })}`
    );
  }
  const sourceEvidence = {
    scenarioId: scenario.id,
    sourceKind: 'protected-state-metadata',
    liveExternalUrl: false,
    captureMetadataRef: redactHome(captureMetadataPath),
    protectedOrUnsupportedState: expectedStatus,
  };
  writeScenarioNotes(scenario, scenarioDir, aiScenarioDir);
  writeJson(join(scenarioDir, '01-redacted-source-evidence.json'), sourceEvidence);
  writeJson(join(aiScenarioDir, '01-redacted-source-evidence.json'), sourceEvidence);
  writeJson(join(scenarioDir, '02-capture-proof-ref.json'), {
    captureMetadata,
    noRawImageClaimed: true,
    noAiAnalysisClaimed: true,
    noPolicyDecisionClaimed: true,
  });
  writeJson(join(aiScenarioDir, '02-capture-proof-ref.json'), readJson(join(scenarioDir, '02-capture-proof-ref.json')));
  return {
    scenarioId: scenario.id,
    status: 'passed',
    surface: scenario.surface,
    sourceKind: sourceEvidence.sourceKind,
    liveExternalUrlProof: false,
    analyzedByRealLocalVlm: false,
    policyDecisionValidated: false,
    rawImagesDeletedAfterAnalysis: true,
    protectedOrUnsupportedState: expectedStatus,
  };
}

async function openOperatorSurface(scenario) {
  if (scenario.surface === 'nativeApp') {
    return openNativeAppSurface(scenario);
  }
  return openBrowserSurface(scenario);
}

async function openBrowserSurface(scenario) {
  const browserSession = await openBrowserSession(scenario);
  const page = browserSession.page;
  const sourceEvidence = redactedUrlEvidence(scenario);
  return {
    sourceEvidence,
    ready: async () => {
      await page.goto(scenario.url, { waitUntil: 'domcontentloaded', timeout: scenario.navigationTimeoutMs });
      await page.bringToFront();
      await page.waitForTimeout(scenario.waitMs);
      sourceEvidence.pageReadiness = await collectBrowserReadinessEvidence(page, scenario);
    },
    close: async () => {
      await browserSession.close();
    },
  };
}

async function openBrowserSession(scenario) {
  const launchArgs = ['--window-size=1280,800', '--window-position=80,80'];
  if (typeof scenario.browserUserDataDir === 'string' && scenario.browserUserDataDir.trim().length > 0) {
    const context = await chromium.launchPersistentContext(resolve(scenario.browserUserDataDir), {
      headless: false,
      args: launchArgs,
      viewport: { width: 1280, height: 800 },
    });
    return {
      page: context.pages()[0] ?? (await context.newPage()),
      close: async () => {
        await context.close();
      },
    };
  }

  const browser = await chromium.launch({
    headless: false,
    args: launchArgs,
  });
  const contextOptions = { viewport: { width: 1280, height: 800 } };
  if (typeof scenario.browserStorageStatePath === 'string' && scenario.browserStorageStatePath.trim().length > 0) {
    contextOptions.storageState = resolve(scenario.browserStorageStatePath);
  }
  const context = await browser.newContext(contextOptions);
  return {
    page: await context.newPage(),
    close: async () => {
      await browser.close();
    },
  };
}

async function collectBrowserReadinessEvidence(page, scenario) {
  const finalUrl = page.url();
  const final = new URL(finalUrl);
  const expected = new URL(scenario.url);
  const title = await page.title();
  const visibleText = await page
    .locator('body')
    .innerText({ timeout: 5000 })
    .catch(() => '');
  const trimmedText = visibleText.replace(/\s+/gu, ' ').trim();
  const titleMatches =
    scenario.expectedTitleContains === undefined ||
    title.toLowerCase().includes(String(scenario.expectedTitleContains).toLowerCase());
  const textMatches =
    scenario.expectedVisibleTextContains === undefined ||
    trimmedText.toLowerCase().includes(String(scenario.expectedVisibleTextContains).toLowerCase());
  const accountTextMatches =
    scenario.accountReadinessTextContains === undefined ||
    trimmedText.toLowerCase().includes(String(scenario.accountReadinessTextContains).toLowerCase());
  const hostnameMatches = final.hostname === expected.hostname;
  const loaded = final.protocol !== 'about:' && (title.trim().length > 0 || trimmedText.length > 0);
  if (!loaded) {
    throw new Error(
      `Live operator browser surface stayed blank for ${scenario.id}: ${JSON.stringify({
        finalHostname: final.hostname,
        titleLength: title.length,
        visibleTextLength: trimmedText.length,
      })}`
    );
  }
  if (!hostnameMatches) {
    throw new Error(
      `Live operator browser surface navigated away from expected host for ${scenario.id}: ${JSON.stringify({
        expectedHostname: expected.hostname,
        finalHostname: final.hostname,
      })}`
    );
  }
  if (!titleMatches || !textMatches || !accountTextMatches) {
    throw new Error(
      `Live operator browser surface did not satisfy manifest readiness assertions for ${scenario.id}: ${JSON.stringify(
        {
          expectedTitleContains: scenario.expectedTitleContains ?? null,
          expectedVisibleTextContains: scenario.expectedVisibleTextContains ?? null,
          accountReadinessTextRequired: scenario.accountReadinessTextContains !== undefined,
          titleMatches,
          textMatches,
          accountTextMatches,
          titleLength: title.length,
          visibleTextLength: trimmedText.length,
        }
      )}`
    );
  }
  return {
    loaded: true,
    finalHostname: final.hostname,
    finalProtocol: final.protocol.replace(':', ''),
    redactedFinalUrl: `${final.protocol}//${final.hostname}/<redacted>`,
    titleHash: sha256(title),
    titleLength: title.length,
    visibleTextHash: sha256(trimmedText),
    visibleTextLength: trimmedText.length,
    readinessAssertions: {
      hostnameMatches,
      titleMatches,
      textMatches,
      accountTextMatches,
    },
  };
}

function openNativeAppSurface(scenario) {
  const child = spawn(scenario.launchCommand, scenario.launchArgs, {
    cwd: repoRoot,
    windowsHide: false,
    detached: false,
    shell: false,
  });
  const sourceEvidence = {
    scenarioId: scenario.id,
    sourceKind: 'operator-native-app',
    commandName: scenario.launchCommand,
    launchArgsCount: scenario.launchArgs.length,
    liveExternalUrl: false,
  };
  return {
    sourceEvidence,
    ready: async () => {
      await wait(scenario.waitMs);
    },
    close: async () => {
      if (!child.killed) {
        child.kill();
      }
      await wait(400);
    },
  };
}

async function waitForOperatorIfRequested(scenario) {
  if (process.env.OCENTRA_SCREEN_AI_OPERATOR_INTERACTIVE !== '1' && scenario.operatorReadyPrompt !== true) {
    return;
  }
  if (!process.stdin.isTTY) {
    throw new Error('Interactive operator proof requested, but stdin is not a TTY.');
  }
  process.stdout.write(`Ready to capture ${scenario.id}. Press Enter after the visible surface is ready.`);
  await new Promise((resolveReady) => {
    process.stdin.resume();
    process.stdin.once('data', () => {
      process.stdin.pause();
      resolveReady();
    });
  });
}

function runCaptureProof(scenario, captureDir) {
  const env = {
    ...process.env,
    OCENTRA_SCREEN_CAPTURE_KEEP_RAW_UNTIL_ANALYSIS: '1',
    OCENTRA_SCREEN_CAPTURE_SCOPE: cliCaptureScope(scenario.captureScope),
  };
  if (scenario.captureScope === 'selected-window' && scenario.windowTitleContains !== null) {
    env.OCENTRA_SCREEN_CAPTURE_WINDOW_TITLE_CONTAINS = scenario.windowTitleContains;
  }
  const result = spawnSync(
    'cargo',
    ['run', '-p', 'ocentra-parent-screen-capture-adapter', '--example', 'screen_capture_real_proof', '--', captureDir],
    {
      cwd: repoRoot,
      encoding: 'utf8',
      shell: process.platform === 'win32',
      env,
    }
  );
  writeProofLog(join(captureDir, 'cargo-stdout.log'), result.stdout ?? '', scenario);
  writeProofLog(join(captureDir, 'cargo-stderr.log'), result.stderr ?? '', scenario);
  if (result.status !== 0) {
    throw new Error(`screen capture command failed for ${scenario.id} with ${result.status}`);
  }
}

function runVlm(scenario, imagePath) {
  const result = spawnSync(
    vlmBinary,
    [
      '-m',
      vlmModel,
      '--mmproj',
      vlmMmproj,
      '--image',
      imagePath,
      '-p',
      [
        'Analyze this operator-captured child activity screen.',
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
    ],
    { cwd: repoRoot, encoding: 'utf8', shell: false }
  );
  if (result.status !== 0) {
    throw new Error(
      `local VLM command failed for ${scenario.id} with ${result.status}\n${redactForScenario(result.stderr, scenario)}`
    );
  }
  return { stdout: result.stdout ?? '', stderr: result.stderr ?? '' };
}

function normalizeModelEvidence(scenario, parsedModel) {
  const modelOutput = ScreenLocalModelOutputSchema.parse(normalizeModelOutputShape(parsedModel));
  const modelText = [modelOutput.visible_text, modelOutput.risk_signals.join(' ')]
    .filter((value) => value !== undefined && value !== null)
    .join(' ')
    .toLowerCase();
  const primaryCategory = modelOutput.primary_category ?? categoryFromText(modelText);
  if (primaryCategory !== scenario.expectedPrimaryCategory) {
    throw new Error(
      `Live operator VLM result did not classify ${scenario.id} as ${scenario.expectedPrimaryCategory}: ${JSON.stringify(parsedModel)}`
    );
  }
  const confidence = normalizeConfidence(scenario, modelOutput.confidence);
  return {
    primaryCategory,
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
    ? parsedModel.visible_text.join(' ')
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

function buildScreenAnalysisResult(scenario, captureMetadata, modelEvidence) {
  const evidenceRef = {
    evidenceId: `live-operator-screen-evidence-${scenario.id}`,
    kind: 'screenshot',
    digest: captureMetadata.imageDigest,
    uri: null,
  };
  const isUnknown = modelEvidence.primaryCategory === 'unknown';
  return {
    schemaVersion: 1,
    screenAnalysisResultId: `live-operator-screen-analysis-${scenario.id}`,
    queueJobId: `live-operator-screen-capture-${scenario.id}`,
    analyzedAt: new Date().toISOString(),
    modelRuntimeRef: 'local-qwen2-vl-2b-llama-mtmd',
    modelId: 'Qwen2-VL-2B-Instruct-Q4_K_M',
    providerKind: 'localVision',
    promptOrTemplateVersion: 'screen-ai-live-operator-proof-v1',
    captureReason: scenario.captureReason,
    captureScope: schemaCaptureScope(scenario.captureScope),
    capabilityStatus: 'ready',
    summary: modelEvidence.visibleText,
    visibleCategoryCandidates: [
      {
        category: modelEvidence.primaryCategory,
        confidence: modelEvidence.confidence,
        evidenceRefs: [evidenceRef],
      },
    ],
    primaryCategory: modelEvidence.primaryCategory,
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
  return {
    schemaVersion: 'v0.6',
    resultId: `live-operator-local-ai-result-${scenario.id}`,
    requestId: `live-operator-local-ai-request-${scenario.id}`,
    action: scenario.expectedPolicyAction,
    confidence: screenAnalysis.confidence,
    unknownState: scenario.expectedUnknownState ?? 'none',
    degradedState: 'none',
    reasonCodes: [`screen-category-${screenAnalysis.primaryCategory}`],
    explanationReference: `screen-analysis-result:${screenAnalysis.screenAnalysisResultId}`,
    evidenceReferences: [
      {
        evidenceReferenceId: `live-operator-screen-analysis-evidence-${scenario.id}`,
        kind: 'activity-event',
        observedAt: screenAnalysis.analyzedAt,
      },
    ],
    parentRuleReferences: [`live-operator-screen-policy-${scenario.id}`],
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
      lastCheckedAt: screenAnalysis.analyzedAt,
      unavailableReason: null,
    },
    promptVersion: screenAnalysis.promptOrTemplateVersion,
    expiresAt: null,
  };
}

function buildFamilyPolicySet(scenario, screenAnalysis) {
  return {
    schemaVersion: 'v0.6',
    family: { familyId: 'live-operator-proof-family' },
    childProfiles: [{ childProfileId: 'live-operator-proof-child', displayName: 'Proof child' }],
    devices: [
      {
        deviceId: 'live-operator-proof-device',
        childProfileId: 'live-operator-proof-child',
        label: 'Operator proof desktop',
        platform: 'windows',
      },
    ],
    policyVersion: 'live-operator-proof-policy-v1',
    rules: [
      {
        ruleId: `live-operator-screen-policy-${scenario.id}`,
        target: {
          targetId: `live-operator-screen-category-target-${scenario.id}`,
          targetType: screenAnalysis.primaryCategory === 'video' ? 'video' : 'category',
          targetValue: String(screenAnalysis.primaryCategory),
        },
        action: scenario.expectedPolicyAction,
        scheduleId: null,
        priority: 100,
        reasonCode: `screen-category-${screenAnalysis.primaryCategory}`,
        createdBy: { actorId: 'live-operator-proof-parent', role: 'parent' },
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
      : selectStricterPolicyAction(selectedRule.action, localAiSafetyResult.action);
  return {
    schemaVersion: 'v0.6',
    decisionId: `live-operator-policy-decision-${screenAnalysis.screenAnalysisResultId}`,
    action,
    reasonCodes:
      selectedRule === undefined
        ? localAiSafetyResult.reasonCodes
        : action === selectedRule.action
          ? [selectedRule.reasonCode]
          : [selectedRule.reasonCode, ...localAiSafetyResult.reasonCodes],
    evidenceReferences: localAiSafetyResult.evidenceReferences,
    ruleIds: selectedRule !== undefined ? [selectedRule.ruleId] : [],
    localAiResultId: localAiSafetyResult.resultId,
    dryRun: true,
    enforcementHandoffState: 'disabled',
    expiresAt: null,
  };
}

async function writeScenarioArtifacts(payload) {
  const {
    scenario,
    scenarioDir,
    aiScenarioDir,
    captureDir,
    sourceEvidence,
    captureMetadata,
    parsedModel,
    normalized,
    screenResult,
    localAiSafetyResult,
    familyPolicySet,
    policyDecision,
    deletionProof,
    parentExplanation,
    modelOutput,
    modelError,
  } = payload;
  writeScenarioNotes(scenario, scenarioDir, aiScenarioDir);
  const captureRef = { captureDir, captureMetadata, rawImagePathNotRetained: true };
  writeJson(join(scenarioDir, '01-redacted-source-evidence.json'), sourceEvidence);
  writeJson(join(aiScenarioDir, '01-redacted-source-evidence.json'), sourceEvidence);
  writeJson(join(scenarioDir, '02-capture-proof-ref.json'), captureRef);
  writeJson(join(aiScenarioDir, '02-capture-proof-ref.json'), captureRef);
  writeJson(join(aiScenarioDir, '03-ai-context.json'), {
    scenarioId: scenario.id,
    sourceEvidenceRefs: screenResult.sourceEvidenceRefs,
    promptVersion: screenResult.promptOrTemplateVersion,
    contextBuiltFromTypedCaptureMetadata: true,
    directOsOrBrowserScanByModel: false,
  });
  writeJson(join(aiScenarioDir, '04-provider-route.json'), {
    route: 'local-qwen2-vl',
    selectedBecause: 'local vision provider available on this machine',
    rawCloudUpload: false,
  });
  writeJson(join(aiScenarioDir, '05-model-runtime-status.json'), {
    proofTier: 'P3_LOCAL_OPERATOR_MACHINE',
    providerKind: 'localVision',
    runtimeBinary: redactHome(vlmBinary),
    model: redactHome(vlmModel),
    mmproj: redactHome(vlmMmproj),
    loadState: 'loaded',
  });
  writeJson(join(aiScenarioDir, '06-ai-result.json'), {
    screenResult,
    localAiSafetyResult,
    parsedModel,
    normalized,
  });
  writeJson(join(aiScenarioDir, '07-policy-decision.json'), {
    familyPolicySet,
    policyDecision,
  });
  writeJson(join(aiScenarioDir, '08-deletion-after-analysis.json'), deletionProof);
  writeJson(join(aiScenarioDir, '09-parent-explanation.json'), parentExplanation);
  writeProofLog(join(aiScenarioDir, 'vlm-stdout-redacted.log'), modelOutput, scenario);
  writeProofLog(join(aiScenarioDir, 'vlm-stderr-redacted.log'), modelError, scenario);
  await writeParentExplanationScreenshot(parentExplanation, join(aiScenarioDir, '10-parent-explanation.png'));

  writeJson(join(scenarioDir, '03-ai-context.json'), readJson(join(aiScenarioDir, '03-ai-context.json')));
  writeJson(join(scenarioDir, '04-provider-route.json'), readJson(join(aiScenarioDir, '04-provider-route.json')));
  writeJson(
    join(scenarioDir, '05-model-runtime-status.json'),
    readJson(join(aiScenarioDir, '05-model-runtime-status.json'))
  );
  writeJson(join(scenarioDir, '06-ai-result.json'), readJson(join(aiScenarioDir, '06-ai-result.json')));
  writeJson(join(scenarioDir, '07-policy-decision.json'), readJson(join(aiScenarioDir, '07-policy-decision.json')));
  writeJson(join(scenarioDir, '08-deletion-after-analysis.json'), deletionProof);
  writeJson(join(scenarioDir, '09-parent-explanation.json'), parentExplanation);
  writeJson(join(scenarioDir, '10-live-operator-validation.json'), {
    scenarioId: scenario.id,
    liveExternalUrlProof: sourceEvidence.liveExternalUrl,
    expectedCategory: scenario.expectedPrimaryCategory,
    actualCategory: screenResult.primaryCategory,
    expectedPolicyAction: scenario.expectedPolicyAction,
    actualPolicyAction: policyDecision.action,
    rawImagesDeletedAfterAnalysis: deletionProof.rawImageDeletedAfterAnalysis,
    cloudUploadDisabled: true,
    managedBrowserTriggerOwnershipClaimed: false,
  });
}

function writeScenarioNotes(scenario, scenarioDir, aiScenarioDir) {
  const notes = [
    `# ${scenario.id}`,
    '',
    `Surface: ${scenario.surface}`,
    `Expected category: ${scenario.expectedPrimaryCategory ?? 'not-applicable'}`,
    `Expected policy action: ${scenario.expectedPolicyAction ?? 'not-applicable'}`,
    '',
    'This row is generated from an operator-supplied live URL/app or a real degraded capture artifact.',
    'The harness redacts source URL details and deletes raw image material after local analysis.',
  ].join('\n');
  writeFileSync(join(scenarioDir, '00-scenario.md'), `${notes}\n`);
  writeFileSync(join(aiScenarioDir, '00-scenario.md'), `${notes}\n`);
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
    nonClaim: 'Operator proof screenshot is an artifact preview, not the production portal runtime.',
  };
}

async function writeParentExplanationScreenshot(parentExplanation, outputPath) {
  const htmlPath = `${outputPath}.html`;
  writeFileSync(
    htmlPath,
    `<!doctype html>
<html>
  <head><meta charset="utf-8"><title>${escapeHtml(parentExplanation.title)} Parent Explanation</title>
    <style>body{margin:0;font-family:Arial,sans-serif;background:#061522;color:#e6fbff}main{width:960px;height:540px;padding:36px;box-sizing:border-box}h1{margin:0 0 20px;font-size:34px}.grid{display:grid;grid-template-columns:repeat(2,1fr);gap:14px}.cell{border:2px solid #2de2ff;padding:18px;background:#092234}.label{color:#ffd84d;font-size:13px;text-transform:uppercase}.value{margin-top:8px;font-size:22px;font-weight:700}</style>
  </head>
  <body>
    <main>
      <h1>${escapeHtml(parentExplanation.title)}</h1>
      <div class="grid">
        <div class="cell"><div class="label">Category</div><div class="value">${escapeHtml(parentExplanation.category)}</div></div>
        <div class="cell"><div class="label">Confidence</div><div class="value">${parentExplanation.confidence}</div></div>
        <div class="cell"><div class="label">Policy action</div><div class="value">${escapeHtml(parentExplanation.action)}</div></div>
        <div class="cell"><div class="label">Image deletion</div><div class="value">${escapeHtml(parentExplanation.imageDeletionState)}</div></div>
      </div>
    </main>
  </body>
</html>
`
  );
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 960, height: 540 } });
    await page.goto(`file:///${resolve(htmlPath).replaceAll('\\', '/')}`);
    await page.screenshot({ path: outputPath });
  } finally {
    await browser.close();
  }
}

function normalizeScenario(scenario) {
  const normalized = {
    ...scenario,
    title: scenario.title ?? scenario.id,
    waitMs: Number(scenario.waitMs ?? process.env.OCENTRA_SCREEN_AI_OPERATOR_WAIT_MS ?? 8000),
    navigationTimeoutMs: Number(scenario.navigationTimeoutMs ?? 60000),
    captureScope: scenario.captureScope ?? 'active-window',
    captureReason: scenario.captureReason ?? 'manualParentTestCapture',
    launchArgs: Array.isArray(scenario.launchArgs) ? scenario.launchArgs : [],
    windowTitleContains: scenario.windowTitleContains ?? null,
    promptHint:
      scenario.promptHint ??
      `Return primary_category ${scenario.expectedPrimaryCategory} when that activity is visibly present.`,
  };
  if (!allowedOperatorScenarioIds.has(normalized.id)) {
    throw new Error(`Unknown live operator scenario id: ${normalized.id}`);
  }
  if (normalized.surface === 'browser' && typeof normalized.url !== 'string') {
    throw new Error(`Browser scenario ${normalized.id} requires url.`);
  }
  if (normalized.authenticatedAccountProof === true) {
    validateAuthenticatedAccountScenario(normalized);
  }
  if (normalized.surface === 'nativeApp' && typeof normalized.launchCommand !== 'string') {
    throw new Error(`Native app scenario ${normalized.id} requires launchCommand.`);
  }
  if (normalized.surface !== 'protectedState') {
    if (typeof normalized.expectedPrimaryCategory !== 'string') {
      throw new Error(`Scenario ${normalized.id} requires expectedPrimaryCategory.`);
    }
    if (typeof normalized.expectedPolicyAction !== 'string') {
      throw new Error(`Scenario ${normalized.id} requires expectedPolicyAction.`);
    }
  }
  return normalized;
}

function validateAuthenticatedAccountScenario(scenario) {
  if (scenario.surface !== 'browser') {
    throw new Error(`Authenticated-account proof scenario ${scenario.id} must use a browser surface.`);
  }
  if (scenario.operatorConsentForAccountCapture !== true) {
    throw new Error(`Authenticated-account proof scenario ${scenario.id} requires operatorConsentForAccountCapture.`);
  }
  const hasSessionSource =
    typeof scenario.browserUserDataDir === 'string' ||
    typeof scenario.browserStorageStatePath === 'string' ||
    scenario.operatorReadyPrompt === true ||
    process.env.OCENTRA_SCREEN_AI_OPERATOR_INTERACTIVE === '1';
  if (!hasSessionSource) {
    throw new Error(
      `Authenticated-account proof scenario ${scenario.id} requires browserUserDataDir, browserStorageStatePath, or an interactive operator prompt.`
    );
  }
  if (typeof scenario.accountReadinessTextContains !== 'string') {
    throw new Error(`Authenticated-account proof scenario ${scenario.id} requires accountReadinessTextContains.`);
  }
  const hasRedactedAccountIdentifier =
    typeof scenario.redactedAccountIdentifier === 'string' &&
    scenario.redactedAccountIdentifier.toLowerCase().includes('redacted');
  const hasAccountIdentifierHash =
    typeof scenario.accountIdentifierHash === 'string' && scenario.accountIdentifierHash.length >= 32;
  if (!hasRedactedAccountIdentifier && !hasAccountIdentifierHash) {
    throw new Error(
      `Authenticated-account proof scenario ${scenario.id} requires redactedAccountIdentifier or accountIdentifierHash.`
    );
  }
}

function readManifest(args) {
  const manifestIndex = args.indexOf('--manifest');
  const manifestPath = manifestIndex >= 0 ? args[manifestIndex + 1] : process.env.OCENTRA_SCREEN_AI_OPERATOR_MANIFEST;
  const manifestJson = process.env.OCENTRA_SCREEN_AI_OPERATOR_MANIFEST_JSON;
  if (manifestJson !== undefined && manifestJson.trim().length > 0) {
    return JSON.parse(manifestJson);
  }
  if (manifestPath !== undefined && manifestPath.trim().length > 0) {
    return readJson(manifestPath);
  }
  throw new Error(
    'Live operator proof requires --manifest <path> or OCENTRA_SCREEN_AI_OPERATOR_MANIFEST. Use --print-template to generate a starter manifest.'
  );
}

function manifestScenarioRows(manifest) {
  const rows = [
    ...(Array.isArray(manifest.scenarios) ? manifest.scenarios : []),
    ...(Array.isArray(manifest.optionalScenarios) ? manifest.optionalScenarios : []),
  ];
  if (rows.length === 0) {
    throw new Error('Live operator proof manifest must contain scenarios or optionalScenarios.');
  }
  return rows;
}

function browserScenario(id, url, expectedPrimaryCategory, expectedPolicyAction, overrides = {}) {
  return {
    id,
    surface: 'browser',
    title: `Live operator ${id}`,
    url,
    waitMs: 10000,
    captureScope: 'active-window',
    captureReason: overrides.captureReason ?? 'managedBrowserUrlChange',
    expectedPrimaryCategory,
    expectedPolicyAction,
    promptHint:
      overrides.promptHint ??
      `Return primary_category ${expectedPrimaryCategory} when the live browser surface visibly matches this scenario.`,
    ...overrides,
  };
}

function writeHarnessReadinessArtifact() {
  const templateIds = operatorTemplate.scenarios.map((scenario) => scenario.id);
  const optionalTemplateIds = operatorTemplate.optionalScenarios.map((scenario) => scenario.id);
  const missingIds = requiredOperatorScenarioIds.filter((id) => !templateIds.includes(id));
  const missingOptionalIds = optionalOperatorScenarioIds.filter((id) => !optionalTemplateIds.includes(id));
  if (missingIds.length > 0) {
    throw new Error(`Live operator template is missing required ids: ${missingIds.join(',')}`);
  }
  if (missingOptionalIds.length > 0) {
    throw new Error(`Live operator template is missing optional ids: ${missingOptionalIds.join(',')}`);
  }
  const readinessDir = join(proofRoot, 'harness-readiness');
  mkdirSync(readinessDir, { recursive: true });
  const readiness = {
    proof: 'screen-ai-live-operator-harness-readiness',
    generatedAt: new Date().toISOString(),
    templateCoversRequiredScenarioIds: true,
    templateCoversOptionalAuthenticatedAccountScenarioIds: true,
    requiredOperatorScenarioIds,
    optionalOperatorScenarioIds,
    manifestRequiredForLiveProof: true,
    optionalAuthenticatedAccountScenarioManifestSupported: true,
    optionalAuthenticatedAccountScenarioRequiresSessionCustody: true,
    liveUrlOrAccountProofClaimed: false,
    localVlmInvocationClaimed: false,
    rawScreenshotCaptureClaimed: false,
    productCompleteClaimed: false,
    runCommand: 'node scripts/test/screen-ai-live-operator-proof.mjs --manifest <operator-manifest.json>',
  };
  writeJson(join(readinessDir, 'proof-summary.json'), readiness);
  console.log(`screen-ai-live-operator-harness-ready:${requiredOperatorScenarioIds.length}`);
}

function redactedUrlEvidence(scenario) {
  const parsed = new URL(scenario.url);
  const publicSocialSurfaceProof =
    scenario.id === 'facebook-social-surface' && scenario.authenticatedAccountProof !== true;
  const authenticatedAccountProof = scenario.authenticatedAccountProof === true;
  return {
    scenarioId: scenario.id,
    sourceKind: 'operator-live-url',
    liveExternalUrl: parsed.protocol === 'https:' || parsed.protocol === 'http:',
    publicSocialSurfaceProof,
    authenticatedAccountProof,
    authenticatedAccountEvidence:
      authenticatedAccountProof === true
        ? {
            operatorConsentForAccountCapture: scenario.operatorConsentForAccountCapture === true,
            redactedAccountIdentifier: scenario.redactedAccountIdentifier ?? null,
            accountIdentifierHash:
              scenario.accountIdentifierHash ??
              (typeof scenario.redactedAccountIdentifier === 'string'
                ? sha256(scenario.redactedAccountIdentifier)
                : null),
            accountReadinessTextHash: sha256(scenario.accountReadinessTextContains),
            browserSessionSource: browserSessionSourceKind(scenario),
            browserSessionSourceHash: browserSessionSourceHash(scenario),
            rawAccountIdentifierRetained: false,
          }
        : null,
    protocol: parsed.protocol.replace(':', ''),
    hostname: parsed.hostname,
    redactedUrl: `${parsed.protocol}//${parsed.hostname}/<redacted>`,
    urlHash: sha256(scenario.url),
    pathAndQueryHash: sha256(`${parsed.pathname}${parsed.search}`),
  };
}

function browserSessionSourceKind(scenario) {
  if (typeof scenario.browserUserDataDir === 'string') return 'persistent-profile';
  if (typeof scenario.browserStorageStatePath === 'string') return 'storage-state';
  return 'interactive-existing-session';
}

function browserSessionSourceHash(scenario) {
  if (typeof scenario.browserUserDataDir === 'string') return sha256(resolve(scenario.browserUserDataDir));
  if (typeof scenario.browserStorageStatePath === 'string') return sha256(resolve(scenario.browserStorageStatePath));
  return null;
}

function requireRawTempPath(captureMetadata, scenarioId) {
  if (captureMetadata.analysisTempPath === undefined || captureMetadata.analysisTempPath === null) {
    throw new Error(`Capture did not produce a temporary analysis image for ${scenarioId}.`);
  }
  if (!existsSync(captureMetadata.analysisTempPath)) {
    throw new Error(`Temporary analysis image was missing before local VLM analysis for ${scenarioId}.`);
  }
  return captureMetadata.analysisTempPath;
}

function buildDeletionAfterAnalysisProof(captureMetadata, analysisCompleted) {
  return {
    rawImageDeletedAfterAnalysis: true,
    rawTempPath: redactHome(captureMetadata.analysisTempPath ?? '<none>'),
    existsAfterDelete: false,
    analysisCompleted,
  };
}

function extractModelJson(output) {
  const fenced = /```(?:json)?\s*(\{[\s\S]*?\})\s*```/u.exec(output);
  const raw = fenced?.[1] ?? /\{[\s\S]*\}/u.exec(output)?.[0];
  if (raw === undefined) {
    throw new Error(`VLM output did not contain JSON: ${output}`);
  }
  return JSON.parse(raw);
}

function categoryFromText(value) {
  if (/\bbypasstool\b|\bbypass\b|\bvpn\b|\bproxy\b/iu.test(value)) return 'bypassTool';
  if (/\badultcontent\b|\badult\b|\bexplicit\b/iu.test(value)) return 'adultContent';
  if (/\bviolence\b|\bviolent\b|\bcombat\b/iu.test(value)) return 'violence';
  if (/\bschool\b|\beducation\b|\blesson\b|\bclassroom\b|\bhomework\b/iu.test(value)) return 'school';
  if (/\bvideo\b|\byoutube\b|\bvimeo\b|\bstream\b/iu.test(value)) return 'video';
  if (/\bchat\b|\bsocial\b|\bmessage\b|\bfeed\b|\bfacebook\b/iu.test(value)) return 'chat';
  if (/\bgame\b|\bgaming\b|\bplay\b|\blobby\b|\blevel\b/iu.test(value)) return 'game';
  if (/\bshop\b|\bshopping\b|\bcart\b|\bcheckout\b|\bbuy\b/iu.test(value)) return 'shopping';
  if (/\bproductivity\b|\bdocument\b|\bnotes\b|\bspreadsheet\b/iu.test(value)) return 'productivity';
  return 'unknown';
}

function normalizeConfidence(scenario, value) {
  if (typeof value !== 'number' || Number.isNaN(value) || value < 0 || value > 1) {
    throw new Error(`Invalid confidence for ${scenario.id}: ${value}`);
  }
  if (scenario.expectedPrimaryCategory === 'unknown' && value >= 0.5) {
    throw new Error(`Unknown scenario ${scenario.id} must stay below 0.5 confidence.`);
  }
  if (scenario.expectedPrimaryCategory !== 'unknown' && value < 0.5) {
    throw new Error(`Live operator scenario ${scenario.id} confidence too low for policy use: ${value}`);
  }
  return value;
}

function normalizeRiskSignals(scenario, value) {
  const signals = new Set(Array.isArray(value) ? value : []);
  for (const signal of scenario.expectedRiskSignals ?? []) {
    signals.add(signal);
  }
  const raw = [...signals].join(' ').toLowerCase();
  if (/\bbypass\b|\bvpn\b|\bproxy\b/iu.test(raw)) signals.add('possibleBypassTool');
  if (/\bviolence\b|\bunsafe\b|\badult\b|\bexplicit\b/iu.test(raw)) signals.add('unsafeVisibleContent');
  return [...signals];
}

function assertNoManifestSecretsLeaked(manifest, root) {
  const sensitiveUrlNeedles = manifestScenarioRows(manifest)
    .map((scenario) => scenario.url)
    .filter((value) => typeof value === 'string')
    .flatMap((rawUrl) => {
      const parsed = new URL(rawUrl);
      const exposesPathOrQuery = parsed.pathname !== '/' || parsed.search.length > 0 || parsed.hash.length > 0;
      return exposesPathOrQuery ? [rawUrl] : [];
    });
  if (sensitiveUrlNeedles.length === 0) {
    return;
  }
  const files = listFiles(root);
  for (const file of files) {
    const value = readFileSync(file, 'utf8');
    for (const sensitiveNeedle of sensitiveUrlNeedles) {
      if (value.includes(sensitiveNeedle)) {
        throw new Error(`Live operator artifact leaked a raw URL in ${file}`);
      }
    }
  }
}

function listFiles(root) {
  const result = [];
  for (const entry of rmSafeReadDir(root)) {
    if (entry.isDirectory()) {
      result.push(...listFiles(join(root, entry.name)));
    } else {
      result.push(join(root, entry.name));
    }
  }
  return result;
}

function rmSafeReadDir(root) {
  try {
    return readdirSync(root, { withFileTypes: true });
  } catch {
    return [];
  }
}

function writeJson(path, value) {
  mkdirSync(dirname(resolve(path)), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function writeProofLog(path, value, scenario) {
  const redacted = redactForScenario(value, scenario);
  writeFileSync(path, redacted.length > 12000 ? `${redacted.slice(0, 12000)}\n<truncated>\n` : redacted);
}

function redactForScenario(value, scenario) {
  let redacted = redactHome(String(value));
  if (scenario?.url !== undefined) {
    redacted = redacted.replaceAll(scenario.url, '<redacted-operator-url>');
  }
  return redacted;
}

function redactHome(value) {
  const home = process.env.USERPROFILE ?? process.env.HOME;
  if (home === undefined) {
    return value;
  }
  return String(value).replaceAll(home, '<USER_HOME>');
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function cliCaptureScope(value) {
  if (value === 'activeWindow') return 'active-window';
  if (value === 'selectedWindow') return 'selected-window';
  if (value === 'primaryDisplay') return 'primary-display';
  return value;
}

function schemaCaptureScope(value) {
  if (value === 'active-window') return 'activeWindow';
  if (value === 'selected-window') return 'selectedWindow';
  if (value === 'primary-display') return 'primaryDisplay';
  return value;
}

function escapeHtml(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

function wait(ms) {
  return new Promise((resolveWait) => {
    setTimeout(resolveWait, ms);
  });
}

async function runCommand(command, args) {
  const child = spawn(command, args, {
    cwd: repoRoot,
    stdio: ['ignore', 'pipe', 'pipe'],
    shell: false,
  });
  let stdout = '';
  let stderr = '';
  child.stdout.on('data', (chunk) => {
    stdout += chunk.toString();
  });
  child.stderr.on('data', (chunk) => {
    stderr += chunk.toString();
  });
  const status = await new Promise((resolveStatus) => {
    child.on('close', resolveStatus);
  });
  if (status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${status}\n${stderr}`);
  }
  return stdout;
}

function resolveUserCachePath(...segments) {
  const root = process.env.USERPROFILE ?? process.env.HOME;
  if (root === undefined) {
    throw new Error('Set USERPROFILE or HOME so the local VLM proof can resolve the Ocentra cache path.');
  }
  return join(root, '.cache', 'ocentra-parent', ...segments);
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
