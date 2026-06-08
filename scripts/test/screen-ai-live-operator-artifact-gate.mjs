import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';

const repoRoot = process.cwd();
const pipelineRoot = join(repoRoot, 'output', 'screen-ai-pipeline-proof', 'live-operator');
const aiRoot = join(repoRoot, 'output', 'ai-plan-proof', 'live-operator');
const gatePath = join(
  repoRoot,
  'output',
  'screen-ai-pipeline-proof',
  'live-operator-artifact-gate',
  'proof-summary.json'
);

const requiredRows = [
  row('youtube-ordinary-video', 'browser', 'video', 'warn'),
  row('youtube-education-video', 'browser', 'school', 'allow'),
  row('vimeo-video', 'browser', 'video', 'warn'),
  row('facebook-social-surface', 'browser', 'chat', 'warn'),
  row('browser-game', 'browser', 'game', 'time-limit'),
  row('shopping-page', 'browser', 'shopping', 'ask-parent'),
  row('school-productivity', 'browser', 'school', 'allow'),
  row('native-app', 'nativeApp', 'productivity', 'allow'),
  row('protected-unsupported-state', 'protectedState', null, null),
];

const failures = [];
const pipelineSummary = readJson(join(pipelineRoot, 'proof-summary.json'));
const aiSummary = readJson(join(aiRoot, 'proof-summary.json'));

validateSummary('pipeline', pipelineSummary);
validateSummary('ai', aiSummary);
compareSummaries();
const scenarioResults = requiredRows.map(validateScenario);

if (failures.length > 0) {
  throw new Error(`screen-ai-live-operator-artifact-gate failed:\n${failures.join('\n')}`);
}

writeProof({
  proof: 'screen-ai-live-operator-artifact-gate',
  generatedAt: new Date().toISOString(),
  validatedPipelineSummary: relative(join(pipelineRoot, 'proof-summary.json')),
  validatedAiSummary: relative(join(aiRoot, 'proof-summary.json')),
  requiredScenarioCount: requiredRows.length,
  validatedScenarioCount: scenarioResults.length,
  realLiveUrlRows: scenarioResults.filter((entry) => entry.liveExternalUrlProof).length,
  publicSocialSurfaceRows: scenarioResults.filter((entry) => entry.publicSocialSurfaceProof).length,
  authenticatedAccountSocialProof: false,
  localVlmRows: scenarioResults.filter((entry) => entry.localVlmAnalysisProof).length,
  policyDryRunRows: scenarioResults.filter((entry) => entry.policyDryRunProof).length,
  rawImagesDeletedAfterAnalysis: true,
  productCompleteClaimed: false,
  managedBrowserTriggerOwnershipClaimed: false,
  scenarios: scenarioResults,
  nonClaims: [
    'This gate verifies existing operator-run live artifacts; it does not rerun browser capture or model inference.',
    'The retained Facebook/social row proves a public live social/feed surface only and does not claim authenticated-account social coverage.',
    'Managed-browser trigger integration and broad browser/network/mobile adapters remain separate gates.',
    'The verified artifacts keep raw screenshots local and deleted after analysis.',
  ],
});

console.log(`screen-ai-live-operator-artifact-gate-ok:${scenarioResults.length}:${relative(gatePath)}`);

function row(id, surface, category, action) {
  return { id, surface, category, action };
}

