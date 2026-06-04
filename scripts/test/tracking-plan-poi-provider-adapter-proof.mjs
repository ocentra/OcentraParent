import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const workpackId = '20-google-places-and-poi-provider-adapter';
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', workpackId);
const testResultRoot = join(repoRoot, 'test-results', 'tracking-plan-poi-provider-adapter-proof');
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
    'tests/tracking-poi-provider-adapter.test.ts',
  ]);

  const tracking = await import(pathToFileURL(join(repoRoot, 'packages', 'activity-domain', 'dist', 'tracking.js')));
  const proof = buildPoiProviderProof(tracking);
  const commit = await gitHead();
  await writeProofArtifacts({ proof, commit });

  console.log('tracking-plan-poi-provider-adapter-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofRoot)}`);
}

function buildPoiProviderProof(tracking) {
  const request = tracking.TrackingGooglePlacesNearbyRequestSchema.parse(validRequest());
  const fieldMaskHeader = tracking.buildTrackingGooglePlacesNearbyFieldMaskHeader(request);
  const mappedEvidence = tracking.buildTrackingGooglePlacesNearbyEvidence({
    request,
    evidenceId: 'google-places-nearby-cinema-proof',
    observedAt: '2026-06-04T10:00:00.000Z',
    response: {
      places: [
        {
          providerRef: 'places/provider-cinema',
          location: { latitude: 43.6538, longitude: -79.3838 },
          primaryType: 'movie_theater',
          types: ['movie_theater', 'point_of_interest'],
          distanceMeters: 118,
        },
      ],
    },
    evidence: [evidenceRef()],
  });
  const ambiguousEvidence = tracking.buildTrackingGooglePlacesNearbyEvidence({
    request,
    evidenceId: 'google-places-nearby-ambiguous-proof',
    observedAt: '2026-06-04T10:02:00.000Z',
    response: {
      places: [
        {
          providerRef: 'places/provider-school',
          location: { latitude: 43.654, longitude: -79.384 },
          primaryType: 'school',
          types: ['school'],
          distanceMeters: 142,
        },
        {
          providerRef: 'places/provider-park',
          location: { latitude: 43.6542, longitude: -79.3842 },
          primaryType: 'park',
          types: ['park'],
          distanceMeters: 166,
        },
      ],
    },
    evidence: [evidenceRef()],
  });
  const unavailableEvidence = tracking.buildTrackingGooglePlacesNearbyUnavailableEvidence({
    request,
    evidenceId: 'google-places-nearby-unavailable-proof',
    observedAt: '2026-06-04T10:03:00.000Z',
    failureReason: 'quota-exhausted',
    evidence: [evidenceRef()],
  });

  return {
    request,
    fieldMaskHeader,
    mappedEvidence,
    ambiguousEvidence,
    unavailableEvidence,
    negativeProof: {
      wildcardProductionFieldMaskRejected: tracking.TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        fieldMask: ['*'],
      }).success,
      broadProductionFieldRejected: tracking.TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        fieldMask: ['places.id', 'places.reviews'],
      }).success,
      zeroRadiusRejected: tracking.TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        radiusMeters: 0,
      }).success,
      maxRadiusRejected: tracking.TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        radiusMeters: 50_001,
      }).success,
      conflictingTypesRejected: tracking.TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        includedTypes: ['school', 'movie_theater'],
        excludedTypes: ['school'],
      }).success,
    },
  };
}

