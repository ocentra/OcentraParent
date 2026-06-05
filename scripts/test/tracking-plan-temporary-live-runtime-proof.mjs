import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const workpackId = '28-temporary-live-tracking-mode';
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', workpackId);
const testResultRoot = join(repoRoot, 'test-results', 'tracking-plan-temporary-live-runtime-proof');
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
    'tests/tracking-temporary-live.test.ts',
  ]);

  const tracking = await import(pathToFileURL(join(repoRoot, 'packages', 'activity-domain', 'dist', 'tracking.js')));
  const proof = buildTemporaryLiveProof(tracking);
  const commit = await gitHead();
  await writeProofArtifacts({ proof, commit });

  console.log('tracking-plan-temporary-live-runtime-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofRoot)}`);
}

function buildTemporaryLiveProof(tracking) {
  const authorization = tracking.TrackingTemporaryLiveAuthorizationSchema.parse(temporaryLiveAuthorization());
  const deviceStatus = tracking.TrackingDeviceStatusEvidenceSchema.parse(deviceStatusEvidence());
  const active = tracking.evaluateTrackingTemporaryLiveRuntime({
    authorization,
    evaluatedAt: '2026-06-03T02:05:00.000Z',
    locationCapabilityStatus: 'live',
    permissionState: 'granted-foreground',
    deviceStatus,
    locationEvidenceId: 'location-evidence-1',
  });
  const expired = tracking.evaluateTrackingTemporaryLiveRuntime({
    authorization,
    evaluatedAt: '2026-06-03T03:00:00.000Z',
    locationCapabilityStatus: 'recent',
    permissionState: 'granted-foreground',
    deviceStatus,
    locationEvidenceId: 'location-evidence-1',
  });
  const stopped = tracking.evaluateTrackingTemporaryLiveRuntime({
    authorization,
    evaluatedAt: '2026-06-03T02:10:00.000Z',
    locationCapabilityStatus: 'live',
    permissionState: 'granted-foreground',
    deviceStatus,
    locationEvidenceId: 'location-evidence-1',
    parentStopRequestedAt: '2026-06-03T02:09:00.000Z',
  });
  const lowPower = tracking.evaluateTrackingTemporaryLiveRuntime({
    authorization,
    evaluatedAt: '2026-06-03T02:06:00.000Z',
    locationCapabilityStatus: 'live',
    permissionState: 'granted-foreground',
    deviceStatus: tracking.TrackingDeviceStatusEvidenceSchema.parse(lowPowerDeviceStatusEvidence()),
    locationEvidenceId: 'location-evidence-1',
  });
  const unboundedRejected = !tracking.TrackingTemporaryLiveAuthorizationSchema.safeParse({
    ...temporaryLiveAuthorization(),
    durationMinutes: 90,
    maxDurationMinutes: 60,
    expiresAt: '2026-06-03T03:30:00.000Z',
  }).success;

  return { active, expired, stopped, lowPower, unboundedRejected };
}

async function writeProofArtifacts({ proof, commit }) {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });
  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot(commit));
  await writeFile(join(proofRoot, '01-contract-proof.log'), contractProofLog());
  await writeJson(join(proofRoot, '03-runtime-location-evidence.json'), runtimeEnvelope(proof));
  await writeJson(join(proofRoot, '09-policy-alert-proof.json'), policyEnvelope(proof));
  await writeFile(join(proofRoot, '13-security-negative-proof.log'), securityProofLog());
  await writeJson(join(proofRoot, '14-retention-delete-proof.json'), retentionEnvelope(proof));
  await writeFile(join(proofRoot, '15-manual-platform-proof.md'), manualPlatformProof());
  await writeFile(join(proofRoot, '16-validation-commands.log'), validationLog());
  await writeJson(join(proofRoot, 'proof-summary.json'), proofSummary({ proof, commit }));
  await writeJson(join(testResultRoot, 'proof.json'), testResultProof({ proof, commit }));
}

function runtimeEnvelope(proof) {
  return tierEnvelope({
    currentStatus: 'simulated',
    payload: {
      active: proof.active,
      expired: proof.expired,
      stopped: proof.stopped,
      lowPower: proof.lowPower,
      unboundedAuthorizationRejected: proof.unboundedRejected,
    },
    missingProofReason:
      'P1 proof covers bounded runtime state transitions only; Android/iOS live sampling and physical-device behavior remain unclaimed.',
  });
}

function policyEnvelope(proof) {
  return tierEnvelope({
    currentStatus: 'simulated',
    payload: {
      parentAuthorizationRef: proof.active.parentAuthorizationRef,
      activeReasonCodes: proof.active.reasonCodes,
      stoppedReasonCodes: proof.stopped.reasonCodes,
      productClaimReady: proof.active.productClaimReady,
    },
    missingProofReason:
      'Parent authorization and stop audit refs are proved as local runtime state; notification delivery and emergency escalation remain unclaimed.',
  });
}

function retentionEnvelope(proof) {
  return tierEnvelope({
    currentStatus: 'simulated',
    payload: {
      retentionMode: proof.active.retentionMode,
      expiresAt: proof.active.expiresAt,
      expiredState: proof.expired.state,
      expiredStopReason: proof.expired.stopReason,
      expiredNextSampleDueAt: proof.expired.nextSampleDueAt,
    },
    missingProofReason:
      'Expiry stops local sampling in the runtime state; live-service retention settings UI and platform deletion proof remain pending.',
  });
}

function tierEnvelope({ currentStatus, payload, missingProofReason }) {
  return {
    schemaVersion: 1,
    checkedAt,
    workpackId,
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus,
    productClaimReady: false,
    missingProofReason,
    payload,
  };
}

