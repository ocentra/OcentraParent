import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-parent-rule-context-builder-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-parent-rule-context-builder-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand(...npmCommand(['run', 'build:contracts']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/ai-domain',
    '--',
    'local-ai-parent-rule-context-builder-proof',
  ])
);

const proofModule = await import('@ocentra-parent/schema-domain/local-ai-parent-rule-context-builder-proof');

const readyInput = parentRuleContextBuilderInput(['screen-summary:school-video']);
const ungroundedInput = parentRuleContextBuilderInput(['screen-summary:uncited-game']);
const proof = proofModule.buildLocalAiParentRuleContextBuilderProof(readyInput, ungroundedInput, generatedAt);
const rejectionChecks = [
  {
    name: 'raw-evidence-retention-overclaim',
    rejected: !proofModule.LocalAiParentRuleContextBuilderProofRowSchema.safeParse({
      ...proof.readyRow,
      rawEvidenceRetained: true,
    }).success,
  },
  {
    name: 'remote-ai-overclaim',
    rejected: !proofModule.LocalAiParentRuleContextBuilderProofRowSchema.safeParse({
      ...proof.readyRow,
      remoteAiUsed: true,
    }).success,
  },
  {
    name: 'policy-authority-overclaim',
    rejected: !proofModule.LocalAiParentRuleContextBuilderProofRowSchema.safeParse({
      ...proof.readyRow,
      policyAuthorityClaimed: true,
    }).success,
  },
  {
    name: 'enforcement-overclaim',
    rejected: !proofModule.LocalAiParentRuleContextBuilderProofRowSchema.safeParse({
      ...proof.readyRow,
      enforcementClaimed: true,
    }).success,
  },
];

if (!rejectionChecks.every((check) => check.rejected)) {
  throw new Error(`parent-rule context builder rejection checks failed: ${JSON.stringify(rejectionChecks)}`);
}

const proofSummary = {
  status: 'ok',
  proofKind: 'local-ai-parent-rule-context-builder-proof',
  generatedAt,
  output: relativePath(ProofPath),
  rows: [
    proofRow('grounded-parent-rule-selected', proof.readyRow),
    proofRow('ungrounded-parent-rule-omitted', proof.ungroundedRow),
  ],
  validationSummary: proof.validationSummary,
  assertions: {
    groundedParentRuleSelected:
      proof.readyRow.selectedParentRuleRefs.length === 1 &&
      proof.readyRow.selectedTargetEvidenceRefs[0] === 'screen-summary:school-video',
    selectedRuleTargetsSelectedEvidence: proof.readyRow.selectedTargetEvidenceRefs.every((referenceId) =>
      proof.readyRow.selectedEvidenceRefs.includes(referenceId)
    ),
    ungroundedRuleRejected:
      proof.ungroundedRow.ungroundedParentRuleReferenceCount === 1 &&
      proof.ungroundedRow.selectedParentRuleRefs.length === 0,
    ungroundedRuleDegrades: proof.ungroundedRow.degradedReasons.includes('parent-rule-missing'),
    noRawEvidenceRetention: [proof.readyRow, proof.ungroundedRow].every((row) => !row.rawEvidenceRetained),
    noRemoteAiUsed: [proof.readyRow, proof.ungroundedRow].every((row) => !row.remoteAiUsed),
    noModelExecutionClaim: [proof.readyRow, proof.ungroundedRow].every((row) => !row.modelExecutionClaimed),
    noModelQualityClaim: [proof.readyRow, proof.ungroundedRow].every((row) => !row.modelQualityClaimed),
    noPolicyAuthorityClaim: [proof.readyRow, proof.ungroundedRow].every((row) => !row.policyAuthorityClaimed),
    noEnforcementClaim: [proof.readyRow, proof.ungroundedRow].every((row) => !row.enforcementClaimed),
    noPortalUiClaim: [proof.readyRow, proof.ungroundedRow].every((row) => !row.portalUiClaimed),
    malformedOverclaimsRejected: rejectionChecks.every((check) => check.rejected),
  },
  rejectionChecks,
  nonClaims: {
    freshCaptureClaimed: false,
    modelExecuted: false,
    productionModelQualityClaimed: false,
    remoteAiUsed: false,
    rawEvidenceRetained: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    portalUiClaimed: false,
  },
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proofSummary, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build:contracts',
    'cmd /c npm run test --workspace @ocentra-parent/ai-domain -- local-ai-parent-rule-context-builder-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-parent-rule-context-builder-proof-ok:${proofSummary.rows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function proofRow(name, row) {
  return {
    name,
    state: row.state,
    contextId: row.contextId,
    requestId: row.requestId,
    builtAt: row.builtAt,
    selectedParentRuleContextRefs: row.selectedParentRuleContextRefs,
    selectedParentRuleRefs: row.selectedParentRuleRefs,
    selectedTargetEvidenceRefs: row.selectedTargetEvidenceRefs,
    selectedEvidenceRefs: row.selectedEvidenceRefs,
    ungroundedParentRuleReferenceCount: row.ungroundedParentRuleReferenceCount,
    degradedReasons: row.degradedReasons,
    custodyBoundarySummary: row.custodyBoundarySummary,
    validationGateSummary: row.validationGateSummary,
    rawEvidenceRetained: row.rawEvidenceRetained,
    remoteAiUsed: row.remoteAiUsed,
    modelExecutionClaimed: row.modelExecutionClaimed,
    modelQualityClaimed: row.modelQualityClaimed,
    policyAuthorityClaimed: row.policyAuthorityClaimed,
    enforcementClaimed: row.enforcementClaimed,
    portalUiClaimed: row.portalUiClaimed,
  };
}

