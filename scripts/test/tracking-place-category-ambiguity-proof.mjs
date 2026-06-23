import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const timestamp = '2026-06-06T18:14:00.000Z';
const testOutputDir = join(repoRoot, 'test-results', 'tracking-place-category-ambiguity-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '21-place-category-taxonomy-and-ambiguity-model');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/tracking-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'tracking-place-category-ambiguity-proof',
  'tracking-poi-provider-adapter',
]);

const tracking = await importSchemaDist('tracking-location-policy.js');
const adapter = await importDist('tracking-poi-provider-adapter.js');
const categoryProof = await importSchemaDist('tracking-place-category-ambiguity-proof.js');
const searchInput = adapter.TrackingGooglePlacesNearbySearchInputSchema.parse(sourceSearchInput(tracking, adapter));
const multipleReadModel = adapter.buildGooglePlacesNearbyReadModel(searchInput, multipleCandidateResponse());
const lowAccuracyReadModel = adapter.buildGooglePlacesNearbyReadModel(
  lowAccuracyInput(tracking, adapter),
  singleCandidateResponse()
);
const multipleCandidateReviews = categoryProof.buildTrackingPlaceCategoryAmbiguityReviews(multipleReadModel);
const lowAccuracyReviews = categoryProof.buildTrackingPlaceCategoryAmbiguityReviews(lowAccuracyReadModel);

const proof = {
  proofMode: 'tracking-place-category-ambiguity-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: {
    multipleCandidateReviewCount: multipleCandidateReviews.length,
    multipleCandidateCategories: multipleCandidateReviews.map((review) => review.category),
    multipleCandidateAmbiguityStates: multipleCandidateReviews.map((review) => review.ambiguityState),
    lowAccuracyAmbiguityStates: lowAccuracyReviews.map((review) => review.ambiguityState),
    reviewStates: [...multipleCandidateReviews, ...lowAccuracyReviews].map((review) => review.reviewState),
    safeCopyAccusationFree: [...multipleCandidateReviews, ...lowAccuracyReviews].every(
      (review) => review.safeCopy.accusationFree && !review.safeCopy.allowsAutomaticAction
    ),
    categoryCanTriggerActionDirectly: [...multipleCandidateReviews, ...lowAccuracyReviews].some(
      (review) => review.categoryCanTriggerActionDirectly
    ),
    parentDefinedZoneOverrideCanTriggerActionDirectly: [...multipleCandidateReviews, ...lowAccuracyReviews].some(
      (review) => review.parentDefinedZoneOverrideCanTriggerActionDirectly
    ),
  },
  nonClaims: {
    liveProviderRequestClaimed: false,
    exactPlaceClaimed: false,
    physicalDeviceProofClaimed: false,
    providerDeliveryClaimed: false,
    automaticActionClaimed: false,
  },
  proofPaths: {
    source: 'packages/schema-domain/src/tracking-place-category-ambiguity-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-place-category-ambiguity-proof.test.ts',
    harness: 'scripts/test/tracking-place-category-ambiguity-proof.mjs',
    evidence: 'test-results/tracking-place-category-ambiguity-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/21-place-category-taxonomy-and-ambiguity-model',
  },
  multipleCandidateReviews,
  lowAccuracyReviews,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeJson(join(testOutputDir, 'multiple-candidate-reviews.json'), multipleCandidateReviews);
await writeJson(join(testOutputDir, 'low-accuracy-reviews.json'), lowAccuracyReviews);
await writeProofPack(proofDir, proof);

console.log('tracking-place-category-ambiguity-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-place-category-ambiguity-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'tracking-domain', 'dist', name)).href);
}

function importSchemaDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function sourceSearchInput(tracking, adapter) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    provider: adapter.TrackingPoiProviderId.GooglePlacesNearby,
    requestId: 'tracking-place-category-ambiguity-request-1',
    requestedAt: timestamp,
    center: {
      latitude: 43.6532,
      longitude: -79.3832,
      accuracyMeters: 22,
      evidenceReferenceId: 'location-evidence-category-query-1',
    },
    radiusMeters: 250,
    maxResultCount: 5,
    includedTypes: ['school', 'bar', 'restaurant'],
    fieldMask: adapter.GooglePlacesNearbyFieldMask,
    auditRefs: ['place-category-ambiguity-proof'],
  };
}

