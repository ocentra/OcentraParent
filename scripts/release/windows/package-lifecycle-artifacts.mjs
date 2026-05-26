import { createHash } from 'node:crypto';
import { existsSync, readFileSync, statSync } from 'node:fs';
import { basename, join, resolve } from 'node:path';

export const WINDOWS_PREVIEW_ARTIFACT_NAME = 'ocentra-parent-windows-x64-preview';
export const WINDOWS_MANIFEST_NAME = 'latest-windows.json';
export const WINDOWS_LATEST_MSI_NAME = 'ocentra-parent-agent-windows-x64-latest.msi';
export const WINDOWS_BOOTSTRAP_NAME = 'install-ocentra-parent-agent-windows.ps1';
export const WINDOWS_AGENT_SERVICE_ID = 'OcentraParentAgent';
export const WINDOWS_UPDATER_SERVICE_ID = 'OcentraParentUpdater';

export class PackageLifecycleProofError extends Error {
  constructor(code, message) {
    super(message);
    this.name = 'PackageLifecycleProofError';
    this.code = code;
  }
}

export function normalizeSha256(value) {
  const normalized = String(value ?? '')
    .trim()
    .toUpperCase();
  if (!/^[A-F0-9]{64}$/u.test(normalized)) {
    throw new PackageLifecycleProofError('invalid-sha256', `Invalid SHA-256 value: ${value}`);
  }
  return normalized;
}

export function sha256File(path) {
  const hash = createHash('sha256');
  hash.update(readFileSync(path));
  return hash.digest('hex').toUpperCase();
}

export function readJsonFile(path) {
  try {
    return JSON.parse(readFileSync(path, 'utf8').replace(/^\uFEFF/u, ''));
  } catch (error) {
    throw new PackageLifecycleProofError('json-parse-failed', `${path}: ${error.message}`);
  }
}

export function parseChecksumLine(text) {
  const line = String(text).trim().split(/\r?\n/u).find(Boolean);
  const match = /^([A-Fa-f0-9]{64})\s+\*?(.+)$/u.exec(line ?? '');
  if (!match) {
    throw new PackageLifecycleProofError(
      'checksum-format-invalid',
      'Checksum sidecar must contain "<sha256>  <file>".'
    );
  }
  return {
    sha256: normalizeSha256(match[1]),
    fileName: match[2].trim(),
  };
}

export function verifyChecksumSidecar({ sidecarPath, artifactPath }) {
  assertExistingFile(sidecarPath, 'checksum-sidecar-missing');
  assertExistingFile(artifactPath, 'artifact-file-missing');
  const parsed = parseChecksumLine(readFileSync(sidecarPath, 'utf8'));
  const expectedFileName = basename(artifactPath);
  if (parsed.fileName !== expectedFileName) {
    throw new PackageLifecycleProofError(
      'checksum-file-name-mismatch',
      `Checksum sidecar names ${parsed.fileName}, expected ${expectedFileName}.`
    );
  }
  const actualSha256 = sha256File(artifactPath);
  if (actualSha256 !== parsed.sha256) {
    throw new PackageLifecycleProofError(
      'checksum-sha256-mismatch',
      `Checksum sidecar expected ${parsed.sha256}, found ${actualSha256}.`
    );
  }
  return {
    fileName: parsed.fileName,
    path: sidecarPath,
    sha256: parsed.sha256,
    status: 'verified',
  };
}