function validateSummary(label, summary) {
  assert(summary.proof === 'screen-ai-live-operator-proof', `${label} summary has unexpected proof id`);
  assert(summary.proofTier === 'P3_LOCAL_OPERATOR_MACHINE', `${label} summary has unexpected proof tier`);
  assert(summary.scenarioCount === requiredRows.length, `${label} summary scenario count mismatch`);
  assert(summary.passedScenarioCount === requiredRows.length, `${label} summary passed count mismatch`);
  assert(summary.fullRequiredMatrixComplete === true, `${label} summary matrix is incomplete`);
  assert(summary.rawImagesDeletedAfterAnalysis === true, `${label} summary does not prove raw deletion`);
  assert(summary.liveExternalUrlProof === true, `${label} summary does not prove live URL rows`);
  assert(summary.localVlmAnalysisProof === true, `${label} summary does not prove local VLM`);
  assert(summary.policyDryRunProof === true, `${label} summary does not prove policy dry-run`);
  assert(summary.controlledFixtureProof === false, `${label} summary claims controlled fixture proof`);
  assert(summary.productCompleteClaimed === false, `${label} summary must not claim product complete`);
  for (const required of requiredRows) {
    assert(summary.requiredScenarioStatus?.[required.id] === true, `${label} summary missing ${required.id}`);
  }
}

function compareSummaries() {
  assert(
    aiSummary.generatedAt === pipelineSummary.generatedAt,
    'AI and pipeline summaries were not generated together'
  );
  assert(aiSummary.passedScenarioCount === pipelineSummary.passedScenarioCount, 'AI and pipeline pass counts differ');
}

function validateScenario(required) {
  if (required.surface === 'protectedState') {
    return validateProtectedScenario(required);
  }
  const scenarioSummary = summaryScenario(required.id);
  const source = readScenarioJson(required.id, '01-redacted-source-evidence.json');
  const capture = readScenarioJson(required.id, '02-capture-proof-ref.json');
  const aiContext = readScenarioJson(required.id, '03-ai-context.json');
  const provider = readScenarioJson(required.id, '04-provider-route.json');
  const runtime = readScenarioJson(required.id, '05-model-runtime-status.json');
  const aiResult = readScenarioJson(required.id, '06-ai-result.json');
  const policy = readScenarioJson(required.id, '07-policy-decision.json');
  const deletion = readScenarioJson(required.id, '08-deletion-after-analysis.json');
  const validation = readScenarioJson(required.id, '10-live-operator-validation.json');

  validateScenarioSummary(required, scenarioSummary);
  validateSource(required, source);
  validateCapture(required, capture);
  validateAiArtifacts(required, aiContext, provider, runtime, aiResult);
  validatePolicy(required, aiResult, policy);
  validateDeletion(required, deletion, capture);
  validateScenarioValidation(required, validation);
  validateParentScreenshot(required, scenarioSummary);
  return scenarioResult(required, scenarioSummary);
}

function validateScenarioSummary(required, scenarioSummary) {
  assert(scenarioSummary.status === 'passed', `${required.id} did not pass`);
  assert(scenarioSummary.surface === required.surface, `${required.id} surface mismatch`);
  assert(scenarioSummary.primaryCategory === required.category, `${required.id} category mismatch`);
  assert(scenarioSummary.policyAction === required.action, `${required.id} policy action mismatch`);
  assert(scenarioSummary.analyzedByRealLocalVlm === true, `${required.id} did not prove local VLM`);
  assert(scenarioSummary.schemaValidated === true, `${required.id} schema validation missing`);
  assert(scenarioSummary.policyDecisionValidated === true, `${required.id} policy validation missing`);
  assert(scenarioSummary.rawImagesDeletedAfterAnalysis === true, `${required.id} raw image deletion missing`);
}

