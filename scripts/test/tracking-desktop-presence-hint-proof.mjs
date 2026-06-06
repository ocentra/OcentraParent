import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const timestamp = '2026-06-06T18:42:00.000Z';
const testOutputDir = join(repoRoot, 'test-results', 'tracking-desktop-presence-hint-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '13-desktop-location-and-presence-hint-model');
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

const presence = await importDist('tracking-desktop-presence-hint-proof.js');
const rows = presence.buildTrackingDesktopPresenceHintRows();
const summary = presence.summarizeTrackingDesktopPresenceHintProof(rows);

const proof = {
  proofMode: 'tracking-desktop-presence-hint-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary,
  nonClaims: {
    preciseLocationClaimed: false,
    physicalPresenceClaimed: false,
    lanPairingAsPresenceClaimed: false,
    staleDisplayedAsLiveClaimed: false,
    desktopOsLocationRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    productionBehaviorClaimed: false,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-desktop-presence-hint-proof.ts',
    test: 'packages/parent-domain/tests/tracking-desktop-presence-hint-proof.test.ts',
    harness: 'scripts/test/tracking-desktop-presence-hint-proof.mjs',
    evidence: 'test-results/tracking-desktop-presence-hint-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/13-desktop-location-and-presence-hint-model',
  },
  rows,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeJson(join(testOutputDir, 'presence-hint-rows.json'), rows);
await writeProofPack(proofDir, proof);

console.log('tracking-desktop-presence-hint-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-desktop-presence-hint-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function assertProof(proof) {
  if (
    proof.summary.sourceCount !== 10 ||
    proof.summary.preciseLocationClaimCount !== 0 ||
    proof.summary.physicalPresenceClaimCount !== 0 ||
    proof.summary.hintOnlyRows.join(',') !== 'lan-pairing,home-wifi,ip-coarse' ||
    proof.summary.manualCheckInSeparatedRows.join(',') !== 'linux-manual-check-in,manual-check-in' ||
    proof.summary.staleOfflineMissingRows.join(',') !== 'stale-cache,offline,missing-device'
  ) {
    throw new Error(`Unexpected desktop presence summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Desktop presence proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP13 Desktop Location And Presence Hint Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-domain desktop presence hint rows for OS-location manual-required, LAN/Wi-Fi/IP hint-only, manual check-in, stale, offline, and missing-device states.',
      '- No desktop OS location runtime, precise GPS, physical presence, physical-device proof, or production behavior is claimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-desktop-presence-hint-proof: PASS',
      '- LAN, Wi-Fi, and IP rows are hint-only and cannot claim precise location or physical presence.',
      '- Manual check-in rows are separate from automatic physical presence.',
      '- Windows/macOS precise desktop location remains manual-required until OS location proof exists.',
      '- Stale, offline, and missing-device rows cannot be displayed as live.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '03-runtime-location-evidence.json'), proof.summary);
  await writeJson(join(path, '17-desktop-presence-hint-proof.json'), proof);
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- LAN pairing, home Wi-Fi, and IP coarse rows reject precise-location and physical-presence upgrades.',
      '- Manual check-in rows must remain separate from automatic presence.',
      '- Stale, offline, and missing-device rows reject live-device upgrades.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeJson(join(path, 'proof-summary.json'), {
    schemaVersion: 1,
    checkedAt: proof.generatedAt,
    commit: proof.commit,
    workpackId: '13-desktop-location-and-presence-hint-model',
    proofState: 'desktop-presence-hint-proof-complete',
    summary: proof.summary,
    commands: proof.commands,
    productClaims: {
      lanWifiIpHintOnly: true,
      manualCheckInSeparated: true,
      desktopPreciseLocationManualRequired: true,
      staleOfflineMissingNotLive: true,
      preciseLocationClaimed: false,
      physicalPresenceClaimed: false,
      physicalDeviceProofClaimed: false,
      productionBehaviorClaimed: false,
    },
  });
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

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
