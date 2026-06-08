import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputDir = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'full-scope-readiness-audit');
const proofPath = join(outputDir, 'proof-summary.json');
const snapshotPath = join(outputDir, '00-source-snapshot.md');
const commandsPath = join(outputDir, '10-validation-commands.log');

const sourceArtifacts = {
  screenPlanClosure: 'output/screen-plan-proof/screen-plan-closure-audit/proof-summary.json',
  aiPlanClosure: 'output/ai-plan-proof/local-ai-plan-closure-audit/proof-summary.json',
  finalProductPath: 'output/screen-ai-pipeline-proof/final-product-path/proof-summary.json',
  finalAdapterAudit: 'output/screen-ai-pipeline-proof/final-adapter-dependency-audit/proof-summary.json',
  productChecklistDelta: 'output/screen-ai-pipeline-proof/product-checklist-delta/proof-summary.json',
  productChecklistDeltaMarkdown:
    'output/screen-ai-pipeline-proof/product-checklist-delta/product-capability-checklist-delta.md',
  pipelineChecklist: 'docs/plans/screen-ai-pipeline-plan/implementation-checklist.md',
};

const failures = [];
const screenPlanClosure = readJson(sourceArtifacts.screenPlanClosure);
const aiPlanClosure = readJson(sourceArtifacts.aiPlanClosure);
const finalProductPath = readJson(sourceArtifacts.finalProductPath);
const finalAdapterAudit = readJson(sourceArtifacts.finalAdapterAudit);
const productChecklistDelta = readJson(sourceArtifacts.productChecklistDelta);
const productChecklistDeltaMarkdown = readText(sourceArtifacts.productChecklistDeltaMarkdown);
const pipelineChecklist = readText(sourceArtifacts.pipelineChecklist);

assert(screenPlanClosure.assertions?.noProductCompleteClaim === true, 'screen-plan closure overclaims completion');
assert(
  screenPlanClosure.assertions?.finalProductPathRequiresAdapterAudit === true,
  'screen-plan closure does not require final adapter audit'
);
assert(
  screenPlanClosure.assertions?.adapterAuditKeepsProductCompletionBlocked === true,
  'screen-plan closure lost adapter blocker'
);
assert(
  aiPlanClosure.closure?.controlledCapturedScreensAnalyzed === true,
  'AI closure lost controlled capture analysis'
);
assert(aiPlanClosure.closure?.liveOperatorArtifactsAnalyzed === true, 'AI closure lost live operator analysis');
assert(
  aiPlanClosure.closure?.serviceOcrAnalyzedCapturedPixels === true,
  'AI closure lost service OCR captured-pixel proof'
);
assert(aiPlanClosure.closure?.remoteApiAiClaimed === false, 'AI closure claims remote/API AI');
assert(aiPlanClosure.closure?.rawImageRetainedByDefault === false, 'AI closure permits raw image retention');
assert(
  aiPlanClosure.closure?.finalProductCompleteDeferredToPipeline === true,
  'AI closure no longer defers final completion to pipeline'
);

assert(finalProductPath.status === 'ok', 'final product path proof is not ok');
assert(finalProductPath.closure?.finalPathEvidenceComplete === true, 'final path evidence is incomplete');
assert(
  finalProductPath.closure?.screenAndAiPrerequisitesStacked === true,
  'final path does not stack screen and AI prerequisites'
);
assert(
  finalProductPath.closure?.serviceEventProducersAndSubscriberCovered === true,
  'final path lost service event producer/subscriber proof'
);
assert(
  finalProductPath.closure?.serviceWinRtOcrLivePolicyCovered === true,
  'final path lost service WinRT OCR policy proof'
);
assert(
  finalProductPath.closure?.singleRuntimeSessionRerun === true,
  'final path lost fresh service OCR source rerun proof'
);
assert(finalProductPath.closure?.rawScreenshotsRetainedByDefault === false, 'final path retains raw screenshots');
assert(finalProductPath.closure?.remoteAiUsedForChildSafety === false, 'final path uses remote AI for child safety');
assert(finalProductPath.closure?.finalPipelineProductComplete === false, 'final path claims product-complete');
assert(
  finalProductPath.closure?.finalPipelineProductCompleteBlockedByAdapterGate === true,
  'final path is not blocked by adapter gate'
);

assert(finalAdapterAudit.status === 'blocked-by-upstream-adapter-artifacts', 'adapter audit is not blocked');
assert(
  finalAdapterAudit.closure?.finalPathFreshServiceRerunProved === true,
  'adapter audit does not consume fresh service rerun proof'
);
assert(finalAdapterAudit.closure?.blockedAdapterRows === 6, 'adapter blocker row count changed');
assert(finalAdapterAudit.closure?.custodyArtifactRows === 3, 'adapter custody row count changed');
assert(finalAdapterAudit.closure?.claimUpgradeRows === 0, 'adapter audit contains claim upgrades');

