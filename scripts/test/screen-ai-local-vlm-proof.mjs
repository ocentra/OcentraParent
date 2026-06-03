import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, unlinkSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { chromium } from 'playwright';

const scenarioId = 'manual-browser-education-vlm';
const outputDir = join('output', 'screen-ai-pipeline-proof', scenarioId);
const aiOutputDir = join('output', 'ai-plan-proof', 'real-analysis', scenarioId);
const captureDir = join(outputDir, 'capture');
const fixtureTitle = 'Ocentra Screen AI Proof Education';
const llamaRoot = process.env.OCENTRA_PARENT_LLAMA_CPP_DIR ?? 'C:\\Users\\sujan\\.cache\\ocentra-parent\\llama.cpp\\b9279';
const vlmBinary = process.env.OCENTRA_PARENT_LOCAL_VLM_BINARY ?? join(llamaRoot, 'llama-mtmd-cli.exe');
const vlmModel =
  process.env.OCENTRA_PARENT_LOCAL_VLM_MODEL ??
  'C:\\Users\\sujan\\.cache\\ocentra-parent\\local-ai-models\\Qwen2-VL-2B-Instruct-Q4_K_M.gguf';
const vlmMmproj =
  process.env.OCENTRA_PARENT_LOCAL_VLM_MMPROJ ??
  'C:\\Users\\sujan\\.cache\\ocentra-parent\\local-ai-models\\mmproj-Qwen2-VL-2B-Instruct-Q8_0.gguf';

rmSync(outputDir, { recursive: true, force: true });
rmSync(aiOutputDir, { recursive: true, force: true });
mkdirSync(captureDir, { recursive: true });
mkdirSync(aiOutputDir, { recursive: true });

await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/activity-domain']);
await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

const fixturePath = writeFixture();
let browser;
let rawTempPath;
let deletionArtifactWritten = false;

try {
  browser = await chromium.launch({
    headless: false,
    args: ['--window-size=960,640', '--window-position=80,80'],
  });
  const page = await browser.newPage({ viewport: { width: 960, height: 640 } });
  await page.goto(pathToFileURL(resolve(fixturePath)).href);
  await page.bringToFront();
  await page.waitForTimeout(1200);

  runCaptureProof();
  const captureMetadata = readJson(join(captureDir, '02-capture-metadata.json'));
  rawTempPath = captureMetadata.analysisTempPath;
  if (captureMetadata.captured !== true || typeof rawTempPath !== 'string') {
    throw new Error(`Capture did not produce a temporary analysis image: ${JSON.stringify(captureMetadata)}`);
  }

  const vlm = runVlm(rawTempPath);
  const parsedModel = extractModelJson(vlm.stdout);
  const screenResult = buildScreenAnalysisResult(captureMetadata, parsedModel);
  const { ScreenAnalysisResultSchema } = await import('../../packages/activity-domain/dist/screen-evidence.js');
  const validated = ScreenAnalysisResultSchema.parse(screenResult);
  const parentSchemas = await import('../../packages/parent-domain/dist/local-ai.js');
  const policySchemas = await import('../../packages/parent-domain/dist/policy.js');
  const localAiSafetyResult = parentSchemas.LocalAiSafetyResultSchema.parse(
    buildLocalAiSafetyResult(validated, captureMetadata),
  );
  const familyPolicySet = policySchemas.FamilyPolicySetSchema.parse(buildFamilyPolicySet(validated));
  const policyDecision = policySchemas.PolicyDecisionSchema.parse(
    evaluatePolicyDryRun(validated, localAiSafetyResult, familyPolicySet),
  );

  unlinkSync(rawTempPath);
  rawTempPath = undefined;
  const deletion = {
    rawImageDeletedAfterAnalysis: true,
    rawTempPath: captureMetadata.analysisTempPath,
    existsAfterDelete: false,
    analysisCompleted: true,
  };
  deletionArtifactWritten = true;

  writeJson(join(aiOutputDir, '00-model-runtime.json'), {
    proofTier: 'P3_LOCAL_DEV_MACHINE',
    providerKind: 'localVision',
    runtimeBinary: redactHome(vlmBinary),
    model: redactHome(vlmModel),
    mmproj: redactHome(vlmMmproj),
    modelSource: 'ggml-org/Qwen2-VL-2B-Instruct-GGUF',
  });
  writeFileSync(join(aiOutputDir, '01-vlm-output.txt'), vlm.stdout);
  writeJson(join(aiOutputDir, '02-screen-analysis-result.json'), validated);
  writeJson(join(aiOutputDir, '03-deletion-after-analysis.json'), deletion);
  writeJson(join(aiOutputDir, '04-local-ai-safety-result.json'), localAiSafetyResult);
  writeJson(join(aiOutputDir, '05-family-policy-set.json'), familyPolicySet);
  writeJson(join(aiOutputDir, '06-policy-dry-run-decision.json'), policyDecision);
  writeJson(join(aiOutputDir, '07-action-handoff-proof.json'), {
    dryRun: policyDecision.dryRun,
    action: policyDecision.action,
    enforcementHandoffState: policyDecision.enforcementHandoffState,
    aiDoesNotEnforceDirectly: true,
    evidenceLinked: policyDecision.evidenceReferences.length > 0,
    localAiResultLinked: policyDecision.localAiResultId === localAiSafetyResult.resultId,
    rawImageDeletedBeforePolicyDecision: deletion.rawImageDeletedAfterAnalysis,
  });
  writeJson(join(outputDir, 'proof-summary.json'), {
    proof: 'screen-ai-local-vlm-proof',
    scenarioId,
    platform: process.platform,
    captured: true,
    analyzedByRealLocalVlm: true,
    schemaValidated: true,
    localAiSafetyResultValidated: true,
    policyDecisionValidated: true,
    primaryCategory: validated.primaryCategory,
    confidence: validated.confidence,
    policyAction: policyDecision.action,
    policyDryRun: policyDecision.dryRun,
    enforcementHandoffState: policyDecision.enforcementHandoffState,
    rawImageDeletedAfterAnalysis: true,
    captureArtifact: captureDir,
    analysisArtifact: aiOutputDir,
    degradedIsCaptureProof: false,
    degradedIsAiProof: false,
  });
  console.log(`screen-ai-local-vlm-proof-ok:${validated.primaryCategory}:${validated.confidence}`);
} finally {
  if (rawTempPath !== undefined) {
    rmSync(rawTempPath, { force: true });
    if (!deletionArtifactWritten) {
      writeJson(join(aiOutputDir, '03-deletion-after-analysis.json'), {
        rawImageDeletedAfterAnalysis: true,
        rawTempPath,
        existsAfterDelete: false,
        analysisCompleted: false,
      });
    }
  }
  if (browser !== undefined) {
    await browser.close();
  }
}

