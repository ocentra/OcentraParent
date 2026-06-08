import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, relative, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'screen-plan-proof', 'external-gates');
const manifestPath = join(outputDir, 'manual-evidence-manifest.json');
const proofPath = join(outputDir, 'proof-summary.json');

const requiredGates = [
  gate('macos-live-capture-permission', 'macos', 'platform-permission-prompt-screenshot', {
    workpack: '10 macOS capture adapter plan/proof',
    requirement:
      'real macOS ScreenCaptureKit session with Screen Recording permission, display/window pixels, OCR, and deletion proof',
  }),
  gate('linux-desktop-session-capture', 'linux-wayland', 'platform-session-recording', {
    workpack: '11 Linux capture adapter plan/proof',
    requirement: 'real Linux X11 or Wayland portal desktop-session capture with deletion proof',
  }),
  gate('android-physical-mediaprojection-capture', 'android-mediaprojection', 'physical-device-capture-recording', {
    workpack: '12 Android MediaProjection adapter plan/proof',
    requirement: 'real physical Android MediaProjection capture, stop callback, deletion, and local OCR proof',
  }),
  gate('ios-physical-replaykit-capture', 'ios-replaykit', 'physical-device-capture-recording', {
    workpack: '13 iOS ReplayKit adapter plan/proof',
    requirement: 'real physical iOS ReplayKit or broadcast-extension capture with deletion proof',
  }),
  gate('live-view-platform-prompt', 'android-mediaprojection', 'platform-permission-prompt-screenshot', {
    workpack: '28 Live view optional mode',
    requirement: 'real live-view platform prompt artifact, not ordinary capture-only permission evidence',
  }),
  gate('live-view-physical-device-parity', 'android-mediaprojection', 'physical-device-capture-recording', {
    workpack: '28 Live view optional mode',
    requirement: 'physical-device parity for live view transport/custody/deletion behavior',
  }),
  gate('live-view-hosted-relay-infrastructure', 'hosted-relay', 'hosted-relay-proof', {
    workpack: '28 Live view optional mode',
    requirement: 'hosted relay infrastructure proof with end-to-end encrypted custody and no raw-frame retention',
  }),
  gate('live-view-privacy-legal-approval', 'policy-approval', 'privacy-legal-approval', {
    workpack: '28 Live view optional mode',
    requirement: 'privacy/legal approval record for optional live view',
  }),
  gate('authenticated-account-social-capture', 'authenticated-social', 'authenticated-account-capture-proof', {
    workpack: '30 Test suite, Playwright, rollout, PR gate',
    requirement:
      'real logged-in social/feed account capture with operator consent, redacted account identifiers, local OCR/VLM analysis, policy dry-run, and raw image deletion proof',
  }),
];

const manifest = readManifestIfPresent();
const manifestEntries = Array.isArray(manifest?.entries) ? manifest.entries : [];
const gateResults = requiredGates.map((requiredGate) => validateGate(requiredGate, manifestEntries));
const negativeChecks = [
  rejects('fixture artifact path is rejected', () =>
    validateArtifact(requiredGates[0], {
      gateId: requiredGates[0].gateId,
      platform: requiredGates[0].platform,
      evidenceKind: requiredGates[0].evidenceKind,
      artifactPath: 'docs/fixtures/fake-prompt.html',
      artifactSha256: 'sha256-placeholder',
      capturedFromRealDeviceOrHost: true,
      capturesLiveSurface: true,
      rawPrivateContentIncluded: false,
    })
  ),
  rejects('raw private content is rejected', () =>
    validateArtifact(requiredGates[1], {
      gateId: requiredGates[1].gateId,
      platform: requiredGates[1].platform,
      evidenceKind: requiredGates[1].evidenceKind,
      artifactPath: 'output/screen-plan-proof/external-gates/artifacts/linux-proof.png',
      artifactSha256: 'sha256-placeholder',
      capturedFromRealDeviceOrHost: true,
      capturesLiveSurface: true,
      rawPrivateContentIncluded: true,
    })
  ),
  rejects('authenticated account proof without consent and redaction is rejected', () =>
    validateArtifact(requiredGates[8], {
      gateId: requiredGates[8].gateId,
      platform: requiredGates[8].platform,
      evidenceKind: requiredGates[8].evidenceKind,
      artifactPath: 'output/screen-plan-proof/external-gates/artifacts/authenticated-social-proof.json',
      artifactSha256: 'sha256-placeholder',
      capturedFromRealDeviceOrHost: true,
      capturesLiveSurface: true,
      rawPrivateContentIncluded: false,
      operatorConsentRecorded: false,
      redactedAccountIdentifiers: false,
    })
  ),
];

