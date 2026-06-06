import { execFileSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'ai-plan-proof', 'local-ai-graph-reference-contract-proof');
const testResultsDir = join(repoRoot, 'test-results', 'local-ai-graph-reference-contract-proof');
const proofSummaryPath = join(outputDir, 'proof-summary.json');
const validationCommandsPath = join(outputDir, 'validation-commands.log');
const testResultPath = join(testResultsDir, 'proof.json');
const observedAt = '2026-06-06T06:40:00.000Z';

await main();

async function main() {
  rmSync(outputDir, { recursive: true, force: true });
  rmSync(testResultsDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });
  mkdirSync(testResultsDir, { recursive: true });

  const commands = [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-graph-reference-contract-proof',
  ];
  runCommand(commands[0]);
  runCommand(commands[1]);

  const contract = await import(
    pathToFileURL(join(repoRoot, 'packages/parent-domain/dist/local-ai-graph-reference-contract-proof.js')).href
  );
  const proof = contract.buildLocalAiGraphReferenceContractProof(buildProofInput());
  const proofSummary = {
    status: 'ok',
    proofKind: proof.proofKind,
    generatedAt: new Date().toISOString(),
    proof: relativePath(proofSummaryPath),
    testResult: relativePath(testResultPath),
    validationCommands: relativePath(validationCommandsPath),
    graphReadState: {
      returnedNodeCount: proof.graphReadResult.returnedNodeCount,
      returnedEdgeCount: proof.graphReadResult.returnedEdgeCount,
      omittedEdgeCount: proof.graphReadResult.omittedEdgeCount,
      degradedReasons: proof.graphReadResult.degradedReasons,
    },
    summary: proof.summary,
    claimBoundaries: proof.claimBoundaries,
    sourceArtifacts: [
      'packages/parent-domain/src/local-ai-references.ts',
      'packages/parent-domain/src/local-ai-activity-memory-graph.ts',
      'packages/parent-domain/src/local-ai-activity-memory-graph-read.ts',
      'packages/parent-domain/src/local-ai-graph-reference-contract-proof.ts',
    ],
    claimsProved: [
      'Local AI graph references are schema-validated and must cite selected source evidence before use.',
      'Minimal graph edges are read from the activity memory graph only when endpoints, selected evidence, policy version, parent action refs, freshness, and time range all match.',
      'The graph proof remains local-only and non-authoritative: no UI, model-quality, policy-authority, enforcement, remote/API AI, or raw-retention claim is made.',
    ],
    nonClaims: [
      'This proof does not execute a model or claim production model quality.',
      'This proof does not render a memory/graph evidence surface.',
      'This proof does not make policy decisions or dispatch enforcement.',
      'This proof does not create a production graph storage/index runtime.',
    ],
  };

  writeJson(proofSummaryPath, proofSummary);
  writeJson(testResultPath, proof);
  writeFileSync(validationCommandsPath, `${commands.join('\n')}\n`);
  console.log(`local-ai-graph-reference-contract-proof-ok:${relativePath(proofSummaryPath)}`);
}

function buildProofInput() {
  const childProfile = { childProfileId: 'child-local-ai-graph', displayName: 'Sam' };
  const device = {
    deviceId: 'device-local-ai-graph',
    childProfileId: 'child-local-ai-graph',
    label: 'Sam Windows PC',
    platform: 'windows',
  };
  const sourceEvidence = {
    evidenceReferenceId: 'local-ai-graph-source-evidence',
    kind: 'journal-event',
    observedAt,
  };
  const parentAction = {
    actionReferenceId: 'local-ai-graph-parent-action',
    actor: { actorId: 'parent-local-ai-graph', role: 'parent' },
    policyVersion: 'local-ai-graph-policy-v1',
    createdAt: '2026-06-06T06:35:00.000Z',
  };
  const trace = graphTrace(sourceEvidence, parentAction);
  const childNode = graphNode('local-ai-graph-child', 'child-profile', 'Sam', childProfile, null, trace);
  const domainNode = graphNode('local-ai-graph-domain', 'domain', 'example.test', null, device, trace);
  return {
    schemaVersion: 'v0.6',
    graphReadInput: {
      query: {
        queryId: 'local-ai-graph-query',
        queryKind: 'activity-by-time-range',
        childProfile,
        device,
        timeRange: {
          observedFrom: '2026-06-06T06:30:00.000Z',
          observedUntil: '2026-06-06T06:45:00.000Z',
        },
        asOf: '2026-06-06T06:45:00.000Z',
        limit: 2,
      },
      nodes: [childNode, domainNode],
      edges: [
        {
          graphId: 'local-ai-graph',
          edgeId: 'local-ai-graph-edge-visited',
          edgeKind: 'visited',
          fromNodeId: 'local-ai-graph-child',
          toNodeId: 'local-ai-graph-domain',
          observedFrom: observedAt,
          observedUntil: '2026-06-06T06:44:00.000Z',
          durationMs: 240000,
          trace,
        },
      ],
      selectedEvidenceReferences: [sourceEvidence],
      selectedPolicyVersions: ['local-ai-graph-policy-v1'],
      selectedParentActionReferences: [parentAction],
    },
    graphReferences: [
      {
        graphReferenceId: 'local-ai-graph-reference',
        kind: 'graph-edge',
        sourceEvidenceReferences: [sourceEvidence],
        sourcePolicyVersion: 'local-ai-graph-policy-v1',
        generatedAt: observedAt,
        confidence: 0.81,
        derivedIndexVersion: 'local-ai-graph-index-v1',
      },
    ],
    claimBoundaries: {
      remoteAiUsed: false,
      apiAiUsed: false,
      modelQualityClaimed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
      uiClaimed: false,
      rawEvidenceRetained: false,
      uncitedGraphAllowed: false,
    },
  };
}

function graphTrace(sourceEvidence, parentAction) {
  return {
    entryStatus: 'usable',
    sourceEvidenceReferences: [sourceEvidence],
    sourcePolicyVersion: 'local-ai-graph-policy-v1',
    sourceParentActionReferences: [parentAction],
    generatedAt: '2026-06-06T06:41:00.000Z',
    expiresAt: '2026-06-06T07:00:00.000Z',
    confidence: 0.84,
    derivedIndexVersion: 'local-ai-graph-index-v1',
    degradedReasons: [],
  };
}

function graphNode(nodeId, nodeKind, label, childProfile, device, trace) {
  return {
    graphId: 'local-ai-graph',
    nodeId,
    nodeKind,
    label,
    childProfile,
    device,
    trace,
  };
}

function runCommand(command) {
  execFileSync('cmd', ['/c', command], { stdio: 'inherit' });
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}
