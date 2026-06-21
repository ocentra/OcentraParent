import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';

import {
  BrowserGamePolicyCompilerInputSchema,
  BrowserGamePolicyDecisionCandidateSchema,
} from '@ocentra-parent/schema-domain/browser-game-policy-compiler';
import {
  compileBrowserGamePolicyCandidate,
} from '@ocentra-parent/browser-domain/browser-game-policy-compiler';

const repoRoot = process.cwd();
const proofId = 'browser-game-policy-compiler-live-evidence-proof';
const resultPath = join(repoRoot, 'test-results', proofId, 'proof.json');
const outputProofPath = join(
  repoRoot,
  'output',
  'browser-plan-proof',
  'game-17-parent-game-policy-compiler',
  '02-live-policy-compiler-shape-proof.json'
);

const targets = [
  {
    targetId: 'code-org-minecraft-allow',
    url: 'https://code.org/minecraft',
    targetKind: 'educational-game',
    actionCandidate: 'allow-candidate',
    reasonCodes: ['educational-benefit-present', 'parent-rule-match'],
    confidence: 'high',
    compilerMode: 'contract-only',
    analysisRefs: ['browser-game-educational-classifier-ref'],
    parentRuleRefs: ['parent-rule-allow-educational-game-ref'],
    scheduleContextRefs: [],
    mobileCapabilityRefs: ['browser-game-managed-browser-capability-ref'],
    fallbackUsed: false,
    parentApprovalRequired: false,
  },
  {
    targetId: 'poki-game-warn',
    url: 'https://poki.com/en/g/subway-surfers',
    targetKind: 'browser-game-url',
    actionCandidate: 'warn-candidate',
    reasonCodes: ['browser-game-risk-high', 'parent-rule-match'],
    confidence: 'medium',
    compilerMode: 'contract-only',
    analysisRefs: ['browser-game-riskbenefit-signal-ref'],
    parentRuleRefs: ['parent-rule-warn-browser-game-ref'],
    scheduleContextRefs: [],
    mobileCapabilityRefs: ['browser-game-managed-browser-capability-ref'],
    fallbackUsed: false,
    parentApprovalRequired: false,
  },
  {
    targetId: 'roblox-parent-review',
    url: 'https://www.roblox.com/discover',
    targetKind: 'ugc-multiplayer-game',
    actionCandidate: 'parent-review-candidate',
    reasonCodes: ['parent-rule-match', 'ugc-chat-risk'],
    confidence: 'medium',
    compilerMode: 'contract-only',
    analysisRefs: ['browser-game-ugc-risk-assessment-ref'],
    parentRuleRefs: ['parent-rule-review-ugc-game-ref'],
    scheduleContextRefs: [],
    mobileCapabilityRefs: ['browser-game-mobile-capability-ref'],
    fallbackUsed: false,
    parentApprovalRequired: true,
  },
  {
    targetId: 'coolmath-time-limit',
    url: 'https://www.coolmathgames.com/0-run',
    targetKind: 'browser-game-url',
    actionCandidate: 'time-limit-candidate',
    reasonCodes: ['parent-rule-match', 'schedule-context'],
    confidence: 'medium',
    compilerMode: 'contract-only',
    analysisRefs: ['browser-game-metadata-extractor-ref'],
    parentRuleRefs: ['parent-rule-time-limit-game-ref'],
    scheduleContextRefs: ['schedule-context-school-night-ref'],
    mobileCapabilityRefs: ['browser-game-managed-browser-capability-ref'],
    fallbackUsed: false,
    parentApprovalRequired: false,
  },
  {
    targetId: 'hooda-unblocked-block',
    url: 'https://www.hoodamath.com/games/unblocked.html',
    targetKind: 'unblocked-game-site',
    actionCandidate: 'block-candidate',
    reasonCodes: ['unblocked-game-site-risk', 'parent-rule-match'],
    confidence: 'high',
    compilerMode: 'contract-only',
    analysisRefs: ['browser-game-unblocked-site-detection-ref'],
    parentRuleRefs: ['parent-rule-block-unblocked-game-site-ref'],
    scheduleContextRefs: ['schedule-context-school-hours-ref'],
    mobileCapabilityRefs: ['browser-game-managed-browser-capability-ref'],
    fallbackUsed: false,
    parentApprovalRequired: false,
  },
  {
    targetId: 'recroom-manual-review',
    url: 'https://recroom.com/',
    targetKind: 'manual-required',
    actionCandidate: 'manual-review-candidate',
    reasonCodes: ['manual-required', 'mobile-capability-manual-required'],
    confidence: 'low',
    compilerMode: 'manual-required',
    analysisRefs: [],
    parentRuleRefs: ['parent-rule-manual-review-game-ref'],
    scheduleContextRefs: [],
    mobileCapabilityRefs: ['browser-game-mobile-capability-manual-ref'],
    fallbackUsed: true,
    parentApprovalRequired: false,
  },
  {
    targetId: 'archive-unknown-fallback',
    url: 'https://archive.org/details/softwarelibrary_msdos_games',
    targetKind: 'manual-required',
    actionCandidate: 'unknown-candidate',
    reasonCodes: ['missing-game-evidence', 'unknown-evidence'],
    confidence: 'unknown',
    compilerMode: 'unavailable',
    analysisRefs: [],
    parentRuleRefs: [],
    scheduleContextRefs: [],
    mobileCapabilityRefs: [],
    fallbackUsed: true,
    parentApprovalRequired: false,
  },
];