if (negativeChecks.some((check) => !check.rejected)) {
  throw new Error(`External gate negative check failed: ${JSON.stringify(negativeChecks)}`);
}

const satisfiedGateCount = gateResults.filter((result) => result.status === 'satisfied').length;
const missingGateCount = gateResults.filter((result) => result.status === 'missing').length;
const invalidGateCount = gateResults.filter((result) => result.status === 'invalid').length;

const summary = {
  proof: 'screen-plan-external-gates',
  generatedAt: new Date().toISOString(),
  manifest: {
    path: relativePath(manifestPath),
    present: manifest !== null,
    entryCount: manifestEntries.length,
  },
  gateResults,
  counts: {
    requiredGateCount: requiredGates.length,
    satisfiedGateCount,
    missingGateCount,
    invalidGateCount,
  },
  assertions: {
    allCurrentExternalGatesEnumerated: requiredGates.length === 9,
    authenticatedAccountSocialGateEnumerated: gateResults.some(
      (result) => result.gateId === 'authenticated-account-social-capture'
    ),
    productCompleteAllowed: satisfiedGateCount === requiredGates.length && invalidGateCount === 0,
    currentBranchMustRemainNonClaim: satisfiedGateCount !== requiredGates.length || invalidGateCount > 0,
    rejectsFixtureOrStaticEvidence: negativeChecks.every((check) => check.rejected),
  },
  negativeChecks,
  nonClaims: [
    'This proof does not fabricate or substitute platform evidence.',
    'This proof does not mark screen-plan product complete while any external gate is missing or invalid.',
    'Real external evidence must be attached through the manifest with digest-backed artifacts from live devices or live host sessions.',
  ],
};

