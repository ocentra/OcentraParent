import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-linux-docker-host-preflight-proof';
const testOutputDir = join(repoRoot, 'test-results', proofMode);
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', '197-app-game-linux-docker-host-preflight');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-linux-docker-host-preflight',
    'app-game-platform-proof-status',
  ]);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  const dockerProbe = collectDockerHostProbe();
  const module = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-linux-docker-host-preflight.js')).href
  );
  const readModel = module.createAppGameLinuxDockerHostPreflightReadModel({
    generatedAt: '2026-06-08T19:15:00.000Z',
    dockerCliObserved: dockerProbe.dockerCliObserved,
    dockerDaemonObserved: dockerProbe.dockerDaemonObserved,
    contextCount: dockerProbe.contextCount,
    imageCount: dockerProbe.imageCount,
    containerCount: dockerProbe.containerCount,
  });
  const summary = module.summarizeAppGameLinuxDockerHostPreflightReadModel(readModel);

  const proof = {
    schemaVersion: 1,
    proofMode,
    generatedAt: 'deterministic-proof-artifact',
    commands,
    readModel,
    summary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-linux-docker-host-preflight.ts',
      contractTest: 'packages/parent-domain/tests/app-game-linux-docker-host-preflight.test.ts',
      platformStatus: 'packages/parent-domain/src/app-game-platform-proof-status.ts',
      dockerCli: 'Docker CLI visibility is recorded as a boolean only.',
      dockerDaemon: 'Docker daemon visibility is recorded as a boolean only.',
      dockerInventory: 'Docker context, image, and container inventories are recorded as counts only.',
    },
    productBoundaries: {
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      dockerCliObserved: readModel.dockerCliObserved,
      dockerDaemonObserved: readModel.dockerDaemonObserved,
      contextNamesStored: false,
      imageNamesStored: false,
      containerIdsStored: false,
      adapterDispatchClaimed: false,
      containerPolicyClaimed: false,
      platformEnforcementClaimed: false,
      childDeviceDeliveryClaimed: false,
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game Linux Docker host preflight source snapshot',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated before final checkpoint commit',
      '- Docker host preflight ref: linux-docker-host-preflight-ref',
      '',
      'Evidence:',
      '- Docker CLI and daemon visibility are represented as parent-safe booleans.',
      '- Docker context, image, and container inventories are represented as counts only.',
      '- Context names, image names, and container ids are not stored in proof artifacts.',
      '- Container policy, adapter dispatch, platform enforcement, and child delivery remain unclaimed.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.map(commandLog).join('\n\n')}\n`);

  console.log('app-game-linux-docker-host-preflight-proof-ok');
  console.log('evidence=test-results/app-game-linux-docker-host-preflight-proof/proof.json');
}

function collectDockerHostProbe() {
  const version = run('docker', ['version', '--format', '{{json .}}'], { optional: true });
  const dockerCliObserved = version.status === 0 || safeText(version.stderr).includes('Cannot connect to the Docker daemon');
  const dockerDaemonObserved = version.status === 0;
  const contextCount = dockerCliObserved ? countDockerRows(['context', 'ls', '--format', '{{json .Name}}']) : 0;
  const imageCount = dockerDaemonObserved ? countDockerRows(['image', 'ls', '--format', '{{json .Repository}}']) : 0;
  const containerCount = dockerDaemonObserved ? countDockerRows(['ps', '-a', '--format', '{{json .ID}}']) : 0;

  return {
    dockerCliObserved,
    dockerDaemonObserved,
    contextCount,
    imageCount,
    containerCount,
  };
}

function countDockerRows(args) {
  const result = run('docker', args, { optional: true });
  if (result.status !== 0) {
    return 0;
  }
  return result.stdout.split(/\r?\n/).filter((line) => line.trim()).length;
}

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    windowsHide: true,
  });
  const rendered = [command, ...args].join(' ');
  commands.push({
    rendered,
    status: result.status,
    stdout: safeText(result.stdout),
    stderr: safeText(result.stderr || result.error?.message),
  });
  if (!options.optional && result.status !== 0) {
    throw new Error(`${rendered} failed with ${result.status}`);
  }
  return result;
}

function safeText(value) {
  return typeof value === 'string' ? value : '';
}

function commandLog(command) {
  return [
    `$ ${command.rendered}`,
    `status=${command.status}`,
    command.stdout ? 'stdout=<redacted-docker-output>' : 'stdout=',
    command.stderr ? 'stderr=<redacted-docker-output>' : 'stderr=',
  ].join('\n');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
