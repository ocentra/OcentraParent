import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-missing-device-mode-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '29-missing-device-mode');
const screenshotDir = join(proofDir, '11-ui-snapshots');
const timestamp = '2026-06-05T15:18:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(screenshotDir, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-missing-device-mode-proof',
  'tracking-location-policy',
]);

const tracking = await importDist('tracking-location-policy.js');
const proofModule = await importDist('tracking-missing-device-mode-proof.js');
const sourceReadModel = tracking.TrackingLocationPolicyReadModelSchema.parse(sourceTrackingReadModel(tracking));
const readModel = proofModule.buildTrackingMissingDeviceModeProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-missing-device-mode-proof',
    familyId: 'family-tracking-missing-device',
    childProfileId: 'child-profile-aarav',
    deviceId: 'device-aarav-phone',
    deviceLabel: 'Aarav phone',
    platform: 'android',
    sourceTrackingReadModelRef: 'tracking-location-policy-read-model-missing-device',
    sourceContractRefs: [
      'tracking-location-policy',
      'device-location-tracking-capability-guide',
      'tracking-control-settings-inventory',
      'tracking-ui-ux-requirements-guide',
      'location-geofence-device-status',
    ],
  },
  sourceReadModel
);

const proof = {
  proofMode: 'tracking-missing-device-mode-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-missing-device-mode-proof.ts',
    test: 'packages/parent-domain/tests/tracking-missing-device-mode-proof.test.ts',
    harness: 'scripts/test/tracking-missing-device-mode-proof.mjs',
    evidence: 'test-results/tracking-missing-device-mode-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/29-missing-device-mode',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-missing-device-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-missing-device-mode-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-missing-device-mode-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function sourceTrackingReadModel(tracking) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    generatedAt: timestamp,
    rules: [],
    decisions: [],
    acknowledgements: [],
    checkInRequests: [],
    checkInResponses: [],
    aiRoutes: [],
    aiResults: [],
    alerts: [],
    escalations: [],
    temporaryLiveGrants: [],
    missingDeviceCases: [
      missingCase(tracking, {
        caseId: 'tracking-missing-device-last-known',
        state: 'last-known-only',
        lastKnownEvidenceId: 'location-evidence-last-known-stale',
        deviceStatusEvidenceId: 'device-status-offline-last-known',
        reasonCodes: ['missing-device-last-known-only'],
      }),
      missingCase(tracking, {
        caseId: 'tracking-missing-device-powered-off',
        state: 'offline',
        lastKnownEvidenceId: 'location-evidence-last-known-powered-off',
        deviceStatusEvidenceId: 'device-status-powered-off',
        reasonCodes: ['missing-device-powered-off-last-known-only'],
      }),
      missingCase(tracking, {
        caseId: 'tracking-missing-device-contact-requested',
        state: 'contact-requested',
        lastKnownEvidenceId: 'location-evidence-last-known-contact-requested',
        deviceStatusEvidenceId: 'device-status-contact-action-queued',
        reasonCodes: ['missing-device-contact-action-queued'],
      }),
      missingCase(tracking, {
        caseId: 'tracking-missing-device-manual-required',
        state: 'manual-required',
        lastKnownEvidenceId: 'location-evidence-last-known-manual-required',
        deviceStatusEvidenceId: 'device-status-platform-proof-required',
        reasonCodes: ['missing-device-platform-proof-required'],
      }),
    ],
    platformProofRoutes: [],
  };
}

function missingCase(tracking, input) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    caseId: input.caseId,
    openedAt: timestamp,
    state: input.state,
    lastKnownEvidence: {
      evidenceReferenceId: input.lastKnownEvidenceId,
      kind: 'journal-event',
      observedAt: '2026-06-05T15:12:00.000Z',
    },
    deviceStatusEvidence: {
      evidenceReferenceId: input.deviceStatusEvidenceId,
      kind: 'query-store-summary',
      observedAt: '2026-06-05T15:13:00.000Z',
    },
    contactActionRefs: [`tracking-contact-action-${input.caseId}`],
    reasonCodes: input.reasonCodes,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    lastKnownOnlyCount: readModel.lastKnownOnlyCount,
    offlineCount: readModel.offlineCount,
    contactRequestedCount: readModel.contactRequestedCount,
    manualRequiredCount: readModel.manualRequiredCount,
    runtimeEvidenceRefs: readModel.runtimeEvidenceRefs.length,
    currentLocationCopyAllowedCount: readModel.rows.filter((row) => row.uiState.currentLocationCopyAllowed).length,
    primaryBadges: countBy(readModel.rows.map((row) => row.uiState.primaryBadge)),
    contactStates: countBy(readModel.rows.map((row) => row.statusSnapshot.contactState)),
  };
}

