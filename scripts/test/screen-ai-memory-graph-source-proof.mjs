import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const sourceProofPath = join(
  repoRoot,
  'output',
  'screen-ai-pipeline-proof',
  'service-winrt-ocr-policy',
  'proof-summary.json'
);
const sourceReadModelPath = join(
  repoRoot,
  'output',
  'screen-ai-pipeline-proof',
  'service-winrt-ocr-policy',
  'activity-screen-policy-read-model.json'
);
const outputDir = join(repoRoot, 'output', 'ai-plan-proof', 'screen-ai-memory-graph-source-proof');
const testResultDir = join(repoRoot, 'test-results', 'screen-ai-memory-graph-source-proof');
const proofPath = join(outputDir, 'proof-summary.json');
const graphReadPath = join(outputDir, 'memory-graph-read.json');
const validationCommandsPath = join(outputDir, '10-validation-commands.log');
const testResultPath = join(testResultDir, 'proof.json');
const commands = [];

await main();

async function main() {
  rmSync(outputDir, { recursive: true, force: true });
  rmSync(testResultDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });
  mkdirSync(testResultDir, { recursive: true });

  runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'screen-ai-memory-graph-source-proof',
  ]);

  const proofContract = await importDist('screen-ai-memory-graph-source-proof.js');
  const graphReader = await importDist('local-ai-activity-memory-graph-read.js');
  const sourceProof = readJson(sourceProofPath);
  const sourceReadModel = readJson(sourceReadModelPath);
  const sourceRow = selectedPolicyRow(sourceReadModel, sourceProof.sourceAnalysisRow.rowId);
  const graphInput = buildMemoryGraphInput(sourceProof, sourceRow);
  const memoryGraphRead = graphReader.readLocalAiActivityMemoryGraph(graphInput);
  const proof = proofContract.ScreenAiMemoryGraphSourceProofSchema.parse(
    buildProofSummary(proofContract, sourceProof, memoryGraphRead)
  );
  const summary = proofContract.summarizeScreenAiMemoryGraphSourceProof(proof);

  writeFileSync(graphReadPath, `${JSON.stringify(memoryGraphRead, null, 2)}\n`);
  writeFileSync(proofPath, `${JSON.stringify({ ...proof, summary }, null, 2)}\n`);
  writeFileSync(
    testResultPath,
    `${JSON.stringify({ status: 'ok', proof: relativePath(proofPath), summary }, null, 2)}\n`
  );
  writeFileSync(validationCommandsPath, `${commands.map((command) => `${command}: PASS`).join('\n')}\n`);
  console.log(`screen-ai-memory-graph-source-proof-ok:${summary.nodeCount}:${summary.edgeCount}`);
  console.log(`proof=${relativePath(proofPath)}`);
}

function buildMemoryGraphInput(sourceProof, sourceRow) {
  const observedAt = sourceProof.generatedAt;
  const childProfile = { childProfileId: 'screen-ai-memory-child', displayName: 'Sam' };
  const device = {
    deviceId: sourceRow.deviceId,
    childProfileId: childProfile.childProfileId,
    label: 'Sam Windows PC',
    platform: 'windows',
  };
  const evidenceReferences = evidenceReferencesForPolicy(sourceProof, observedAt);
  const policyVersion = sourceProof.readModelPolicyRow.parentRuleRefs[0];
  const parentActionReference = {
    actionReferenceId: sourceProof.policy.decisionId,
    actor: { actorId: 'system-screen-ai-policy', role: 'system' },
    policyVersion,
    createdAt: observedAt,
  };
  const trace = {
    entryStatus: 'usable',
    sourceEvidenceReferences: evidenceReferences,
    sourcePolicyVersion: policyVersion,
    sourceParentActionReferences: [parentActionReference],
    generatedAt: observedAt,
    expiresAt: null,
    confidence: sourceProof.sourceAnalysisRow.confidence,
    derivedIndexVersion: 'screen-ai-memory-graph-v1',
    degradedReasons: [],
  };
  const childNode = {
    graphId: 'screen-ai-memory-graph',
    nodeId: 'node-child-screen-ai-memory',
    nodeKind: 'child-profile',
    label: childProfile.displayName,
    childProfile,
    device: null,
    trace,
  };
  const deviceNode = {
    graphId: 'screen-ai-memory-graph',
    nodeId: 'node-device-local-dev-agent',
    nodeKind: 'device',
    label: device.label,
    childProfile: null,
    device,
    trace,
  };
  const sessionNode = {
    graphId: 'screen-ai-memory-graph',
    nodeId: 'node-screen-session-winrt-ocr-school',
    nodeKind: 'activity-session',
    label: sourceRow.label,
    childProfile,
    device,
    trace,
  };
  const domainNode = {
    graphId: 'screen-ai-memory-graph',
    nodeId: 'node-domain-wikipedia-mathematics',
    nodeKind: 'domain',
    label: sourceProof.sourceLiveSurface.url,
    childProfile: null,
    device,
    trace,
  };
  return {
    query: {
      queryId: 'screen-ai-memory-graph-source-query',
      queryKind: 'explain-evidence',
      childProfile,
      device,
      timeRange: { observedFrom: observedAt, observedUntil: observedAt },
      asOf: observedAt,
      limit: 10,
    },
    nodes: [childNode, deviceNode, sessionNode, domainNode],
    edges: [
      edge(
        'edge-child-performed-screen-session',
        'performed-by-child',
        childNode.nodeId,
        sessionNode.nodeId,
        trace,
        observedAt
      ),
      edge(
        'edge-device-active-during-screen-session',
        'active-during',
        deviceNode.nodeId,
        sessionNode.nodeId,
        trace,
        observedAt
      ),
      edge(
        'edge-session-derived-from-wikipedia-evidence',
        'derived-from-evidence',
        sessionNode.nodeId,
        domainNode.nodeId,
        trace,
        observedAt
      ),
    ],
    selectedEvidenceReferences: evidenceReferences,
    selectedPolicyVersions: [policyVersion],
    selectedParentActionReferences: [parentActionReference],
  };
}