function validateSource(required, source) {
  assert(source.scenarioId === required.id, `${required.id} source id mismatch`);
  if (required.surface === 'browser') {
    assert(source.sourceKind === 'operator-live-url', `${required.id} did not use live URL source`);
    assert(source.liveExternalUrl === true, `${required.id} live external URL flag missing`);
    assert(source.redactedUrl?.includes('<redacted>'), `${required.id} URL is not redacted`);
    assert(source.pageReadiness?.loaded === true, `${required.id} page did not load`);
    assert(source.pageReadiness?.titleLength > 0, `${required.id} missing title readiness`);
    assert(source.pageReadiness?.visibleTextLength > 0, `${required.id} missing visible text readiness`);
    assert(source.pageReadiness?.readinessAssertions?.hostnameMatches === true, `${required.id} host mismatch`);
    assert(source.pageReadiness?.readinessAssertions?.titleMatches === true, `${required.id} title readiness failed`);
    assert(source.pageReadiness?.readinessAssertions?.textMatches === true, `${required.id} text readiness failed`);
    if (required.id === 'facebook-social-surface') {
      assert(source.hostname === 'www.facebook.com', `${required.id} expected Facebook public social host`);
      assert(source.publicSocialSurfaceProof !== false, `${required.id} must preserve public social surface proof`);
      assert(source.authenticatedAccountProof !== true, `${required.id} must not claim authenticated-account proof`);
    } else {
      assert(source.authenticatedAccountProof !== true, `${required.id} must not claim account proof`);
    }
    return;
  }
  assert(source.sourceKind === 'operator-native-app', `${required.id} did not use native app source`);
  assert(source.commandName === 'notepad.exe', `${required.id} native command mismatch`);
  assert(source.liveExternalUrl === false, `${required.id} native row should not claim live URL`);
}

function validateCapture(required, capture) {
  const metadata = capture.captureMetadata;
  assert(capture.rawImagePathNotRetained === true, `${required.id} capture ref retains raw path`);
  assert(metadata?.captured === true, `${required.id} capture did not happen`);
  assert(metadata?.status === 'available', `${required.id} capture status mismatch`);
  assert(metadata?.rawImagePersistedInProof === false, `${required.id} raw image persisted in proof`);
  assert(metadata?.imageByteSize > 0, `${required.id} missing image bytes`);
  assert(metadata?.width > 0 && metadata?.height > 0, `${required.id} missing image dimensions`);
  assert(
    typeof metadata?.imageDigest === 'string' && metadata.imageDigest.length >= 32,
    `${required.id} missing digest`
  );
  assert(!existsMaybeRelative(metadata.analysisTempPath), `${required.id} raw temp image still exists`);
}

function validateAiArtifacts(required, aiContext, provider, runtime, aiResult) {
  assert(aiContext.contextBuiltFromTypedCaptureMetadata === true, `${required.id} context did not use typed metadata`);
  assert(aiContext.directOsOrBrowserScanByModel === false, `${required.id} model scanned OS/browser directly`);
  assert(provider.rawCloudUpload === false, `${required.id} provider allowed raw cloud upload`);
  assert(runtime.providerKind === 'localVision', `${required.id} runtime is not local vision`);
  assert(runtime.loadState === 'loaded', `${required.id} runtime was not loaded`);
  assert(aiResult.screenResult.primaryCategory === required.category, `${required.id} screen result category mismatch`);
  assert(aiResult.screenResult.providerKind === 'localVision', `${required.id} screen result provider mismatch`);
  assert(aiResult.screenResult.rawImageRetained === false, `${required.id} screen result retained raw image`);
  assert(aiResult.screenResult.imageDeletionState === 'deleted', `${required.id} screen result deletion mismatch`);
  assert(
    aiResult.localAiSafetyResult.modelRuntime.privacyMode === 'local-only',
    `${required.id} model is not local-only`
  );
}

function validatePolicy(required, aiResult, policy) {
  assert(aiResult.localAiSafetyResult.action === required.action, `${required.id} local AI action mismatch`);
  assert(policy.policyDecision.action === required.action, `${required.id} policy action mismatch`);
  assert(policy.policyDecision.dryRun === true, `${required.id} policy was not dry-run`);
  assert(
    policy.policyDecision.localAiResultId === aiResult.localAiSafetyResult.resultId,
    `${required.id} AI id not linked`
  );
}

