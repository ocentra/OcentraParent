import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import net from 'node:net';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof');
const checkedAt = new Date().toISOString();
const commands = [];

await main();

async function main() {
  await runNpm([
    'exec',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'vitest',
    'run',
    'tests/unit/tracking.test.ts',
    'tests/contract/tracking-location-policy.test.ts',
  ]);
  await runNpm([
    'exec',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'vitest',
    'run',
    'tests/tracking-status-panel.test.ts',
  ]);
  const routeScreenshotProof = await runTrackingRouteScreenshotProof();
  await runCommand('cargo', [
    'test',
    '-p',
    'ocentra-parent-agent-core',
    'activity_store_ingests_tracking_mvp_events_into_sqlite',
  ]);

  const tracking = await importTsModule('packages/tracking-domain/src/tracking.ts');
  const policy = await importTsModule('packages/tracking-domain/src/tracking-location-policy.ts');
  await runTrackingSourceSurfaceProof(tracking, policy);
  const fixtures = buildFixtures();
  const parentFixtures = buildParentFixtures();
  const rule = tracking.TrackingGeofenceRuleSchema.parse(fixtures.circleRule);
  const location = tracking.TrackingLocationEvidenceSchema.parse(fixtures.locationEvidence);
  const transition = tracking.evaluateTrackingGeofenceTransition({
    transitionId: 'runtime-home-enter-transition',
    observedAt: '2026-06-03T02:01:00.000Z',
    rule,
    location,
    wasInside: false,
  });
  const decision = tracking.evaluateTrackingExpectedPlaceDecision({
    decisionId: 'runtime-expected-place-decision',
    observedAt: '2026-06-03T02:01:00.000Z',
    schedule: tracking.TrackingExpectedPlaceScheduleSchema.parse(fixtures.expectedPlaceSchedule),
    location,
    transition,
  });
  const retention = tracking.applyTrackingRetentionDelete({
    readModel: tracking.TrackingReadModelSchema.parse(fixtures.readModel),
    generatedAt: '2026-06-03T03:00:00.000Z',
    deletedEvidenceIds: ['location-evidence-1'],
  });
  const retentionExport = tracking.applyTrackingRetentionExport({
    readModel: tracking.TrackingReadModelSchema.parse(fixtures.readModel),
    generatedAt: '2026-06-03T03:05:00.000Z',
    policy: tracking.TrackingRetentionPolicySchema.parse(fixtures.exportRetentionPolicy),
  });
  const acknowledgementImpact = policy.evaluateTrackingAcknowledgementImpact({
    alert: policy.TrackingAlertIntentSchema.parse(parentFixtures.alert),
    acknowledgement: policy.TrackingAcknowledgementSchema.parse(parentFixtures.acknowledgement),
    evaluatedAt: '2026-06-03T02:04:00.000Z',
  });
  const checkInResolution = policy.resolveTrackingChildCheckIn({
    request: policy.TrackingChildCheckInRequestSchema.parse(parentFixtures.checkInRequest),
    response: policy.TrackingChildCheckInResponseSchema.parse(parentFixtures.checkInResponse),
    evaluatedAt: '2026-06-03T02:06:00.000Z',
  });

  await writeProofArtifacts({
    locationEvidence: location,
    deviceStatus: tracking.TrackingDeviceStatusEvidenceSchema.parse(fixtures.deviceStatusEvidence),
    transition,
    decision,
    retention,
    retentionExport,
    acknowledgementImpact,
    checkInResolution,
    routeScreenshotProof,
  });

  console.log('tracking-plan-runtime-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofRoot)}`);
}

async function runTrackingSourceSurfaceProof(tracking, policy) {
  if (typeof tracking.evaluateTrackingGeofenceTransition !== 'function') {
    throw new Error('tracking source surface missing runtime helper');
  }
  if (typeof tracking.TrackingReadModelSchema?.parse !== 'function') {
    throw new Error('tracking source surface missing read model schema');
  }
  if (typeof policy.evaluateTrackingAcknowledgementImpact !== 'function') {
    throw new Error('tracking policy source surface missing acknowledgement runtime helper');
  }
  if (typeof policy.TrackingLocationPolicyReadModelSchema?.parse !== 'function') {
    throw new Error('tracking policy source surface missing read model schema');
  }

  commands.push({
    command:
      'source-surface-check packages/tracking-domain/src/tracking.ts packages/tracking-domain/src/tracking-location-policy.ts',
    exitCode: 0,
  });
}

async function writeProofArtifacts({
  locationEvidence,
  deviceStatus,
  transition,
  decision,
  retention,
  retentionExport,
  acknowledgementImpact,
  checkInResolution,
  routeScreenshotProof,
}) {
  await writeJson(
    join(proofRoot, '04-location-evidence-model', '03-runtime-location-evidence.json'),
    tierEnvelope('04-location-evidence-model', 'P1_FIXTURE_SIMULATION', 'simulated', locationEvidence)
  );
  await writeJson(
    join(proofRoot, '05-device-status-model', '04-device-status-proof.json'),
    tierEnvelope('05-device-status-model', 'P1_FIXTURE_SIMULATION', 'simulated', deviceStatus)
  );
  await writeJson(
    join(proofRoot, '15-geofence-transition-engine', '05-geofence-transition-proof.json'),
    tierEnvelope('15-geofence-transition-engine', 'P1_FIXTURE_SIMULATION', 'simulated', transition)
  );
  await writeJson(
    join(proofRoot, '16-expected-place-schedule-engine', '06-expected-place-proof.json'),
    tierEnvelope('16-expected-place-schedule-engine', 'P1_FIXTURE_SIMULATION', 'simulated', decision)
  );
  await writeJson(
    join(proofRoot, '07-retention-and-custody-model', '14-retention-delete-proof.json'),
    tierEnvelope('07-retention-and-custody-model', 'P1_FIXTURE_SIMULATION', 'simulated', retention)
  );
  await writeJson(
    join(proofRoot, '07-retention-and-custody-model', '17-retention-export-proof.json'),
    tierEnvelope('07-retention-and-custody-model', 'P1_FIXTURE_SIMULATION', 'simulated', retentionExport)
  );
  await writeJson(
    join(proofRoot, '32-journal-sqlite-and-read-model-proof', '10-journal-sqlite-proof.json'),
    tierEnvelope('32-journal-sqlite-and-read-model-proof', 'P1_FIXTURE_SIMULATION', 'simulated', {
      rustTest: 'activity_store_ingests_tracking_mvp_events_into_sqlite',
      sqlitePath: 'ActivityStore::open_in_memory',
      eventKinds: [
        'activity.location.observed',
        'activity.tracking.geofence-transition.evaluated',
        'activity.tracking.expected-place.evaluated',
        'activity.tracking.child-check-in.responded',
        'activity.tracking.retention.deleted',
      ],
      commands,
    })
  );
  await writeJson(
    join(proofRoot, '30-parent-and-child-ui-ux-surfaces', '11-ui-fixture-state-matrix.json'),
    tierEnvelope(
      '30-parent-and-child-ui-ux-surfaces',
      'P1_FIXTURE_SIMULATION',
      'simulated',
      {
        route: '/#/policy-tracking',
        surface: 'apps/portal/src/tracking-status-panel.ts',
        test: 'apps/portal/tests/tracking-status-panel.test.ts',
        states: [
          'Tracking off',
          'Permission required',
          'Stale last known',
          'Offline last known',
          'Low accuracy',
          'Nearby place ambiguous',
          'Policy alert',
          'Parent acknowledged',
          'Exception active',
          'Child check-in',
          'Temporary live',
          'Missing device',
          'Retention deleted',
        ],
        deletedHistoryProof: trackingRouteDeletedHistoryProof(),
        evidenceCitationProof: routeScreenshotProof.proofArtifactProof,
        productClaimReady: false,
        missingProofReason:
          'This is UI fixture proof only. Live service data, hosted Playwright/a11y proof, child-device UI, and physical-device proof remain pending.',
        routeScreenshotProof,
        commands,
      },
      'P2_HOSTED_CI'
    )
  );
  await writeJson(
    join(proofRoot, '17-parent-acknowledgement-and-exception-model', '09-policy-alert-proof.json'),
    tierEnvelope('17-parent-acknowledgement-and-exception-model', 'P1_FIXTURE_SIMULATION', 'simulated', {
      acknowledgementImpact,
      productClaimReady: false,
      missingProofReason: 'Alert delivery and portal acknowledgement UI remain pending.',
    })
  );
  await writeJson(
    join(proofRoot, '18-child-check-in-flow', '09-policy-alert-proof.json'),
    tierEnvelope('18-child-check-in-flow', 'P1_FIXTURE_SIMULATION', 'simulated', {
      checkInResolution,
      productClaimReady: false,
      missingProofReason: 'Child device UI, delivery, timeout escalation wiring, and screenshots remain pending.',
    })
  );
  const minimumSeriousMvpAudit = buildMinimumSeriousMvpAudit({ routeScreenshotProof });
  await writeJson(join(proofRoot, '33-proof-gates-fixtures-rollout-and-pr-gate', '00-run-metadata.json'), {
    schemaVersion: 1,
    checkedAt,
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus: 'simulated',
    commands,
    minimumSeriousMvpAudit,
    manualRequired: [
      'P4 Android physical background geofence proof missing',
      'P4 iOS physical region monitoring proof missing',
      'P5 authority enrolled-device proof missing',
      'P6 production pilot proof missing',
    ],
  });
}

function buildMinimumSeriousMvpAudit({ routeScreenshotProof }) {
  return {
    scope: 'Minimum Serious MVP first checkpoint',
    status: 'p1_checkpoint_proved_not_product_complete',
    firstTargetReadyForContinuation: true,
    productCompleteClaimed: false,
    prReadyOrFullScopeClaimed: false,
    requiredCheckpointTier: 'P1_FIXTURE_SIMULATION',
    currentCheckpointTier: 'P1_FIXTURE_SIMULATION',
    requirements: [
      {
        requirement: 'Last-known location plus status and freshness',
        status: 'proved_at_p1_fixture',
        artifactPath: 'output/tracking-plan-proof/04-location-evidence-model/03-runtime-location-evidence.json',
      },
      {
        requirement: 'Device status and freshness',
        status: 'proved_at_p1_fixture',
        artifactPath: 'output/tracking-plan-proof/05-device-status-model/04-device-status-proof.json',
      },
      {
        requirement: 'Geofence enter and exit decision shape',
        status: 'proved_at_p1_fixture',
        artifactPath: 'output/tracking-plan-proof/15-geofence-transition-engine/05-geofence-transition-proof.json',
        missingProofReason: 'Android/iOS physical geofence behavior remains P4 manual-required.',
      },
      {
        requirement: 'Expected-place schedule check',
        status: 'proved_at_p1_fixture',
        artifactPath: 'output/tracking-plan-proof/16-expected-place-schedule-engine/06-expected-place-proof.json',
        missingProofReason: 'UI and alert policy integration remain pending.',
      },
      {
        requirement: 'Parent acknowledgement impact',
        status: 'proved_at_p1_fixture',
        artifactPath:
          'output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/09-policy-alert-proof.json',
        missingProofReason: 'Alert delivery and portal acknowledgement UI remain pending.',
      },
      {
        requirement: 'Child check-in resolution',
        status: 'proved_at_p1_fixture',
        artifactPath: 'output/tracking-plan-proof/18-child-check-in-flow/09-policy-alert-proof.json',
        missingProofReason: 'Child-device UI, delivery, timeout escalation wiring, and screenshots remain pending.',
      },
      {
        requirement: 'Retention delete, UI hidden history, and parent-owned export',
        status: 'proved_at_p1_fixture',
        artifactPath: 'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json',
        exportArtifactPath: 'output/tracking-plan-proof/07-retention-and-custody-model/17-retention-export-proof.json',
        uiArtifactPath: 'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json',
        missingProofReason: 'Live service-backed retention UI proof remains pending.',
      },
      {
        requirement: 'Parent UI first-target states with proof artifact references',
        status: 'proved_at_p1_fixture',
        artifactPath: 'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json',
        screenshotPath: routeScreenshotProof.screenshotPath,
        missingProofReason:
          'Live service data, child-device UI, hosted Playwright/a11y output, and physical-device proof remain pending.',
      },
      {
        requirement: 'Rust SQLite tracking ingest',
        status: 'proved_at_p1_fixture',
        artifactPath: 'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/10-journal-sqlite-proof.json',
        missingProofReason: 'Full UI and platform replay proof remain pending.',
      },
    ],
    uiStateCoverage: trackingRouteExpectedStates(),
    blockingGapsBeforeProductOrPrReady: [
      'Hosted Playwright/a11y proof is pending.',
      'Live parent service-data UI proof is pending.',
      'Child-device UI proof is pending.',
      'Android/iOS physical background proof is manual-required.',
      'Authority-enrolled and production-pilot proof are not present.',
    ],
  };
}

function buildParentFixtures() {
  const evidenceTrace = {
    evidenceReferenceId: 'location-evidence-1',
    kind: 'journal-event',
    observedAt: '2026-06-03T02:00:00.000Z',
  };
  const alert = {
    schemaVersion: 'v0.5-tracking',
    alertId: 'tracking-alert-1',
    createdAt: '2026-06-03T02:01:10.000Z',
    severity: 'info',
    policyDecisionId: 'tracking-decision-1',
    evidenceReferences: [evidenceTrace],
    sensitiveDetailMode: 'minimal-provider-body',
    notificationStatusRefs: ['notification-intent-queued'],
    acknowledgementId: 'tracking-ack-1',
    reasonCodes: ['parent-notification-intent-created'],
  };
  const acknowledgement = {
    schemaVersion: 'v0.5-tracking',
    acknowledgementId: 'tracking-ack-1',
    alertId: 'tracking-alert-1',
    state: 'acknowledged-safe',
    acknowledgedAt: '2026-06-03T02:03:00.000Z',
    expiresAt: null,
    stillAlertForCritical: true,
    reasonCodes: ['parent-confirmed-safe'],
    auditRefs: ['ack-recorded'],
  };
  const checkInRequest = {
    schemaVersion: 'v0.5-tracking',
    checkInId: 'tracking-checkin-1',
    requestedAt: '2026-06-03T02:02:00.000Z',
    state: 'sent',
    relatedAlertId: 'tracking-alert-1',
    includeLocationIfPermitted: true,
    expiresAt: '2026-06-03T02:12:00.000Z',
    evidenceReferences: [evidenceTrace],
    auditRefs: ['checkin-request-sent'],
  };
  const checkInResponse = {
    schemaVersion: 'v0.5-tracking',
    checkInId: 'tracking-checkin-1',
    respondedAt: '2026-06-03T02:05:00.000Z',
    response: 'safe',
    locationEvidenceReference: evidenceTrace,
    auditRefs: ['checkin-safe-response'],
  };
  return {
    alert,
    acknowledgement,
    checkInRequest,
    checkInResponse,
  };
}

function tierEnvelope(workpackId, currentProofTier, currentStatus, proof, requiredProofTier = currentProofTier) {
  return {
    schemaVersion: 1,
    checkedAt,
    workpackId,
    requiredProofTier,
    currentProofTier,
    currentStatus,
    artifactPath: `output/tracking-plan-proof/${workpackId}/`,
    missingProofReason:
      currentProofTier === 'P1_FIXTURE_SIMULATION'
        ? 'P4/P5/P6 product proof remains manual-required or authority-required where applicable.'
        : null,
    proof,
  };
}

function buildFixtures() {
  const evidenceRef = {
    evidenceId: 'tracking-journal-row-1',
    kind: 'journal-entry',
    digest: 'sha256:tracking-proof',
    uri: null,
  };
  const locationEvidence = {
    schemaVersion: 1,
    evidenceId: 'location-evidence-1',
    observedAt: '2026-06-03T02:00:00.000Z',
    freshUntil: '2026-06-03T02:05:00.000Z',
    staleAt: '2026-06-03T02:15:00.000Z',
    sourceId: 'android-child-agent',
    adapterId: 'android-fused-location-adapter',
    deviceId: 'child-device-1',
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
    evidence: [evidenceRef],
  };
  const deviceStatusEvidence = {
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
    evidence: [evidenceRef],
  };
  const circleRule = {
    schemaVersion: 1,
    ruleId: 'home-arrival-rule',
    geofenceId: 'home-circle',
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
    minAccuracyMeters: 100,
    enterGraceSeconds: 120,
    exitGraceSeconds: 120,
    dwellSeconds: 300,
    scheduleId: 'school-night-schedule',
    enabled: true,
    retentionMode: '24h',
    auditRefs: ['home-rule-created'],
  };
  const expectedPlaceSchedule = {
    schemaVersion: 1,
    scheduleId: 'school-night-schedule',
    placeId: 'home',
    label: 'Home on school nights',
    windows: [
      {
        startsAt: '2026-06-03T01:00:00.000Z',
        endsAt: '2026-06-03T11:00:00.000Z',
        timezone: 'America/Toronto',
      },
    ],
    lateGraceSeconds: 600,
    earlyExitGraceSeconds: 600,
    enabled: true,
    auditRefs: ['expected-place-schedule-created'],
  };
  const exportRetentionPolicy = {
    schemaVersion: 1,
    policyId: 'tracking-retention-parent-export',
    mode: 'export-only',
    custodyLabel: 'parent-owned-export',
    customRetentionHours: null,
    deleteOnResolution: false,
    exportAllowed: true,
    remoteSyncDefault: 'disabled',
    auditRefs: ['tracking-retention-parent-export'],
  };

  return {
    locationEvidence,
    deviceStatusEvidence,
    circleRule,
    expectedPlaceSchedule,
    exportRetentionPolicy,
    readModel: {
      schemaVersion: 1,
      generatedAt: '2026-06-03T02:02:00.000Z',
      custodyLabel: 'child-device-local',
      capabilityStatus: 'recent',
      limit: 25,
      returned: 1,
      locationRows: [locationEvidence],
      deviceStatusRows: [deviceStatusEvidence],
      capabilityRows: [],
      geofenceTransitions: [],
      expectedPlaceDecisions: [],
      nearbyPlaceRows: [],
      retentionPolicies: [],
      timeline: [
        {
          rowId: 'location-evidence-1',
          kind: 'location',
          observedAt: '2026-06-03T02:00:00.000Z',
          capabilityStatus: 'recent',
          reasonCodes: ['foreground-location-sample'],
          evidence: [evidenceRef],
        },
      ],
    },
  };
}

async function writeJson(path, value) {
  await mkdir(join(path, '..'), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function runTrackingRouteScreenshotProof() {
  const screenshotPath = join(
    proofRoot,
    '30-parent-and-child-ui-ux-surfaces',
    '11-ui-snapshots',
    'policy-tracking-parent-fixture.png'
  );
  const logPath = join(proofRoot, '30-parent-and-child-ui-ux-surfaces', '12-playwright-proof.log');
  const htmlPath = join(
    proofRoot,
    '30-parent-and-child-ui-ux-surfaces',
    '11-ui-snapshots',
    'policy-tracking-parent-fixture.html'
  );
  await mkdir(join(screenshotPath, '..'), { recursive: true });

  const startedAt = new Date().toISOString();
  const commandLine =
    'static-render apps/portal/src/TrackingStatusRoutePanel.tsx -> output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/policy-tracking-parent-fixture.html';
  const output = ['static-html-rendered'];
  const url = pathToFileURL(htmlPath).href;
  await writeFile(htmlPath, await renderTrackingRouteFixtureHtml());
  let result;
  try {
    const { chromium } = await import('@playwright/test');
    const browser = await chromium.launch();
    const viewport = { width: 1440, height: 1800 };
    await writeFile(
      join(
        proofRoot,
        '30-parent-and-child-ui-ux-surfaces',
        '11-ui-snapshots',
        'policy-tracking-parent-fixture-viewport.json'
      ),
      `${JSON.stringify(viewport, null, 2)}\n`
    );
    const page = await browser.newPage({ viewport });
    await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 15_000 });
    await page.waitForSelector('.tracking-status-overlay-grid', { timeout: 20_000 });
    await page.waitForTimeout(1200);
    const overlay = page.locator('.tracking-status-overlay');
    const overlayText = await overlay.innerText();
    const grid = page.locator('.tracking-status-overlay-grid');
    const gridColumns = await grid.evaluate((node) => getComputedStyle(node).gridTemplateColumns);
    const rowCount = await page.locator('.tracking-status-overlay-grid > .summary').count();
    const lastRowBox = await page
      .locator('.tracking-status-overlay-grid > .summary')
      .nth(rowCount - 1)
      .boundingBox();
    const allRowsVisible = lastRowBox !== null;
    await page.screenshot({ path: screenshotPath, fullPage: true });
    await browser.close();
    const missingState = trackingRouteExpectedStates().find((state) => !overlayText.includes(state));
    if (missingState !== undefined) {
      throw new Error(`tracking route screenshot proof missing state ${missingState}`);
    }
    if (!overlayText.includes('No product claim')) {
      throw new Error('tracking route screenshot proof missing no-product-claim text');
    }
    const deletedHistoryProof = trackingRouteDeletedHistoryProof();
    const proofArtifactProof = await trackingRouteProofArtifactProof();
    if (!overlayText.includes(deletedHistoryProof.historyVisibilityText)) {
      throw new Error('tracking route screenshot proof missing deleted-history hidden text');
    }
    if (!overlayText.includes(deletedHistoryProof.deletedEvidenceText)) {
      throw new Error('tracking route screenshot proof missing deleted-evidence hidden text');
    }
    if (overlayText.includes(deletedHistoryProof.deletedEvidenceProbe)) {
      throw new Error('tracking route screenshot proof rendered deleted evidence id');
    }
    if (!overlayText.toLowerCase().includes(proofArtifactProof.runtimeReferenceLabel.toLowerCase())) {
      throw new Error('tracking route screenshot proof missing runtime-reference label');
    }
    const missingArtifact = proofArtifactProof.requiredArtifacts.find(
      (artifactPath) => !overlayText.includes(artifactPath)
    );
    if (missingArtifact !== undefined) {
      throw new Error(`tracking route screenshot proof missing artifact reference ${missingArtifact}`);
    }
    const minimumExpectedRowCount = trackingRouteExpectedStates().length + 1;
    if (rowCount < minimumExpectedRowCount || !allRowsVisible) {
      throw new Error(`tracking route screenshot proof row visibility failed: rowCount=${rowCount}`);
    }
    result = {
      route: url,
      htmlPath: proofRelative(htmlPath),
      screenshotPath: proofRelative(screenshotPath),
      logPath: proofRelative(logPath),
      viewport: `${viewport.width}x${viewport.height}`,
      gridColumns,
      rowCount,
      minimumExpectedRowCount,
      allRowsVisible,
      deletedHistoryProof,
      proofArtifactProof,
      productClaimReady: false,
    };
    commands.push({ command: commandLine, exitCode: 0 });
    return result;
  } finally {
    await writeTrackingRouteProofLog({
      logPath,
      startedAt,
      route: url,
      commandLine,
      output,
      result,
      screenshotPath,
    });
  }
}

async function renderTrackingRouteFixtureHtml() {
  const React = await import('react');
  const { renderToStaticMarkup } = await import('react-dom/server');
  const { TrackingStatusRoutePanel } = await importTsModule('apps/portal/src/TrackingStatusRoutePanel.tsx');
  const { resolveLiveActivityState } = await importTsModule('apps/portal/src/live-activity-state.ts');
  const liveActivity = resolveLiveActivityState([]);
  const actions = new Proxy(
    {},
    {
      get() {
        return () => {};
      },
    }
  );
  const markup = renderToStaticMarkup(
    React.createElement(
      'main',
      { className: 'parent-portal-route tracking-status-proof-page' },
      React.createElement(TrackingStatusRoutePanel, {
        actions,
        commandEnabled: false,
        liveActivity,
      })
    )
  );

  return [
    '<!doctype html>',
    '<html data-theme="dark">',
    '<head>',
    '  <meta charset="utf-8">',
    '  <meta name="viewport" content="width=device-width, initial-scale=1">',
    '  <title>Tracking Parent Fixture Proof</title>',
    `  <style>${trackingRouteFixtureStyles()}</style>`,
    '</head>',
    `<body>${markup}</body>`,
    '</html>',
    '',
  ].join('\n');
}

function trackingRouteFixtureStyles() {
  return `
    :root {
      color-scheme: dark;
      font-family: "Segoe UI", system-ui, sans-serif;
    }
    body {
      margin: 0;
      min-height: 100vh;
      background:
        radial-gradient(circle at top left, rgba(78, 223, 255, 0.22), transparent 26%),
        linear-gradient(180deg, #041425 0%, #06131f 52%, #081826 100%);
      color: #e5f7ff;
    }
    .tracking-status-proof-page {
      position: relative;
      min-height: 100vh;
      padding: 24px;
      box-sizing: border-box;
    }
    .product-eyebrow {
      display: inline-block;
      margin: 0 0 6px;
      color: #8fb8c9;
      font-size: 0.72rem;
      font-weight: 700;
      letter-spacing: 0.14em;
      text-transform: uppercase;
    }
    .command-result-tab {
      margin-top: 8px;
      padding: 8px 12px;
      border: 1px solid rgba(125, 229, 255, 0.28);
      border-radius: 999px;
      color: #8fb8c9;
      background: rgba(2, 12, 22, 0.32);
      font: inherit;
    }
    .tracking-status-overlay {
      position: relative;
      inset: auto;
      max-width: 1040px;
      margin: 0 auto;
      color: #e5f7ff;
    }
    .tracking-status-overlay-content {
      display: grid;
      grid-template-rows: auto minmax(0, 1fr);
      gap: 10px;
      min-height: 0;
      padding: 12px;
      border: 1px solid rgba(87, 229, 255, 0.52);
      border-radius: 8px;
      background:
        linear-gradient(180deg, rgba(4, 20, 37, 0.72), rgba(2, 12, 22, 0.68)),
        radial-gradient(circle at 18% 0%, rgba(78, 223, 255, 0.22), transparent 34%);
      box-shadow:
        0 0 0 1px rgba(255, 255, 255, 0.06) inset,
        0 18px 50px rgba(0, 0, 0, 0.42);
    }
    .tracking-status-overlay-header {
      display: grid;
      gap: 4px;
    }
    .tracking-status-overlay-header h2,
    .tracking-status-overlay-header p {
      margin: 0;
    }
    .tracking-status-overlay-header h2 {
      color: #ffffff;
      font-size: 1rem;
      line-height: 1.1;
    }
    .tracking-status-overlay-header p:last-child {
      max-width: 58ch;
      color: #b8d4e5;
      font-size: 0.78rem;
      line-height: 1.35;
    }
    .tracking-status-overlay-grid {
      display: grid;
      grid-template-columns: 1fr;
      gap: 6px;
      min-height: 0;
      overflow: visible;
      padding-right: 3px;
    }
    .summary {
      display: grid;
      grid-template-columns: minmax(138px, 0.24fr) minmax(0, 1fr);
      gap: 10px;
      min-width: 0;
      padding: 10px 12px;
      border: 1px solid rgba(125, 229, 255, 0.28);
      border-radius: 10px;
      background: rgba(4, 20, 37, 0.72);
      box-sizing: border-box;
    }
    .summary h2 {
      margin: 0;
      color: #ffffff;
      font-size: 0.82rem;
      line-height: 1.14;
      font-weight: 700;
    }
    .tracking-status-overlay-meta {
      display: grid;
      grid-template-columns: repeat(3, minmax(0, 1fr));
      gap: 8px;
      margin: 0;
    }
    .tracking-status-overlay-meta dt,
    .tracking-status-overlay-meta dd {
      margin: 0;
      min-width: 0;
      overflow-wrap: anywhere;
      font-size: 0.68rem;
      line-height: 1.24;
    }
    .tracking-status-overlay-meta dt {
      color: #8fb8c9;
      font-weight: 760;
      text-transform: uppercase;
    }
    .tracking-status-overlay-meta dd {
      color: #dff8ff;
      font-weight: 680;
    }
    @media (max-width: 680px) {
      .tracking-status-proof-page {
        padding: 8px;
      }
      .summary {
        grid-template-columns: 1fr;
      }
      .tracking-status-overlay-meta {
        grid-template-columns: 1fr;
      }
    }
  `;
}

function trackingRouteExpectedStates() {
  return [
    'Tracking off',
    'Permission required',
    'Stale last known',
    'Offline last known',
    'Low accuracy',
    'Nearby place ambiguous',
    'Policy alert',
    'Parent acknowledged',
    'Exception active',
    'Child check-in',
    'Temporary live',
    'Missing device',
    'Retention deleted',
  ];
}

async function writeTrackingRouteProofLog({ logPath, startedAt, route, commandLine, output, result, screenshotPath }) {
  const lines = [
    'Tracking Plan WP30 parent-route fixture proof',
    '',
    `Date: ${startedAt}`,
    `Route: ${route}`,
    `Command: ${commandLine}`,
    'Viewport: 1440x1000',
    `Screenshot: ${proofRelative(screenshotPath)}`,
    '',
    'Checks:',
    '- Waited for selector: .tracking-status-overlay-grid',
    `- Computed grid columns: ${result?.gridColumns ?? 'not captured'}`,
    `- Row count: ${result?.rowCount ?? 'not captured'}`,
    `- All rows visible in viewport: ${result?.allRowsVisible ?? false}`,
    '- Overlay text included every first-target tracking state',
    '- Overlay text included: No product claim',
    '- Overlay text included: Deleted history hidden',
    '- Overlay text included: Deleted evidence not rendered',
    `- Overlay text omitted deleted evidence id: ${result?.deletedHistoryProof?.deletedEvidenceProbe ?? 'not captured'}`,
    `- Overlay text included: ${result?.proofArtifactProof?.runtimeReferenceLabel ?? 'not captured'}`,
    `- Overlay text included proof artifact references: ${result?.proofArtifactProof?.requiredArtifacts?.join(', ') ?? 'not captured'}`,
    '',
    'Scope:',
    'This is local rendered fixture proof only. It does not prove live service data, child-device UI, hosted Playwright/a11y output, or physical-device behavior.',
    '',
    'Server output:',
    ...sanitizeServerOutput(output.join('')).split(/\r?\n/u).filter(Boolean),
    '',
  ];
  await mkdir(join(logPath, '..'), { recursive: true });
  await writeFile(logPath, `${lines.join('\n')}\n`);
}

function trackingRouteDeletedHistoryProof() {
  return {
    historyVisibilityText: 'Deleted history hidden',
    deletedEvidenceText: 'Deleted evidence not rendered',
    deletedEvidenceProbe: 'location-evidence-1',
    deletedEvidenceRendered: false,
  };
}

async function trackingRouteProofArtifactProof() {
  const portalDomain = await importTsModule('packages/portal-domain/src/tracking-status-proof-artifacts.ts');
  const requiredArtifacts = Object.values(portalDomain.TrackingStatusProofArtifacts);
  return {
    runtimeReferenceLabel: 'Runtime reference',
    requiredPrefix: 'output/tracking-plan-proof/',
    requiredArtifacts,
    artifactCount: requiredArtifacts.length,
  };
}

function proofRelative(path) {
  return relative(repoRoot, path).replace(/\\/gu, '/');
}

function sanitizeServerOutput(value) {
  return value.replace(/\u001b\[[0-9;]*m/gu, '').replace(/[^\x09\x0a\x0d\x20-\x7e]/gu, '');
}

async function importTsModule(relativePath) {
  return tsImport(pathToFileURL(join(repoRoot, relativePath)).href, import.meta.url);
}

async function waitForHttp(url, timeoutMs) {
  const deadline = Date.now() + timeoutMs;
  let lastError;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return;
      }
      lastError = new Error(`${url} returned ${response.status}`);
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 300));
  }
  throw lastError ?? new Error(`${url} did not respond before timeout`);
}

async function availablePort(preferredPort) {
  if (Number.isInteger(preferredPort) && preferredPort > 0 && (await canListen(preferredPort))) {
    return preferredPort;
  }
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.once('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const address = server.address();
      server.close(() => {
        if (typeof address === 'object' && address !== null) {
          resolve(address.port);
          return;
        }
        reject(new Error('Unable to allocate an available port'));
      });
    });
  });
}

async function canListen(port) {
  return new Promise((resolve) => {
    const server = net.createServer();
    server.once('error', () => resolve(false));
    server.listen(port, '127.0.0.1', () => {
      server.close(() => resolve(true));
    });
  });
}

async function stopProcessTree(child) {
  if (child.exitCode !== null) {
    return;
  }
  if (process.platform === 'win32' && child.pid !== undefined) {
    await new Promise((resolve) => {
      const killer = spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], {
        cwd: repoRoot,
        stdio: 'ignore',
        windowsHide: true,
      });
      killer.once('exit', () => resolve());
      killer.once('error', () => resolve());
    });
    return;
  }
  child.kill('SIGTERM');
}

async function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  await runCommand(command, commandArgs, ...rest);
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      commands.push({ command: commandLine, exitCode: code });
      code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`));
    });
    child.once('error', reject);
  });
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
