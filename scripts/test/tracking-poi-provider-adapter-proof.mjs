import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-poi-provider-adapter-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '20-google-places-and-poi-provider-adapter');
const timestamp = '2026-06-05T14:45:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-poi-provider-adapter',
  'tracking-location-policy',
]);

const tracking = await importDist('tracking-location-policy.js');
const adapter = await importDist('tracking-poi-provider-adapter.js');
const searchInput = adapter.TrackingGooglePlacesNearbySearchInputSchema.parse(sourceSearchInput(tracking, adapter));
const request = adapter.buildGooglePlacesNearbySearchRequest(searchInput);
const readModel = adapter.buildGooglePlacesNearbyReadModel(searchInput, googleNearbyResponse());
const failureReadModel = adapter.buildGooglePlacesProviderFailureReadModel(
  searchInput,
  'google-places-quota-unavailable'
);

const proof = {
  proofMode: 'tracking-poi-provider-adapter-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  sources: {
    googleNearbySearch: 'https://developers.google.com/maps/documentation/places/web-service/nearby-search',
    googleChooseFields: 'https://developers.google.com/maps/documentation/places/web-service/choose-fields',
    googleSearchNearbyRest:
      'https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places/searchNearby',
  },
  summary: summarize(readModel, failureReadModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-poi-provider-adapter.ts',
    test: 'packages/parent-domain/tests/tracking-poi-provider-adapter.test.ts',
    harness: 'scripts/test/tracking-poi-provider-adapter-proof.mjs',
    evidence: 'test-results/tracking-poi-provider-adapter-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/20-google-places-and-poi-provider-adapter',
  },
  request,
  readModel,
  failureReadModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'google-places-request.json'), request);
await writeJson(join(testOutputDir, 'tracking-poi-provider-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'provider-failure-read-model.json'), failureReadModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-poi-provider-adapter-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-poi-provider-adapter-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function sourceSearchInput(tracking, adapter) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    provider: adapter.TrackingPoiProviderId.GooglePlacesNearby,
    requestId: 'tracking-google-places-request-1',
    requestedAt: timestamp,
    center: {
      latitude: 43.6532,
      longitude: -79.3832,
      accuracyMeters: 22,
      evidenceReferenceId: 'location-evidence-poi-query-1',
    },
    radiusMeters: 250,
    maxResultCount: 5,
    includedTypes: ['school', 'restaurant', 'transit_station'],
    fieldMask: adapter.GooglePlacesNearbyFieldMask,
    auditRefs: ['google-places-field-mask-reviewed', 'nearby-place-provider-proof'],
  };
}

function googleNearbyResponse() {
  return {
    places: [
      {
        id: 'google-place-school-1',
        name: 'places/google-place-school-1',
        displayName: { text: 'Central School' },
        location: { latitude: 43.65335, longitude: -79.383 },
        primaryType: 'school',
        types: ['school', 'point_of_interest', 'establishment'],
      },
      {
        id: 'google-place-cafe-1',
        name: 'places/google-place-cafe-1',
        displayName: { text: 'Corner Cafe' },
        location: { latitude: 43.654, longitude: -79.3829 },
        primaryType: 'cafe',
        types: ['cafe', 'food', 'point_of_interest', 'establishment'],
      },
    ],
  };
}

function summarize(readModel, failureReadModel) {
  return {
    provider: readModel.provider,
    status: readModel.status,
    candidateCount: readModel.candidates.length,
    categories: readModel.candidates.map((candidate) => candidate.category),
    ambiguityStates: readModel.candidates.map((candidate) => candidate.ambiguityState),
    fieldMask: readModel.fieldMask,
    radiusMeters: readModel.radiusMeters,
    failureStatus: failureReadModel.status,
    failureReason: failureReadModel.providerFailureReason,
  };
}

function nonClaims(readModel) {
  return {
    credentialsStored: readModel.credentialsStored,
    liveProviderRequestClaimed: readModel.liveProviderRequestClaimed,
    exactPlaceClaimed: readModel.exactPlaceClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.candidateCount !== 2 ||
    proof.summary.categories.join(',') !== 'school,food' ||
    proof.summary.ambiguityStates.every((state) => state === 'multiple-candidates') !== true ||
    proof.summary.radiusMeters !== 250 ||
    proof.summary.failureStatus !== 'provider-unavailable'
  ) {
    throw new Error(`Unexpected tracking POI provider proof summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Tracking POI provider proof overclaimed live behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP20 Google Places And POI Provider Adapter Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: Google Places Nearby Search request/response adapter contract behind the nearby-place abstraction.',
      '- Source inspected: WP20 workpack, tracking location feature doc, location expectations, Google Nearby Search docs, Google field-mask docs, and Google searchNearby REST reference.',
      '- No live provider call, credentials, physical-device proof, or exact-place claim is made by this proof.',
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
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-poi-provider-adapter tracking-location-policy: PASS',
      '- Google Places Nearby Search request uses POST, a bounded circle locationRestriction, included types, maxResultCount 1..20, and the minimal production field mask.',
      '- Provider response mapping preserves provider id/resource, display text, primary type, category, distance, confidence, ambiguity, and source evidence reference.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '07-nearby-place-proof.json'), proof.summary);
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Wildcard Google Places field masks are rejected.',
      '- Radius is bounded to 25..1000 meters and the request must include at least one included type.',
      '- Credentials are not stored and live Google provider execution is not claimed.',
      '- Mapped nearby-place rows preserve ambiguity and do not claim exact child location or physical-device proof.',
      '- Provider quota/unavailable errors degrade to provider-unavailable instead of silent success.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(path, 'README.md'),
    '# WP20 Google Places And POI Provider Adapter Proof\n\nThis proof pack records a schema-backed Google Places Nearby Search request/response adapter boundary with bounded location restriction, minimal production field mask, provider failure degradation, and no live provider/credential/physical-device/exact-place claims.\n',
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

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