export function inspectWindowsPreviewArtifact(artifactDirectory) {
  const artifactRoot = resolve(artifactDirectory);
  const manifestPath = join(artifactRoot, WINDOWS_MANIFEST_NAME);
  const latestMsiPath = join(artifactRoot, WINDOWS_LATEST_MSI_NAME);
  const latestChecksumPath = `${latestMsiPath}.sha256`;
  const bootstrapPath = join(artifactRoot, WINDOWS_BOOTSTRAP_NAME);

  assertExistingFile(manifestPath, 'manifest-missing');
  assertExistingFile(latestMsiPath, 'latest-msi-missing');
  assertExistingFile(latestChecksumPath, 'latest-checksum-missing');
  assertExistingFile(bootstrapPath, 'bootstrap-missing');

  const manifest = readJsonFile(manifestPath);
  const payload = validateManifest(manifest);
  const versionedMsiPath = join(artifactRoot, payload.artifact.name);
  const versionedChecksumPath = `${versionedMsiPath}.sha256`;
  assertExistingFile(versionedMsiPath, 'versioned-msi-missing');
  assertExistingFile(versionedChecksumPath, 'versioned-checksum-missing');

  const expectedSha256 = normalizeSha256(payload.artifact.sha256);
  const versionedSha256 = sha256File(versionedMsiPath);
  const latestSha256 = sha256File(latestMsiPath);
  if (versionedSha256 !== expectedSha256) {
    throw new PackageLifecycleProofError(
      'manifest-artifact-sha256-mismatch',
      `Manifest expected ${expectedSha256}, versioned MSI hash is ${versionedSha256}.`
    );
  }
  if (latestSha256 !== expectedSha256) {
    throw new PackageLifecycleProofError(
      'latest-artifact-sha256-mismatch',
      `Manifest expected ${expectedSha256}, latest MSI hash is ${latestSha256}.`
    );
  }

  const latestSidecar = verifyChecksumSidecar({
    sidecarPath: latestChecksumPath,
    artifactPath: latestMsiPath,
  });
  const versionedSidecar = verifyChecksumSidecar({
    sidecarPath: versionedChecksumPath,
    artifactPath: versionedMsiPath,
  });
  const bootstrap = inspectBootstrap(bootstrapPath);

  return {
    artifactRoot,
    bootstrap,
    files: {
      bootstrap: fileRecord(bootstrapPath),
      latestMsi: fileRecord(latestMsiPath, latestSha256),
      latestChecksum: fileRecord(latestChecksumPath),
      manifest: fileRecord(manifestPath),
      versionedMsi: fileRecord(versionedMsiPath, versionedSha256),
      versionedChecksum: fileRecord(versionedChecksumPath),
    },
    manifest: {
      artifactName: payload.artifact.name,
      channel: payload.channel,
      generatedAt: payload.generatedAt,
      installer: payload.installer,
      package: payload.package,
      product: payload.product,
      schemaVersion: payload.schemaVersion,
      service: payload.service,
      signature: {
        algorithm: manifest.signature.algorithm,
        keyId: manifest.signature.keyId,
        status: 'present',
      },
      target: payload.target,
      version: payload.version,
    },
    sidecars: [latestSidecar, versionedSidecar],
    status: 'verified',
  };
}

function validateManifest(manifest) {
  if (!manifest || typeof manifest !== 'object') {
    throw new PackageLifecycleProofError('manifest-shape-invalid', 'Manifest root must be an object.');
  }
  if (!manifest.payload || !manifest.signature) {
    throw new PackageLifecycleProofError('manifest-unsigned', 'Manifest must contain payload and signature.');
  }
  const { payload, signature } = manifest;
  assertField(payload.schemaVersion, 1, 'manifest-schema-version-invalid');
  assertField(payload.product, 'Ocentra Parent', 'manifest-product-invalid');
  assertField(payload.package, 'ocentra-parent-agent', 'manifest-package-invalid');
  assertField(payload.target, 'windows-x64', 'manifest-target-invalid');
  assertField(payload.installer?.type, 'msi', 'manifest-installer-type-invalid');
  assertField(payload.installer?.scope, 'per-machine', 'manifest-installer-scope-invalid');
  assertField(payload.installer?.silentArgs, '/qn /norestart', 'manifest-silent-args-invalid');
  assertField(payload.service?.id, WINDOWS_AGENT_SERVICE_ID, 'manifest-service-id-invalid');
  assertField(payload.service?.updaterId, WINDOWS_UPDATER_SERVICE_ID, 'manifest-updater-id-invalid');
  assertNonEmpty(payload.version, 'manifest-version-missing');
  assertNonEmpty(payload.artifact?.name, 'manifest-artifact-name-missing');
  normalizeSha256(payload.artifact?.sha256);
  assertNonEmpty(payload.artifact?.downloadUrl, 'manifest-download-url-missing');
  assertNonEmpty(signature.algorithm, 'manifest-signature-algorithm-missing');
  assertNonEmpty(signature.keyId, 'manifest-signature-key-id-missing');
  assertNonEmpty(signature.value, 'manifest-signature-value-missing');
  return payload;
}

function inspectBootstrap(bootstrapPath) {
  const text = readFileSync(bootstrapPath, 'utf8');
  for (const pattern of ['Release manifest is not signed', 'msiexec.exe', '/qn', '/norestart']) {
    if (!text.includes(pattern)) {
      throw new PackageLifecycleProofError('bootstrap-policy-missing', `Bootstrap installer is missing: ${pattern}`);
    }
  }
  return {
    path: bootstrapPath,
    status: 'verified',
  };
}

function fileRecord(path, sha256 = undefined) {
  const stats = statSync(path);
  return {
    name: basename(path),
    path,
    sha256,
    size: stats.size,
  };
}

function assertExistingFile(path, code) {
  if (!existsSync(path) || !statSync(path).isFile()) {
    throw new PackageLifecycleProofError(code, `Required file is missing: ${path}`);
  }
}

function assertField(actual, expected, code) {
  if (actual !== expected) {
    throw new PackageLifecycleProofError(code, `Expected ${expected}, found ${actual ?? '<missing>'}.`);
  }
}

function assertNonEmpty(actual, code) {
  if (typeof actual !== 'string' || actual.trim() === '') {
    throw new PackageLifecycleProofError(code, `Required manifest field is empty.`);
  }
}