function parentRuleContextBuilderInput(targetEvidenceRefs) {
  const observedAt = '2026-06-06T09:56:00.000Z';
  const childProfile = { childProfileId: 'child:parent-rule-context', displayName: 'Maya' };
  const device = {
    deviceId: 'device:parent-rule-context-windows',
    childProfileId: childProfile.childProfileId,
    label: 'Maya Windows laptop',
    platform: 'windows',
  };
  const sourceEvidence = {
    evidenceReferenceId: 'journal:screen-summary-parent-rule',
    kind: 'journal-event',
    observedAt,
  };

  return {
    contextId: 'context:parent-rule-builder',
    request: {
      schemaVersion: 'v0.6',
      requestId: 'request:parent-rule-builder',
      requestedAt: observedAt,
      childProfile,
      device,
      requestedEvaluationKind: 'screen-summary',
      requiredEvidenceKinds: ['screen-summary'],
      parentRuleContextReferences: [
        {
          parentRuleRefId: 'parent-rule-context:screen-school',
          policyVersion: 'policy:screen-v1',
          family: { familyId: 'family:parent-rule-context' },
          childProfile,
          device,
          rule: {
            ruleId: 'rule:screen-school',
            target: { targetId: 'target:school', targetType: 'category', targetValue: 'school' },
            action: 'allow',
            scheduleId: null,
            priority: 20,
            reasonCode: 'schoolwork-allowed',
            createdBy: { actorId: 'parent:maya', role: 'parent' },
            enabled: true,
            effectiveFrom: null,
            effectiveUntil: null,
          },
          targetEvidenceRefs,
          custody: 'parent-device-cache',
          updatedAt: observedAt,
          expiresAt: null,
        },
      ],
      modelTaskRequirements: [],
      allowedCustody: ['child-device-query-store'],
      promptVersion: 'prompt:parent-rule-context-v1',
    },
    evidenceReferences: [
      {
        evidenceRefId: 'screen-summary:school-video',
        evidence: {
          evidenceReferenceId: 'query-store:screen-summary-school-video',
          kind: 'query-store-summary',
          observedAt,
        },
        evidenceKind: 'screen-summary',
        sourceSchemaVersion: 'v0.6',
        observedAt,
        ingestedAt: '2026-06-06T09:56:02.000Z',
        freshUntil: null,
        sourceId: 'source:screen-summary-school-video',
        adapterId: 'adapter:winrt-ocr',
        device,
        childProfile,
        custody: 'child-device-query-store',
        retentionState: 'deleted-source',
        confidence: 0.91,
        confidenceKind: 'classifier',
        capabilityStatus: 'available',
        degradedReasons: ['screen-image-deleted'],
        unknownReasons: [],
        sourceEvidenceReferences: [sourceEvidence],
      },
    ],
    runtimeReferences: [],
    memoryReferences: [],
    graphReferences: [],
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