function validateDeletion(required, deletion, capture) {
  assert(deletion.rawImageDeletedAfterAnalysis === true, `${required.id} deletion proof missing`);
  assert(deletion.existsAfterDelete === false, `${required.id} deletion proof still sees raw file`);
  assert(deletion.analysisCompleted === true, `${required.id} analysis did not complete before deletion`);
  assert(deletion.rawTempPath === capture.captureMetadata.analysisTempPath, `${required.id} deletion path mismatch`);
  assert(!existsMaybeRelative(deletion.rawTempPath), `${required.id} deletion path still exists`);
}

function validateScenarioValidation(required, validation) {
  assert(validation.actualCategory === required.category, `${required.id} validation category mismatch`);
  assert(validation.actualPolicyAction === required.action, `${required.id} validation action mismatch`);
  assert(validation.rawImagesDeletedAfterAnalysis === true, `${required.id} validation deletion missing`);
  assert(validation.cloudUploadDisabled === true, `${required.id} validation cloud upload enabled`);
  assert(validation.managedBrowserTriggerOwnershipClaimed === false, `${required.id} claimed trigger ownership`);
}

function validateParentScreenshot(required, scenarioSummary) {
  const screenshot = resolve(repoRoot, scenarioSummary.parentExplanationScreenshot);
  assert(existsSync(screenshot), `${required.id} parent explanation screenshot missing`);
}

function validateProtectedScenario(required) {
  const scenarioSummary = summaryScenario(required.id);
  const source = readScenarioJson(required.id, '01-redacted-source-evidence.json');
  const capture = readScenarioJson(required.id, '02-capture-proof-ref.json');
  assert(scenarioSummary.status === 'passed', `${required.id} did not pass`);
  assert(source.sourceKind === 'protected-state-metadata', `${required.id} source kind mismatch`);
  assert(capture.noRawImageClaimed === true, `${required.id} claims raw image`);
  assert(capture.noAiAnalysisClaimed === true, `${required.id} claims AI analysis`);
  assert(capture.noPolicyDecisionClaimed === true, `${required.id} claims policy decision`);
  assert(
    capture.captureMetadata.protectedSkipResult.policyEligible === false,
    `${required.id} protected result is eligible`
  );
  assert(
    capture.captureMetadata.protectedSkipResult.imageDeletionState === 'unavailableNoImage',
    `${required.id} deletion mismatch`
  );
  return scenarioResult(required, scenarioSummary);
}

function scenarioResult(required, scenarioSummary) {
  return {
    scenarioId: required.id,
    surface: required.surface,
    category: required.category,
    policyAction: required.action,
    liveExternalUrlProof: scenarioSummary.liveExternalUrlProof,
    publicSocialSurfaceProof:
      required.id === 'facebook-social-surface' && scenarioSummary.authenticatedAccountProof !== true,
    authenticatedAccountProof: scenarioSummary.authenticatedAccountProof === true,
    localVlmAnalysisProof: scenarioSummary.analyzedByRealLocalVlm === true,
    policyDryRunProof: scenarioSummary.policyDecisionValidated === true,
  };
}

function summaryScenario(id) {
  const scenario = pipelineSummary.scenarios.find((entry) => entry.scenarioId === id);
  assert(scenario !== undefined, `${id} missing from summary scenarios`);
  return scenario ?? {};
}

function readScenarioJson(id, fileName) {
  return readJson(join(pipelineRoot, id, fileName));
}

function readJson(path) {
  assert(existsSync(path), `missing artifact ${relative(path)}`);
  return JSON.parse(readFileSync(path, 'utf8'));
}

function existsMaybeRelative(path) {
  if (typeof path !== 'string' || path.length === 0) return false;
  return existsSync(resolve(repoRoot, path));
}

function writeProof(value) {
  mkdirSync(dirname(gatePath), { recursive: true });
  writeFileSync(gatePath, `${JSON.stringify(value, null, 2)}\n`);
}

function relative(path) {
  return path.replace(`${repoRoot}\\`, '').replaceAll('\\', '/');
}

function assert(condition, message) {
  if (!condition) failures.push(`- ${message}`);
}
