import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'invalid-output');
const artifactSummaryPath = join(outputRoot, 'proof-summary.json');

await mkdir(outputRoot, { recursive: true });
runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/activity-domain']));

const { ScreenAnalysisResultSchema, ScreenEvidenceSchemaVersion, ScreenLocalModelOutputSchema } =
  await import('../../packages/activity-domain/dist/screen-evidence.js');

const validModelOutput = {
  primary_category: 'school',
  confidence: 0.82,
  visible_text: 'A study page is visible in the active window.',
  risk_signals: [],
};

const validControl = attemptPolicyCandidate(validModelOutput);
if (!validControl.policyCandidateCreated) {
  throw new Error(`Valid control did not reach policy candidate: ${JSON.stringify(validControl)}`);
}

const cases = [
  {
    caseId: 'invalid-category',
    payload: { ...validModelOutput, primary_category: 'vpn' },
  },
  {
    caseId: 'invalid-confidence-high',
    payload: { ...validModelOutput, confidence: 1.2 },
  },
  {
    caseId: 'invalid-risk-signal',
    payload: { ...validModelOutput, risk_signals: ['random-model-string'] },
  },
  {
    caseId: 'missing-visible-text',
    payload: {
      primary_category: 'school',
      confidence: 0.82,
      risk_signals: [],
    },
  },
];

const outcomes = cases.map(({ caseId, payload }) => ({
  caseId,
  ...attemptPolicyCandidate(payload),
}));

const escapedCases = outcomes.filter((outcome) => outcome.policyCandidateCreated);
if (escapedCases.length > 0) {
  throw new Error(`Invalid model output reached policy candidate: ${JSON.stringify(escapedCases)}`);
}

const summary = {
  status: 'ok',
  proofKind: 'screen-ai-invalid-output-policy-guard',
  artifact: artifactSummaryPath,
  validControl,
  outcomes,
  assertion: 'Invalid local model output is rejected before ScreenAnalysisResult and policy candidate creation.',
  nonClaims: [
    'This is a contract/proof-runner guard over local model output shape.',
    'It does not claim model quality or live external account classification.',
  ],
};

await writeFile(artifactSummaryPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(`screen-ai-invalid-output-proof-ok ${artifactSummaryPath}`);

function attemptPolicyCandidate(modelOutput) {
  try {
    const parsedModelOutput = ScreenLocalModelOutputSchema.parse(modelOutput);
    const screenResult = ScreenAnalysisResultSchema.parse(buildScreenAnalysisResult(parsedModelOutput));
    return {
      modelOutputAccepted: true,
      screenAnalysisResultCreated: true,
      policyCandidateCreated: screenResult.policyEligible,
      primaryCategory: screenResult.primaryCategory,
    };
  } catch (error) {
    return {
      modelOutputAccepted: false,
      screenAnalysisResultCreated: false,
      policyCandidateCreated: false,
      rejection: error instanceof Error ? error.message.split('\n')[0] : String(error),
    };
  }
}

function buildScreenAnalysisResult(modelOutput) {
  const evidenceRef = {
    evidenceId: 'screen-ai-invalid-output-proof-evidence',
    kind: 'journal-entry',
    digest: 'sha256:screen-ai-invalid-output-proof',
    uri: null,
  };
  return {
    schemaVersion: ScreenEvidenceSchemaVersion,
    screenAnalysisResultId: 'screen-analysis-invalid-output-valid-control',
    queueJobId: 'screen-queue-invalid-output-valid-control',
    analyzedAt: '2026-06-03T20:15:00.000Z',
    modelRuntimeRef: 'local-vlm-runtime-invalid-output-proof',
    modelId: 'local-vlm-proof-model',
    providerKind: 'localVision',
    promptOrTemplateVersion: 'screen-visible-activity-v1',
    captureReason: 'manualParentTestCapture',
    captureScope: 'activeWindow',
    capabilityStatus: 'ready',
    summary: modelOutput.visible_text,
    visibleCategoryCandidates: [
      {
        category: modelOutput.primary_category,
        confidence: modelOutput.confidence,
        evidenceRefs: [evidenceRef],
      },
    ],
    primaryCategory: modelOutput.primary_category,
    riskSignals: modelOutput.risk_signals.map((signal) => ({
      signal,
      confidence: modelOutput.confidence,
      evidenceRefs: [evidenceRef],
    })),
    ocrTextSnippets: [
      {
        text: modelOutput.visible_text,
        confidence: modelOutput.confidence,
        evidenceRefs: [evidenceRef],
      },
    ],
    redactionNotes: [],
    confidence: modelOutput.confidence,
    uncertaintyReason: null,
    sourceEvidenceRefs: [evidenceRef],
    imageDigest: 'sha256:screen-ai-invalid-output-proof',
    rawImageRetained: false,
    imageDeletionState: 'deleted',
    custodyState: 'child-device-query-store',
    policyEligible: true,
  };
}

function runCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
