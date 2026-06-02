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
    packageRuntimeEvidence: buildPackageRuntimeEvidence(),
    platformCapabilityMatrix: buildPlatformCapabilityMatrix(),
    schemaVersion: 1,
    signingStoreClaims: buildSigningStoreClaims(),
    supportDiagnostics: {
      allowedFields: SUPPORT_DIAGNOSTIC_ALLOWED_FIELDS,
      forbiddenTerms: SUPPORT_DIAGNOSTIC_FORBIDDEN_TERMS,
      sample: redactSupportDiagnostic(options.sampleDiagnostic ?? {}),
    },
    updateChannelRollback: buildUpdateChannelRollback(),
    workpacks: {
      completed: ['04', '06', '09', '10', '11', '12', '15', '16', '17', '18', '20'],
      partial: ['19'],
      partialReason:
        'docs/product-capability-checklist.md is currently locked by codex-a; feature docs carry D-owned state and checklist wording must be reconciled by primary after lock release.',
    },
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

function buildPackageRuntimeEvidence() {
  return {
    packageFrontendSource: 'built-portal-dist',
    backendBoundary: 'rust-service-boundary',
    serviceLaunchOwner: 'package-service-manager',
    serviceHealthState: 'implemented',
    connectOrDegradeState: 'degraded',
    fixedAgentAddress: '127.0.0.1:4477',
    portOwnership: 'fixed-loopback',
    portConflictPolicy: 'no-foreign-process-reclaim',
    processOwnership: 'parent-shell-only',
    blankWindowGuard: 'frontend-dist-required',
    updateRollbackPosture: 'signed-channel-required',
    artifactState: 'manual-required',
    supportDiagnosticState: 'preview-only',
    nonClaim: 'CI package preview is not signing not production not store distribution proof',
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
    matrixRow('parent-desktop', 'unsigned-preview', 'implemented', 'preview-only', 'preview-only'),
    matrixRow('parent-mobile', 'scaffold', 'manual-required', 'manual-required', 'manual-required'),
    matrixRow('child-desktop', 'preview-only', 'implemented', 'preview-only', 'manual-required'),
    matrixRow('child-android', 'scaffold', 'manual-required', 'manual-required', 'manual-required'),
    matrixRow('child-ios', 'scaffold', 'manual-required', 'manual-required', 'manual-required'),
    matrixRow('relay', 'not-implemented', 'not-implemented', 'not-implemented', 'not-ready'),
    matrixRow('signing', 'signature-required', 'manual-required', 'manual-required', 'manual-required'),
    matrixRow('store', 'manual-required', 'manual-required', 'manual-required', 'manual-required'),
    matrixRow('support', 'preview-only', 'preview-only', 'preview-only', 'preview-only'),
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

function matrixRow(target, packageState, serviceState, capabilityState, proofLevel) {
  return {
    target,
    packageState,
    serviceState,
    routeState: target === 'relay' ? 'not-implemented' : 'preview-only',
    capabilityState,
    proofLevel,
    nonClaim: `${target} state is limited to the named proof level and does not upgrade unsupported platform behavior`,
  };
}

function safeDiagnosticValue(value, redactions) {
  if (Array.isArray(value)) {
    return value.map((entry) => safeDiagnosticValue(entry, redactions));
  }
  if (typeof value === 'string') {
    return safeDiagnosticString(value, redactions);
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

function safeDiagnosticString(value, redactions) {
  const normalized = value.toLowerCase().replace(/[\s_-]/gu, '');
  const term = SUPPORT_DIAGNOSTIC_FORBIDDEN_TERMS.find((forbidden) => normalized.includes(forbidden));
  if (term !== undefined) {
    redactions.add(term);
    return '[redacted]';
  }
  if (/https?:\/\//iu.test(value)) {
    redactions.add('rawurl');
    return '[redacted]';
  }
  if (/[A-Za-z]:\\/u.test(value)) {
    redactions.add('privatepath');
    return '[redacted]';
  }
  if (/\bbearer\s+\S+/iu.test(value) || /sk-[A-Za-z0-9]/u.test(value)) {
    redactions.add('token');
    return '[redacted]';
  }
  return value;
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
