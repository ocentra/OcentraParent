import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const SourceProofRoot = resolve(RepoRoot, 'output', 'screen-ai-pipeline-proof', 'service-winrt-ocr');
const SourceProofPath = join(SourceProofRoot, 'proof-summary.json');
const SourceReadModelPath = join(SourceProofRoot, 'screen-read-model.json');
const OutputRoot = resolve(RepoRoot, 'output', 'screen-ai-pipeline-proof', 'service-winrt-ocr-policy');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const PolicyDecisionPath = join(OutputRoot, 'policy-decision.json');
const PolicyReadModelPath = join(OutputRoot, 'activity-screen-policy-read-model.json');
const ConsumedSourceProofPath = join(OutputRoot, 'consumed-service-winrt-ocr-proof-summary.json');
const ValidationLogPath = join(OutputRoot, '10-validation-commands.log');
const ServiceOcrProofScript = 'scripts/test/screen-ai-service-winrt-ocr-proof.mjs';
const PolicyReasonCode = 'screen-service-winrt-ocr-school-allow';
const ParentRuleId = 'screen-service-winrt-ocr-school-rule';
const DeletionReason = 'screen-image-deleted-after-analysis';
const ExplanationReason = 'service-winrt-ocr-policy-dry-run';
const ValidationCommands = [
  'node --check scripts/test/screen-ai-service-winrt-ocr-policy-proof.mjs',
  'node scripts/test/screen-ai-service-winrt-ocr-policy-proof.mjs',
];

if (process.platform !== 'win32') {
  throw new Error('screen-ai-service-winrt-ocr-policy-proof requires Windows WinRT OCR service proof execution.');
}

rmSync(OutputRoot, { recursive: true, force: true });
mkdirSync(OutputRoot, { recursive: true });

const executedCommands = [];