mkdirSync(outputDir, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(`screen-plan-external-gates-proof-ok:${proofPath}`);

function gate(gateId, platform, evidenceKind, details) {
  return {
    gateId,
    platform,
    evidenceKind,
    ...details,
  };
}

function validateGate(requiredGate, entries) {
  const entry = entries.find((candidate) => candidate?.gateId === requiredGate.gateId);
  if (entry === undefined) {
    return {
      ...requiredGate,
      status: 'missing',
      artifactPath: null,
      reason: 'No matching manifest entry exists.',
    };
  }

  const validation = validateArtifact(requiredGate, entry);
  return {
    ...requiredGate,
    status: validation.ok ? 'satisfied' : 'invalid',
    artifactPath: typeof entry.artifactPath === 'string' ? normalizeArtifactPath(entry.artifactPath) : null,
    artifactSha256: typeof entry.artifactSha256 === 'string' ? entry.artifactSha256 : null,
    reason: validation.reason,
  };
}

function validateArtifact(requiredGate, entry) {
  if (entry.platform !== requiredGate.platform || entry.evidenceKind !== requiredGate.evidenceKind) {
    return rejected('platform or evidence kind does not match the required gate');
  }

  if (typeof entry.artifactPath !== 'string' || !artifactPathIsAllowed(entry.artifactPath)) {
    return rejected('artifact path must live under output/screen-plan-proof/external-gates/artifacts');
  }

  if (!artifactExtensionIsAllowed(requiredGate.evidenceKind, entry.artifactPath)) {
    return rejected('artifact extension is not allowed for the required evidence kind');
  }

  if (typeof entry.artifactSha256 !== 'string' || entry.artifactSha256.length < 16) {
    return rejected('artifact digest is missing or too short');
  }

  if (entry.capturedFromRealDeviceOrHost !== true || entry.capturesLiveSurface !== true) {
    return rejected('artifact must come from a real device or host and capture a live surface');
  }

  if (entry.rawPrivateContentIncluded !== false) {
    return rejected('artifact must not include raw private content');
  }

  if (
    requiredGate.evidenceKind === 'authenticated-account-capture-proof' &&
    (entry.operatorConsentRecorded !== true ||
      entry.redactedAccountIdentifiers !== true ||
      typeof entry.localAnalysisProofRef !== 'string' ||
      typeof entry.policyDryRunProofRef !== 'string' ||
      typeof entry.rawImageDeletionProofRef !== 'string')
  ) {
    return rejected(
      'authenticated account proof must record operator consent, redacted account identifiers, local analysis proof, policy dry-run proof, and raw image deletion proof'
    );
  }

  const absoluteArtifactPath = resolve(repoRoot, entry.artifactPath);
  if (!existsSync(absoluteArtifactPath)) {
    return rejected('artifact file is not present in the current checkout');
  }

  if (statSync(absoluteArtifactPath).size === 0) {
    return rejected('artifact file is empty');
  }

  const digest = createHash('sha256').update(readFileSync(absoluteArtifactPath)).digest('hex');
  if (entry.artifactSha256 !== digest) {
    return rejected('artifact digest does not match the current file bytes');
  }

  return { ok: true, reason: 'Artifact entry satisfies the current gate contract.' };
}

function artifactPathIsAllowed(path) {
  const normalized = normalizeArtifactPath(path);
  if (!normalized.startsWith('output/screen-plan-proof/external-gates/artifacts/')) {
    return false;
  }

  const lowerName = basename(normalized).toLowerCase();
  return !lowerName.includes('fixture') && !lowerName.includes('fake') && !lowerName.includes('placeholder');
}

function artifactExtensionIsAllowed(evidenceKind, path) {
  const lowerPath = path.toLowerCase();
  if (evidenceKind === 'privacy-legal-approval' || evidenceKind === 'hosted-relay-proof') {
    return lowerPath.endsWith('.json') || lowerPath.endsWith('.md');
  }

  if (evidenceKind === 'authenticated-account-capture-proof') {
    return (
      lowerPath.endsWith('.json') ||
      lowerPath.endsWith('.md') ||
      lowerPath.endsWith('.png') ||
      lowerPath.endsWith('.jpg') ||
      lowerPath.endsWith('.jpeg') ||
      lowerPath.endsWith('.webp')
    );
  }

  if (evidenceKind === 'platform-session-recording' || evidenceKind === 'physical-device-capture-recording') {
    return (
      lowerPath.endsWith('.png') ||
      lowerPath.endsWith('.jpg') ||
      lowerPath.endsWith('.jpeg') ||
      lowerPath.endsWith('.webp') ||
      lowerPath.endsWith('.mp4') ||
      lowerPath.endsWith('.mov')
    );
  }

  return (
    lowerPath.endsWith('.png') ||
    lowerPath.endsWith('.jpg') ||
    lowerPath.endsWith('.jpeg') ||
    lowerPath.endsWith('.webp')
  );
}

function readManifestIfPresent() {
  if (!existsSync(manifestPath)) {
    return null;
  }

  return JSON.parse(readFileSync(manifestPath, 'utf8'));
}

function normalizeArtifactPath(path) {
  return path.replaceAll('\\', '/');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function rejected(reason) {
  return { ok: false, reason };
}

function rejects(name, validator) {
  return { name, rejected: validator().ok === false };
}
