import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'tamper-integrity-audit-contract-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/logging-domain']));
  await runCommand(
    ...npmCommand(['run', 'test', '--workspace', '@ocentra-parent/logging-domain', '--', 'tamper-integrity-audit'])
  );

  const { TamperIntegrityAuditRequiredPayloadFields } =
    await import('@ocentra-parent/logging-domain/tamper-integrity-audit');
  const { TamperIntegrityAuditReadModel } =
    await import('@ocentra-parent/logging-domain/tamper-integrity-audit-read-model');
  const summary = summarizeReadModel(TamperIntegrityAuditReadModel);
  assertPackageExports(TamperIntegrityAuditRequiredPayloadFields, TamperIntegrityAuditReadModel);
  assertReadModel(TamperIntegrityAuditReadModel, summary);

  const proof = {
    schemaVersion: 1,
    proofMode: 'tamper-integrity-audit-contract-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      tsContract: 'packages/logging-domain/src/tamper-integrity-audit.ts',
      tsReadModel: 'packages/logging-domain/src/tamper-integrity-audit-read-model.ts',
      tsContractTest: 'packages/logging-domain/tests/tamper-integrity-audit.test.ts',
      featureDoc: 'docs/features/enforcement-integrity-tamper.md',
      expectationDoc: 'docs/expectations/tamper-uninstall-protection.md',
      proofHarness: 'scripts/test/tamper-integrity-audit-contract-proof.mjs',
      proofArtifact: 'test-results/tamper-integrity-audit-contract-proof/proof.json',
      packageExports: [
        '@ocentra-parent/logging-domain/tamper-integrity-audit',
        '@ocentra-parent/logging-domain/tamper-integrity-audit-read-model',
      ],
    },
    counts: summary,
    claimsProved: [
      'Tamper integrity audit logs cover stale heartbeat, offline heartbeat, permission loss, stopped service, removed agent, uninstall detection, tamper/manual-required, and admin removal flow rows',
      'Rows carry redaction-safe operational fields, audit refs, integrity refs, parent alert refs, authenticated drill-in refs, and admin removal flow refs where required',
      'Uninstall detection and tamper/manual rows require manual proof before platform protection or anti-tamper claims can be upgraded',
      'Raw child data, raw evidence payloads, raw URLs, screenshots, command lines, private paths, and message contents are excluded from the logging contract',
      'Stealth behavior, privilege escalation, hidden persistence, provider delivery, and admin-removal blocking remain explicit non-claims',
    ],
    claimsNotProved: [
      'anti-tamper behavior',
      'stealth or hidden persistence',
      'privilege escalation',
      'notification provider delivery',
      'admin removal blocking',
      'platform uninstall artifact capture',
      'permission restoration',
      'raw child activity or evidence payload custody',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`tamper-integrity-audit-contract-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function assertPackageExports(requiredPayloadFields, readModel) {
  assertArrayEqual(
    requiredPayloadFields,
    [
      'audit-entry-ref',
      'family-scope-ref',
      'device-scope-ref',
      'integrity-state',
      'signal-kind',
      'severity',
      'reason-code',
      'first-seen-at',
      'last-seen-at',
      'parent-drill-in-ref',
      'admin-removal-flow-ref',
      'manual-proof-ref',
    ],
    'package export required payload fields'
  );
  assertEqual(readModel.schemaVersion, 1, 'package export read model schema version');
  proofLabels.push('tamper-integrity-audit.package-exports');
}

function summarizeReadModel(readModel) {
  return {
    entries: readModel.entries.length,
    bySignalKind: countBy(readModel.entries.map((entry) => entry.signalKind)),
    byHeartbeatState: countBy(readModel.entries.map((entry) => entry.heartbeatState)),
    byPermissionState: countBy(readModel.entries.map((entry) => entry.permissionState)),
    byServicePresenceState: countBy(readModel.entries.map((entry) => entry.servicePresenceState)),
    byUninstallState: countBy(readModel.entries.map((entry) => entry.uninstallState)),
    byTamperState: countBy(readModel.entries.map((entry) => entry.tamperState)),
    redactionSafePayloadFieldSets: new Set(readModel.entries.map((entry) => entry.redactionSafePayloadFields.join('|')))
      .size,
    providerDeliveryClaimed: countTrue(readModel.entries, 'providerDeliveryClaimed'),
    stealthBehaviorClaimed: countTrue(readModel.entries, 'stealthBehaviorClaimed'),
    privilegeEscalationClaimed: countTrue(readModel.entries, 'privilegeEscalationClaimed'),
    hiddenPersistenceClaimed: countTrue(readModel.entries, 'hiddenPersistenceClaimed'),
    blocksAdminRemovalClaimed: countTrue(readModel.entries, 'blocksAdminRemovalClaimed'),
    rawChildDataIncluded: countTrue(readModel.entries, 'rawChildDataIncluded'),
    rawEvidencePayloadIncluded: countTrue(readModel.entries, 'rawEvidencePayloadIncluded'),
    rawUrlsIncluded: countTrue(readModel.entries, 'rawUrlsIncluded'),
    screenshotsIncluded: countTrue(readModel.entries, 'screenshotsIncluded'),
    commandLinesIncluded: countTrue(readModel.entries, 'commandLinesIncluded'),
    privatePathsIncluded: countTrue(readModel.entries, 'privatePathsIncluded'),
    messageContentsIncluded: countTrue(readModel.entries, 'messageContentsIncluded'),
  };
}

function assertReadModel(readModel, summary) {
  assertEqual(readModel.readModelId, 'tamper-integrity-audit-contract-proof', 'read model id');
  assertEqual(summary.entries, 8, 'entry count');
  for (const signal of [
    'heartbeat-stale',
    'heartbeat-offline',
    'permission-loss',
    'service-stopped',
    'agent-removed',
    'uninstall-detected',
    'tamper-manual-required',
    'admin-removal-flow',
  ]) {
    assertEqual(summary.bySignalKind[signal], 1, `${signal} count`);
  }
  assertEqual(summary.byHeartbeatState.stale, 1, 'stale heartbeat count');
  assertEqual(summary.byHeartbeatState.offline, 1, 'offline heartbeat count');
  assertEqual(summary.byPermissionState['permission-lost'], 1, 'permission-loss count');
  assertEqual(summary.byServicePresenceState.stopped, 1, 'stopped count');
  assertEqual(summary.byServicePresenceState.removed, 1, 'removed count');
  assertEqual(summary.byUninstallState.detected, 1, 'uninstall detected count');
  assertEqual(summary.byTamperState['manual-required'], 1, 'tamper manual-required count');
  assertEqual(summary.redactionSafePayloadFieldSets, 1, 'redaction-safe field set count');
  for (const claim of [
    'providerDeliveryClaimed',
    'stealthBehaviorClaimed',
    'privilegeEscalationClaimed',
    'hiddenPersistenceClaimed',
    'blocksAdminRemovalClaimed',
    'rawChildDataIncluded',
    'rawEvidencePayloadIncluded',
    'rawUrlsIncluded',
    'screenshotsIncluded',
    'commandLinesIncluded',
    'privatePathsIncluded',
    'messageContentsIncluded',
  ]) {
    assertEqual(summary[claim], 0, `${claim} count`);
  }

  const uninstall = entryFor(readModel, 'tamper-audit-uninstall-detected');
  assertArrayEqual(
    uninstall.adminRemovalFlowRefs,
    ['documented-parent-admin-removal-flow-ref'],
    'uninstall admin refs'
  );
  assertArrayEqual(
    uninstall.manualProofRequirements,
    ['platform uninstall artifact before removal detection can be claimed'],
    'uninstall proof requirements'
  );
  const tamper = entryFor(readModel, 'tamper-audit-manual-required');
  assertArrayEqual(
    tamper.manualProofRequirements,
    ['security product review before anti-tamper behavior can be claimed'],
    'tamper proof requirements'
  );

  proofLabels.push('tamper-integrity-audit.signal-coverage');
  proofLabels.push('tamper-integrity-audit.redaction-safe-fields');
  proofLabels.push('tamper-integrity-audit.admin-removal-flow-refs');
  proofLabels.push('tamper-integrity-audit.no-provider-delivery-claim');
  proofLabels.push('tamper-integrity-audit.no-stealth-or-privilege-claim');
  proofLabels.push('tamper-integrity-audit.no-raw-child-data-fields');
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`))));
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

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function countTrue(entries, key) {
  return entries.filter((entry) => entry[key] === true).length;
}

function entryFor(readModel, auditEntryId) {
  const entry = readModel.entries.find((candidate) => candidate.auditEntryId === auditEntryId);
  if (entry === undefined) {
    throw new Error(`missing tamper integrity audit entry: ${auditEntryId}`);
  }
  return entry;
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertArrayEqual(actual, expected, label) {
  if (JSON.stringify(actual) !== JSON.stringify(expected)) {
    throw new Error(`${label}: expected ${JSON.stringify(expected)}, received ${JSON.stringify(actual)}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
