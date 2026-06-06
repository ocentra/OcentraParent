import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '77-source-gated-policy-preview-export-readiness';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-source-gated-policy-preview-export-readiness-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T01:30:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}
for (const path of [join(appGameProofDir, '06-ui-snapshots'), join(appProofDir, '06-ui-snapshots')]) {
  await mkdir(path, { recursive: true });
}

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-source-gated-policy-preview-export-readiness',
  'app-game-source-gated-policy-preview-read-model',
]);

const readinessContract = await importDist('app-game-source-gated-policy-preview-export-readiness.js');
const wp76Proof = await readJson(
  join(repoRoot, 'test-results', 'app-game-source-gated-policy-preview-read-model-proof', 'proof.json')
);
const packageJson = await readJson(join(repoRoot, 'packages', 'parent-domain', 'package.json'));
const readiness = readinessContract.buildAppGameSourceGatedPolicyPreviewExportReadiness(
  readinessOptions(await importDist('reference-primitives.js')),
  wp76Proof.readModel
);
const packageManifestHasDeferredExport = Object.hasOwn(packageJson.exports ?? {}, readiness.requiredExportSubpath);
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-export-readiness',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp76Branch: 'codex/app-game-source-gated-policy-preview-read-model',
    wp75Branch: 'codex/app-game-source-freshness-preview-gate',
    reason:
      'WP77 consumes WP76 source-gated policy preview read-model contracts. Package manifest export is intentionally deferred while another lane owns packages/parent-domain/package.json.',
  },
  manifestCheck: {
    packagePath: 'packages/parent-domain/package.json',
    requiredExportSubpath: readiness.requiredExportSubpath,
    packageManifestHasDeferredExport,
    packageManifestUpdatedByThisProof: readiness.packageManifestUpdated,
  },
  summary: summarize(readiness),
  nonClaims: {
    packageManifestUpdated: readiness.packageManifestUpdated,
    serviceRuntimeEventClaimed: readiness.serviceRuntimeEventClaimed,
    portalUiRendered: readiness.portalUiRendered,
    policyEvaluatorRuntimeClaimed: readiness.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: readiness.timerRuntimeClaimed,
    adapterDispatchClaimed: readiness.adapterDispatchClaimed,
    childDeliveryClaimed: readiness.childDeliveryClaimed,
    platformEnforcementClaimed: readiness.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: readiness.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-source-gated-policy-preview-export-readiness.ts',
    rules: 'packages/parent-domain/src/app-game-source-gated-policy-preview-export-readiness-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-export-readiness.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-export-readiness-proof.mjs',
    evidence: 'test-results/app-game-source-gated-policy-preview-export-readiness-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  readiness,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'export-readiness.json'), readiness);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP77');
await writeProofPack(appProofDir, proof, 'app WP77');

console.log('app-game-source-gated-policy-preview-export-readiness-proof-ok');
console.log(
  `evidence=${join('test-results', 'app-game-source-gated-policy-preview-export-readiness-proof', 'proof.json')}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function readinessOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    readinessId: 'app-game-source-gated-policy-preview-export-readiness-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-read-model',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/policy.md',
    ],
  };
}

function summarize(readiness) {
  return {
    requiredExportSubpath: readiness.requiredExportSubpath,
    requiredExportSymbolCount: readiness.requiredExportSymbols.length,
    readinessState: readiness.readinessState,
    manifestState: readiness.manifestState,
    nativeAppRowCount: readiness.nativeAppRowCount,
    nativeGameRowCount: readiness.nativeGameRowCount,
    previewReadyVisibleCount: readiness.previewReadyVisibleCount,
    sourceManualRequiredVisibleCount: readiness.sourceManualRequiredVisibleCount,
    compilerManualRequiredVisibleCount: readiness.compilerManualRequiredVisibleCount,
  };
}

function assertProof(proof) {
  const requiredSymbols = new Set([
    'AppGameSourceGatedPolicyPreviewReadModelSchema',
    'AppGameSourceGatedPolicyPreviewReadModelRowSchema',
    'buildAppGameSourceGatedPolicyPreviewReadModel',
    'decodeAppGameSourceGatedPolicyPreviewReadModel',
    'AppGameSourceGatedPolicyPreviewReadModelProjectionState',
    'AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary',
  ]);
  if (proof.readiness.requiredExportSubpath !== './app-game-source-gated-policy-preview-read-model') {
    throw new Error('Expected readiness to name the WP76 package export subpath');
  }
  for (const symbol of requiredSymbols) {
    if (!proof.readiness.requiredExportSymbols.includes(symbol)) {
      throw new Error(`Missing required export symbol ${symbol}`);
    }
  }
  if (proof.manifestCheck.packageManifestHasDeferredExport) {
    throw new Error('Expected package manifest export to remain deferred in WP77');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error('Expected WP77 proof to avoid package, runtime, UI, adapter, and raw-source claims');
  }
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function writeProofPack(dir, proof, label) {
  await writeFile(
    join(dir, '00-source-snapshot.md'),
    [
      `# ${label} source snapshot`,
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort.length === 0 ? 'clean before proof generation' : proof.gitStatusShort}`,
      '',
    ].join('\n')
  );
  await writeFile(join(dir, '10-validation-commands.log'), `${proof.commands.join('\n\n')}\n`);
  await writeJson(join(dir, 'proof.json'), proof);
}

function run(command, args) {
  const rendered = `${command} ${args.join(' ')}`;
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push(`${rendered}\nexit=${result.status}\n${result.stdout}${result.stderr}`);
  if (result.status !== 0) {
    throw new Error(`${rendered} failed with exit ${result.status}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}
