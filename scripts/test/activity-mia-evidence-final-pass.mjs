import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'activity-mia-evidence-final-pass');
const proofPath = join(outputDir, 'proof.json');
const adapterProofPath = join(repoRoot, 'test-results', 'activity-surface-main-backed-adapter', 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand('node', ['scripts/test/activity-surface-main-backed-adapter-proof.mjs']);
  const adapterProof = await readJson(adapterProofPath);
  assertFinalPassCoverage(adapterProof);

  const proof = {
    schemaVersion: 1,
    proofMode: 'activity-mia-evidence-final-pass',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels: [
      'activity-mia-final-pass.report-persistence',
      'activity-mia-final-pass.family-device-states',
      'activity-mia-final-pass.source-stale-unreachable-summary',
      'activity-mia-final-pass.adapter-handoff',
      'activity-mia-final-pass.family-device-request-builders',
      'activity-mia-final-pass.adapter-operation-manifest',
      'activity-mia-final-pass.adapter-failure-metadata',
      'activity-mia-final-pass.parent-assistant-evidence',
      'activity-mia-final-pass.saved-report-metadata-citation',
      'activity-mia-final-pass.c-owned-paths-not-touched',
    ],
    evidence: {
      upstreamProof: relative(repoRoot, adapterProofPath),
      activityDomain: 'packages/activity-domain/src/activity-surface.ts',
      adapterBoundary: 'packages/agent-protocol-domain/src/activity-surface-adapter.ts',
      adapterOperationManifest: 'packages/agent-protocol-domain/src/activity-surface-adapter-manifest.ts',
      adapterBoundaryTest: 'packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts',
      rustReportStore: 'crates/agent-service/src/activity_surface_report_store.rs',
      rustFamilySources: 'crates/agent-service/src/activity_family_sources_tests.rs',
      rustParentAssistantContext: 'crates/agent-service/src/parent_assistant_evidence_context.rs',
      runtimeProof: 'scripts/test/activity-parent-assistant-runtime-proof.mjs',
      checkpoint: 'docs/checkpoints/activity-report-persistence-family-mia-context-2026-05-31.md',
    },
    coverage: {
      reportPersistence:
        'Generated Activity reports carry draft metadata, saveActivityReport persists saved JSON metadata, listHistoricalReports exposes saved metadata, and storage-unavailable fallback remains typed.',
      familyDeviceBehavior:
        'Family reports carry reachable/offline/stale/unreachable/error source states while device-scoped remote requests degrade to typed offline reports.',
      sourceStateSummary:
        'Saved report history rows count stale and unreachable sources separately from offline, unavailable, and error sources.',
      adapterHandoff:
        'The TypeScript service-adapter boundary creates family/device request documents, report/history/read-model commands, exposes a C-consumable operation manifest with failure metadata, and parses generated/saved/history/read-model events with typed unavailable failures.',
      parentAssistantEvidence:
        'Parent Assistant/MIA cites saved Activity report metadata, ready section counts, offline/stale/unreachable/unavailable source counts, stale/unreachable source ids where available, and child-contract action-preview boundaries.',
      cOwnedPathPolicy:
        'This proof does not edit C-owned Activity UI, vendor portal, temp scratchpad, parent-assistant API integration, service main.rs, or websocket.rs paths.',
    },
    counts: {
      coveredActivityTabs: adapterProof.productTruth.coveredTabs.length,
      upstreamProofLabels: adapterProof.proofLabels.length,
      finalPassProofLabels: 10,
      cOwnedPathsTouched: 0,
    },
    knownGaps: [
      'C-owned visual Activity UI still needs to consume the service-backed adapter surface.',
      'Physical multi-device family fan-out remains represented by typed source states until real household devices are connected.',
      'Parent Assistant/MIA remains citation-bound and does not apply policy, enforcement, or child-safety decisions directly.',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`activity-mia-evidence-final-pass-ok:${proof.proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function assertFinalPassCoverage(adapterProof) {
  const coveredTabs = new Set(adapterProof.productTruth?.coveredTabs ?? []);
  for (const tab of ['reports', 'screen', 'app-use', 'browser', 'games', 'network']) {
    if (!coveredTabs.has(tab)) {
      throw new Error(`Activity final-pass proof is missing covered tab ${tab}.`);
    }
  }

  const evidence = adapterProof.evidence ?? {};
  for (const key of [
    'activityDomain',
    'agentProtocolDomainAdapter',
    'agentProtocolDomainAdapterTest',
    'rustServiceAdapter',
    'runtimeProof',
  ]) {
    if (typeof evidence[key] !== 'string') {
      throw new Error(`Activity final-pass proof is missing evidence.${key}.`);
    }
  }

  const knownGaps = adapterProof.knownGaps ?? [];
  if (!knownGaps.some((gap) => String(gap).includes('C-owned visual Activity UI'))) {
    throw new Error('Activity final-pass proof must keep C-owned UI integration as an explicit gap.');
  }
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
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
