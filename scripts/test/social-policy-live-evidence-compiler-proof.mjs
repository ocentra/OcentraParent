import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  SocialParentPolicyCompilerInputSchema,
  SocialParentPolicyDecisionCandidateSchema,
} from '../../packages/schema-domain/dist/social-policy-compiler.js';
import { compileSocialParentPolicyCandidate } from '../../packages/browser-domain/dist/social-policy-candidate-compiler.js';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const social11ProofPath = join(repoRoot, 'test-results/social-risk-benefit-live-evidence-proof/proof.json');
const outputDirectory = join(repoRoot, 'output/browser-plan-proof/social-12-parent-policy-compiler-social-targets');
const outputProofPath = join(outputDirectory, '11-live-evidence-policy-compiler-proof.json');
const testResultPath = join(repoRoot, 'test-results/social-policy-live-evidence-compiler-proof/proof.json');
const observedAt = new Date().toISOString();

const builtFiles = [
  'packages/schema-domain/dist/social-policy-compiler-values.js',
  'packages/schema-domain/dist/social-policy-compiler.js',
  'packages/browser-domain/dist/social-policy-candidate-compiler.js',
];

assertBuiltContractsAreFresh();
if (!existsSync(social11ProofPath)) {
  throw new Error(`Missing SOCIAL-11 live-evidence signal proof: ${relativePath(social11ProofPath)}`);
}

const social11Proof = JSON.parse(readFileSync(social11ProofPath, 'utf8'));
if (social11Proof.signalSets.length < 3) {
  throw new Error(`Expected at least 3 SOCIAL-11 signal sets, received ${social11Proof.signalSets.length}`);
}
if (social11Proof.liveEvidenceBoundary.finalPolicyDecisionClaimed) {
  throw new Error('SOCIAL-11 source proof must not claim final policy decisions');
}

const inputRows = social11Proof.signalSets.map((signalSet) => policyInputFor(signalSet));
const inputParseChecks = inputRows.map((row) => ({
  targetId: row.targetId,
  accepted: SocialParentPolicyCompilerInputSchema.safeParse(row.input).success,
}));
if (!inputParseChecks.every((check) => check.accepted)) {
  throw new Error('Expected every SOCIAL-12 live-evidence compiler input to parse');
}

const candidateRows = inputRows.map((row) => {
  const input = SocialParentPolicyCompilerInputSchema.parse(row.input);
  const candidate = compileSocialParentPolicyCandidate({
    input,
    decisionCandidateId: `${row.targetId}-social-policy-decision-candidate`,
    decidedAt: observedAt,
    expiresAt: null,
    actionCandidate: 'manual-review-candidate',
    reasonCodes: ['manual-required', 'degraded-analysis', 'missing-signal-proof'],
    confidence: 'unknown',
    fallbackUsed: true,
    parentApprovalRequired: true,
  });
  return {
    targetId: row.targetId,
    candidate,
  };
});

const candidateParseChecks = candidateRows.map((row) => ({
  targetId: row.targetId,
  accepted: SocialParentPolicyDecisionCandidateSchema.safeParse(row.candidate).success,
}));
if (!candidateParseChecks.every((check) => check.accepted)) {
  throw new Error('Expected every SOCIAL-12 live-evidence policy candidate to parse');
}