async function writeProofArtifacts({ proof, commit }) {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });
  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot(commit));
  await writeFile(join(proofRoot, '01-contract-proof.log'), contractProofLog(proof));
  await writeJson(
    join(proofRoot, '07-nearby-place-proof.json'),
    tierEnvelope('P1_FIXTURE_SIMULATION', 'simulated', {
      request: proof.request,
      fieldMaskHeader: proof.fieldMaskHeader,
      mappedEvidence: proof.mappedEvidence,
      ambiguousEvidence: proof.ambiguousEvidence,
      unavailableEvidence: proof.unavailableEvidence,
    })
  );
  await writeFile(join(proofRoot, '13-security-negative-proof.log'), securityProofLog(proof));
  await writeFile(join(proofRoot, '15-manual-platform-proof.md'), manualPlatformProof());
  await writeFile(
    join(proofRoot, '16-validation-commands.log'),
    commands.map((entry) => entry.command).join('\n') + '\n'
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
      'POI provider adapter proof passes for bounded Google Places nearby requests, minimal production field masks, deterministic category mapping, ambiguity, and provider failure degradation.',
    proofArtifacts: [
      `output/tracking-plan-proof/${workpackId}/07-nearby-place-proof.json`,
      `output/tracking-plan-proof/${workpackId}/13-security-negative-proof.log`,
    ],
    commands,
    productClaims: productClaims(),
    missingProofReason:
      'This is P1 contract/runtime helper proof. It does not call Google Places, prove provider credentials, prove production billing/terms configuration, render UI, or prove mobile physical-device background behavior.',
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
      'Live provider credentials, provider delivery, UI, hosted accessibility, and physical-device behavior remain outside this proof.',
    payload,
  };
}

function sourceSnapshot(commit) {
  return [
    '# 20-google-places-and-poi-provider-adapter Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- source: packages/activity-domain/src/tracking-poi-provider-adapter.ts',
    '- test: packages/activity-domain/tests/tracking-poi-provider-adapter.test.ts',
    '- command: npm run test:tracking-plan-poi-provider-adapter-proof',
    '',
  ].join('\n');
}

function contractProofLog(proof) {
  return [
    `workpack=${workpackId}`,
    'activity-domain tracking POI provider adapter test passed',
    `fieldMaskHeader=${proof.fieldMaskHeader}`,
    `mappedCategory=${proof.mappedEvidence.category}`,
    `ambiguousState=${proof.ambiguousEvidence.ambiguityState}`,
    `unavailableState=${proof.unavailableEvidence.ambiguityState}`,
    '',
  ].join('\n');
}

function securityProofLog(proof) {
  const negatives = proof.negativeProof;
  return [
    `workpack=${workpackId}`,
    `wildcardProductionFieldMaskRejected=${!negatives.wildcardProductionFieldMaskRejected}`,
    `broadProductionFieldRejected=${!negatives.broadProductionFieldRejected}`,
    `zeroRadiusRejected=${!negatives.zeroRadiusRejected}`,
    `maxRadiusRejected=${!negatives.maxRadiusRejected}`,
    `conflictingTypesRejected=${!negatives.conflictingTypesRejected}`,
    'No API key, live Google request, Ocentra-hosted storage, or exact-place claim is used by this proof.',
    '',
  ].join('\n');
}

function manualPlatformProof() {
  return [
    '# WP20 Manual Platform Boundary',
    '',
    'This proof does not claim live Google Places credentials, production provider billing setup, Android/iOS background location, hosted UI, or physical-device behavior.',
    'It proves only local request/response adapter contracts and degraded provider behavior at P1 fixture tier.',
    '',
  ].join('\n');
}

function validRequest() {
  return {
    schemaVersion: 1,
    requestId: 'google-places-nearby-request-proof',
    requestedAt: '2026-06-04T09:59:00.000Z',
    providerKind: 'google-places',
    providerRef: 'google-places-nearby-search',
    locationEvidenceId: 'location-evidence-1',
    center: {
      latitude: 43.6532,
      longitude: -79.3832,
    },
    radiusMeters: 1000,
    includedTypes: ['school', 'movie_theater', 'park'],
    excludedTypes: ['bar', 'night_club'],
    fieldMask: ['places.id', 'places.location', 'places.primaryType', 'places.types'],
    maxResultCount: 5,
    rankPreference: 'DISTANCE',
    productionUse: true,
    auditRefs: ['google-places-nearby-request-proof'],
  };
}

function evidenceRef() {
  return {
    evidenceId: 'tracking-location-row-1',
    kind: 'journal-entry',
    digest: 'sha256:tracking-poi-provider-proof',
    uri: null,
  };
}

function productClaims() {
  return {
    boundedProviderRequestProved: true,
    minimalProductionFieldMaskProved: true,
    providerFailureDegrades: true,
    liveGooglePlacesApiClaimed: false,
    providerCredentialsClaimed: false,
    providerDeliveryClaimed: false,
    exactChildPlaceClaimed: false,
    androidIosBackgroundLocationClaimed: false,
    hostedUiAccessibilityClaimed: false,
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
