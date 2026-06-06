import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-desktop-presence-hint-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '13-desktop-location-and-presence-hint-model');
const timestamp = '2026-06-06T09:45:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-desktop-presence-hint-proof',
]);

const proofModule = await importDist('tracking-desktop-presence-hint-proof.js');
const readModel = proofModule.buildTrackingDesktopPresenceHintProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-desktop-presence-hint-proof',
    familyId: 'family-desktop-presence',
    deviceId: 'device-avery-laptop',
    childProfileId: 'child-profile-avery',
    deviceLabel: 'Avery laptop',
    platform: 'windows',
    sourceProofRefs: [
      'location-geofence-device-status',
      'workpack-13-desktop-location-and-presence-hint-model',
      'v0-5-location-platform-deep-dive',
      'location-geofence-expectation',
      'platforms-expectation',
    ],
  },
  inputRows()
);

const proof = {
  proofMode: 'tracking-desktop-presence-hint-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-desktop-presence-hint-proof.ts',
    test: 'packages/parent-domain/tests/tracking-desktop-presence-hint-proof.test.ts',
    harness: 'scripts/test/tracking-desktop-presence-hint-proof.mjs',
    evidence: 'test-results/tracking-desktop-presence-hint-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/13-desktop-location-and-presence-hint-model',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-desktop-presence-hint-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-desktop-presence-hint-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-desktop-presence-hint-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function inputRows() {
  return [
    row({
      rowId: 'desktop-presence-lan-hint',
      caseKind: 'lan-presence-hint',
      source: 'lan-pairing-presence',
      freshnessState: 'fresh-hint',
      evidenceRefs: ['lan-pairing-seen-on-home-network'],
      auditRefs: ['audit-desktop-lan-hint-no-gps'],
    }),
    row({
      rowId: 'desktop-presence-wifi-hint',
      caseKind: 'wifi-presence-hint',
      source: 'wifi-network-hint',
      freshnessState: 'fresh-hint',
      evidenceRefs: ['wifi-ssid-seen-home-network'],
      auditRefs: ['audit-desktop-wifi-hint-no-gps'],
    }),
    row({
      rowId: 'desktop-presence-ip-coarse-hint',
      caseKind: 'ip-coarse-hint',
      source: 'ip-coarse-hint',
      freshnessState: 'fresh-hint',
      evidenceRefs: ['ip-coarse-region-home-city'],
      auditRefs: ['audit-desktop-ip-hint-no-precise-location'],
    }),
    row({
      rowId: 'desktop-presence-manual-check-in',
      caseKind: 'manual-check-in',
      source: 'child-manual-check-in',
      freshnessState: 'manual-reported',
      evidenceRefs: ['manual-check-in-avery-laptop-20260606'],
      auditRefs: ['audit-desktop-manual-check-in'],
      manualCheckInRef: 'manual-check-in-avery-laptop-20260606',
    }),
    row({
      rowId: 'desktop-presence-stale-offline-last-known',
      caseKind: 'stale-offline-last-known',
      source: 'query-store-last-known',
      freshnessState: 'stale',
      evidenceRefs: ['last-known-desktop-presence-stale'],
      auditRefs: ['audit-desktop-stale-offline-no-current-location'],
      lastKnownEvidenceRef: 'last-known-desktop-presence-stale',
      stale: true,
      offline: true,
    }),
    row({
      rowId: 'desktop-presence-missing-device',
      caseKind: 'missing-device',
      source: 'query-store-missing-device',
      freshnessState: 'missing',
      evidenceRefs: ['missing-device-desktop-query-store'],
      auditRefs: ['audit-desktop-missing-device-no-current-location'],
      missingDevice: true,
      offline: true,
    }),
    row({
      rowId: 'desktop-presence-os-location-manual-required',
      caseKind: 'desktop-os-location-manual-required',
      source: 'manual-platform-plan',
      freshnessState: 'manual-required',
      evidenceRefs: ['desktop-os-location-runtime-not-proved'],
      auditRefs: ['audit-desktop-os-location-manual-required'],
    }),
  ];
}

function row(input) {
  return {
    observedAt: timestamp,
    platform: 'windows',
    lastKnownEvidenceRef: null,
    manualCheckInRef: null,
    stale: false,
    offline: false,
    missingDevice: false,
    ...input,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    hintOnlyCount: readModel.hintOnlyCount,
    manualCheckInCount: readModel.manualCheckInCount,
    staleOfflineCount: readModel.staleOfflineCount,
    missingDeviceCount: readModel.missingDeviceCount,
    manualRequiredCount: readModel.manualRequiredCount,
    runtimeEvidenceRefs: readModel.runtimeEvidenceRefs.length,
    parentVisibleStatusTokens: countBy(readModel.rows.map((row) => row.parentVisibleStatusToken)),
    claimStates: countBy(readModel.rows.map((row) => row.claimState)),
  };
}

function nonClaims(readModel) {
  return {
    preciseLocationClaimed: readModel.preciseLocationClaimed,
    gpsClaimed: readModel.gpsClaimed,
    physicalPresenceClaimed: readModel.physicalPresenceClaimed,
    lanPairingPhysicalProofClaimed: readModel.lanPairingPhysicalProofClaimed,
    wifiPhysicalPresenceClaimed: readModel.wifiPhysicalPresenceClaimed,
    ipPhysicalPresenceClaimed: readModel.ipPhysicalPresenceClaimed,
    osLocationRuntimeClaimed: readModel.osLocationRuntimeClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    productReadyDesktopTrackingClaimed: readModel.productReadyDesktopTrackingClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 7 ||
    proof.summary.hintOnlyCount !== 3 ||
    proof.summary.manualCheckInCount !== 1 ||
    proof.summary.staleOfflineCount !== 1 ||
    proof.summary.missingDeviceCount !== 1 ||
    proof.summary.manualRequiredCount !== 1 ||
    proof.summary.runtimeEvidenceRefs !== 7
  ) {
    throw new Error(`Unexpected desktop presence summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Desktop presence proof overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP13 Desktop Location And Presence Hint Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-domain desktop presence hint read model for LAN, Wi-Fi, IP coarse hint, manual check-in, stale/offline last-known state, missing-device state, and desktop OS location manual-required state.',
      '- Source inspected: location/geofence feature doc, WP13 tracking workpack, V0.5 location platform deep dive, location/geofence expectation, platform expectation, portal README, and parent-domain README.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '03-runtime-location-evidence.json'), {
    rows: proof.readModel.rows.map((row) => ({
      rowId: row.rowId,
      caseKind: row.caseKind,
      source: row.source,
      claimState: row.claimState,
      freshnessState: row.freshnessState,
      parentVisibleStatusToken: row.parentVisibleStatusToken,
      preciseLocationClaimed: row.preciseLocationClaimed,
      gpsClaimed: row.gpsClaimed,
      physicalPresenceClaimed: row.physicalPresenceClaimed,
      lanPairingPhysicalProofClaimed: row.lanPairingPhysicalProofClaimed,
      osLocationRuntimeClaimed: row.osLocationRuntimeClaimed,
    })),
    runtimeEvidenceRefs: proof.readModel.runtimeEvidenceRefs,
  });
  await writeJson(join(path, '04-desktop-presence-read-model.json'), proof.readModel);
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- LAN, Wi-Fi, and IP rows are hint-only and do not claim GPS, precise location, or physical presence.',
      '- LAN pairing is explicitly not physical-presence proof.',
      '- Manual check-in is a child-reported check-in, separate from automatic physical proof.',
      '- Stale/offline and missing-device rows preserve degraded state without current-location claims.',
      '- Desktop OS location remains manual-required because this proof does not wire a runtime OS location provider.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '15-manual-platform-proof.md'),
    [
      '# WP13 Manual Platform Proof',
      '',
      '- Windows, macOS, and Linux desktop rows are represented as hint/manual-required until an OS location runtime sample is implemented and separately proved.',
      '- This proof only establishes contract/read-model boundaries and negative claims for desktop presence hints.',
      '- It does not claim product-ready desktop tracking, physical child presence, provider delivery, device-owner controls, or background location parity with Android/iOS.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(path, 'README.md'),
    '# WP13 Desktop Presence Hint Proof\n\nThis proof pack records a parent-domain desktop presence hint read model for LAN, Wi-Fi, IP coarse hint, manual check-in, stale/offline last-known state, missing-device state, and desktop OS location manual-required state. It intentionally does not claim GPS, precise location, physical presence, OS location runtime, physical-device proof, or product-ready desktop tracking.\n',
    'utf8'
  );
  await writeJson(join(path, 'proof.json'), proof);
}

function run(command, args) {
  commands.push([command, ...args].join(' '));
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