function nonClaims(readModel) {
  return {
    currentLocationRuntimeClaimed: readModel.currentLocationRuntimeClaimed,
    liveTrackingRuntimeClaimed: readModel.liveTrackingRuntimeClaimed,
    poweredOffDeviceTrackingClaimed: readModel.poweredOffDeviceTrackingClaimed,
    remoteSyncRuntimeClaimed: readModel.remoteSyncRuntimeClaimed,
    providerDeliveryClaimed: readModel.providerDeliveryClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    portalRuntimeUiClaimed: readModel.portalRuntimeUiClaimed,
    osLostModeApiClaimed: readModel.osLostModeApiClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 4 ||
    proof.summary.lastKnownOnlyCount !== 1 ||
    proof.summary.offlineCount !== 1 ||
    proof.summary.contactRequestedCount !== 1 ||
    proof.summary.manualRequiredCount !== 1 ||
    proof.summary.currentLocationCopyAllowedCount !== 0
  ) {
    throw new Error(`Unexpected tracking missing-device summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Tracking missing-device proof overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP29 Missing-Device Mode Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-domain missing-device read model for last-known location, battery, connectivity, stale/offline, pending upload, contact actions, and UI state tokens.',
      '- Source inspected: device location tracking capability guide, raw tracking control settings inventory, UI/UX requirements guide, location/geofence feature doc, and WP29 checklist.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '03-runtime-location-evidence.json'), {
    rows: proof.readModel.rows.map((row) => ({
      caseId: row.caseId,
      state: row.state,
      lastKnownEvidenceRef: row.lastKnownEvidenceRef,
      deviceStatusEvidenceRef: row.deviceStatusEvidenceRef,
      currentLocationClaimed: row.currentLocationClaimed,
      poweredOffTrackingClaimed: row.poweredOffTrackingClaimed,
      remoteSyncRequired: row.remoteSyncRequired,
    })),
    runtimeEvidenceRefs: proof.readModel.runtimeEvidenceRefs,
  });
  await writeJson(join(path, '04-device-status-proof.json'), {
    rows: proof.readModel.rows.map((row) => ({
      caseId: row.caseId,
      contactState: row.statusSnapshot.contactState,
      lastContactAt: row.statusSnapshot.lastContactAt,
      batteryPercent: row.statusSnapshot.batteryPercent,
      pendingUploadCount: row.statusSnapshot.pendingUploadCount,
      batteryEvidenceRef: row.statusSnapshot.batteryEvidenceRef,
      connectivityEvidenceRef: row.statusSnapshot.connectivityEvidenceRef,
      pendingUploadEvidenceRef: row.statusSnapshot.pendingUploadEvidenceRef,
    })),
  });
  await writeJson(join(screenshotDir, 'missing-device-ui-state-matrix.json'), {
    rows: proof.readModel.rows.map((row) => ({
      caseId: row.caseId,
      primaryBadge: row.uiState.primaryBadge,
      secondaryBadges: row.uiState.secondaryBadges,
      headlineToken: row.uiState.headlineToken,
      detailToken: row.uiState.detailToken,
      evidenceDrawerRefs: row.uiState.evidenceDrawerRefs,
      actionKinds: row.uiState.actionKinds,
      accessibilityStateToken: row.uiState.accessibilityStateToken,
      currentLocationCopyAllowed: row.uiState.currentLocationCopyAllowed,
    })),
  });
  await writeFile(
    join(path, '12-playwright-proof.log'),
    [
      'Playwright/UI proof:',
      '',
      '- No browser surface was changed or claimed in this workpack.',
      '- Contract-backed UI state matrix written to 11-ui-snapshots/missing-device-ui-state-matrix.json.',
      '- Parent portal runtime screenshots remain gated by WP30 hosted UI work; this WP29 proof only proves parent-domain UI state tokens and no-current-location copy gates.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Current location, live tracking runtime, powered-off device tracking, remote sync runtime, provider delivery, physical-device proof, portal runtime UI, and OS lost-mode API integrations remain false.',
      '- Powered-off/offline rows show last-known and device-contact evidence instead of claiming live location.',
      '- Parent actions are explicit audit/action references and do not dispatch child-device commands in this proof.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(path, 'README.md'),
    '# WP29 Missing-Device Mode Proof\n\nThis proof pack records parent-domain missing-device mode rows for last-known location, offline/powered-off contact state, battery/connectivity/pending-upload evidence, parent contact actions, and UI state tokens without claiming current location, live tracking runtime, remote sync runtime, provider delivery, portal runtime UI, physical-device proof, or OS lost-mode APIs.\n',
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
