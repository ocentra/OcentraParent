import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { validateCheckpointScenarios } from './test/real-evidence-proof-checkpoint.mjs';

const repoRoot = process.cwd();
const matrixPath = join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json');
const requiredPlatforms = ['windows', 'linux', 'macos', 'android', 'ios'];
const allowedCoverageStatuses = new Set([
  'ci-mechanical-proof',
  'real-local-windows-proof',
  'real-macos-linux-proof',
  'real-android-ios-proof',
  'scaffold-only',
  'manual-required',
  'not-yet-proven',
  'not-applicable',
]);

function fail(message) {
  throw new Error(message);
}

function assertNonEmptyString(value, label) {
  if (typeof value !== 'string' || value.trim().length === 0) {
    fail(`${label} must be a non-empty string.`);
  }
}

function assertStringArray(value, label) {
  if (!Array.isArray(value) || value.length === 0) {
    fail(`${label} must be a non-empty array.`);
  }

  for (const [index, item] of value.entries()) {
    assertNonEmptyString(item, `${label}[${index}]`);
  }
}

function assertUnique(values, label) {
  const seen = new Set();
  for (const value of values) {
    if (seen.has(value)) {
      fail(`${label} contains duplicate value: ${value}`);
    }
    seen.add(value);
  }
}

function readMatrix() {
  if (!existsSync(matrixPath)) {
    fail(`Proof matrix does not exist: ${matrixPath}`);
  }

  return JSON.parse(readFileSync(matrixPath, 'utf8'));
}

function validateClaim(claim) {
  assertNonEmptyString(claim.id, 'claim.id');
  assertNonEmptyString(claim.roadmapSlice, `${claim.id}.roadmapSlice`);
  assertNonEmptyString(claim.claim, `${claim.id}.claim`);
  assertStringArray(claim.expectationFiles, `${claim.id}.expectationFiles`);
  assertStringArray(claim.knownGaps, `${claim.id}.knownGaps`);

  for (const expectationFile of claim.expectationFiles) {
    const expectationPath = join(repoRoot, expectationFile);
    if (!existsSync(expectationPath)) {
      fail(`${claim.id} references missing expectation file: ${expectationFile}`);
    }
  }

  if (claim.privilegedCapability !== true && claim.privilegedCapability !== false) {
    fail(`${claim.id}.privilegedCapability must be boolean.`);
  }

  if (typeof claim.ciProof !== 'object' || claim.ciProof === null) {
    fail(`${claim.id}.ciProof must be an object.`);
  }
  assertStringArray(claim.ciProof.commands, `${claim.id}.ciProof.commands`);
  assertStringArray(claim.ciProof.jobs, `${claim.id}.ciProof.jobs`);

  if (typeof claim.platformCoverage !== 'object' || claim.platformCoverage === null) {
    fail(`${claim.id}.platformCoverage must be an object.`);
  }

  let hasManualRequiredCoverage = false;
  for (const platform of requiredPlatforms) {
    const status = claim.platformCoverage[platform];
    if (!allowedCoverageStatuses.has(status)) {
      fail(`${claim.id}.platformCoverage.${platform} has unsupported status: ${status}`);
    }
    if (status === 'manual-required') {
      hasManualRequiredCoverage = true;
    }
  }

  if (claim.privilegedCapability && !hasManualRequiredCoverage) {
    fail(`${claim.id} is privileged and must include at least one manual-required platform.`);
  }

  if (hasManualRequiredCoverage) {
    assertStringArray(claim.manualProofChecklist, `${claim.id}.manualProofChecklist`);
  } else if (!Array.isArray(claim.manualProofChecklist)) {
    fail(`${claim.id}.manualProofChecklist must be an array.`);
  }
}

function validateMatrix(matrix) {
  if (matrix.schemaVersion !== 1) {
    fail('Proof matrix schemaVersion must be 1.');
  }

  assertNonEmptyString(matrix.title, 'title');
  assertNonEmptyString(matrix.realEvidenceExpectation, 'realEvidenceExpectation');
  const realEvidencePath = join(repoRoot, matrix.realEvidenceExpectation);
  if (!existsSync(realEvidencePath)) {
    fail(`Missing real evidence expectation: ${matrix.realEvidenceExpectation}`);
  }

  assertStringArray(matrix.requiredCompletedClaimIds, 'requiredCompletedClaimIds');
  assertUnique(matrix.requiredCompletedClaimIds, 'requiredCompletedClaimIds');

  if (!Array.isArray(matrix.claims) || matrix.claims.length === 0) {
    fail('claims must be a non-empty array.');
  }

  const claimIds = matrix.claims.map((claim) => claim.id);
  assertUnique(claimIds, 'claims.id');

  const claimIdSet = new Set(claimIds);
  for (const requiredClaimId of matrix.requiredCompletedClaimIds) {
    if (!claimIdSet.has(requiredClaimId)) {
      fail(`Missing completed claim proof entry: ${requiredClaimId}`);
    }
  }

  for (const claim of matrix.claims) {
    validateClaim(claim);
  }

  return validateCheckpointScenarios(matrix, { repoRoot });
}

try {
  const matrix = readMatrix();
  const checkpointSummary = validateMatrix(matrix);
  console.log(
    `pre-ai-proof-ok: ${matrix.claims.length} claims checked across ${requiredPlatforms.length} platforms; ` +
      `${checkpointSummary.scenarioCount} checkpoint scenarios checked.`
  );
} catch (error) {
  console.error('pre-ai-proof-failed');
  console.error(error.message);
  process.exit(1);
}