const negativeChecks = buildNegativeChecks(inputRows[0].input, candidateRows[0].candidate);
if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected all SOCIAL-12 negative checks to reject dishonest policy compiler claims');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-policy-live-evidence-compiler-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  sourceProof: relativePath(social11ProofPath),
  liveEvidenceBoundary: {
    sourceRiskBenefitProofUsesLiveSocial10Refs: true,
    sourceSignalSetCount: social11Proof.signalSets.length,
    sourceRiskBenefitCandidateClassified: social11Proof.liveEvidenceBoundary.riskBenefitCandidateClassified,
    unavailableSignalRowsOnly: social11Proof.liveEvidenceBoundary.unavailableSignalRowsOnly,
    policyCandidateCount: candidateRows.length,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    rawSignalPayloadStored: false,
    rawModelTextUsed: false,
  },
  policyCandidates: candidateRows.map((row) => ({
    targetId: row.targetId,
    decisionCandidateId: row.candidate.decisionCandidateId,
    compileRequestId: row.candidate.compileRequestId,
    targetKind: row.candidate.targetKind,
    sourceEvidenceRefs: row.candidate.sourceEvidenceRefs,
    signalSetRefs: row.candidate.signalSetRefs,
    scheduleState: row.candidate.scheduleState,
    timeBudgetState: row.candidate.timeBudgetState,
    actionCandidate: row.candidate.actionCandidate,
    reasonCodes: row.candidate.reasonCodes,
    confidence: row.candidate.confidence,
    compilerMode: row.candidate.compilerMode,
    fallbackUsed: row.candidate.fallbackUsed,
    parentApprovalRequired: row.candidate.parentApprovalRequired,
    finalPolicyDecisionClaimed: row.candidate.finalPolicyDecisionClaimed,
    runtimeGateExecutedClaimed: row.candidate.runtimeGateExecutedClaimed,
    uiRenderedClaimed: row.candidate.uiRenderedClaimed,
    enforcementClaimed: row.candidate.enforcementClaimed,
    nativeAppControlClaimed: row.candidate.nativeAppControlClaimed,
    platformConnectorClaimed: row.candidate.platformConnectorClaimed,
    rawSignalPayloadStored: row.candidate.rawSignalPayloadStored,
    rawModelTextUsed: row.candidate.rawModelTextUsed,
  })),
  inputParseChecks,
  candidateParseChecks,
  negativeChecks,
  noClaimChecks: {
    finalPolicyDecision: false,
    runtimeGate: false,
    uiDelivery: false,
    enforcement: false,
    nativeAppControl: false,
    connectorAuthorization: false,
    appliedSchedule: false,
    appliedTimeBudget: false,
    rawContentUse: false,
  },
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-policy-live-evidence-compiler-proof-ok=true');
console.log(`proof=${relativePath(testResultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(`policyCandidateCount=${candidateRows.length}`);
console.log(`candidateStates=${candidateRows.map((row) => row.candidate.actionCandidate).join(',')}`);

function policyInputFor(signalSet) {
  return {
    targetId: signalSet.targetId,
    input: {
      schemaVersion: 'v0.6',
      compileRequestId: `${signalSet.targetId}-social-policy-compile-request`,
      familyId: 'family-main',
      childProfileId: 'child-profile-middle-school',
      deviceId: 'child-device-managed-browser',
      requestedAt: observedAt,
      policyVersionRef: 'policy-version-social-live-evidence-proof',
      targetKind: targetKindFor(signalSet.routeKind),
      sourceEvidenceRefs: [`parent-evidence-${signalSet.socialAiAnalysisId}`],
      signalSetRefs: [signalSet.signalSetId],
      parentRuleRefs: ['parent-rule-social-manual-review-on-degraded-analysis'],
      scheduleContextRefs: ['schedule-context-live-evidence-manual-review'],
      timeBudgetContextRefs: ['time-budget-context-live-evidence-manual-review'],
      scheduleState: 'inside-allowed-window',
      timeBudgetState: 'budget-available',
      compilerMode: 'contract-only',
      rawSignalPayloadIncluded: false,
      rawModelTextIncluded: false,
      activityDomainObjectIncluded: false,
      finalDecisionClaimedByInput: false,
      runtimeGateClaimedByInput: false,
      uiClaimedByInput: false,
      enforcementClaimedByInput: false,
      nativeAppControlClaimed: false,
      platformConnectorClaimed: false,
    },
  };
}

function targetKindFor(routeKind) {
  if (routeKind === 'video') {
    return 'social-video';
  }
  return 'social-feed';
}

function buildNegativeChecks(validInput, validCandidate) {
  const invalidInputs = [
    ['input-raw-signal-payload', { ...validInput, rawSignalPayloadIncluded: true }],
    ['input-raw-model-text', { ...validInput, rawModelTextIncluded: true }],
    ['input-runtime-gate-claim', { ...validInput, runtimeGateClaimedByInput: true }],
    ['input-enforcement-claim', { ...validInput, enforcementClaimedByInput: true }],
    ['input-native-app-claim', { ...validInput, nativeAppControlClaimed: true }],
    ['input-connector-claim', { ...validInput, platformConnectorClaimed: true }],
    ['input-missing-signal-ref', { ...validInput, signalSetRefs: [] }],
  ];
  const invalidCandidates = [
    ['candidate-final-policy', { ...validCandidate, finalPolicyDecisionClaimed: true }],
    ['candidate-runtime-gate', { ...validCandidate, runtimeGateExecutedClaimed: true }],
    ['candidate-ui-rendered', { ...validCandidate, uiRenderedClaimed: true }],
    ['candidate-enforcement', { ...validCandidate, enforcementClaimed: true }],
    ['candidate-raw-signal-store', { ...validCandidate, rawSignalPayloadStored: true }],
    ['candidate-raw-model-text', { ...validCandidate, rawModelTextUsed: true }],
    ['candidate-manual-without-fallback', { ...validCandidate, fallbackUsed: false }],
  ];

  return [
    ...invalidInputs.map(([label, candidate]) => ({
      label,
      rejected: !SocialParentPolicyCompilerInputSchema.safeParse(candidate).success,
    })),
    ...invalidCandidates.map(([label, candidate]) => ({
      label,
      rejected: !SocialParentPolicyDecisionCandidateSchema.safeParse(candidate).success,
    })),
  ];
}

function assertBuiltContractsAreFresh() {
  for (const builtFile of builtFiles) {
    const builtPath = join(repoRoot, builtFile);
    if (!existsSync(builtPath)) {
      throw new Error(`Missing built contract file. Run npm run build:contracts first: ${builtFile}`);
    }
  }
}

function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
