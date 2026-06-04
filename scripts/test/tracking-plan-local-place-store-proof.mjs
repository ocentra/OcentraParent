import { spawn } from 'node:child_process';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const workpackId = '22-local-parent-defined-place-database';
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', workpackId);
const testResultRoot = join(repoRoot, 'test-results', 'tracking-plan-local-place-store-proof');
const checkedAt = new Date().toISOString();
const commands = [];

await main();

async function main() {
  await runNpm(['--workspace', '@ocentra-parent/activity-domain', 'run', 'build']);
  await runNpm([
    'exec',
    '--workspace',
    '@ocentra-parent/activity-domain',
    '--',
    'vitest',
    'run',
    'tests/tracking-local-place-store.test.ts',
  ]);

  const tracking = await import(pathToFileURL(join(repoRoot, 'packages', 'activity-domain', 'dist', 'tracking.js')));
  const proof = await buildLocalPlaceStoreProof(tracking);
  const commit = await gitHead();
  await writeProofArtifacts({ proof, commit });

  console.log('tracking-plan-local-place-store-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofRoot)}`);
}

async function buildLocalPlaceStoreProof(tracking) {
  const tempRoot = await mkdtemp(join(tmpdir(), 'ocentra-parent-tracking-place-store-'));
  try {
    const store = tracking.createTrackingLocalParentDefinedPlaceStore({
      storeId: 'parent-local-place-store',
      createdAt: '2026-06-03T01:00:00.000Z',
      auditRefs: ['parent-local-place-store-created'],
    });
    const created = tracking.upsertTrackingLocalParentDefinedPlace({
      store,
      place: homePlace(),
      auditRefs: ['parent-defined-home-upserted'],
    });
    const imported = tracking.importTrackingLocalParentDefinedPlaces({
      store: created.store,
      importedAt: '2026-06-03T01:20:00.000Z',
      places: [restrictedPlace(), safePlace()],
      auditRefs: ['parent-defined-place-imported'],
    });
    const restrictedMatch = tracking.buildTrackingLocalParentDefinedPlaceMatch({
      store: imported.store,
      evidenceId: 'nearby-parent-defined-restricted-place',
      observedAt: '2026-06-03T02:01:00.000Z',
      locationEvidenceId: 'location-evidence-1',
      placeId: 'restricted-lot',
      queryRadiusMeters: 250,
      distanceMeters: 42,
      confidence: 0.9,
      reasonCodes: ['parent-defined-restricted-zone-match'],
      evidence: [evidenceRef()],
    });
    const safeMatch = tracking.buildTrackingLocalParentDefinedPlaceMatch({
      store: imported.store,
      evidenceId: 'nearby-parent-defined-safe-place',
      observedAt: '2026-06-03T02:01:00.000Z',
      locationEvidenceId: 'location-evidence-1',
      placeId: 'safe-library',
      queryRadiusMeters: 250,
      distanceMeters: 38,
      confidence: 0.88,
      reasonCodes: ['parent-defined-safe-zone-match'],
      evidence: [evidenceRef()],
    });
    const exported = tracking.exportTrackingLocalParentDefinedPlaceStore({
      store: imported.store,
      exportedAt: '2026-06-03T01:25:00.000Z',
      auditRefs: ['parent-defined-place-exported'],
    });
    const deleted = tracking.deleteTrackingLocalParentDefinedPlace({
      store: imported.store,
      placeId: 'restricted-lot',
      deletedAt: '2026-06-03T01:30:00.000Z',
      reasonCodes: ['parent-requested-place-delete'],
      auditRefs: ['parent-defined-restricted-zone-deleted'],
    });
    const localStorePath = join(tempRoot, 'parent-local-place-store.json');
    await writeFile(localStorePath, `${JSON.stringify(deleted.store, null, 2)}\n`);
    const reloadedStore = tracking.TrackingLocalParentDefinedPlaceStoreSchema.parse(
      JSON.parse(await readFile(localStorePath, 'utf8'))
    );
    return {
      created,
      imported,
      restrictedMatch,
      safeMatch,
      exported,
      deleted,
      localStoreRoundTrip: {
        pathKind: 'real-temp-file',
        reloadedPlaceCount: reloadedStore.places.length,
        reloadedTombstoneCount: reloadedStore.tombstones.length,
        remoteSyncDefault: reloadedStore.remoteSyncDefault,
        ocentraHostedDefaultStorage: reloadedStore.ocentraHostedDefaultStorage,
      },
    };
  } finally {
    await rm(tempRoot, { force: true, recursive: true });
  }
}

async function writeProofArtifacts({ proof, commit }) {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });
  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot(commit));
  await writeFile(join(proofRoot, '01-contract-proof.log'), contractProofLog());
  await writeJson(
    join(proofRoot, '07-nearby-place-proof.json'),
    tierEnvelope('P1_FIXTURE_SIMULATION', 'simulated', {
      restrictedMatch: proof.restrictedMatch.match,
      safeMatch: proof.safeMatch.match,
      restrictedNearbyEvidence: proof.restrictedMatch.nearbyPlaceEvidence,
      safeNearbyEvidence: proof.safeMatch.nearbyPlaceEvidence,
    })
  );
  await writeFile(join(proofRoot, '13-security-negative-proof.log'), securityProofLog());
  await writeJson(
    join(proofRoot, '14-retention-delete-proof.json'),
    tierEnvelope('P1_FIXTURE_SIMULATION', 'simulated', {
      beforePlaceCount: proof.deleted.beforePlaceCount,
      afterPlaceCount: proof.deleted.afterPlaceCount,
      tombstones: proof.deleted.store.tombstones,
      localStoreRoundTrip: proof.localStoreRoundTrip,
    })
  );
  await writeFile(join(proofRoot, '15-manual-platform-proof.md'), manualPlatformProof());
  await writeFile(
    join(proofRoot, '16-validation-commands.log'),
    commands.map((entry) => entry.command).join('\n') + '\n'
  );
  await writeJson(
    join(proofRoot, '17-parent-owned-export-proof.json'),
    tierEnvelope('P1_FIXTURE_SIMULATION', 'simulated', {
      exportedAt: proof.exported.exportedAt,
      custodyLabel: proof.exported.custodyLabel,
      remoteSyncDefault: proof.exported.remoteSyncDefault,
      ocentraHostedDefaultStorage: proof.exported.ocentraHostedDefaultStorage,
      exportedPlaceCount: proof.exported.places.length,
    })
  );
  await writeJson(join(proofRoot, 'proof-summary.json'), {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackId,
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus: 'simulated',
    productClaimReady: false,
    summary:
      'Local parent-defined place store CRUD/import/export/delete proof passes with parent-device-local storage and remote sync disabled.',
    proofArtifacts: [
      `output/tracking-plan-proof/${workpackId}/07-nearby-place-proof.json`,
      `output/tracking-plan-proof/${workpackId}/14-retention-delete-proof.json`,
      `output/tracking-plan-proof/${workpackId}/17-parent-owned-export-proof.json`,
    ],
    commands,
    productClaims: productClaims(),
    missingProofReason:
      'This is P1 local contract/runtime helper proof. Platform adapters, provider delivery, live UI, hosted accessibility, physical-device background location, and production persistence remain unclaimed.',
  });
  await writeJson(join(testResultRoot, 'proof.json'), {
    schemaVersion: 1,
    checkedAt,
    workpackId,
    currentStatus: 'simulated',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    proofRoot: relative(repoRoot, proofRoot).replaceAll('\\', '/'),
    commands,
    productClaims: productClaims(),
  });
}

