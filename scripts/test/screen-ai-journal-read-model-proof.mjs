import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const RepoRoot = process.cwd();
const SourceProofPath = resolve(
  RepoRoot,
  'output',
  'screen-ai-pipeline-proof',
  'service-winrt-ocr-policy',
  'proof-summary.json'
);
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'screen-ai-journal-read-model-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'screen-ai-journal-read-model-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const TestResultPath = join(TestResultRoot, 'proof.json');
const ClaimBoundaries = {
  rawImageRetained: false,
  remoteAiUsed: false,
  apiAiUsed: false,
  policyAuthorityClaimed: false,
  portalRuntimeClaimed: false,
  enforcementClaimed: false,
  runtimeSqliteWriterClaimed: false,
};

runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

const { buildScreenAiJournalReadModelSnapshot } = await importDist('screen-ai-journal-read-model-proof.js');
const sourceProof = JSON.parse(readFileSync(SourceProofPath, 'utf8'));
const generatedAt = new Date().toISOString();
const sourceRows = [sourceRowFromServicePolicyProof(sourceProof)];
const snapshot = buildScreenAiJournalReadModelSnapshot({
  schemaVersion: 'v0.6',
  snapshotId: 'screen-ai-journal-read-model-snapshot',
  generatedAt,
  sourceProof: relativePath(SourceProofPath),
  sourceRows,
  claimBoundaries: ClaimBoundaries,
});
const failures = snapshot.rows.filter(rowFailsValidation).map((row) => `${row.rowId} failed journal read-model proof`);

if (failures.length > 0) {
  throw new Error(`screen AI journal read-model proof failed:\n${failures.join('\n')}`);
}

const proof = {
  status: 'ok',
  proofKind: 'screen-ai-journal-read-model-proof',
  generatedAt,
  sourceProof: relativePath(SourceProofPath),
  sourceProofKind: sourceProof.proof,
  sourceLiveSurface: sourceProof.sourceLiveSurface,
  output: relativePath(ProofPath),
  snapshot,
  assertions: {
    consumedRealServiceOcrPolicyArtifact: sourceProof.assertions.sourceUsedLivePublicBrowserPixels === true,
    sourceRanWindowsWinRtOcr: sourceProof.assertions.sourceRanWindowsWinRtOcr === true,
    sourcePolicyDecisionParsedByParentDomain: sourceProof.assertions.policyDecisionParsedByParentDomain === true,
    sourceActivityReadModelCarriesPolicyRefs: sourceProof.assertions.activityReadModelCarriesPolicyRefs === true,
    journalEntryRefsPresent: snapshot.rows.every((row) => row.journalEntryRef.length > 0),
    sqliteProjectionRefsPresent: snapshot.rows.every((row) => row.sqliteRowRef.length > 0),
    modelMetadataPreserved: snapshot.rows.every(
      (row) =>
        row.modelId === sourceProof.sourceAnalysisRow.modelId &&
        row.promptOrTemplateVersion === sourceProof.sourceAnalysisRow.promptOrTemplateVersion
    ),
    policyRefsPreserved: snapshot.rows.every((row) => row.policyDecisionRef === sourceProof.policy.decisionId),
    deletedImageNoRawRetentionPreserved: snapshot.summary.deletedImageRowCount === snapshot.summary.rowCount,
    noRemoteOrApiAiClaimed: !snapshot.summary.remoteAiUsed && !snapshot.summary.apiAiUsed,
    noRuntimeSqliteWriterClaimed: !snapshot.claimBoundaries.runtimeSqliteWriterClaimed,
  },
  nonClaims: [
    'This proof consumes the existing real service WinRT OCR policy artifact and validates local AI journal/read-model projection contracts.',
    'It does not rerun live capture, add a production SQLite writer, render portal UI, use remote/API AI, claim policy authority, or claim enforcement.',
    'Raw screenshot custody remains deleted/no-retention; only refs, model metadata, policy refs, and projection refs are retained in this proof artifact.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`screen-ai-journal-read-model-proof-ok:${ProofPath}`);

function sourceRowFromServicePolicyProof(proof) {
  return {
    analysisRowId: proof.sourceAnalysisRow.rowId,
    queueJobId: proof.sourceAnalysisRow.queueJobId,
    localAiResultId: proof.policy.localAiResultId,
    modelRuntimeRef: proof.sourceAnalysisRow.modelRuntimeRef,
    modelId: proof.sourceAnalysisRow.modelId,
    promptOrTemplateVersion: proof.sourceAnalysisRow.promptOrTemplateVersion,
    primaryCategory: proof.sourceAnalysisRow.primaryCategory,
    confidence: proof.sourceAnalysisRow.confidence,
    imageDigest: proof.sourceAnalysisRow.imageDigest,
    imageDeletionState: proof.sourceAnalysisRow.imageDeletionState,
    rawImageRetained: proof.sourceAnalysisRow.rawImageRetained,
    custodyState: proof.sourceAnalysisRow.custodyState,
    evidenceReferenceIds: proof.policy.evidenceReferenceIds,
    policyDecisionRef: proof.policy.decisionId,
    policyAction: proof.policy.action,
    policyReasonCodes: proof.policy.reasonCodes,
    policyDryRun: proof.policy.dryRun,
    enforcementHandoffState: proof.policy.enforcementHandoffState,
    parentRuleRefs: proof.policy.ruleIds,
    readModelRowId: proof.readModelPolicyRow.rowId,
    readModelRawImageRetained: proof.readModelPolicyRow.rawImageRetained,
    readModelImageDeletionState: proof.readModelPolicyRow.imageDeletionState,
  };
}

function rowFailsValidation(row) {
  return (
    row.journalState !== 'journaled' ||
    row.sqliteProjectionState !== 'read-model-present' ||
    row.journalEntryRef.length === 0 ||
    row.sqliteRowRef.length === 0 ||
    row.evidenceReferenceIds.length === 0 ||
    row.parentRuleRefs.length === 0 ||
    row.imageDeletionState !== 'deleted' ||
    row.rawImageRetained ||
    !row.policyDryRun ||
    row.enforcementHandoffState === 'handed-off' ||
    Object.values(row.claimBoundaries).some((claim) => claim !== false)
  );
}

async function importDist(fileName) {
  return import(pathToFileURL(join(RepoRoot, 'packages', 'parent-domain', 'dist', fileName)).href);
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}

function relativePath(path) {
  return relative(RepoRoot, path).replaceAll('\\', '/');
}