assert(
  productChecklistDelta.status === 'doc-delta-ready-product-checklist-locked',
  'product checklist delta status changed'
);
assert(productChecklistDelta.closure?.productChecklistEdited === false, 'product checklist delta edited checklist');
assert(
  productChecklistDelta.closure?.finalPathFreshServiceRerunProved === true,
  'product checklist delta does not carry fresh service rerun proof'
);
assert(
  productChecklistDeltaMarkdown.includes('fresh service OCR source rerun evidence'),
  'product checklist delta markdown omits fresh service rerun evidence'
);
assert(
  pipelineChecklist.includes(
    '- [ ] Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.'
  ),
  'pipeline checklist product-complete adapter row is no longer open'
);

if (failures.length > 0) {
  throw new Error(
    `Screen AI full-scope readiness audit failed:\n${failures.map((failure) => `- ${failure}`).join('\n')}`
  );
}

const blockedAdapterRows = finalAdapterAudit.nextRequiredArtifacts.map((row) => ({
  rowId: row.rowId,
  boundary: row.boundary,
  handoffOwner: row.handoffOwner,
  expectedProofFile: row.expectedProofFile,
  custodyArtifact: row.custodyArtifact,
  missingArtifact: row.missingArtifact,
}));

const proof = {
  status: 'ready-except-external-adapter-and-product-checklist-dependencies',
  proofKind: 'screen-ai-full-scope-readiness-audit',
  generatedAt: new Date().toISOString(),
  sourceArtifacts,
  closure: {
    screenPlanPrerequisitesAudited: true,
    aiPlanPrerequisitesAudited: true,
    finalPipelineEvidenceComplete: true,
    serviceEventRuntimeCovered: true,
    serviceWinRtOcrPolicyFreshRerunCovered: true,
    householdMeshNoRawProviderValidationCovered: true,
    rawScreenshotsRetainedByDefault: false,
    remoteAiUsedForChildSafety: false,
    productChecklistDeltaReadyButNotApplied: true,
    finalPipelineProductComplete: false,
    finalPipelineProductCompleteBlockedByAdapterGate: true,
    externalAdapterDependencyRows: blockedAdapterRows.length,
  },
  blockedAdapterRows,
  productChecklistDelta: {
    status: productChecklistDelta.status,
    deltaMarkdown: productChecklistDelta.deltaMarkdown,
    productChecklistEdited: productChecklistDelta.closure?.productChecklistEdited === true,
    finalPathFreshServiceRerunProved: productChecklistDelta.closure?.finalPathFreshServiceRerunProved === true,
  },
  nonClaims: [
    'This audit does not edit docs/product-capability-checklist.md.',
    'This audit does not claim product-complete screen, AI, or pipeline execution.',
    'This audit does not implement broad installed-app, host network/domain, managed active-tab, Android, iOS, or Linux execution artifacts.',
    'This audit does not replace live macOS, Linux desktop, physical Android, physical iOS, authenticated-account social, live-view production, or production OCR/VLM quality gates still listed by the screen-plan closure audit.',
  ],
};

mkdirSync(outputDir, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(snapshotPath, sourceSnapshot(proof));
writeFileSync(commandsPath, validationCommands());
console.log(`screen-ai-full-scope-readiness-audit-ok:${relativePath(proofPath)}`);

function readJson(path) {
  return JSON.parse(readText(path));
}

function readText(path) {
  const absolute = resolve(repoRoot, path);
  assert(existsSync(absolute), `missing source artifact ${path}`);
  return readFileSync(absolute, 'utf8');
}

function sourceSnapshot(proof) {
  const rows = Object.entries(sourceArtifacts)
    .map(([name, path]) => `- ${name}: \`${path}\``)
    .join('\n');
  return [
    '# Screen AI Full Scope Readiness Audit',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    '## Source Artifacts',
    '',
    rows,
    '',
    '## Closure',
    '',
    '```json',
    JSON.stringify(proof.closure, null, 2),
    '```',
    '',
  ].join('\n');
}

function validationCommands() {
  return [
    'node --check scripts/test/screen-ai-full-scope-readiness-audit.mjs',
    'node scripts/test/screen-ai-full-scope-readiness-audit.mjs',
    'git diff --check',
    'npm run lanes:guard',
    'npm run hub:guard',
    '',
  ].join('\n');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function assert(condition, message) {
  if (!condition) {
    failures.push(message);
  }
}
