import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-notification-parent-surface-ui-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '67-notification-parent-surface-renderer');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '67-notification-parent-surface-renderer');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/text-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/portal-domain']));
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

  const rendererAssertions = await collectRendererAssertions();
  assertRendererAssertions(rendererAssertions);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-notification-parent-surface-ui',
    checkedAt: new Date().toISOString(),
    implementationCommit: await gitHead(),
    commands,
    rendererAssertions,
    evidence: {
      routePanel: 'apps/portal/src/AppGameNotificationParentSurfaceRoutePanel.tsx',
      panelIntentReexport: 'apps/portal/src/app-game-notification-parent-surface-panel.ts',
      portalDomainPanelIntent: 'packages/portal-domain/src/app-game-notification-parent-surface-panel.ts',
      portalTest: 'apps/portal/tests/app-game-notification-parent-surface-panel.test.ts',
      proofHarness: 'scripts/test/app-game-notification-parent-surface-ui-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/67-notification-parent-surface-renderer',
      appProofPack: 'output/app-plan-proof/67-notification-parent-surface-renderer',
    },
    claimsProved: [
      'Portal App/Game Sessions route renders schema-backed app/game notification parent-surface intent rows',
      'Missing or invalid service input renders an explicit empty state instead of invented notification rows',
      'Rendered rows expose status, drill-in refs, scheduler/outbox refs, quiet-hours status, manual proof requirements, and no-runtime claims',
      'Parent-surface renderer is gated to the App/Game Sessions route',
    ],
    claimsNotProved: [
      'live service event ingestion for parent-surface intent rows',
      'parent preference mutation, frequency controls, or quiet-hours editor behavior',
      'provider delivery, receipt ingestion, credentials, webhook handling, cloud routing, or production retry workers',
      'child-device delivery, mobile UI, policy evaluator execution, adapter dispatch, broad blocking, or platform support',
    ],
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP67');
  await writeProofPack(appProofDir, proof, 'app WP67');

  console.log(`app-game-notification-parent-surface-ui-proof-ok:${rendererAssertions.routeGateAssertion}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function assertRendererAssertions(rendererAssertions) {
  for (const [key, value] of Object.entries(rendererAssertions)) {
    if (value !== true) {
      throw new Error(`App/game notification parent-surface UI proof assertion failed: ${key}`);
    }
  }
}

async function collectRendererAssertions() {
  const routePanelSource = await readFile(
    join(repoRoot, 'apps', 'portal', 'src', 'AppGameNotificationParentSurfaceRoutePanel.tsx'),
    'utf8'
  );
  const panelIntentSource = await readFile(
    join(repoRoot, 'packages', 'portal-domain', 'src', 'app-game-notification-parent-surface-panel.ts'),
    'utf8'
  );
  const portalTestSource = await readFile(
    join(repoRoot, 'apps', 'portal', 'tests', 'app-game-notification-parent-surface-panel.test.ts'),
    'utf8'
  );

  return {
    routeGateAssertion:
      routePanelSource.includes('PortalRoute.AppGameSessions') &&
      portalTestSource.includes('mounts only on the App/Game Sessions route'),
    schemaBackedRowsAssertion:
      panelIntentSource.includes('AppGameNotificationParentSurfaceIntentReadModelSchema') &&
      portalTestSource.includes('renders schema-backed parent-surface intent rows without runtime claims'),
    emptyStateAssertion:
      panelIntentSource.includes('service event not reported') &&
      portalTestSource.includes('keeps absent or invalid service input explicit instead of inventing rows'),
    noRuntimeClaimAssertion:
      panelIntentSource.includes('AppGameNotificationParentSurfaceNoRuntimeClaim') &&
      portalTestSource.includes(
        'provider delivery, preference mutation, child delivery, and runtime dispatch remain unclaimed'
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
      `# ${label} Notification Parent Surface Renderer`,
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
