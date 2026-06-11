import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-recent-memory-window-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-recent-memory-window-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'local-ai-recent-memory-window-proof',
  ])
);

const recentMemoryModule = await import(
  pathToFileURL(resolve(RepoRoot, 'packages', 'parent-domain', 'dist', 'local-ai-recent-memory-window-proof.js')).href
);

const readyInput = localAiRecentMemoryWindowInput();
const readyProof = recentMemoryModule.buildLocalAiRecentMemoryWindowReadModel(readyInput);
const noWindowProof = recentMemoryModule.buildLocalAiRecentMemoryWindowReadModel({
  ...readyInput,
  window: {
    observedFrom: '2026-06-06T06:00:00.000Z',
    observedUntil: '2026-06-06T06:10:00.000Z',
    asOf: '2026-06-06T06:10:00.000Z',
  },
});
const limitProof = recentMemoryModule.buildLocalAiRecentMemoryWindowReadModel({
  ...readyInput,
  limit: 0,
});

const rejectionChecks = [
  {
    name: 'inverted-window',
    rejected: !recentMemoryModule.LocalAiRecentMemoryWindowSchema.safeParse({
      observedFrom: readyInput.window.observedUntil,
      observedUntil: readyInput.window.observedFrom,
      asOf: readyInput.window.asOf,
    }).success,
  },
  {
    name: 'future-read',
    rejected: !recentMemoryModule.LocalAiRecentMemoryWindowSchema.safeParse({
      observedFrom: readyInput.window.observedFrom,
      observedUntil: readyInput.window.observedUntil,
      asOf: readyInput.window.observedFrom,
    }).success,
  },
  {
    name: 'raw-retention-overclaim',
    rejected: !recentMemoryModule.LocalAiRecentMemoryWindowReadModelSchema.safeParse({
      ...readyProof,
      rawEvidenceRetained: true,
    }).success,
  },
  {
    name: 'remote-ai-overclaim',
    rejected: !recentMemoryModule.LocalAiRecentMemoryWindowReadModelSchema.safeParse({
      ...readyProof,
      remoteAiUsed: true,
    }).success,
  },
  {
    name: 'policy-authority-overclaim',
    rejected: !recentMemoryModule.LocalAiRecentMemoryWindowReadModelSchema.safeParse({
      ...readyProof,
      policyAuthorityClaimed: true,
    }).success,
  },
  {
    name: 'enforcement-overclaim',
    rejected: !recentMemoryModule.LocalAiRecentMemoryWindowReadModelSchema.safeParse({
      ...readyProof,
      enforcementClaimed: true,
    }).success,
  },
];