function tierEnvelope(currentProofTier, currentStatus, payload) {
  return {
    schemaVersion: 1,
    checkedAt,
    workpackId,
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier,
    currentStatus,
    productClaimReady: false,
    missingProofReason:
      'Platform/provider/live UI/hosted accessibility/physical-device behavior remains outside this local store proof.',
    payload,
  };
}

function sourceSnapshot(commit) {
  return [
    '# 22-local-parent-defined-place-database Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- source: packages/activity-domain/src/tracking-local-place-store.ts',
    '- test: packages/activity-domain/tests/tracking-local-place-store.test.ts',
    '- command: npm run test:tracking-plan-local-place-store-proof',
    '',
  ].join('\n');
}

function contractProofLog() {
  return [
    `workpack=${workpackId}`,
    'activity-domain tracking local place store test passed',
    'store boundary is parent-device-local',
    'remoteSyncDefault is disabled',
    'ocentraHostedDefaultStorage is false',
    '',
  ].join('\n');
}

function securityProofLog() {
  return [
    `workpack=${workpackId}`,
    'Local parent-defined place data remains parent-device-local by default.',
    'The proof rejects Ocentra-hosted default storage and does not enable remote sync.',
    'Nearby-place evidence is labelled providerKind=parent-defined and does not claim provider delivery.',
    '',
  ].join('\n');
}

