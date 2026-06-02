import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import {
  buildReleaseSupportProof,
  redactSupportDiagnostic,
  writeReleaseSupportProof,
} from '../release/parent-desktop-release-support-proof.mjs';

const repoRoot = process.cwd();

test('support diagnostic redaction keeps only support-safe fields', () => {
  const redacted = redactSupportDiagnostic({
    appVersion: '0.1.1',
    authToken: 'do-not-keep',
    correlationId: 'support-case-123',
    eventTypes: ['service.health.changed', 'package.preview.generated'],
    featureFlags: {
      desktopRuntime: true,
      privatePath: 'C:\\Users\\child\\Downloads',
    },
    journalPath: 'agent-journal.ndjson',
    mode: 'preview',
    packageSource: 'package-preview',
    platform: 'windows',
    rawChildActivity: {
      rawUrl: 'https://example.invalid/private',
    },
    releaseChannel: 'main-package-preview',
    schemaVersions: {
      support: 1,
    },
    screenshotBytes: 'base64',
    serviceVersion: '0.1.1',
    sqliteSnapshot: 'binary',
  });

  assert.deepEqual(redacted, {
    appVersion: '0.1.1',
    correlationId: 'support-case-123',
    eventTypes: ['service.health.changed', 'package.preview.generated'],
    featureFlags: {
      desktopRuntime: true,
    },
    mode: 'preview',
    packageSource: 'package-preview',
    platform: 'windows',
    redactionsApplied: [
      'authtoken',
      'journalpath',
      'privatepath',
      'rawchildactivity',
      'rawurl',
      'screenshotbytes',
      'sqlitesnapshot',
    ],
    releaseChannel: 'main-package-preview',
    schemaVersions: {
      support: 1,
    },
    serviceVersion: '0.1.1',
  });
});

test('release-support proof separates preview mechanics from product claims', () => {
  const proof = buildReleaseSupportProof({
    generatedAt: '2026-06-02T00:00:00.000Z',
  });
  const matrixRows = Object.fromEntries(proof.platformCapabilityMatrix.map((row) => [row.id, row]));

  assert.deepEqual(proof.workpacks, ['09', '10', '11', '12', '15', '16', '17', '18']);
  assert.equal(proof.branchBoundary.main.productionPublish, false);
  assert.equal(proof.branchBoundary.production.productionPublish, true);
  assert.equal(proof.updateChannelRollback.productionUpdate.manifestSignature, 'required');
  assert.equal(proof.updateChannelRollback.productionUpdate.unsignedPreviewAccepted, false);
  assert.equal(matrixRows['windows-msi-preview'].state, 'implemented-preview');
  assert.equal(matrixRows['android-debug-apk-preview'].state, 'scaffold-only');
  assert.equal(matrixRows['ios-simulator-preview'].state, 'scaffold-only');
  assert.equal(matrixRows['production-signing-and-stores'].state, 'manual-required');
  assert.equal(matrixRows['support-diagnostic-bundle'].state, 'scaffolded-contract');
  assert.deepEqual(
    proof.signingStoreClaims.map((claim) => [claim.id, claim.productionClaim]),
    [
      ['windows-production-signing', false],
      ['macos-notarization', false],
      ['android-play-signing', false],
      ['ios-testflight-store', false],
    ]
  );
});

test('release-support proof matches current CI preview and production release boundaries', () => {
  const packagePreviewWorkflow = readFileSync(join(repoRoot, '.github', 'workflows', 'package-preview.yml'), 'utf8');
  const releaseWorkflow = readFileSync(join(repoRoot, '.github', 'workflows', 'release.yml'), 'utf8');
  const productionSecrets = readFileSync(join(repoRoot, 'scripts', 'release', 'check-production-secrets.mjs'), 'utf8');
  const proof = buildReleaseSupportProof();

  for (const artifactName of proof.ciArtifactProof.previewArtifacts) {
    assert.match(packagePreviewWorkflow, new RegExp(`name: ${escapeRegex(artifactName)}`, 'u'));
  }
  assert.match(releaseWorkflow, /branches:\s*\n\s+- production/u);
  assert.match(releaseWorkflow, /Check production release secrets/u);
  assert.match(productionSecrets, /OCENTRA_PARENT_UPDATE_SIGNING_KEY_BASE64/u);
  assert.match(productionSecrets, /OCENTRA_PARENT_MACOS_DEVELOPER_ID_CERT_BASE64/u);
  assert.match(productionSecrets, /OCENTRA_PARENT_ANDROID_RELEASE_KEYSTORE_BASE64/u);
  assert.match(productionSecrets, /OCENTRA_PARENT_APPLE_API_PRIVATE_KEY_BASE64/u);
});

test('manual platform proof runbook names the evidence needed before product claims', () => {
  const proof = buildReleaseSupportProof();

  assert.deepEqual(proof.manualProofRunbook.requiredFields, [
    'platform',
    'commit',
    'version',
    'commandOrUiAction',
    'permissionState',
    'logOrScreenshotArtifact',
    'proofJsonPath',
    'observedResult',
  ]);
  assert.equal(proof.manualProofRunbook.status, 'manual-required-for-platform-product-claims');
});

test('release-support CLI writer emits proof JSON for handoff artifacts', () => {
  const outputDirectory = mkdtempSync(join(tmpdir(), 'ocentra-release-support-proof-'));

  try {
    const proofPath = writeReleaseSupportProof({
      generatedAt: '2026-06-02T00:00:00.000Z',
      outputDirectory,
    });
    const proof = JSON.parse(readFileSync(proofPath, 'utf8'));

    assert.equal(proof.generatedAt, '2026-06-02T00:00:00.000Z');
    assert.equal(proof.schemaVersion, 1);
    assert.equal(proof.branchBoundary.main.role, 'ci-preview-integration');
    assert.equal(proof.branchBoundary.production.role, 'explicit-promotion-release');
  } finally {
    rmSync(outputDirectory, { force: true, recursive: true });
  }
});

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
}
