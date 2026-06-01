import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'activity-reports-family-fanout-mia-final-pass');
const proofPath = join(outputDir, 'proof.json');
const upstreamProofPath = join(
  repoRoot,
  'test-results',
  'activity-mia-report-history-action-preview-proof',
  'proof.json'
);
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runNpmWorkspace('@ocentra-parent/activity-domain', ['test', '--', 'activity-family-aggregation']);
  await runCommand('node', ['scripts/test/activity-mia-report-history-action-preview-proof.mjs']);

  const upstreamProof = await readJson(upstreamProofPath);
  assertUpstreamProof(upstreamProof);
  const touchedFiles = await gitDiffNames();
  assertNoCOwnedPaths(touchedFiles);

  const proof = {
    schemaVersion: 1,
    proofMode: 'activity-reports-family-fanout-mia-final-pass',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels: [
      'activity-reports-family-fanout.family-aggregation-contract',
      'activity-reports-family-fanout.service-owned-product-data',
      'activity-reports-family-fanout.vite-not-product-owner',
      'activity-reports-family-fanout.offline-stale-unavailable-source-ids',
      'activity-reports-family-fanout.storage-unavailable-fallback',
      'activity-reports-family-fanout.mia-report-history-citations',
      'activity-reports-family-fanout.c-owned-paths-not-touched',
    ],
    evidence: {
      activityFamilyAggregationContract: 'packages/activity-domain/src/activity-family-aggregation.ts',
      activityFamilyAggregationTest: 'packages/activity-domain/tests/activity-family-aggregation.test.ts',
      upstreamReportHistoryProof: relative(repoRoot, upstreamProofPath),
      activitySurfaceContract: 'packages/activity-domain/src/activity-surface.ts',
      adapterBoundary: 'packages/agent-protocol-domain/src/activity-surface-adapter.ts',
      rustReportStore: 'crates/agent-service/src/activity_surface_report_store.rs',
      rustFamilySources: 'crates/agent-service/src/activity_family_sources.rs',
      rustParentAssistantContext: 'crates/agent-service/src/parent_assistant_evidence_context.rs',
      checkpoint: 'docs/checkpoints/activity-reports-family-fanout-mia-final-pass-2026-06-01.md',
    },
    coverage: {
      familyAggregation:
        'A typed Activity family aggregation model derives renderable source counts and source ids from service-owned report/history documents.',
      productDataOwnership:
        'The aggregation contract requires dataOwner=rust-service-read-model and viteDataOwner=false so Portal remains a renderer/consumer.',
      sourceStates:
        'Ready, offline, stale, unavailable, unreachable, and error source ids remain explicit for family fan-out and MIA citation context.',
      storageFallback:
        'A storage-unavailable history response stays renderable as unavailable with zero sources instead of being promoted to ready.',
      cOwnedPathPolicy: 'The proof rejects C-owned portal UI/domain path edits in this non-visual worker slice.',
    },
    touchedFiles,
    counts: {
      upstreamProofLabels: upstreamProof.proofLabels.length,
      proofLabels: 7,
      cOwnedPathsTouched: 0,
    },
    knownGaps: [
      'C-owned visual Activity UI still needs to render family aggregation and report-history metadata.',
      'Physical household multi-device fan-out remains represented by typed source states until real household devices are connected.',
      'Parent Assistant/MIA remains citation context only and does not write policy, authorize API AI, or enforce on child devices.',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`activity-reports-family-fanout-mia-final-pass-ok:${proof.proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function assertUpstreamProof(proof) {
  for (const label of [
    'activity-mia-report-history.saved-metadata',
    'activity-mia-report-history.source-state-summary',
    'activity-mia-action-preview.report-source-id-citations',
    'activity-mia-action-preview.stale-unreachable-source-id-citations',
  ]) {
    if (!proof.proofLabels.includes(label)) {
      throw new Error(`Upstream Activity report-history proof is missing ${label}.`);
    }
  }
}

function assertNoCOwnedPaths(paths) {
  const blockedPrefixes = ['apps/portal/', 'packages/portal-domain/', 'vendor/ocentra-parent-core-ui/', '.codex/'];
  const blocked = paths.filter((path) => blockedPrefixes.some((prefix) => path.startsWith(prefix)));
  if (blocked.length > 0) {
    throw new Error(`Activity family fanout proof touched C-owned paths: ${blocked.join(',')}`);
  }
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

async function runNpmWorkspace(workspaceName, args) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', '--workspace', workspaceName, ...args]);
    return;
  }
  await runCommand('npm', ['--workspace', workspaceName, ...args]);
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${command} exited with ${code}`))));
    child.once('error', reject);
  });
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

async function gitDiffNames() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['diff', '--name-only', 'origin/main...HEAD'], {
      cwd: repoRoot,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git diff --name-only failed'))));
    child.once('error', reject);
  });
  return chunks
    .join('')
    .split(/\r?\n/u)
    .map((path) => path.trim())
    .filter(Boolean);
}
