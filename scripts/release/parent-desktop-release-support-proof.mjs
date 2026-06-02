#!/usr/bin/env node

import { mkdirSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

export const SUPPORT_DIAGNOSTIC_ALLOWED_FIELDS = Object.freeze([
  'appVersion',
  'serviceVersion',
  'platform',
  'packageSource',
  'releaseChannel',
  'mode',
  'featureFlags',
  'correlationId',
  'schemaVersions',
  'eventTypes',
  'redactionsApplied',
]);

export const SUPPORT_DIAGNOSTIC_FORBIDDEN_TERMS = Object.freeze([
  'token',
  'secret',
  'childname',
  'childactivity',
  'rawurl',
  'screenshot',
  'journal',
  'sqlite',
  'privatepath',
  'commandline',
  'keystroke',
  'clipboard',
  'messagecontent',
]);

export function redactSupportDiagnostic(diagnostic) {
  const source = isRecord(diagnostic) ? diagnostic : {};
  const redactions = new Set();
  collectForbiddenTerms(source, redactions);

  const redacted = {};
  for (const field of SUPPORT_DIAGNOSTIC_ALLOWED_FIELDS) {
    if (field === 'redactionsApplied') {
      continue;
    }
    if (Object.hasOwn(source, field)) {
      redacted[field] = safeDiagnosticValue(source[field], redactions);
    }
  }
  redacted.redactionsApplied = Array.from(redactions).sort();
  return redacted;
}

export function buildReleaseSupportProof(options = {}) {
  return {
    branchBoundary: buildBranchBoundary(),
    ciArtifactProof: buildCiArtifactProof(),
    generatedAt: options.generatedAt ?? new Date().toISOString(),
    manualProofRunbook: buildManualProofRunbook(),
    platformCapabilityMatrix: buildPlatformCapabilityMatrix(),
    schemaVersion: 1,
    signingStoreClaims: buildSigningStoreClaims(),
    supportDiagnostics: {
      allowedFields: SUPPORT_DIAGNOSTIC_ALLOWED_FIELDS,
      forbiddenTerms: SUPPORT_DIAGNOSTIC_FORBIDDEN_TERMS,
      sample: redactSupportDiagnostic(options.sampleDiagnostic ?? {}),
    },
    updateChannelRollback: buildUpdateChannelRollback(),
    workpacks: ['09', '10', '11', '12', '15', '16', '17', '18'],
  };
}

export function writeReleaseSupportProof(options = {}) {
  const outputDirectory = resolve(
    options.outputDirectory ?? join('test-results', 'parent-desktop-release-support-proof')
  );
  mkdirSync(outputDirectory, { recursive: true });
  const proof = buildReleaseSupportProof(options);
  const proofPath = join(outputDirectory, 'proof.json');
  writeFileSync(`${proofPath}`, `${JSON.stringify(proof, null, 2)}\n`, 'utf8');
  return proofPath;
}

function buildUpdateChannelRollback() {
  return {
    productionUpdate: {
      channel: 'production',
      manifestSignature: 'required',
      rollback: 'manual-required-until-signed-channel-proof',
      unsignedPreviewAccepted: false,
    },
    previewUpdate: {
      channel: 'main-package-preview',
      manifestSignature: 'ephemeral-preview-key-only',
      rollback: 'scaffolded-not-product-claimed',
      unsignedPreviewAccepted: true,
    },
  };
}

function buildSigningStoreClaims() {
  return [
    claimRow('windows-production-signing', 'parent-desktop', 'windows', 'manual-required'),
    claimRow('macos-notarization', 'parent-desktop', 'macos', 'manual-required'),
    claimRow('android-play-signing', 'child-mobile', 'android', 'manual-required'),
    claimRow('ios-testflight-store', 'child-mobile', 'ios', 'manual-required'),
  ];
}

function buildPlatformCapabilityMatrix() {
  return [
    matrixRow('windows-msi-preview', 'parent-desktop', 'windows', 'implemented-preview'),
    matrixRow('linux-deb-preview', 'parent-desktop', 'linux', 'implemented-preview'),
    matrixRow('macos-pkg-preview', 'parent-desktop', 'macos', 'implemented-preview'),
    matrixRow('android-debug-apk-preview', 'child-mobile', 'android', 'scaffold-only'),
    matrixRow('ios-simulator-preview', 'child-mobile', 'ios', 'scaffold-only'),
    matrixRow('production-signing-and-stores', 'release', 'cross-platform', 'manual-required'),
    matrixRow('support-diagnostic-bundle', 'support', 'cross-platform', 'scaffolded-contract'),
  ];
}

function buildBranchBoundary() {
  return {
    main: {
      packagePreview: true,
      productionPublish: false,
      role: 'ci-preview-integration',
    },
    production: {
      packagePreview: true,
      productionPublish: true,
      role: 'explicit-promotion-release',
      tagDecisionRequired: true,
    },
  };
}

function buildCiArtifactProof() {
  return {
    packagePreviewWorkflow: '.github/workflows/package-preview.yml',
    previewArtifacts: [
      'ocentra-parent-windows-x64-preview',
      'ocentra-parent-linux-amd64-preview',
      'ocentra-parent-macos-preview',
      'ocentra-parent-android-preview',
      'ocentra-parent-ios-simulator-preview',
    ],
    productionWorkflow: '.github/workflows/release.yml',
    requiredProductionSecrets: ['OCENTRA_PARENT_UPDATE_SIGNING_KEY_BASE64'],
  };
}

function buildManualProofRunbook() {
  return {
    requiredFields: [
      'platform',
      'commit',
      'version',
      'commandOrUiAction',
      'permissionState',
      'logOrScreenshotArtifact',
      'proofJsonPath',
      'observedResult',
    ],
    status: 'manual-required-for-platform-product-claims',
  };
}

function claimRow(id, surface, platform, state) {
  return { id, platform, productionClaim: false, state, surface };
}

function matrixRow(id, surface, platform, state) {
  return {
    id,
    platform,
    productClaim: state === 'implemented-preview' ? 'preview-only' : 'not-claimed',
    state,
    surface,
  };
}

function safeDiagnosticValue(value, redactions) {
  if (Array.isArray(value)) {
    return value.map((entry) => safeDiagnosticValue(entry, redactions));
  }
  if (!isRecord(value)) {
    return value;
  }
  const sanitized = {};
  for (const [key, nested] of Object.entries(value)) {
    const normalized = key.toLowerCase();
    if (SUPPORT_DIAGNOSTIC_FORBIDDEN_TERMS.some((term) => normalized.includes(term))) {
      redactions.add(normalized);
      continue;
    }
    sanitized[key] = safeDiagnosticValue(nested, redactions);
  }
  return sanitized;
}

function collectForbiddenTerms(value, redactions) {
  if (Array.isArray(value)) {
    for (const entry of value) {
      collectForbiddenTerms(entry, redactions);
    }
    return;
  }
  if (!isRecord(value)) {
    return;
  }
  for (const [key, nested] of Object.entries(value)) {
    const normalized = key.toLowerCase();
    for (const term of SUPPORT_DIAGNOSTIC_FORBIDDEN_TERMS) {
      if (normalized.includes(term)) {
        redactions.add(normalized);
      }
    }
    collectForbiddenTerms(nested, redactions);
  }
}

function isRecord(value) {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function parseArgs(args) {
  const parsed = {};
  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (arg !== '--out-dir') {
      throw new Error(`Unknown argument: ${arg}`);
    }
    const next = args[index + 1];
    if (next === undefined) {
      throw new Error('Missing value for --out-dir.');
    }
    parsed.outputDirectory = next;
    index += 1;
  }
  return parsed;
}

async function main() {
  const proofPath = writeReleaseSupportProof(parseArgs(process.argv.slice(2)));
  console.log(`parent-desktop-release-support-proof=${proofPath}`);
}

const invokedPath = process.argv[1] ? pathToFileURL(resolve(process.argv[1])).href : '';
if (import.meta.url === invokedPath || fileURLToPath(import.meta.url) === process.argv[1]) {
  main().catch((error) => {
    console.error(error.message);
    process.exitCode = 1;
  });
}
