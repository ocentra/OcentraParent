import { execFileSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'ai-plan-proof', 'screen-ai-remote-boundary-proof');
const testResultsDir = join(repoRoot, 'test-results', 'screen-ai-remote-boundary-proof');
const proofSummaryPath = join(outputDir, 'proof-summary.json');
const proofResultPath = join(testResultsDir, 'proof.json');
const validationCommandsPath = join(outputDir, 'validation-commands.log');
const commands = [];

await main();

async function main() {
  rmSync(outputDir, { recursive: true, force: true });
  rmSync(testResultsDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });
  mkdirSync(testResultsDir, { recursive: true });

  runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'screen-ai-remote-boundary-proof',
  ]);

  const contract = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'screen-ai-remote-boundary-proof.js')).href
  );
  const proof = contract.ScreenAiRemoteBoundaryProofSchema.parse(buildProof());
  const summary = contract.summarizeScreenAiRemoteBoundaryProof(proof);
  const assertions = {
    requiredRowsPresent: contract.screenAiRemoteBoundaryProofCoversRequiredRows(proof),
    childSafetyStaysLocalOnly: summary.childSafetyLocalOnlyRows === 1 && summary.childSafetyRemoteClaimRows === 0,
    parentRemoteStatesAreParentOnly: summary.parentOnlyRemoteStateRows === 2,
    remoteCannotSetPolicy: summary.remotePolicyAuthorityRows === 0,
    remoteCannotEnforce: summary.remoteEnforcementRows === 0,
    rawImagesNotRetained: summary.rawImageRetainedRows === 0,
  };
  const proofSummary = {
    proof: 'screen-ai-remote-boundary-proof',
    proofTier: 'P3_CONTRACT_REMOTE_BOUNDARY',
    generatedAt: proof.generatedAt,
    summary,
    assertions,
    sourceArtifacts: Array.from(new Set(proof.rows.flatMap((row) => row.sourceArtifacts))),
    validationCommands: relativePath(validationCommandsPath),
    claimsProved: [
      'Child-safety screen AI rows require local-only routing and reject remote/API authorization or execution.',
      'Parent assistant and parent report remote/API states are explicit parent-only unavailable/degraded surfaces.',
      'Remote/API results cannot set policy, cannot enforce, and cannot retain raw screen images in this boundary proof.',
    ],
    nonClaims: [
      'This proof does not execute a remote API provider or local model.',
      'This proof does not implement portal rendering, SQLite journal ingest, production model quality, or enforcement.',
      'This proof does not replace the existing local AI provider scheduler or screen evidence context builder.',
    ],
  };

  if (!Object.values(assertions).every((assertion) => assertion === true)) {
    throw new Error(`Screen AI remote boundary proof failed: ${JSON.stringify(assertions)}`);
  }

  writeFileSync(proofSummaryPath, `${JSON.stringify(proofSummary, null, 2)}\n`);
  writeFileSync(proofResultPath, `${JSON.stringify({ proof, proofSummary }, null, 2)}\n`);
  writeFileSync(validationCommandsPath, `${commands.map((command) => `${command}: PASS`).join('\n')}\n`);

  console.log(
    `screen-ai-remote-boundary-proof-ok:${summary.rowCount}:${summary.childSafetyLocalOnlyRows}:${summary.parentOnlyRemoteStateRows}`
  );
  console.log(`proof=${relativePath(proofSummaryPath)}`);
}

function buildProof() {
  return {
    schemaVersion: 'v0.6',
    proofId: 'screen-ai-remote-boundary-proof',
    generatedAt: new Date().toISOString(),
    rows: [
      baseRow({
        rowId: 'screen-ai-child-safety-local-only',
        purpose: 'child-safety-screen-analysis',
        evidenceKind: 'screen-summary',
        sourceArtifacts: [
          'output/ai-plan-proof/screen-summary-ai-context/proof-summary.json',
          'output/ai-plan-proof/screen-ai-memory-graph-source-guard-proof/proof-summary.json',
        ],
        boundaryState: 'child-safety-local-only',
        decision: 'route-child-local',
        childSafetyInputAllowed: true,
        parentOnlySurfaceAllowed: false,
        localRuntimeRequired: true,
        remoteApiCredentialState: 'not-used',
        remoteApiExecutionState: 'not-executed',
      }),
      baseRow({
        rowId: 'screen-ai-parent-assistant-api-unavailable',
        purpose: 'parent-assistant',
        evidenceKind: 'parent-assistant-context',
        sourceArtifacts: ['output/ai-plan-proof/local-ai-parent-assistant-runtime-proof/proof-summary.json'],
        boundaryState: 'parent-assistant-api-authorized-unavailable',
        decision: 'surface-parent-unavailable',
        childSafetyInputAllowed: false,
        parentOnlySurfaceAllowed: true,
        localRuntimeRequired: false,
        remoteApiCredentialState: 'authorized-unavailable',
        remoteApiExecutionState: 'unavailable',
      }),
      baseRow({
        rowId: 'screen-ai-parent-report-api-degraded',
        purpose: 'parent-report',
        evidenceKind: 'parent-report-context',
        sourceArtifacts: ['output/ai-plan-proof/screen-summary-parent-explanation-read-model/proof-summary.json'],
        boundaryState: 'parent-report-api-authorized-degraded',
        decision: 'surface-parent-degraded',
        childSafetyInputAllowed: false,
        parentOnlySurfaceAllowed: true,
        localRuntimeRequired: false,
        remoteApiCredentialState: 'authorized-degraded',
        remoteApiExecutionState: 'degraded',
      }),
    ],
  };
}

function baseRow(overrides) {
  return {
    schemaVersion: 'v0.6',
    rowId: 'screen-ai-remote-boundary-row',
    purpose: 'child-safety-screen-analysis',
    evidenceKind: 'screen-summary',
    sourceEvidenceReferences: [
      {
        evidenceReferenceId: 'screen-summary-evidence-wikipedia-school',
        kind: 'activity-event',
        observedAt: '2026-06-05T21:26:00.000Z',
      },
    ],
    sourceArtifacts: ['output/ai-plan-proof/screen-summary-ai-context/proof-summary.json'],
    boundaryState: 'child-safety-local-only',
    decision: 'route-child-local',
    childSafetyInputAllowed: true,
    parentOnlySurfaceAllowed: false,
    localRuntimeRequired: true,
    remoteApiCredentialState: 'not-used',
    remoteApiExecutionState: 'not-executed',
    rawImageState: 'deleted',
    claimFlags: {
      remoteAiUsedForChildSafety: false,
      remoteApiAllowedForChildSafety: false,
      remoteResultCanSetPolicy: false,
      remoteResultCanEnforce: false,
      rawScreenImageRetained: false,
    },
    ...overrides,
  };
}

function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  execFileSync(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
