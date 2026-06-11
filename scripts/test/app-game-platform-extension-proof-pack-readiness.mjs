import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const testResultDir = join(repoRoot, 'test-results', 'app-game-platform-extension-proof-pack-readiness');
const appGameOutputDir = join(repoRoot, 'output', 'app-game-plan-proof', '73-platform-extension-proof-pack-readiness');
const appOutputDir = join(repoRoot, 'output', 'app-plan-proof', '73-platform-extension-proof-pack-readiness');
const proofPath = join(testResultDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(testResultDir, { recursive: true });
  await mkdir(appGameOutputDir, { recursive: true });
  await mkdir(appOutputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'tests/app-game-platform-extension-proof-pack-readiness.test.ts',
    ])
  );

  const proofModule = await loadProofModule();
  const readModel = proofModule.AppGamePlatformExtensionProofPackReadinessReadModel;
  const summary = proofModule.summarizeAppGamePlatformExtensionProofPackReadiness(readModel);

  assert.deepEqual(summary, {
    rows: 4,
    platforms: 4,
    nativeAppRows: 4,
    nativeGameRows: 4,
    manualRequiredRows: 4,
    adapterExecutedRows: 0,
    broadBlockingClaimedRows: 0,
    privilegedMobileClaimedRows: 0,
  });
  assert.deepEqual(readModel.rows.map((row) => row.platform).sort(), ['android', 'ios', 'linux', 'macos']);
  assert.equal(readModel.nonClaims.includes('no-live-platform-adapter'), true);
  assert.equal(readModel.nonClaims.includes('no-adapter-dispatch'), true);
  assert.equal(readModel.nonClaims.includes('no-broad-installed-app-blocking'), true);
  assert.equal(readModel.nonClaims.includes('no-mobile-privileged-control'), true);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'app-game-platform-extension-proof-pack-readiness',
    commands,
    packageExportState:
      'deferred: packages/parent-domain/package.json was locked by another lane during this slice; proof imports built dist module by path.',
    evidence: {
      contract: 'packages/parent-domain/src/app-game-platform-extension-proof-pack-readiness.ts',
      test: 'packages/parent-domain/tests/app-game-platform-extension-proof-pack-readiness.test.ts',
      featureDoc: 'docs/features/app-game-control.md',
      appGameChecklist: 'docs/plans/app-game-plan/implementation-checklist.md',
      appChecklist: 'docs/plans/app-plan/implementation-checklist.md',
      workpack: 'docs/plans/app-game-plan/workpacks/73-platform-extension-proof-pack-readiness.md',
      output: relative(repoRoot, proofPath),
    },
    summary,
    rows: readModel.rows.map((row) => ({
      platform: row.platform,
      productMeanings: row.productMeanings,
      checklistRowIds: row.checklistRowIds,
      authorityTier: row.authorityTier,
      setupState: row.setupState,
      proofPackState: row.proofPackState,
      requiredProofRefs: row.requiredProofRefs,
      adapterExecutionClaim: row.adapterExecutionClaim,
      broadBlockingClaimed: row.broadBlockingClaimed,
      privilegedMobileClaimed: row.privilegedMobileClaimed,
      storeOrMdmProviderExecutionClaimed: row.storeOrMdmProviderExecutionClaimed,
      childDeviceDeliveryClaimed: row.childDeviceDeliveryClaimed,
      claimBoundary: row.claimBoundary,
    })),
    nonClaims: readModel.nonClaims,
    knownGaps: readModel.knownGaps,
  };

  await writeJson(proofPath, proof);
  await writeProofOutputs(appGameOutputDir, proof);
  await writeProofOutputs(appOutputDir, proof);
  console.log(`app-game-platform-extension-proof-pack-readiness-ok:${relative(repoRoot, proofPath)}`);
}

async function loadProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-game-platform-extension-proof-pack-readiness.js'
  );
  return import(pathToFileURL(modulePath).href);
}

async function writeProofOutputs(outputDir, proof) {
  await writeFile(
    join(outputDir, 'README.md'),
    [
      '# WP73 App/Game Platform Extension Proof-Pack Readiness',
      '',
      'This proof keeps native app and native game product meanings separate while mapping non-Windows platform checklist rows to manual proof-pack requirements.',
      '',
      'It does not claim live platform adapters, adapter dispatch, broad installed-app blocking, privileged mobile controls, store/MDM provider execution, or child-device delivery.',
      '',
    ].join('\n')
  );
  await writeFile(join(outputDir, '01-contract-proof.log'), commands.join('\n'));
  await writeFile(join(outputDir, '03-runtime-evidence.json'), `${JSON.stringify(proof.rows, null, 2)}\n`);
  await writeFile(join(outputDir, '05-policy-action-proof.json'), `${JSON.stringify(proof.summary, null, 2)}\n`);
  await writeFile(join(outputDir, '08-security-negative-proof.log'), proof.nonClaims.join('\n'));
  await writeFile(join(outputDir, '10-validation-commands.log'), commands.join('\n'));
  await writeFile(join(outputDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function gitHead() {
  const output = await commandOutput('git', ['rev-parse', 'HEAD']);
  return output.trim();
}

async function commandOutput(command, args) {
  const chunks = [];
  const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
  child.stdout.on('data', (chunk) => chunks.push(chunk));
  child.stderr.on('data', (chunk) => chunks.push(chunk));
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  const output = Buffer.concat(chunks).toString('utf8');
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}\n${output}`);
  }
  return output;
}

async function runCommand(command, args) {
  commands.push(`${command} ${args.join(' ')}`);
  await commandOutput(command, args);
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