const startedAt = new Date().toISOString();
const branch = git(['rev-parse', '--abbrev-ref', 'HEAD']);
const commit = git(['rev-parse', 'HEAD']);
const baseCommit = git(['rev-parse', 'origin/main']);
const captures = await Promise.all(targets.map(captureTarget));
const inputs = captures.map(inputFor);
const decisions = captures.map(decisionFor);
const negativeChecks = runNegativeChecks(inputs[0], decisions[0]);

if (!captures.every((capture) => capture.responseOk)) {
  throw new Error('Expected all browser-game policy compiler public captures to return HTTP 2xx/3xx responses');
}
if (!inputs.every((input) => BrowserGamePolicyCompilerInputSchema.safeParse(input).success)) {
  throw new Error('Expected every browser-game policy compiler input to parse');
}
if (!decisions.every((decision) => BrowserGamePolicyDecisionCandidateSchema.safeParse(decision).success)) {
  throw new Error('Expected every browser-game policy decision candidate to parse');
}
if (!negativeChecks.every((check) => check.rejected)) {
  const failedChecks = negativeChecks.filter((check) => !check.rejected).map((check) => check.name);
  throw new Error(
    `Expected browser-game policy compiler negative checks to reject overclaims: ${failedChecks.join(', ')}`
  );
}

const proof = {
  schemaVersion: 1,
  proofId,
  generatedAt: startedAt,
  branch,
  commit,
  baseCommit,
  captureMode: 'real-public-browser-game-policy-compiler-shapes',
  targets: captures,
  inputs,
  decisions,
  negativeChecks,
  summary: {
    targetCount: captures.length,
    inputCount: inputs.length,
    decisionCount: decisions.length,
    negativeChecks: negativeChecks.length,
    targetKinds: [...new Set(inputs.map((input) => input.targetKind))],
    actionCandidates: [...new Set(decisions.map((decision) => decision.actionCandidate))],
    rawUrlPersisted: false,
    rawGamePayloadIncluded: false,
    rawModelTextIncluded: false,
    activityDomainObjectIncluded: false,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    productChecklistUpgradeClaimed: false,
  },
};

await writeJson(resultPath, proof);
await writeJson(outputProofPath, proof);

