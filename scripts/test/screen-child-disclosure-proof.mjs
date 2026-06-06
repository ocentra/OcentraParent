import { strict as assert } from 'node:assert';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));
const outputDir = join(repoRoot, 'test-results', 'screen-child-disclosure-proof');
const outputPath = join(outputDir, 'proof.json');
const planOutputDir = join(repoRoot, 'output', 'screen-plan-proof', 'screen-child-disclosure');
const planOutputPath = join(planOutputDir, 'proof-summary.json');

run('npx', ['vitest', 'run', 'packages/activity-domain/tests/screen-child-disclosure.test.ts']);
run('npm', ['run', 'build', '--workspace=@ocentra-parent/activity-domain']);

const screenEvidence = await import('../../packages/activity-domain/dist/screen-evidence.js');
const snapshots = screenEvidence.screenChildDisclosureProofSnapshots();
const parsed = snapshots.map((snapshot) => screenEvidence.ScreenChildDisclosureSnapshotSchema.parse(snapshot));

assert.deepEqual(
  parsed.map((snapshot) => snapshot.state),
  ['disabledByParent', 'pausedByParent', 'captureActive', 'protectedSurface', 'deletedSummaryReady']
);
assert.equal(parsed[2].surface, 'child-agent-capture-banner');
assert.equal(parsed[2].captureActive, true);
assert.equal(parsed[4].deletionState, 'deleted');
assert.equal(
  parsed.every((snapshot) => snapshot.visibleToChildRequired),
  true
);
assert.equal(
  parsed.every((snapshot) => !snapshot.hiddenCaptureClaimed),
  true
);
assert.equal(
  parsed.every((snapshot) => !snapshot.rawScreenshotShownToChild),
  true
);

const proof = {
  proofId: 'screen-child-disclosure-proof',
  generatedAt: '2026-06-06T21:55:00Z',
  source: '@ocentra-parent/activity-domain screen child disclosure contracts',
  assertions: [
    'child-visible screen disclosure states are schema-backed',
    'disabled-by-parent state cannot claim cadence, trigger, or active capture',
    'active capture requires ready capability, approved non-unsupported scope, and child-agent capture banner surface',
    'protected-surface state stays visible without queued raw image custody',
    'deleted-summary-ready state requires deleted local custody before summary display',
    'hidden capture, raw screenshot display, remote viewer, and policy-authority claims are rejected',
    'rendered child-agent delivery remains unclaimed until a real child UX surface is implemented',
  ],
  parsed: {
    states: parsed.map((snapshot) => snapshot.state),
    activeSurface: parsed[2].surface,
    summaryCustody: parsed[4].custodyState,
    renderedChildAgentDeliveryClaimed: parsed.some((snapshot) => snapshot.renderedChildAgentDeliveryClaimed),
  },
};

mkdirSync(outputDir, { recursive: true });
mkdirSync(planOutputDir, { recursive: true });
writeFileSync(outputPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(planOutputPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(`screen-child-disclosure-proof-ok: ${outputPath}`);

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    shell: process.platform === 'win32',
    stdio: 'inherit',
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${result.status}`);
  }
}