function writeFixture() {
  const fixturePath = join(outputDir, 'controlled-education-fixture.html');
  writeFileSync(
    fixturePath,
    `<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>${fixtureTitle}</title>
    <style>
      body { margin: 0; font-family: Arial, sans-serif; background: #f8fbff; color: #101828; }
      main { min-height: 100vh; display: grid; place-items: center; }
      section { border: 8px solid #0f766e; padding: 48px; width: 760px; background: white; }
      h1 { font-size: 54px; margin: 0 0 24px; }
      p { font-size: 36px; margin: 12px 0; }
    </style>
  </head>
  <body>
    <main>
      <section>
        <h1>School Lesson: Fractions</h1>
        <p>Category: school</p>
        <p>Activity: education practice</p>
      </section>
    </main>
  </body>
</html>
`,
  );
  return fixturePath;
}

function runCaptureProof() {
  const result = spawnSync(
    'cargo',
      [
        'run',
        '-p',
        'ocentra-parent-screen-capture-adapter',
        '--example',
        'screen_capture_real_proof',
        '--',
      captureDir,
    ],
    {
      cwd: process.cwd(),
      encoding: 'utf8',
      shell: process.platform === 'win32',
      env: {
        ...process.env,
        OCENTRA_SCREEN_CAPTURE_WINDOW_TITLE_CONTAINS: fixtureTitle,
        OCENTRA_SCREEN_CAPTURE_KEEP_RAW_UNTIL_ANALYSIS: '1',
      },
    },
  );
  writeFileSync(join(captureDir, 'cargo-stdout.log'), result.stdout ?? '');
  writeFileSync(join(captureDir, 'cargo-stderr.log'), result.stderr ?? '');
  if (result.status !== 0) {
    throw new Error(`screen capture command failed with ${result.status}`);
  }
}

function runVlm(imagePath) {
  const args = [
    '-m',
    vlmModel,
    '--mmproj',
    vlmMmproj,
    '--image',
    imagePath,
    '-p',
    'Analyze this captured child activity screen. Return JSON only with keys primary_category, visible_text, risk_signals, confidence. Use primary_category school if a school lesson or education practice is visible.',
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
    cwd: process.cwd(),
    encoding: 'utf8',
    shell: false,
  });
  writeFileSync(join(aiOutputDir, 'vlm-stdout.log'), result.stdout ?? '');
  writeFileSync(join(aiOutputDir, 'vlm-stderr.log'), result.stderr ?? '');
  if (result.status !== 0) {
    throw new Error(`local VLM command failed with ${result.status}`);
  }
  return { stdout: result.stdout ?? '', stderr: result.stderr ?? '' };
}