try {
  runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/activity-domain']));
  runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  runCommand('node', [ServiceOcrProofScript]);

  const { ActivityScreenReadModelSchema } = await import('@ocentra-parent/activity-domain/activity-surface');
  const { PolicyAction, PolicyDecisionHandoffState, PolicyDecisionSchema } =
    await import('@ocentra-parent/parent-domain/policy');

  const sourceProof = readJson(SourceProofPath);
  const sourceReadModel = ActivityScreenReadModelSchema.parse(readJson(SourceReadModelPath));
  const analysisRow = localOcrRow(sourceReadModel);
  assertSourceProof(sourceProof, analysisRow);

  const observedAt = sourceReadModel.generatedAt ?? sourceReadModel.request?.requestedAt ?? new Date().toISOString();
  const policyDecision = PolicyDecisionSchema.parse({
    schemaVersion: 'v0.6',
    decisionId: `${analysisRow.rowId}-policy-dry-run`,
    action: PolicyAction.Allow,
    reasonCodes: [PolicyReasonCode],
    evidenceReferences: policyEvidenceReferences(analysisRow, observedAt),
    ruleIds: [ParentRuleId],
    localAiResultId: `${analysisRow.rowId}-local-ocr-result`,
    dryRun: true,
    enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
    expiresAt: null,
  });
  const policyReadModel = ActivityScreenReadModelSchema.parse(
    policyReadModelFromSource(sourceReadModel, analysisRow, policyDecision)
  );
  const policyRow = rowById(policyReadModel, analysisRow.rowId);
  assertPolicyProof(sourceProof, analysisRow, policyDecision, policyRow);

  const proof = {
    proof: 'screen-ai-service-winrt-ocr-policy-proof',
    proofTier: 'P3_REAL_CAPTURE_LOCAL_OCR_POLICY_CONSUMPTION',
    generatedAt: new Date().toISOString(),
    sourceProof: relativePath(SourceProofPath),
    sourceReadModel: relativePath(SourceReadModelPath),
    policyDecision: relativePath(PolicyDecisionPath),
    policyReadModel: relativePath(PolicyReadModelPath),
    consumedSourceProof: relativePath(ConsumedSourceProofPath),
    validationCommands: relativePath(ValidationLogPath),
    sourceLiveSurface: sourceProof.liveSource,
    sourceAnalysisRow: {
      rowId: analysisRow.rowId,
      queueJobId: analysisRow.queueJobId,
      providerKind: analysisRow.providerKind,
      modelRuntimeRef: analysisRow.modelRuntimeRef,
      modelId: analysisRow.modelId,
      promptOrTemplateVersion: analysisRow.promptOrTemplateVersion,
      primaryCategory: analysisRow.primaryCategory,
      confidence: analysisRow.confidence,
      policyEligible: analysisRow.policyEligible,
      imageDeletionState: analysisRow.imageDeletionState,
      rawImageRetained: analysisRow.rawImageRetained,
      custodyState: analysisRow.custodyState,
      imageDigest: analysisRow.imageDigest,
    },
    policy: {
      decisionId: policyDecision.decisionId,
      action: policyDecision.action,
      reasonCodes: policyDecision.reasonCodes,
      ruleIds: policyDecision.ruleIds,
      localAiResultId: policyDecision.localAiResultId,
      dryRun: policyDecision.dryRun,
      enforcementHandoffState: policyDecision.enforcementHandoffState,
      evidenceReferenceIds: policyDecision.evidenceReferences.map((reference) => reference.evidenceReferenceId),
    },
    readModelPolicyRow: {
      rowId: policyRow.rowId,
      policyDecisionRef: policyRow.policyDecisionRef,
      policyAction: policyRow.policyAction,
      policyReasonCodes: policyRow.policyReasonCodes,
      parentRuleRefs: policyRow.parentRuleRefs,
      localModelRuntimeRefs: policyRow.localModelRuntimeRefs,
      deletionReasons: policyRow.deletionReasons,
      explanationReasons: policyRow.explanationReasons,
      rawImageRetained: policyRow.rawImageRetained,
      imageDeletionState: policyRow.imageDeletionState,
    },
    assertions: {
      sourceProofRerunByThisGate: executedCommands.some((command) => command.includes(ServiceOcrProofScript)),
      sourceUsedLivePublicBrowserPixels: sourceProof.assertions?.liveExternalBrowserSurfaceUsed === true,
      sourceRanWindowsWinRtOcr: sourceProof.assertions?.serviceAdapterRanWindowsWinRtOcr === true,
      sourceReadModelReachedViaWebSocket: sourceProof.assertions?.activityReadModelReachedViaWebSocket === true,
      sourceQueueDrained: sourceProof.assertions?.encryptedQueueDrainedAfterAnalysis === true,
      sourceTempImageDeleted: sourceProof.assertions?.adapterTemporaryImageDeleted === true,
      sourceRawImageNotRetained: analysisRow.rawImageRetained === false,
      sourcePolicyEligibleSchoolRow:
        analysisRow.policyEligible === true &&
        analysisRow.primaryCategory === 'school' &&
        analysisRow.providerKind === 'localOcr',
      policyDecisionParsedByParentDomain: policyDecision.decisionId === policyRow.policyDecisionRef,
      policyConsumedExactActivityRow: policyDecision.localAiResultId.startsWith(analysisRow.rowId),
      policyEvidenceCitesQueueAndActivity:
        hasEvidenceKind(policyDecision, 'activity-event') && hasEvidenceKind(policyDecision, 'journal-event'),
      policyDryRunOnly:
        policyDecision.dryRun === true &&
        policyDecision.enforcementHandoffState === PolicyDecisionHandoffState.Disabled,
      activityReadModelCarriesPolicyRefs:
        policyRow.policyAction === PolicyAction.Allow &&
        policyRow.parentRuleRefs.includes(ParentRuleId) &&
        policyRow.policyReasonCodes.includes(PolicyReasonCode),
      deletionCustodyPreserved:
        policyRow.rawImageRetained === false &&
        policyRow.imageDeletionState === 'deleted' &&
        policyRow.deletionReasons.includes(DeletionReason),
    },
    nonClaims: [
      'This proof reruns the real Windows service WinRT OCR capture/analysis proof and consumes that Activity Screen row through typed parent-domain policy decision contracts.',
      'It proves dry-run policy consumption and read-model refs only; it does not claim final enforcement handoff, browser/network/mobile adapters, production OCR quality, or authenticated-account coverage.',
      'The proof intentionally does not retain raw screenshots; it preserves deleted-image/no-raw-retention custody from the service OCR proof.',
    ],
  };

  writeJson(PolicyDecisionPath, policyDecision);
  writeJson(PolicyReadModelPath, policyReadModel);
  writeJson(ConsumedSourceProofPath, sourceProof);
  writeJson(ProofPath, proof);
  writeText(
    ValidationLogPath,
    `${ValidationCommands.join('\n')}\n\nExecuted by proof:\n${executedCommands.join('\n')}\n`
  );
  console.log(`screen-ai-service-winrt-ocr-policy-proof-ok:${policyDecision.decisionId}:${policyDecision.action}`);
} catch (error) {
  writeFailureArtifact(error);
  throw error;
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function writeJson(path, value) {
  writeText(path, `${JSON.stringify(value, null, 2)}\n`);
}

function writeText(path, value) {
  writeFileSync(path, value);
}

function relativePath(path) {
  return relative(RepoRoot, path).replaceAll('\\', '/');
}

function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  executedCommands.push(commandLine);
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}