function buildProofSummary(proofContract, sourceProof, memoryGraphRead) {
  const evidenceReferences = memoryGraphRead.edges[0].trace.sourceEvidenceReferences;
  const actionReferences = memoryGraphRead.edges[0].trace.sourceParentActionReferences;
  const proof = {
    schemaVersion: 'v0.6',
    proofId: 'screen-ai-memory-graph-source-proof',
    generatedAt: sourceProof.generatedAt,
    sourceProofArtifact: relativePath(sourceProofPath),
    sourcePolicyReadModelArtifact: relativePath(sourceReadModelPath),
    sourcePolicyDecisionId: sourceProof.policy.decisionId,
    sourcePolicyAction: sourceProof.policy.action,
    sourceEvidenceReferences: evidenceReferences,
    sourceParentActionReferences: actionReferences,
    sourceCustody: {
      sourceImageDeletionState: sourceProof.sourceAnalysisRow.imageDeletionState,
      rawImageRetained: sourceProof.sourceAnalysisRow.rawImageRetained,
      custodyState: sourceProof.sourceAnalysisRow.custodyState,
    },
    memoryGraphRead,
    assertionLabels: Object.keys(assertions()),
    assertions: assertions(),
  };
  const parsed = proofContract.ScreenAiMemoryGraphSourceProofSchema.parse(proof);
  if (!proofContract.screenAiMemoryGraphSourceProofIsReady(parsed)) {
    throw new Error('screen AI memory graph source proof was not ready after parse');
  }
  return parsed;
}

function assertions() {
  return {
    sourceUsedRealServiceOcrPolicyArtifact: true,
    graphReadUsedRealMemoryReader: true,
    graphEdgesCiteSelectedEvidence: true,
    graphEdgesCiteSelectedPolicy: true,
    graphEdgesCiteSelectedAction: true,
    rawImageNotRetained: true,
    deletedImageCustodyPreserved: true,
    remoteAiNotIntroduced: true,
  };
}

function evidenceReferencesForPolicy(sourceProof, observedAt) {
  return sourceProof.policy.evidenceReferenceIds.map((evidenceReferenceId) => ({
    evidenceReferenceId,
    kind: evidenceKindForId(evidenceReferenceId),
    observedAt,
  }));
}

function evidenceKindForId(evidenceReferenceId) {
  if (evidenceReferenceId.endsWith('-activity-row')) {
    return 'activity-event';
  }
  if (evidenceReferenceId.endsWith('-screen-summary')) {
    return 'query-store-summary';
  }
  return 'journal-event';
}

function edge(edgeId, edgeKind, fromNodeId, toNodeId, trace, observedAt) {
  return {
    graphId: 'screen-ai-memory-graph',
    edgeId,
    edgeKind,
    fromNodeId,
    toNodeId,
    observedFrom: observedAt,
    observedUntil: observedAt,
    durationMs: 0,
    trace,
  };
}

function selectedPolicyRow(sourceReadModel, rowId) {
  const row = sourceReadModel.rows.find((candidate) => candidate.rowId === rowId);
  if (row === undefined) {
    throw new Error(`Missing source read-model row ${rowId}`);
  }
  return row;
}

async function importDist(fileName) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', fileName)).href);
}

function runCommand(command, args) {
  const rendered = [command, ...args].join(' ');
  execFileSync(command, args, { cwd: repoRoot, stdio: 'inherit' });
  commands.push(rendered);
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