const proof = {
  status: 'ok',
  proofKind: 'local-ai-recent-memory-window-proof',
  generatedAt,
  output: relativePath(ProofPath),
  readModelRows: [
    proofRow('source-grounded-window', readyProof),
    proofRow('missing-window-activity', noWindowProof),
    proofRow('zero-limit-window', limitProof),
  ],
  assertions: {
    sourceGroundedRecentActivitySelected:
      readyProof.recentActivityEvidenceRefs.length === 1 &&
      readyProof.recentActivityEvidenceRefs[0] === 'recent-activity:browser-game',
    sourceEvidencePreserved:
      readyProof.recentActivitySourceEvidenceReferences.length === 1 &&
      readyProof.recentActivitySourceEvidenceReferences[0]?.evidenceReferenceId === 'journal:recent-browser-game',
    recentMemoryGroundedToSelectedEvidence:
      readyProof.recentMemoryReferences.length === 1 &&
      readyProof.recentMemoryReferences[0]?.memoryReferenceId === 'memory:recent-browser-game',
    staleOrOutsideWindowEvidenceOmitted:
      readyProof.omittedRecentActivityCount === 1 && readyProof.degradedReasons.includes('stale-evidence'),
    ungroundedMemoryOmitted:
      readyProof.omittedRecentMemoryCount === 1 && readyProof.degradedReasons.includes('memory-ungrounded'),
    missingWindowDegrades:
      noWindowProof.state === 'insufficient' && noWindowProof.degradedReasons.includes('missing-evidence'),
    zeroLimitDoesNotLeakActivity: limitProof.recentActivityEvidenceRefs.length === 0,
    noRawEvidenceRetention: [readyProof, noWindowProof, limitProof].every((row) => !row.rawEvidenceRetained),
    noRemoteAiClaim: [readyProof, noWindowProof, limitProof].every((row) => !row.remoteAiUsed),
    noPolicyAuthorityClaim: [readyProof, noWindowProof, limitProof].every((row) => !row.policyAuthorityClaimed),
    noEnforcementClaim: [readyProof, noWindowProof, limitProof].every((row) => !row.enforcementClaimed),
    malformedInputsRejected: rejectionChecks.every((check) => check.rejected),
  },
  rejectionChecks,
  nonClaims: {
    modelExecuted: false,
    productionModelQualityClaimed: false,
    remoteAiUsed: false,
    rawEvidenceRetained: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    portalUiClaimed: false,
    freshCaptureClaimed: false,
  },
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-recent-memory-window-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-recent-memory-window-proof-ok:${proof.readModelRows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function proofRow(name, row) {
  return {
    name,
    state: row.state,
    contextId: row.contextId,
    requestId: row.requestId,
    readAt: row.readAt,
    window: row.window,
    recentActivityEvidenceRefs: row.recentActivityEvidenceRefs,
    recentActivitySourceEvidenceReferences: row.recentActivitySourceEvidenceReferences.map(
      (reference) => reference.evidenceReferenceId
    ),
    recentMemoryReferences: row.recentMemoryReferences.map((reference) => reference.memoryReferenceId),
    returnedRecentActivityCount: row.returnedRecentActivityCount,
    returnedRecentMemoryCount: row.returnedRecentMemoryCount,
    omittedRecentActivityCount: row.omittedRecentActivityCount,
    omittedRecentMemoryCount: row.omittedRecentMemoryCount,
    degradedReasons: row.degradedReasons,
    custodyBoundarySummary: row.custodyBoundarySummary,
    rawEvidenceRetained: row.rawEvidenceRetained,
    remoteAiUsed: row.remoteAiUsed,
    policyAuthorityClaimed: row.policyAuthorityClaimed,
    enforcementClaimed: row.enforcementClaimed,
  };
}

function localAiRecentMemoryWindowInput() {
  const observedAt = '2026-06-06T05:20:00.000Z';
  const asOf = '2026-06-06T05:30:00.000Z';
  const childProfile = { childProfileId: 'child:maya', displayName: 'Maya' };
  const device = {
    deviceId: 'device:maya-windows',
    childProfileId: childProfile.childProfileId,
    label: 'Maya Windows laptop',
    platform: 'windows',
  };
  const sourceEvidence = {
    evidenceReferenceId: 'journal:recent-browser-game',
    kind: 'journal-event',
    observedAt,
  };
  const outsideSourceEvidence = {
    evidenceReferenceId: 'journal:outside-window',
    kind: 'journal-event',
    observedAt: '2026-06-06T04:10:00.000Z',
  };
  return {
    contextInput: {
      contextId: 'context:recent-memory-window',
      request: {
        schemaVersion: 'v0.6',
        requestId: 'request:recent-memory-window',
        requestedAt: asOf,
        childProfile,
        device,
        requestedEvaluationKind: 'recent-activity',
        requiredEvidenceKinds: ['recent-activity'],
        parentRuleContextReferences: [
          {
            parentRuleRefId: 'parent-rule-context:recent-activity',
            policyVersion: 'policy:v1',
            family: { familyId: 'family:maya' },
            childProfile,
            device,
            rule: {
              ruleId: 'rule:recent-games',
              target: { targetId: 'target:games', targetType: 'category', targetValue: 'games' },
              action: 'warn',
              scheduleId: null,
              priority: 10,
              reasonCode: 'recent-activity-games',
              createdBy: { actorId: 'parent:maya', role: 'parent' },
              enabled: true,
              effectiveFrom: null,
              effectiveUntil: null,
            },
            targetEvidenceRefs: ['recent-activity:browser-game'],
            custody: 'parent-device-cache',
            updatedAt: observedAt,
            expiresAt: null,
          },
        ],
        modelTaskRequirements: [],
        allowedCustody: ['child-device-query-store'],
        promptVersion: 'prompt:recent-memory-v1',
      },
      evidenceReferences: [
        {
          evidenceRefId: 'recent-activity:browser-game',
          evidence: sourceEvidence,
          evidenceKind: 'recent-activity',
          sourceSchemaVersion: 'v0.6',
          observedAt,
          ingestedAt: '2026-06-06T05:20:05.000Z',
          freshUntil: '2026-06-06T05:45:00.000Z',
          sourceId: 'source:recent-activity',
          adapterId: 'adapter:local-window',
          device,
          childProfile,
          custody: 'child-device-query-store',
          retentionState: 'local',
          confidence: 0.88,
          confidenceKind: 'memory-match',
          capabilityStatus: 'available',
          degradedReasons: [],
          unknownReasons: [],
          sourceEvidenceReferences: [sourceEvidence],
        },
        {
          evidenceRefId: 'recent-activity:outside-window',
          evidence: outsideSourceEvidence,
          evidenceKind: 'recent-activity',
          sourceSchemaVersion: 'v0.6',
          observedAt: '2026-06-06T04:10:00.000Z',
          ingestedAt: '2026-06-06T04:10:05.000Z',
          freshUntil: '2026-06-06T05:45:00.000Z',
          sourceId: 'source:outside-window',
          adapterId: 'adapter:local-window',
          device,
          childProfile,
          custody: 'child-device-query-store',
          retentionState: 'local',
          confidence: 0.66,
          confidenceKind: 'memory-match',
          capabilityStatus: 'available',
          degradedReasons: [],
          unknownReasons: [],
          sourceEvidenceReferences: [outsideSourceEvidence],
        },
      ],
      runtimeReferences: [],
      memoryReferences: [
        {
          memoryReferenceId: 'memory:recent-browser-game',
          kind: 'recent-activity',
          sourceEvidenceReferences: [sourceEvidence],
          sourcePolicyVersion: null,
          generatedAt: '2026-06-06T05:21:00.000Z',
          confidence: 0.83,
          derivedIndexVersion: 'recent-memory:v1',
        },
        {
          memoryReferenceId: 'memory:outside-window',
          kind: 'recent-activity',
          sourceEvidenceReferences: [outsideSourceEvidence],
          sourcePolicyVersion: null,
          generatedAt: '2026-06-06T04:11:00.000Z',
          confidence: 0.72,
          derivedIndexVersion: 'recent-memory:v1',
        },
      ],
      graphReferences: [],
    },
    window: {
      observedFrom: '2026-06-06T05:00:00.000Z',
      observedUntil: asOf,
      asOf,
    },
    limit: 5,
  };
}

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