console.log('browser-game-policy-compiler-live-evidence-proof-ok=true');
console.log(`proof=${relativePath(resultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(
  `targets=${captures.length} inputs=${inputs.length} decisions=${decisions.length} negativeChecks=${negativeChecks.length}`
);

async function captureTarget(target) {
  const inputUrl = new URL(target.url);
  const response = await fetch(target.url, {
    redirect: 'follow',
    headers: {
      'user-agent': 'Mozilla/5.0 OcentraParentBrowserGameProof/1.0',
      accept: 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
    },
  });
  const body = Buffer.from(await response.arrayBuffer());
  const finalUrl = new URL(response.url);
  return {
    targetId: target.targetId,
    status: response.status,
    responseOk: response.status >= 200 && response.status < 400,
    contentType: response.headers.get('content-type') ?? 'unknown',
    contentLength: body.length,
    bodySha256: sha256(body),
    inputOriginSha256: sha256(inputUrl.origin),
    inputPathSha256: sha256(inputUrl.pathname),
    finalOriginSha256: sha256(finalUrl.origin),
    finalPathSha256: sha256(finalUrl.pathname),
    rawUrlPersisted: false,
    rawPageBodyPersisted: false,
    rawGamePayloadPersisted: false,
    rawModelTextPersisted: false,
  };
}

function inputFor(capture) {
  const target = targetFor(capture.targetId);
  return {
    schemaVersion: 'v0.6',
    compileRequestId: `browser-game-policy-compile-${capture.targetId}`,
    familyId: 'family-browser-game-policy-live-proof',
    childProfileId: 'child-browser-game-policy-live-proof',
    deviceId: 'device-browser-game-policy-live-proof',
    requestedAt: startedAt,
    policyVersionRef: 'policy-version-browser-game-live-proof',
    targetKind: target.targetKind,
    sourceEvidenceRefs: [
      `parent-proof-${proofId}-${capture.targetId}-source`,
      `parent-proof-${proofId}-${capture.targetId}-response-hash`,
    ],
    analysisRefs: target.analysisRefs,
    mobileCapabilityRefs: target.mobileCapabilityRefs,
    parentRuleRefs: target.parentRuleRefs,
    scheduleContextRefs: target.scheduleContextRefs,
    compilerMode: target.compilerMode,
    rawGamePayloadIncluded: false,
    rawModelTextIncluded: false,
    activityDomainObjectIncluded: false,
    finalDecisionClaimedByInput: false,
    runtimeGateClaimedByInput: false,
    uiClaimedByInput: false,
    enforcementClaimedByInput: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
  };
}

function decisionFor(capture) {
  const target = targetFor(capture.targetId);
  return compileBrowserGamePolicyCandidate({
    input: inputFor(capture),
    decisionCandidateId: `browser-game-policy-decision-${capture.targetId}`,
    decidedAt: startedAt,
    expiresAt: target.compilerMode === 'contract-only' ? '2026-06-06T22:30:00.000Z' : null,
    actionCandidate: target.actionCandidate,
    reasonCodes: target.reasonCodes,
    confidence: target.confidence,
    fallbackUsed: target.fallbackUsed,
    parentApprovalRequired: target.parentApprovalRequired,
  });
}

function runNegativeChecks(validInput, validDecision) {
  const invalidInputs = [
    ['inputRawGamePayloadIncluded', { rawGamePayloadIncluded: true }],
    ['inputRawModelTextIncluded', { rawModelTextIncluded: true }],
    ['inputActivityDomainObjectIncluded', { activityDomainObjectIncluded: true }],
    ['inputFinalDecisionClaimed', { finalDecisionClaimedByInput: true }],
    ['inputRuntimeGateClaimed', { runtimeGateClaimedByInput: true }],
    ['inputUiClaimed', { uiClaimedByInput: true }],
    ['inputEnforcementClaimed', { enforcementClaimedByInput: true }],
    ['inputNativeGameControlClaimed', { nativeGameControlClaimed: true }],
    ['inputCloudFrameAnalysisClaimed', { cloudFrameAnalysisClaimed: true }],
    ['inputContractOnlyMissingAnalysis', { analysisRefs: [] }],
    ['inputContractOnlyManualTarget', { targetKind: 'manual-required' }],
  ].map(([name, override]) => ({
    name,
    rejected: !BrowserGamePolicyCompilerInputSchema.safeParse({ ...validInput, ...override }).success,
  }));

  const invalidDecisions = [
    ['decisionFinalPolicyDecisionClaimed', { finalPolicyDecisionClaimed: true }],
    ['decisionRuntimeGateExecutedClaimed', { runtimeGateExecutedClaimed: true }],
    ['decisionUiRenderedClaimed', { uiRenderedClaimed: true }],
    ['decisionEnforcementClaimed', { enforcementClaimed: true }],
    ['decisionNativeGameControlClaimed', { nativeGameControlClaimed: true }],
    ['decisionCloudFrameAnalysisClaimed', { cloudFrameAnalysisClaimed: true }],
    ['decisionRawGamePayloadStored', { rawGamePayloadStored: true }],
    ['decisionRawModelTextUsed', { rawModelTextUsed: true }],
    ['unknownWithoutFallback', { actionCandidate: 'unknown-candidate', fallbackUsed: false }],
    ['manualWithoutFallback', { actionCandidate: 'manual-review-candidate', fallbackUsed: false }],
    ['parentReviewWithoutApproval', { actionCandidate: 'parent-review-candidate', parentApprovalRequired: false }],
    ['allowWithoutAllowReason', { actionCandidate: 'allow-candidate', reasonCodes: ['browser-game-risk-high'] }],
    ['timeLimitWithoutSchedule', { actionCandidate: 'time-limit-candidate', reasonCodes: ['parent-rule-match'] }],
  ].map(([name, override]) => ({
    name,
    rejected: !BrowserGamePolicyDecisionCandidateSchema.safeParse({ ...validDecision, ...override }).success,
  }));

  return [...invalidInputs, ...invalidDecisions];
}

function targetFor(targetId) {
  const target = targets.find((item) => item.targetId === targetId);
  if (!target) {
    throw new Error(`Unknown target: ${targetId}`);
  }
  return target;
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
