import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofId = '71-policy-preview-service-read-model';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-policy-preview-service-read-model');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofId);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofId);
const generatedAt = '2026-06-05T18:55:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}
for (const path of [join(appGameProofDir, '06-ui-snapshots'), join(appProofDir, '06-ui-snapshots')]) {
  await mkdir(path, { recursive: true });
}

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/activity-domain']);
run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain']);
run('cmd', [
  '/c',
  'npm',
  'exec',
  '--workspace',
  '@ocentra-parent/agent-protocol-domain',
  '--',
  'vitest',
  'run',
  'tests/app-game-policy-preview-read-model.test.ts',
  'tests/contracts.test.ts',
]);
run('cargo', [
  'test',
  '-p',
  'ocentra-parent-agent-service',
  'policy_preview_payload_exposes_latest_dry_run_decision_without_enforcement',
]);

const parser = await import('@ocentra-parent/agent-protocol-domain/app-game-policy-preview-read-model');
commands.push('node import @ocentra-parent/agent-protocol-domain/app-game-policy-preview-read-model');
if (typeof parser.parseAgentAppGamePolicyPreviewEvent !== 'function') {
  throw new Error('Expected package export to expose parseAgentAppGamePolicyPreviewEvent');
}

const readModel = parseProofReadModel(parser);
const proof = {
  proofMode: 'app-game-policy-preview-service-read-model',
  generatedAt,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: {
    rows: readModel.rows.length,
    nativeAppPreviewRowCount: readModel.nativeAppPreviewRowCount,
    nativeGamePreviewRowCount: readModel.nativeGamePreviewRowCount,
    notAppGameRowCount: readModel.notAppGameRowCount,
    nativeGamePromotionClaimed: readModel.nativeGamePromotionClaimed,
    nativeGameUnavailableReason: readModel.nativeGameUnavailableReason,
  },
  nonClaims: {
    policyEvaluatorRuntimeClaimed: readModel.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: readModel.timerRuntimeClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    childDeliveryClaimed: readModel.childDeliveryClaimed,
    platformEnforcementClaimed: readModel.platformEnforcementClaimed,
  },
  proofPaths: {
    servicePayload: 'crates/agent-service/src/policy_preview_payload.rs',
    servicePayloadTest: 'crates/agent-service/src/policy_preview_tests.rs',
    protocolAdapter: 'packages/agent-protocol-domain/src/app-game-policy-preview-read-model.ts',
    protocolAdapterTest: 'packages/agent-protocol-domain/tests/app-game-policy-preview-read-model.test.ts',
    packageExport: 'packages/agent-protocol-domain/package.json',
    harness: 'scripts/test/app-game-policy-preview-service-read-model-proof.mjs',
    evidence: 'test-results/app-game-policy-preview-service-read-model/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofId}`,
    appProofPack: `output/app-plan-proof/${proofId}`,
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'policy-preview-service-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP71');
await writeProofPack(appProofDir, proof, 'app WP71');

console.log('app-game-policy-preview-service-read-model-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-policy-preview-service-read-model', 'proof.json')}`);

function parseProofReadModel(parser) {
  const result = parser.parseAgentAppGamePolicyPreviewEvent({
    schemaVersion: 1,
    eventId: 'policy-preview-service-proof-event',
    correlationId: 'policy-preview-service-proof-command',
    sentAt: generatedAt,
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.policy.preview.read-model.reported',
    severity: 'info',
    payload: {
      payload: JSON.stringify(policyPreviewServiceReadModel()),
    },
    snapshot: null,
  });
  if (!result.ok) {
    throw new Error(`Expected proof event to parse: ${result.reason}`);
  }
  return result.value;
}

function policyPreviewServiceReadModel() {
  return {
    schemaVersion: 'policy-dry-run-v0.6',
    generatedAt,
    custody: 'activity-store',
    limit: 5,
    returned: 2,
    capabilityStatus: 'ready',
    rows: [
      {
        previewId: 'policy-preview-native-app-proof',
        sourceEventId: 'activity-event-native-app-proof',
        observedAt: generatedAt,
        target: {
          targetId: 'target-native-app-proof',
          targetType: 'app',
          targetValue: 'opaque-app-ref-proof',
        },
        evidenceReferences: [{ evidenceReferenceId: 'evidence-native-app-proof' }],
        parentRuleContextReferences: [{ parentRuleRefId: 'parent-rule-context-native-app-proof' }],
        decision: {
          schemaVersion: 'policy-dry-run-v0.6',
          decisionId: 'policy-decision-native-app-proof',
          action: 'time-limit',
          reasonCodes: ['parent-rule-time-limit'],
          evidenceReferences: [],
          ruleIds: ['parent-rule-native-app-proof'],
          localAiResultId: null,
          dryRun: true,
          enforcementHandoffState: 'disabled',
          expiresAt: null,
        },
      },
      {
        previewId: 'policy-preview-domain-proof',
        sourceEventId: 'activity-event-domain-proof',
        observedAt: generatedAt,
        target: {
          targetId: 'target-domain-proof',
          targetType: 'domain',
          targetValue: 'example.invalid',
        },
        evidenceReferences: [],
        parentRuleContextReferences: [],
        decision: {
          schemaVersion: 'policy-dry-run-v0.6',
          decisionId: 'policy-decision-domain-proof',
          action: 'unknown',
          reasonCodes: ['no-matching-parent-rule'],
          evidenceReferences: [],
          ruleIds: [],
          localAiResultId: null,
          dryRun: true,
          enforcementHandoffState: 'disabled',
          expiresAt: null,
        },
      },
    ],
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 2 ||
    proof.summary.nativeAppPreviewRowCount !== 1 ||
    proof.summary.nativeGamePreviewRowCount !== 0 ||
    proof.summary.notAppGameRowCount !== 1 ||
    proof.summary.nativeGamePromotionClaimed !== false ||
    proof.summary.nativeGameUnavailableReason !== 'source-target-kind-not-persisted'
  ) {
    throw new Error(`Unexpected WP71 service preview summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Policy preview service bridge overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(proofDir, proof, label) {
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: bridge the existing service policy preview event into a typed app/game policy preview read model.',
      '- Source inspected: generic service policy preview payload, app/game WP70 handoff contract, agent-protocol-domain parsers, feature docs, and implementation checklists.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain: PASS',
      '- cmd /c npm exec --workspace @ocentra-parent/agent-protocol-domain -- vitest run tests/app-game-policy-preview-read-model.test.ts tests/contracts.test.ts: PASS',
      '- cargo test -p ocentra-parent-agent-service policy_preview_payload_exposes_latest_dry_run_decision_without_enforcement: PASS',
      '- node import @ocentra-parent/agent-protocol-domain/app-game-policy-preview-read-model: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust protocol proof: existing agent.policy.preview.read-model.reported payload now includes the full serialized read model under the existing payload field; no command/event constants were added.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof.summary);
  await writeJson(join(proofDir, '04-journal-sqlite-proof.json'), {
    schemaVersion: 1,
    journalSqliteChanged: false,
    servicePayloadField: 'payload',
    readModelArtifact: 'test-results/app-game-policy-preview-service-read-model/policy-preview-service-read-model.json',
  });
  await writeJson(join(proofDir, '05-policy-action-proof.json'), {
    schemaVersion: 1,
    nativeAppPreviewRowCount: proof.summary.nativeAppPreviewRowCount,
    nativeGamePreviewRowCount: proof.summary.nativeGamePreviewRowCount,
    nativeGamePromotionClaimed: false,
    policyEvaluatorRuntimeClaimed: false,
    timerRuntimeClaimed: false,
    adapterDispatchClaimed: false,
    childDeliveryClaimed: false,
    platformEnforcementClaimed: false,
  });
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo parent portal or child-facing UI source changed in this service/read-model slice.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '07-playwright-ui-proof.log'),
    'Playwright proof not applicable: no UI source changed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Parser rejects non-dry-run service rows.',
      '- Parser rejects rows whose enforcement handoff state is not disabled.',
      '- Native-game promotion remains false because the generic service event does not yet persist WP70 sourceTargetKind.',
      '- Policy evaluator runtime, timers, adapter dispatch, child delivery, and platform enforcement remain false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nNo platform authority tier is raised. This service/read-model bridge does not execute policy, timers, provider sends, child delivery, or adapters.\n',
    'utf8'
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nThe bridge exposes existing service preview rows only. It does not create or upgrade platform authority, capability, or enforcement proof.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '12-rollback-proof.md'),
    '# Rollback Proof\n\nNo runtime state is created. Rollback is limited to removing the serialized payload field addition, parser/export, and generated proof artifacts.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, 'README.md'),
    `# ${label} Policy Preview Service Read Model Proof\n\nThis proof pack records the existing policy preview service event carrying a full serialized read model and a typed app/game parser that preserves no-execution claims and does not promote native-game rows without persisted source target kind.\n`,
    'utf8'
  );
  await writeJson(join(proofDir, 'proof.json'), proof);
}

function run(command, args) {
  commands.push([command, ...args].join(' '));
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
