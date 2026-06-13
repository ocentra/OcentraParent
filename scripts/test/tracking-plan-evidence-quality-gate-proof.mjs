import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const workpack33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const proofResultDir = path.join(repoRoot, 'test-results', 'tracking-plan-evidence-quality-gate-proof');
const proofPath = path.join(proofResultDir, 'proof.json');
const outputProofPath = path.join(workpack33, '19-evidence-quality-gate-proof.json');
const validationLogPath = path.join(workpack33, '20-evidence-quality-gate-validation.log');
const commands = [];

async function main() {
  await mkdir(workpack33, { recursive: true });
  await mkdir(proofResultDir, { recursive: true });
  await runNpm(['--workspace', '@ocentra-parent/tracking-domain', 'run', 'build']);
  await runNpm(['--workspace', '@ocentra-parent/parent-domain', 'run', 'build']);
  await runNpm(['--workspace', '@ocentra-parent/portal-domain', 'run', 'build']);
  await runNpm([
    '--workspace',
    '@ocentra-parent/tracking-domain',
    'run',
    'test',
    '--',
    'tracking-evidence-quality-gate',
  ]);
  await runNpm(['--workspace', '@ocentra-parent/parent-domain', 'run', 'test', '--', 'tracking-location-policy']);
  await runNpm(['--workspace', '@ocentra-parent/portal', 'run', 'test', '--', 'tracking-status-panel']);

  const activity = await import('@ocentra-parent/tracking-domain/tracking');
  const policy = await import('@ocentra-parent/tracking-domain/tracking-location-policy');
  const checkedAt = new Date().toISOString();
  const readModel = activity.TrackingReadModelSchema.parse(trackingReadModelSample());
  const retentionDeleteProof = activity.applyTrackingRetentionDelete({
    readModel,
    generatedAt: '2026-06-05T04:45:00.000Z',
    deletedEvidenceIds: ['location-evidence-quality-1'],
  });
  const retentionExportProof = activity.applyTrackingRetentionExport({
    readModel,
    generatedAt: '2026-06-05T04:46:00.000Z',
    policy: activity.TrackingRetentionPolicySchema.parse(RetentionPolicy),
  });
  const activityGate = activity.evaluateTrackingEvidenceQualityGate({
    readModel,
    retentionDeleteProof,
    retentionExportProof,
  });
  const parentGate = evaluateParentPolicyGates(policy);

  if (!activityGate.passed || !parentGate.passed) {
    throw new Error(`tracking evidence quality gate failed: ${JSON.stringify({ activityGate, parentGate })}`);
  }

  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit: gitHead(),
    workpackIds: ['33-proof-gates-fixtures-rollout-and-pr-gate'],
    proofMode: 'tracking-plan-evidence-quality-gate-proof',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus: 'proved-locally',
    productClaimReady: false,
    gates: {
      locationDerivedUiEvidenceRefs: {
        status: 'passed',
        evidenceReferenceCount: activityGate.locationEvidenceReferenceCount,
        portalValidation: 'npm --workspace @ocentra-parent/portal run test -- tracking-status-panel',
      },
      geofenceTransitionRuleAndSourceRefs: {
        status: 'passed',
        transitionCount: activityGate.geofenceTransitionCount,
      },
      nearbyPlaceProviderContext: {
        status: 'passed',
        resultCount: activityGate.nearbyPlaceResultCount,
        carriedFields: [
          'providerKind',
          'queryRadiusMeters',
          'distanceMeters',
          'category',
          'confidence',
          'ambiguityState',
        ],
      },
      trackingAiSourceRefsAndNoFinalAction: parentGate.aiGate,
      alertPolicyDecisionRefs: parentGate.alertGate,
      retentionDeleteExportBeforeAfterProof: {
        status: 'passed',
        deleteBeforeRows: activityGate.retentionDeleteBeforeLocationRows,
        deleteAfterRows: activityGate.retentionDeleteAfterLocationRows,
        exportSourceRows: activityGate.retentionExportSourceLocationRows,
        exportedRows: activityGate.retentionExportedLocationRows,
      },
    },
    validationCommands: commands,
    artifacts: {
      proof: relativePath(proofPath),
      workpackProof: relativePath(outputProofPath),
      validationLog: relativePath(validationLogPath),
    },
    nonClaims: [
      'This proof does not claim physical Android, iOS, WSL, LAN, or production tracking behavior.',
      'This proof does not claim provider delivery, emergency-contact delivery, or device runtime execution.',
      'This proof keeps AI evidence-only and rejects direct alert or final-authority claims.',
      'This proof uses schemas, parser-backed fixtures, portal tests, and retention helpers; full hosted CI and device screenshots remain separate gates.',
    ],
    remainingGapsBeforeProductReady: [
      'Full live parent and child tracking UI screenshots beyond current hosted parent route remain pending.',
      'Physical device background/foreground location and geofence proof remain pending.',
      'Provider notification delivery, emergency escalation delivery, and production pilot evidence remain pending.',
    ],
  };
  const proofContent = `${JSON.stringify(proof, null, 2)}\n`;
  await writeFile(proofPath, proofContent);
  await writeFile(outputProofPath, proofContent);
  await writeFile(
    validationLogPath,
    `${commands.map(({ command, exitCode }) => `${command} # exit ${exitCode}`).join('\n')}\n`
  );

  console.log('tracking-plan-evidence-quality-gate-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

function evaluateParentPolicyGates(policyDomain) {
  const evidenceTrace = {
    evidenceReferenceId: 'location-evidence-quality-1',
    kind: 'journal-event',
    observedAt: '2026-06-05T04:40:00.000Z',
  };
  const decision = policyDomain.TrackingPolicyDecisionSchema.parse({
    schemaVersion: policyDomain.TrackingPolicySchemaVersion,
    decisionId: 'tracking-quality-decision-1',
    decidedAt: '2026-06-05T04:41:00.000Z',
    ruleId: 'quality-home-arrival-rule',
    action: 'notify-parent',
    dryRun: false,
    evidenceReferences: [evidenceTrace],
    aiAnalysisId: 'tracking-quality-ai-1',
    alertIntentId: 'tracking-quality-alert-1',
    reasonCodes: ['policy-rule-matched'],
    auditRefs: ['quality-gate-decision'],
  });
  const aiInput = policyDomain.TrackingLocationAiAnalysisInputSchema.parse({
    schemaVersion: policyDomain.TrackingPolicySchemaVersion,
    analysisId: 'tracking-quality-ai-1',
    requestedAt: '2026-06-05T04:40:30.000Z',
    evidenceReferences: [evidenceTrace],
    policyVersion: 'tracking-policy-v1',
    providerRouteId: 'tracking-quality-ai-route-1',
  });
  const aiResult = policyDomain.TrackingLocationAiAnalysisResultSchema.parse({
    schemaVersion: policyDomain.TrackingPolicySchemaVersion,
    analysisId: 'tracking-quality-ai-1',
    completedAt: '2026-06-05T04:41:00.000Z',
    riskLevel: 'low',
    confidence: 0.77,
    providerRouteId: 'tracking-quality-ai-route-1',
    evidenceReferences: [evidenceTrace],
    reasonCodes: ['where-expected'],
    canTriggerAlertDirectly: false,
    isFinalAuthority: false,
  });
  const alert = policyDomain.TrackingAlertIntentSchema.parse({
    schemaVersion: policyDomain.TrackingPolicySchemaVersion,
    alertId: 'tracking-quality-alert-1',
    createdAt: '2026-06-05T04:41:10.000Z',
    severity: 'info',
    policyDecisionId: decision.decisionId,
    evidenceReferences: [evidenceTrace],
    sensitiveDetailMode: 'minimal-provider-body',
    notificationStatusRefs: ['notification-intent-queued'],
    acknowledgementId: null,
    reasonCodes: ['parent-notification-intent-created'],
  });
  const noEvidenceAiInputRejected = !policyDomain.TrackingLocationAiAnalysisInputSchema.safeParse({
    ...aiInput,
    evidenceReferences: [],
  }).success;
  const directAiActionRejected = !policyDomain.TrackingLocationAiAnalysisResultSchema.safeParse({
    ...aiResult,
    canTriggerAlertDirectly: true,
  }).success;
  const noEvidencePolicyDecisionRejected = !policyDomain.TrackingPolicyDecisionSchema.safeParse({
    ...decision,
    evidenceReferences: [],
  }).success;

  return {
    passed:
      aiInput.evidenceReferences.length > 0 &&
      aiResult.evidenceReferences.length > 0 &&
      !aiResult.canTriggerAlertDirectly &&
      !aiResult.isFinalAuthority &&
      alert.policyDecisionId === decision.decisionId &&
      alert.evidenceReferences.length > 0 &&
      noEvidenceAiInputRejected &&
      directAiActionRejected &&
      noEvidencePolicyDecisionRejected,
    aiGate: {
      status: 'passed',
      inputEvidenceRefs: aiInput.evidenceReferences.length,
      resultEvidenceRefs: aiResult.evidenceReferences.length,
      canTriggerAlertDirectly: aiResult.canTriggerAlertDirectly,
      isFinalAuthority: aiResult.isFinalAuthority,
      noEvidenceAiInputRejected,
      directAiActionRejected,
    },
    alertGate: {
      status: 'passed',
      policyDecisionId: alert.policyDecisionId,
      evidenceRefs: alert.evidenceReferences.length,
      noEvidencePolicyDecisionRejected,
    },
  };
}

function trackingReadModelSample() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-05T04:43:00.000Z',
    custodyLabel: 'child-device-local',
    capabilityStatus: 'recent',
    limit: 25,
    returned: 3,
    locationRows: [LocationEvidence],
    deviceStatusRows: [DeviceStatusEvidence],
    capabilityRows: [CapabilityStatus],
    geofenceTransitions: [GeofenceTransition],
    expectedPlaceDecisions: [ExpectedPlaceDecision],
    nearbyPlaceRows: [NearbyPlaceEvidence],
    retentionPolicies: [RetentionPolicy],
    timeline: [TrackingTimelineRow],
  };
}

