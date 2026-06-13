import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-notification-live-parent-surface-proof');
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '68-notification-live-parent-surface-read-model'
);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '68-notification-live-parent-surface-read-model');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/text-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/portal-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/portal-domain',
      '--',
      '--run',
      'tests/app-game-notification-parent-surface-panel.test.ts',
    ])
  );
  await runCommand(
    ...npmCommand([
      'exec',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'vitest',
      'run',
      'tests/app-game-notification-parent-surface-panel.test.ts',
    ])
  );

  const liveAssertions = await collectLiveAssertions();
  assertLiveAssertions(liveAssertions);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-notification-live-parent-surface-read-model',
    checkedAt: new Date().toISOString(),
    implementationCommit: await gitHead(),
    commands,
    liveAssertions,
    evidence: {
      liveProjection: 'packages/portal-domain/src/app-game-notification-parent-surface-live-readiness.ts',
      overviewCommands: 'packages/portal-domain/src/commands.ts',
      liveActivityState: 'packages/portal-domain/src/live-activity-state.ts',
      commandResultEvents: 'packages/portal-domain/src/command-results.ts',
      portalDomainTest: 'packages/portal-domain/tests/app-game-notification-parent-surface-panel.test.ts',
      portalRouteTest: 'apps/portal/tests/app-game-notification-parent-surface-panel.test.ts',
      proofHarness: 'scripts/test/app-game-notification-live-parent-surface-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/68-notification-live-parent-surface-read-model',
      appProofPack: 'output/app-plan-proof/68-notification-live-parent-surface-read-model',
    },
    claimsProved: [
      'Portal overview commands request the existing app/game notification readiness service read model',
      'Portal live state parses the service readiness event before deriving parent-surface rows',
      'Portal-domain projection maps validated readiness rows into schema-backed manual/unavailable parent-surface setup rows',
      'Rendered parent-surface rows keep scheduler/outbox runtime refs unreported unless proved by a later service row',
    ],
    claimsNotProved: [
      'provider delivery, receipt ingestion, credentials, webhook handling, cloud routing, or production retry workers',
      'parent preference mutation, frequency controls, quiet-hours editor behavior, or parent notification delivery UI',
      'scheduler runtime, local outbox runtime, durable production outbox/history storage, or adapter dispatch',
      'child-device delivery, mobile UI, policy evaluator execution, broad blocking, or platform support',
    ],
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP68');
  await writeProofPack(appProofDir, proof, 'app WP68');

  console.log(`app-game-notification-live-parent-surface-proof-ok:${liveAssertions.liveStateAssertion}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function assertLiveAssertions(liveAssertions) {
  for (const [key, value] of Object.entries(liveAssertions)) {
    if (value !== true) {
      throw new Error(`App/game notification live parent-surface proof assertion failed: ${key}`);
    }
  }
}

async function collectLiveAssertions() {
  const commandsSource = await readFile(join(repoRoot, 'packages', 'portal-domain', 'src', 'commands.ts'), 'utf8');
  const projectionSource = await readFile(
    join(repoRoot, 'packages', 'portal-domain', 'src', 'app-game-notification-parent-surface-live-readiness.ts'),
    'utf8'
  );
  const liveStateSource = await readFile(
    join(repoRoot, 'packages', 'portal-domain', 'src', 'live-activity-state.ts'),
    'utf8'
  );
  const eventResultsSource = await readFile(
    join(repoRoot, 'packages', 'portal-domain', 'src', 'command-results.ts'),
    'utf8'
  );
  const portalDomainTestSource = await readFile(
    join(repoRoot, 'packages', 'portal-domain', 'tests', 'app-game-notification-parent-surface-panel.test.ts'),
    'utf8'
  );
  const portalTestSource = await readFile(
    join(repoRoot, 'apps', 'portal', 'tests', 'app-game-notification-parent-surface-panel.test.ts'),
    'utf8'
  );

  return {
    overviewCommandAssertion: commandsSource.includes('ActivityAppGameNotificationReadinessReadModelGet'),
    eventResultAssertion: eventResultsSource.includes('ActivityAppGameNotificationReadinessReadModelReported'),
    liveStateAssertion:
      liveStateSource.includes('parseAgentAppGameNotificationReadinessEvent') &&
      liveStateSource.includes('createAppGameNotificationParentSurfaceReadModelFromReadiness'),
    projectionAssertion:
      projectionSource.includes('AgentAppGameNotificationReadinessReadModelSchema') &&
      projectionSource.includes('AppGameNotificationParentSurfaceIntentReadModelSchema') &&
      projectionSource.includes('sourceSchedulerEntryRef: null') &&
      projectionSource.includes('sourceOutboxRecordRef: null'),
    portalDomainTestAssertion: portalDomainTestSource.includes(
      'projects live service readiness rows into parent-surface setup rows without runtime claims'
    ),
    portalRouteTestAssertion: portalTestSource.includes(
      'derives parent-surface rows from the live notification readiness service event'
    ),
  };
}

async function runCommand(command, args) {
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: repoRoot,
      shell: false,
      stdio: 'inherit',
      windowsHide: true,
    });
    child.on('error', reject);
    child.on('exit', (code) => {
      const commandLine = [command, ...args].join(' ');
      commands.push({
        command: commandLine,
        exitCode: code,
      });
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`${commandLine} exited ${code}`));
      }
    });
  });
}

async function gitHead() {
  return new Promise((resolve) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], {
      cwd: repoRoot,
      shell: false,
      stdio: ['ignore', 'pipe', 'ignore'],
      windowsHide: true,
    });
    let output = '';
    child.stdout.on('data', (chunk) => {
      output += chunk.toString();
    });
    child.on('exit', () => {
      resolve(output.trim());
    });
  });
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

async function writeProofPack(outputDir, proof, label) {
  await writeFile(
    join(outputDir, 'README.md'),
    [
      `# ${label} Notification Live Parent Surface Read Model`,
      '',
      `Checked at: ${proof.checkedAt}`,
      `Implementation commit: ${proof.implementationCommit}`,
      '',
      '## Claims Proved',
      ...proof.claimsProved.map((claim) => `- ${claim}`),
      '',
      '## Claims Not Proved',
      ...proof.claimsNotProved.map((claim) => `- ${claim}`),
      '',
      '## Evidence',
      ...Object.entries(proof.evidence).map(([key, value]) => `- ${key}: ${value}`),
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(outputDir, 'proof.json'), proof);
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
