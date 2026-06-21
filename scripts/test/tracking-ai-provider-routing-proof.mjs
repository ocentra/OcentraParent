import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const timestamp = '2026-06-06T18:32:00.000Z';
const testOutputDir = join(repoRoot, 'test-results', 'tracking-ai-provider-routing-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '24-ai-provider-routing');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/tracking-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'tracking-ai-provider-routing-proof',
  'tracking-location-policy',
]);

const routing = await importDist('tracking-ai-provider-routing-proof.js');
const rows = routing.buildTrackingAiProviderRoutingProofRows();
const summary = routing.summarizeTrackingAiProviderRoutingProof(rows);

const proof = {
  proofMode: 'tracking-ai-provider-routing-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary,
  nonClaims: {
    remoteAiDefaultClaimed: false,
    assistantPolicyWriteClaimed: false,
    assistantEnforcementClaimed: false,
    childDeviceRuntimeClaimed: false,
    modelExecutionClaimed: false,
    providerDeliveryClaimed: false,
    productionBehaviorClaimed: false,
  },
  proofPaths: {
    source: 'packages/tracking-domain/src/tracking-ai-provider-routing-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-ai-provider-routing-proof.test.ts',
    harness: 'scripts/test/tracking-ai-provider-routing-proof.mjs',
    evidence: 'test-results/tracking-ai-provider-routing-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/24-ai-provider-routing',
  },
  rows,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeJson(join(testOutputDir, 'route-rows.json'), rows);
await writeProofPack(proofDir, proof);

console.log('tracking-ai-provider-routing-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-ai-provider-routing-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'tracking-domain', 'dist', name)).href);
}

function assertProof(proof) {
  if (
    proof.summary.routeModes.join(',') !==
      'child-local,parent-local,family-ai-hub,parent-approved-remote,metadata-only,no-ai' ||
    proof.summary.defaultChildSafetyRouteCount !== 1 ||
    proof.summary.remoteAllowedRouteCount !== 1 ||
    !proof.summary.remoteAllowedRoutesRequireParentApproval ||
    proof.summary.degradedOrUnavailableRouteCount !== 4 ||
    proof.summary.assistantCanWritePolicyDirectly ||
    proof.summary.assistantCanEnforceDirectly ||
    !proof.summary.allRowsHaveEvidenceAndCustody
  ) {
    throw new Error(`Unexpected tracking AI provider routing summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Tracking AI provider routing proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP24 AI Provider Routing Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking-domain AI provider route proof for child-local default, remote approval, degraded/unavailable rows, custody refs, and assistant no-write boundary.',
      '- No model execution, child-device runtime, provider delivery, assistant policy write, enforcement, or production behavior is claimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/tracking-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/tracking-domain -- tracking-ai-provider-routing-proof tracking-location-policy: PASS',
      '- Child-local route is the only default child-safety path.',
      '- Parent-approved remote is the only route with remote data allowed and requires recorded approval.',
      '- Family hub, metadata-only, and no-AI routes preserve degraded/unavailable/disabled states.',
      '- Assistant rows are preview-only and cannot write policy or enforce directly.',
      '- Every route carries evidence and custody refs.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '08-ai-analysis-proof.json'), proof.summary);
  await writeJson(join(path, '18-ai-provider-routing-custody-proof.json'), proof);
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Remote data is rejected unless route mode is parent-approved-remote and approval is recorded.',
      '- Assistant direct policy-write and enforcement upgrades are rejected by literal false fields.',
      '- Child-device runtime, model execution, provider delivery, production behavior, and enforcement are not claimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeJson(join(path, 'proof-summary.json'), {
    schemaVersion: 1,
    checkedAt: proof.generatedAt,
    commit: proof.commit,
    workpackId: '24-ai-provider-routing',
    proofState: 'ai-provider-routing-custody-proof-complete',
    summary: proof.summary,
    commands: proof.commands,
    productClaims: {
      childLocalDefaultSafetyPath: true,
      explicitRemoteParentApprovalRequired: true,
      providerUnavailableDegradedStatePreserved: true,
      assistantOnlyPolicyWritesPrevented: true,
      evidenceAndCustodyCited: true,
      modelExecutionClaimed: false,
      providerDeliveryClaimed: false,
      productionBehaviorClaimed: false,
    },
  });
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