const EvidenceRef = {
  evidenceId: 'tracking-quality-journal-row-1',
  kind: 'journal-entry',
  digest: 'sha256:tracking-quality-gate-proof',
  uri: null,
};
const LocationEvidence = {
  schemaVersion: 1,
  evidenceId: 'location-evidence-quality-1',
  observedAt: '2026-06-05T04:40:00.000Z',
  freshUntil: '2026-06-05T04:45:00.000Z',
  staleAt: '2026-06-05T04:55:00.000Z',
  sourceId: 'android-child-agent-fixture',
  adapterId: 'android-fused-location-adapter-fixture',
  deviceId: 'child-device-quality-1',
  sourceKind: 'android-fused-location',
  capabilityStatus: 'recent',
  permissionState: 'granted-foreground',
  coordinate: {
    latitude: 43.6532,
    longitude: -79.3832,
  },
  accuracyMeters: 22,
  hint: {
    quality: 'gps',
    coarseRadiusMeters: null,
    label: null,
  },
  confidence: 0.92,
  custodyLabel: 'child-device-local',
  retentionMode: '24h',
  reasonCodes: ['foreground-location-sample'],
  evidence: [EvidenceRef],
};
const DeviceStatusEvidence = {
  schemaVersion: 1,
  evidenceId: 'device-status-quality-1',
  observedAt: '2026-06-05T04:40:00.000Z',
  freshUntil: '2026-06-05T04:45:00.000Z',
  staleAt: '2026-06-05T04:55:00.000Z',
  sourceId: 'android-child-agent-fixture',
  adapterId: 'android-device-status-adapter-fixture',
  deviceId: 'child-device-quality-1',
  sourceKind: 'android-device-status',
  capabilityStatus: 'recent',
  lastLocationEvidenceId: 'location-evidence-quality-1',
  heartbeatStatus: 'healthy',
  battery: {
    percent: 64,
    chargingState: 'discharging',
    lowPowerMode: 'disabled',
  },
  connectivityStatus: 'online',
  pendingUploadCount: 0,
  custodyLabel: 'child-device-local',
  retentionMode: '24h',
  degradedReasons: [],
  evidence: [EvidenceRef],
};
const CapabilityStatus = {
  schemaVersion: 1,
  checkedAt: '2026-06-05T04:40:00.000Z',
  platform: 'android',
  foregroundLocation: 'manual-required',
  backgroundLocation: 'manual-required',
  geofenceTransitions: 'manual-required',
  deviceStatus: 'manual-required',
  permissionState: 'manual-required',
  manualActionRequired: true,
  sourceId: 'tracking-plan-proof',
  adapterId: 'tracking-contract-proof',
  reasonCodes: ['real-device-permission-proof-required'],
  auditRefs: ['output/tracking-plan-proof/manual-platform-proof'],
};
const RetentionPolicy = {
  schemaVersion: 1,
  policyId: 'tracking-retention-quality-local',
  mode: '24h',
  custodyLabel: 'child-device-local',
  customRetentionHours: null,
  deleteOnResolution: false,
  exportAllowed: true,
  remoteSyncDefault: 'disabled',
  auditRefs: ['tracking-retention-local-first'],
};
const GeofenceTransition = {
  schemaVersion: 1,
  transitionId: 'quality-home-enter-transition',
  observedAt: '2026-06-05T04:41:00.000Z',
  ruleId: 'quality-home-arrival-rule',
  geofenceId: 'quality-home-circle',
  locationEvidenceId: 'location-evidence-quality-1',
  transition: 'enter',
  capabilityStatus: 'recent',
  distanceMeters: 42,
  reasonCodes: ['inside-circle-with-accuracy'],
  evidence: [EvidenceRef],
};
const ExpectedPlaceDecision = {
  schemaVersion: 1,
  decisionId: 'quality-expected-place-decision-1',
  observedAt: '2026-06-05T04:41:00.000Z',
  scheduleId: 'quality-school-night-schedule',
  locationEvidenceId: 'location-evidence-quality-1',
  outcome: 'where-expected',
  reasonCodes: ['home-window-active'],
  evidence: [EvidenceRef],
};
const NearbyPlaceEvidence = {
  schemaVersion: 1,
  evidenceId: 'nearby-place-quality-1',
  observedAt: '2026-06-05T04:41:00.000Z',
  locationEvidenceId: 'location-evidence-quality-1',
  providerKind: 'parent-defined',
  providerRef: 'parent-place-db',
  queryRadiusMeters: 250,
  distanceMeters: 42,
  category: 'home',
  confidence: 0.91,
  ambiguityState: 'clear',
  reasonCodes: ['parent-defined-place-match'],
  evidence: [EvidenceRef],
};
const TrackingTimelineRow = {
  rowId: 'location-evidence-quality-1',
  kind: 'location',
  observedAt: '2026-06-05T04:40:00.000Z',
  capabilityStatus: 'recent',
  reasonCodes: ['foreground-location-sample'],
  evidence: [EvidenceRef],
};

await main();

function runNpm(args) {
  if (process.platform === 'win32') {
    return runCommand(...npmCommand([...args]));
  }
  return runCommand('npm', args);
}

function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8' });
  process.stdout.write(result.stdout ?? '');
  process.stderr.write(result.stderr ?? '');
  const exitCode = result.status ?? 1;
  commands.push({ command: commandLine, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${commandLine} exited with ${exitCode}`);
  }
}

function gitHead() {
  const result = spawnSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' });
  if (result.status !== 0) {
    throw new Error('git rev-parse HEAD failed');
  }
  return result.stdout.trim();
}

function relativePath(value) {
  return path.relative(repoRoot, value).replace(/\\/gu, '/');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
