import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '80-source-gated-policy-preview-package-exports';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-source-gated-policy-preview-package-exports-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T03:00:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);
const exportPaths = [
  './app-game-source-gated-policy-preview-read-model',
  './app-game-source-gated-policy-preview-timer-handoff',
  './app-game-source-gated-policy-preview-timer-status',
];

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
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
  'app-game-source-gated-policy-preview-package-exports',
]);

const packageJson = JSON.parse(await readFile(join(repoRoot, 'packages', 'parent-domain', 'package.json'), 'utf8'));
const exportsObject = packageJson.exports ?? {};
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-package-exports',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp79Branch: 'codex/app-game-source-gated-policy-preview-timer-status',
    reason:
      'WP80 exposes WP76/WP78/WP79 parent-domain source-gated preview contracts after the package manifest lock cleared.',
  },
  exportPaths,
  packageExports: Object.fromEntries(exportPaths.map((path) => [path, exportsObject[path]])),
  nonClaims: {
    serviceRuntimeEventClaimed: false,
    portalUiRendered: false,
    policyEvaluatorRuntimeClaimed: false,
    timerRuntimeClaimed: false,
    timerScheduled: false,
    adapterDispatchClaimed: false,
    childDeliveryClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  },
  proofPaths: {
    packageJson: 'packages/parent-domain/package.json',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-package-exports.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-package-exports-proof.mjs',
    evidence: 'test-results/app-game-source-gated-policy-preview-package-exports-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
};

assertProof(proof);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP80');
await writeProofPack(appProofDir, proof, 'app WP80');

console.log('app-game-source-gated-policy-preview-package-exports-proof-ok');
console.log(
  `evidence=${join('test-results', 'app-game-source-gated-policy-preview-package-exports-proof', 'proof.json')}`
);

function assertProof(proof) {
  for (const exportPath of exportPaths) {
    const moduleName = exportPath.slice(2);
    const entry = proof.packageExports[exportPath];
    if (entry?.import !== `./dist/${moduleName}.js` || entry?.types !== `./dist/${moduleName}.d.ts`) {
      throw new Error(`Expected package export ${exportPath} to point at dist JS and d.ts artifacts`);
    }
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP80 export proof to avoid runtime, UI, timer scheduling, adapter, child, platform, and raw-source claims'
    );
  }
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
  await writeFile(join(dir, '10-validation-commands.log'), `${proof.commands.join('\n\n').trimEnd()}\n`);
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