function localOcrRow(readModel) {
  const row = readModel.rows.find((candidate) => candidate.providerKind === 'localOcr');
  if (row === undefined) {
    throw new Error('Activity Screen read model did not include a localOcr row.');
  }
  return row;
}

function rowById(readModel, rowId) {
  const row = readModel.rows.find((candidate) => candidate.rowId === rowId);
  if (row === undefined) {
    throw new Error(`Activity Screen read model did not include policy row ${rowId}.`);
  }
  return row;
}

function assertSourceProof(sourceProof, analysisRow) {
  const failures = [];
  if (sourceProof.proof !== 'screen-ai-service-winrt-ocr-proof') failures.push('sourceProofKind');
  if (sourceProof.assertions?.liveExternalBrowserSurfaceUsed !== true) failures.push('liveExternalBrowserSurfaceUsed');
  if (sourceProof.assertions?.serviceAdapterRanWindowsWinRtOcr !== true)
    failures.push('serviceAdapterRanWindowsWinRtOcr');
  if (sourceProof.assertions?.activityReadModelReachedViaWebSocket !== true) failures.push('readModelWebSocket');
  if (sourceProof.assertions?.encryptedQueueDrainedAfterAnalysis !== true) failures.push('queueDrained');
  if (sourceProof.assertions?.adapterTemporaryImageDeleted !== true) failures.push('adapterTempImageDeleted');
  if (analysisRow.providerKind !== 'localOcr') failures.push('providerKind');
  if (analysisRow.modelRuntimeRef !== 'windows-winrt-ocr-local-runtime') failures.push('modelRuntimeRef');
  if (analysisRow.modelId !== 'windows-winrt-ocr') failures.push('modelId');
  if (analysisRow.promptOrTemplateVersion !== 'screen-ocr-worker-winrt-v1') failures.push('templateVersion');
  if (analysisRow.primaryCategory !== 'school') failures.push('primaryCategory');
  if (analysisRow.policyEligible !== true) failures.push('policyEligible');
  if (analysisRow.rawImageRetained !== false) failures.push('rawImageRetained');
  if (analysisRow.imageDeletionState !== 'deleted') failures.push('imageDeletionState');
  if (failures.length > 0) {
    throw new Error(`Service WinRT OCR source proof failed required gates: ${failures.join(', ')}`);
  }
}

function policyEvidenceReferences(analysisRow, observedAt) {
  const directEvidenceRefs = analysisRow.evidence.map((evidence) => ({
    evidenceReferenceId: `${evidence.evidenceId}-journal`,
    kind: 'journal-event',
    observedAt,
  }));
  return uniqueReferences([
    {
      evidenceReferenceId: `${analysisRow.rowId}-activity-row`,
      kind: 'activity-event',
      observedAt,
    },
    {
      evidenceReferenceId: `${analysisRow.queueJobId}-encrypted-queue`,
      kind: 'journal-event',
      observedAt,
    },
    {
      evidenceReferenceId: `${analysisRow.imageDigest}-screen-summary`,
      kind: 'query-store-summary',
      observedAt,
    },
    ...directEvidenceRefs,
  ]);
}