function lowAccuracyInput(tracking, adapter) {
  return adapter.TrackingGooglePlacesNearbySearchInputSchema.parse({
    ...sourceSearchInput(tracking, adapter),
    requestId: 'tracking-place-category-low-accuracy-request-1',
    center: {
      latitude: 43.6532,
      longitude: -79.3832,
      accuracyMeters: 130,
      evidenceReferenceId: 'location-evidence-category-low-accuracy-1',
    },
  });
}

function multipleCandidateResponse() {
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
        id: 'google-place-bar-1',
        name: 'places/google-place-bar-1',
        displayName: { text: 'Market Bar' },
        location: { latitude: 43.654, longitude: -79.3829 },
        primaryType: 'bar',
        types: ['bar', 'point_of_interest', 'establishment'],
      },
    ],
  };
}

function singleCandidateResponse() {
  return {
    places: [multipleCandidateResponse().places[0]],
  };
}

function assertProof(proof) {
  if (
    proof.summary.multipleCandidateReviewCount !== 2 ||
    proof.summary.multipleCandidateCategories.join(',') !== 'school,sensitive' ||
    proof.summary.multipleCandidateAmbiguityStates.every((state) => state === 'multiple-candidates') !== true ||
    proof.summary.lowAccuracyAmbiguityStates.every((state) => state === 'low-accuracy') !== true
  ) {
    throw new Error(`Unexpected place category ambiguity summary: ${JSON.stringify(proof.summary)}`);
  }
  if (
    !proof.summary.safeCopyAccusationFree ||
    proof.summary.categoryCanTriggerActionDirectly ||
    proof.summary.parentDefinedZoneOverrideCanTriggerActionDirectly
  ) {
    throw new Error(`Place category proof overclaimed action authority: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Place category proof overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP21 Place Category Taxonomy And Ambiguity Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking-domain place category ambiguity review proof built on the existing POI provider adapter.',
      '- No live provider execution, exact-place claim, physical-device proof, automatic action, or provider delivery is claimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/schema-domain: PASS',
      '- cmd /c npm run build --workspace @ocentra-parent/tracking-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/tracking-domain -- tracking-place-category-ambiguity-proof tracking-poi-provider-adapter: PASS',
      '- Multiple nearby candidates remain manual-required review rows.',
      '- Low accuracy rows remain manual-required review rows.',
      '- Category evidence is policy input only and cannot trigger action directly.',
      '- Parent-defined zone override remains a policy-review input, not an automatic action.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '07-nearby-place-proof.json'), proof.summary);
  await writeJson(join(path, '17-category-ambiguity-no-accusation-proof.json'), proof);
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Safe copy rejects accusation/exact-presence wording such as is-at, visited, caught, and definitely-at claims.',
      '- Review rows require a policy decision and reject direct-action upgrades.',
      '- This proof preserves existing POI provider no-claim boundaries for live provider request, exact place, physical device, and provider delivery.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeJson(join(path, 'proof-summary.json'), {
    schemaVersion: 1,
    checkedAt: proof.generatedAt,
    commit: proof.commit,
    workpackId: '21-place-category-taxonomy-and-ambiguity-model',
    proofState: 'category-ambiguity-proof-complete',
    summary: proof.summary,
    commands: proof.commands,
    productClaims: {
      noAccusationCopyProof: true,
      lowAccuracyAmbiguityProof: true,
      multiplePlaceAmbiguityProof: true,
      categoryAsPolicyInputOnly: true,
      parentDefinedZoneOverrideRequiresPolicyDecision: true,
      providerDeliveryClaimed: false,
      physicalDeviceProofClaimed: false,
      automaticActionClaimed: false,
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