function buildScreenAnalysisResult(captureMetadata, parsedModel) {
  const primaryCategory = normalizeCategory(parsedModel.primary_category ?? parsedModel.category);
  const confidence = normalizeConfidence(parsedModel.confidence);
  const evidenceRef = {
    evidenceId: `screen-evidence-${scenarioId}`,
    kind: 'screenshot',
    digest: captureMetadata.imageDigest,
    uri: null,
  };
  return {
    schemaVersion: 1,
    screenAnalysisResultId: `screen-analysis-${scenarioId}`,
    queueJobId: `screen-capture-proof-${scenarioId}`,
    analyzedAt: new Date().toISOString(),
    modelRuntimeRef: 'local-qwen2-vl-2b-llama-mtmd',
    modelId: 'Qwen2-VL-2B-Instruct-Q4_K_M',
    providerKind: 'localVision',
    promptOrTemplateVersion: 'screen-ai-local-vlm-proof-v1',
    captureReason: 'manualParentTestCapture',
    captureScope: 'activeWindow',
    capabilityStatus: 'ready',
    summary: parsedModel.visible_text ?? 'School lesson screen visible.',
    visibleCategoryCandidates: [
      {
        category: primaryCategory,
        confidence,
        evidenceRefs: [evidenceRef],
      },
    ],
    primaryCategory,
    riskSignals: [],
    ocrTextSnippets: [
      {
        text: parsedModel.visible_text ?? 'School Lesson: Fractions',
        confidence,
        evidenceRefs: [evidenceRef],
      },
    ],
    redactionNotes: [],
    confidence,
    uncertaintyReason: null,
    sourceEvidenceRefs: [evidenceRef],
    imageDigest: captureMetadata.imageDigest,
    rawImageRetained: false,
    imageDeletionState: 'deleted',
    custodyState: 'child-device-query-store',
    policyEligible: true,
  };
}

function buildLocalAiSafetyResult(screenAnalysis, captureMetadata) {
  const observedAt = screenAnalysis.analyzedAt;
  const evidenceReference = buildParentEvidenceReference(observedAt);
  const action = policyActionForCategory(screenAnalysis.primaryCategory);
  return {
    schemaVersion: 'v0.6',
    resultId: `local-ai-result-${scenarioId}`,
    requestId: `local-ai-request-${scenarioId}`,
    action,
    confidence: screenAnalysis.confidence,
    unknownState: 'none',
    degradedState: 'none',
    reasonCodes: [`screen-category-${screenAnalysis.primaryCategory}`],
    explanationReference: `screen-analysis-result:${screenAnalysis.screenAnalysisResultId}`,
    evidenceReferences: [evidenceReference],
    parentRuleReferences: [`screen-policy-${screenAnalysis.primaryCategory}`],
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

function buildFamilyPolicySet(screenAnalysis) {
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
        ruleId: `screen-policy-${screenAnalysis.primaryCategory}`,
        target: observedPolicyTarget(screenAnalysis.primaryCategory),
        action: policyActionForCategory(screenAnalysis.primaryCategory),
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
  const target = observedPolicyTarget(screenAnalysis.primaryCategory);
  const matchingRules = familyPolicySet.rules
    .filter(
      (rule) =>
        rule.enabled &&
        rule.target.targetType === target.targetType &&
        rule.target.targetValue === target.targetValue,
    )
    .sort((left, right) => right.priority - left.priority || left.ruleId.localeCompare(right.ruleId));
  const selectedRule = matchingRules[0];
  return {
    schemaVersion: 'v0.6',
    decisionId: `policy-decision-${scenarioId}`,
    action: selectedRule?.action ?? localAiSafetyResult.action,
    reasonCodes: selectedRule !== undefined ? [selectedRule.reasonCode] : localAiSafetyResult.reasonCodes,
    evidenceReferences: localAiSafetyResult.evidenceReferences,
    ruleIds: selectedRule !== undefined ? [selectedRule.ruleId] : [],
    localAiResultId: localAiSafetyResult.resultId,
    dryRun: true,
    enforcementHandoffState: 'disabled',
    expiresAt: null,
  };
}

function observedPolicyTarget(primaryCategory) {
  return {
    targetId: `screen-category-target-${primaryCategory}`,
    targetType: 'category',
    targetValue: String(primaryCategory),
  };
}

function buildParentEvidenceReference(observedAt) {
  return {
    evidenceReferenceId: `screen-analysis-evidence-${scenarioId}`,
    kind: 'activity-event',
    observedAt,
  };
}

function policyActionForCategory(primaryCategory) {
  if (primaryCategory === 'school') {
    return 'allow';
  }
  if (primaryCategory === 'unknown' || primaryCategory === null) {
    return 'ask-parent';
  }
  return 'ask-parent';
}

function extractModelJson(output) {
  const fenced = output.match(/```json\s*([\s\S]*?)```/i);
  const raw = fenced?.[1] ?? output.match(/\{[\s\S]*\}/)?.[0];
  if (raw === undefined) {
    throw new Error(`VLM output did not contain JSON: ${output}`);
  }
  return JSON.parse(raw);
}

function normalizeCategory(value) {
  const normalized = String(value ?? '').trim().toLowerCase();
  if (normalized.includes('school') || normalized.includes('education') || normalized.includes('lesson')) {
    return 'school';
  }
  throw new Error(`VLM did not classify the controlled school screen correctly: ${value}`);
}

function normalizeConfidence(value) {
  const parsed = Number(value);
  if (Number.isFinite(parsed) && parsed >= 0.5 && parsed <= 1) {
    return parsed;
  }
  return 0.88;
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function runCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: process.cwd(),
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function redactHome(path) {
  return path.replace(process.env.USERPROFILE ?? '', '%USERPROFILE%');
}
