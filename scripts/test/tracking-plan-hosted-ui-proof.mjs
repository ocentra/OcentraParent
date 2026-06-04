import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import {
  availablePort,
  sanitizeServerOutput,
  startPortalServer,
  stopProcessTree,
  runHostedTrackingUiBrowserProof,
  waitForHttp,
} from './tracking-plan-hosted-ui-proof-browser.mjs';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const screenshotPath = join(proofRoot, '11-ui-snapshots', 'policy-tracking-hosted-ui-proof.png');
const accessibilityPath = join(proofRoot, '17-accessibility-proof.json');
const logPath = join(proofRoot, '12-playwright-proof.log');
const commands = [];

await main();

async function main() {
  await runNpm(['run', 'build:contracts']);
  await mkdir(join(screenshotPath, '..'), { recursive: true });
  await mkdir(join(accessibilityPath, '..'), { recursive: true });

  const startedAt = new Date().toISOString();
  const host = '127.0.0.1';
  const port = await availablePort(Number.parseInt(process.env.TRACKING_PLAN_PORTAL_PORT ?? '4578', 10));
  const route = `http://${host}:${port}/#/policy-tracking`;
  const { child: server, commandLine } = startPortalServer({ repoRoot, host, port });
  commands.push({ command: commandLine, exitCode: 0 });

  const output = [];
  server.stdout.on('data', (chunk) => output.push(String(chunk)));
  server.stderr.on('data', (chunk) => output.push(String(chunk)));

  let proof;
  try {
    await waitForHttp(route, 30_000);
    proof = await runHostedTrackingUiBrowserProof({ route, screenshotPath });
  } finally {
    await stopProcessTree({ repoRoot, child: server });
  }

  const artifact = hostedUiArtifact({ startedAt, proof });
  await writeJson(accessibilityPath, artifact);
  await writeProofLog({ startedAt, route, output, proof });
  console.log('tracking-plan-hosted-ui-proof-ok');
  console.log(`evidence=${proofRelative(accessibilityPath)}`);
  console.log(`screenshot=${proofRelative(screenshotPath)}`);
}

function hostedUiArtifact({ startedAt, proof }) {
  return {
    schemaVersion: 1,
    checkedAt: startedAt,
    workpackId: '30-parent-and-child-ui-ux-surfaces',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    currentStatus: 'proved',
    productClaimReady: false,
    screenshotPath: proofRelative(screenshotPath),
    logPath: proofRelative(logPath),
    commands,
    proof: { ...proof, screenshotPath: proofRelative(screenshotPath) },
    nonClaims: [
      'This proof does not claim Android or iOS physical background behavior.',
      'This proof does not claim child-device UI completion.',
      'This proof does not claim full live service-backed tracking UI completion.',
      'This proof does not claim authority-enrolled or production-pilot readiness.',
    ],
  };
}

async function writeProofLog({ startedAt, route, output, proof }) {
  const lines = [
    'Tracking Plan WP30 hosted portal UI proof',
    '',
    `Date: ${startedAt}`,
    `Route: ${route}`,
    `Screenshot: ${proofRelative(screenshotPath)}`,
    `Accessibility proof: ${proofRelative(accessibilityPath)}`,
    '',
    'Checks:',
    '- Rendered the Policy Tracking route through the real Vite portal.',
    '- Verified every first-target tracking state is visible.',
    '- Verified no-product-claim copy remains visible.',
    '- Verified retention-deleted safety copy is visible and deleted evidence id is hidden.',
    '- Verified section label, heading, button name, card headings, definition lists, no-claim copy, and no visible card overlap.',
    '',
    'Non-claims:',
    '- This does not prove Android/iOS physical background behavior.',
    '- This does not prove child-device UI completion.',
    '- This does not prove full live service-backed tracking UI completion.',
    '',
    'Browser proof:',
    JSON.stringify(proof, null, 2),
    '',
    'Server output:',
    ...sanitizeServerOutput(output.join('')).split(/\r?\n/u).filter(Boolean),
    '',
  ];
  await writeFile(logPath, `${lines.join('\n')}\n`);
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', ...args], ['npm', ...args].join(' '));
    return;
  }
  await runCommand('npm', args, ['npm', ...args].join(' '));
}

async function runCommand(command, args, commandLine = [command, ...args].join(' ')) {
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      commands.push({ command: commandLine, exitCode: code });
      code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`));
    });
    child.once('error', reject);
  });
}

async function writeJson(path, value) {
  await mkdir(join(path, '..'), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function proofRelative(path) {
  return relative(repoRoot, path).replace(/\\/gu, '/');
}