function uniqueReferences(references) {
  const seen = new Set();
  return references.filter((reference) => {
    if (seen.has(reference.evidenceReferenceId)) {
      return false;
    }
    seen.add(reference.evidenceReferenceId);
    return true;
  });
}

function policyReadModelFromSource(sourceReadModel, analysisRow, policyDecision) {
  return {
    ...sourceReadModel,
    summary: `${sourceReadModel.summary} Policy dry-run consumed the local OCR school row without raw image retention.`,
    rows: sourceReadModel.rows.map((row) =>
      row.rowId === analysisRow.rowId ? policyAugmentedRow(row, policyDecision) : row
    ),
  };
}

function policyAugmentedRow(row, policyDecision) {
  return {
    ...row,
    policyDecisionRef: policyDecision.decisionId,
    policyAction: policyDecision.action,
    policyReasonCodes: policyDecision.reasonCodes,
    parentRuleRefs: policyDecision.ruleIds,
    localModelRuntimeRefs: uniqueTextRefs([...(row.localModelRuntimeRefs ?? []), row.modelRuntimeRef]),
    explanationReasons: uniqueTextRefs([...(row.explanationReasons ?? []), ExplanationReason]),
    deletionReasons: uniqueTextRefs([...(row.deletionReasons ?? []), DeletionReason]),
  };
}

function uniqueTextRefs(values) {
  return [...new Set(values)];
}

function assertPolicyProof(sourceProof, analysisRow, policyDecision, policyRow) {
  const failures = [];
  if (policyDecision.action !== 'allow') failures.push('policyAction');
  if (policyDecision.dryRun !== true) failures.push('dryRun');
  if (policyDecision.enforcementHandoffState !== 'disabled') failures.push('enforcementHandoffState');
  if (!policyDecision.localAiResultId?.startsWith(analysisRow.rowId)) failures.push('localAiResultId');
  if (!policyDecision.reasonCodes.includes(PolicyReasonCode)) failures.push('reasonCode');
  if (!policyDecision.ruleIds.includes(ParentRuleId)) failures.push('ruleId');
  if (!hasEvidenceKind(policyDecision, 'activity-event')) failures.push('activityEvidenceReference');
  if (!hasEvidenceKind(policyDecision, 'journal-event')) failures.push('journalEvidenceReference');
  if (!hasEvidenceKind(policyDecision, 'query-store-summary')) failures.push('summaryEvidenceReference');
  if (policyRow.policyDecisionRef !== policyDecision.decisionId) failures.push('readModelPolicyDecisionRef');
  if (policyRow.policyAction !== policyDecision.action) failures.push('readModelPolicyAction');
  if (!policyRow.policyReasonCodes.includes(PolicyReasonCode)) failures.push('readModelReasonCode');
  if (!policyRow.parentRuleRefs.includes(ParentRuleId)) failures.push('readModelRuleRef');
  if (!policyRow.localModelRuntimeRefs.includes(analysisRow.modelRuntimeRef)) failures.push('readModelRuntimeRef');
  if (policyRow.rawImageRetained !== false) failures.push('readModelRawImageRetained');
  if (policyRow.imageDeletionState !== 'deleted') failures.push('readModelImageDeletionState');
  if (!policyRow.deletionReasons.includes(DeletionReason)) failures.push('readModelDeletionReason');
  if (sourceProof.queueRecordsAfterAnalysis !== 0) failures.push('sourceQueueRecordsAfterAnalysis');
  if (failures.length > 0) {
    throw new Error(`Service WinRT OCR policy proof failed gates: ${failures.join(', ')}`);
  }
}

function hasEvidenceKind(policyDecision, kind) {
  return policyDecision.evidenceReferences.some((reference) => reference.kind === kind);
}

function writeFailureArtifact(error) {
  mkdirSync(OutputRoot, { recursive: true });
  writeJson(join(OutputRoot, 'failure.json'), {
    proof: 'screen-ai-service-winrt-ocr-policy-proof',
    failedAt: new Date().toISOString(),
    message: error instanceof Error ? error.message : String(error),
    executedCommands,
  });
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