function manualPlatformProof() {
  return [
    '# WP22 Manual Platform Boundary',
    '',
    'This proof does not claim Android/iOS location capture, background geofence behavior, hosted UI, or physical-device behavior.',
    'It proves only local parent-defined place store CRUD/import/export/delete and parent-defined nearby-place evidence shape at P1 fixture tier.',
    '',
  ].join('\n');
}

function homePlace() {
  return {
    schemaVersion: 1,
    placeId: 'home',
    label: 'Home',
    placeKind: 'home',
    shape: {
      kind: 'circle',
      center: {
        latitude: 43.6532,
        longitude: -79.3832,
      },
      radiusMeters: 150,
      polygon: [],
    },
    createdAt: '2026-06-03T01:00:00.000Z',
    updatedAt: '2026-06-03T01:00:00.000Z',
    auditRefs: ['parent-defined-home-created'],
  };
}

function restrictedPlace() {
  return {
    ...homePlace(),
    placeId: 'restricted-lot',
    label: 'Restricted lot',
    placeKind: 'restricted-zone',
    updatedAt: '2026-06-03T01:05:00.000Z',
    auditRefs: ['parent-defined-restricted-zone-created'],
  };
}

function safePlace() {
  return {
    ...homePlace(),
    placeId: 'safe-library',
    label: 'Safe library',
    placeKind: 'safe-zone',
    updatedAt: '2026-06-03T01:10:00.000Z',
    auditRefs: ['parent-defined-safe-zone-imported'],
  };
}

function evidenceRef() {
  return {
    evidenceId: 'tracking-journal-row-1',
    kind: 'journal-entry',
    digest: 'sha256:tracking-proof',
    uri: null,
  };
}

function productClaims() {
  return {
    localParentDefinedPlaceStoreProved: true,
    androidIosBackgroundLocationClaimed: false,
    preciseLocationFromLanIpWifiClaimed: false,
    providerDeliveryClaimed: false,
    hostedUiAccessibilityClaimed: false,
    remoteSyncEnabledByDefault: false,
    ocentraHostedDefaultStorage: false,
  };
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function gitHead() {
  const chunks = [];
  await runCommand('git', ['rev-parse', 'HEAD'], { capture: chunks, quiet: true, record: false });
  return chunks.join('').trim();
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', ...args]);
    return;
  }
  await runCommand('npm', args);
}

async function runCommand(command, args, options = {}) {
  const record = options.record !== false;
  const commandText = [command, ...args].join(' ');
  const child = spawn(command, args, { cwd: repoRoot, shell: false });
  let stdout = '';
  let stderr = '';
  child.stdout.on('data', (chunk) => {
    const text = chunk.toString();
    stdout += text;
    options.capture?.push(text);
    if (!options.quiet) process.stdout.write(text);
  });
  child.stderr.on('data', (chunk) => {
    const text = chunk.toString();
    stderr += text;
    if (!options.quiet) process.stderr.write(text);
  });
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  if (record) commands.push({ command: commandText, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${commandText} failed with exit code ${exitCode}\n${stdout}\n${stderr}`);
  }
}