function proofSummary({ proof, commit }) {
  return {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackId,
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus: 'simulated',
    productClaimReady: false,
    summary:
      'Temporary live tracking runtime proof covers parent authorization, bounded duration/cadence, expiry, parent stop audit state, and low-power degradation without claiming physical-device behavior.',
    proofArtifacts: proofArtifacts(),
    commands,
    productClaims: productClaims(),
    assertions: {
      activeNextSampleDueAt: proof.active.nextSampleDueAt,
      expiredStopReason: proof.expired.stopReason,
      parentStoppedCapabilityStatus: proof.stopped.capabilityStatus,
      lowPowerCapabilityStatus: proof.lowPower.capabilityStatus,
      unboundedAuthorizationRejected: proof.unboundedRejected,
    },
    missingProofReason:
      'This is P1 local contract/runtime helper proof. Android/iOS runtime sampling, provider delivery, notification delivery, child-device UI, hosted full product UI, authority, and physical-device proof remain unclaimed.',
  };
}

function testResultProof({ proof, commit }) {
  return {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackId,
    currentStatus: 'simulated',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    proofRoot: relative(repoRoot, proofRoot).replaceAll('\\', '/'),
    productClaimReady: false,
    assertions: proofSummary({ proof, commit }).assertions,
    commands,
  };
}

function sourceSnapshot(commit) {
  return [
    '# 28-temporary-live-tracking-mode Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- source: packages/activity-domain/src/tracking-temporary-live.ts',
    '- test: packages/activity-domain/tests/tracking-temporary-live.test.ts',
    '- command: npm run test:tracking-plan-temporary-live-runtime-proof',
    '',
  ].join('\n');
}

function contractProofLog() {
  return [
    `workpack=${workpackId}`,
    'activity-domain temporary live runtime test passed',
    'parent authorization ref is required by schema',
    'duration and cadence are bounded',
    'expiry, parent stop, and low-power degraded states are evaluated by real runtime helper',
    '',
  ].join('\n');
}

function securityProofLog() {
  return [
    `workpack=${workpackId}`,
    'Temporary live runtime proof requires parentAuthorizationRef and bounded duration.',
    'Runtime states keep productClaimReady=false.',
    'Remote sync remains disabled in the proof authorization.',
    'No Android/iOS physical-device, provider delivery, notification delivery, or emergency contact claim is made.',
    '',
  ].join('\n');
}

function manualPlatformProof() {
  return [
    '# WP28 Manual Platform Boundary',
    '',
    'This proof does not claim Android/iOS location sampling, background geofence behavior, provider delivery, notification delivery, child-device runtime UI, or physical-device authority.',
    'It proves only local domain/runtime state transitions for a parent-authorized temporary live tracking session.',
    '',
  ].join('\n');
}

function validationLog() {
  return commands.map((entry) => `${entry.command} => ${entry.exitCode}`).join('\n') + '\n';
}

function proofArtifacts() {
  return [
    `output/tracking-plan-proof/${workpackId}/03-runtime-location-evidence.json`,
    `output/tracking-plan-proof/${workpackId}/09-policy-alert-proof.json`,
    `output/tracking-plan-proof/${workpackId}/14-retention-delete-proof.json`,
    `output/tracking-plan-proof/${workpackId}/proof-summary.json`,
  ];
}

function productClaims() {
  return {
    temporaryLiveRuntimeStateProved: true,
    parentAuthorizationRequired: true,
    boundedDurationAndCadenceProved: true,
    expiryStopsSamplingState: true,
    lowPowerDegradedStateProved: true,
    androidIosPhysicalLocationClaimed: false,
    providerDeliveryClaimed: false,
    notificationDeliveryClaimed: false,
    childDeviceRuntimeUiClaimed: false,
    remoteSyncEnabledByDefault: false,
  };
}

function temporaryLiveAuthorization() {
  return {
    schemaVersion: 1,
    sessionId: 'temporary-live-session-1',
    childDeviceId: 'child-device-1',
    requestedAt: '2026-06-03T02:00:00.000Z',
    startsAt: '2026-06-03T02:00:00.000Z',
    expiresAt: '2026-06-03T03:00:00.000Z',
    parentAuthorizationRef: 'parent-approved-live-session',
    durationMinutes: 60,
    maxDurationMinutes: 60,
    cadenceSeconds: 60,
    retentionMode: 'delete-on-resolution',
    disclosureRequired: true,
    remoteSyncDefault: 'disabled',
    auditRefs: ['temporary-live-parent-approved'],
    evidence: [evidenceRef()],
  };
}

function deviceStatusEvidence() {
  return {
    schemaVersion: 1,
    evidenceId: 'device-status-1',
    observedAt: '2026-06-03T02:00:00.000Z',
    freshUntil: '2026-06-03T02:05:00.000Z',
    staleAt: '2026-06-03T02:15:00.000Z',
    sourceId: 'android-child-agent',
    adapterId: 'android-device-status-adapter',
    deviceId: 'child-device-1',
    sourceKind: 'android-device-status',
    capabilityStatus: 'recent',
    lastLocationEvidenceId: 'location-evidence-1',
    heartbeatStatus: 'healthy',
    battery: { percent: 64, chargingState: 'discharging', lowPowerMode: 'disabled' },
    connectivityStatus: 'online',
    pendingUploadCount: 0,
    custodyLabel: 'child-device-local',
    retentionMode: '24h',
    degradedReasons: [],
    evidence: [evidenceRef()],
  };
}

function lowPowerDeviceStatusEvidence() {
  return {
    ...deviceStatusEvidence(),
    evidenceId: 'device-status-low-power',
    capabilityStatus: 'battery-throttled',
    battery: { percent: 18, chargingState: 'discharging', lowPowerMode: 'enabled' },
    degradedReasons: ['android-low-power-mode'],
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
