import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { LocalArtifactProviderProtocolVersion } from './local-artifact-provider-protocol';

export interface ProviderManifest {
  readonly schemaVersion: 1;
  readonly protocolVersion: number;
  readonly packageName: '@ocentra-parent/logging-domain';
  readonly packageVersion: string;
  readonly providerPackageName: 'ocentra-logging-local-artifact-provider';
  readonly platform: 'win32';
  readonly binaryPath: string;
  readonly binarySha256: string;
}

export interface LoggingPackageIdentity {
  readonly name: '@ocentra-parent/logging-domain';
  readonly version: string;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value != null && !Array.isArray(value);
}

function hasOnlyKeys(value: Record<string, unknown>, keys: readonly string[]): boolean {
  const allowed = new Set(keys);
  return Object.keys(value).every((key) => allowed.has(key));
}

export function parseProviderManifest(value: unknown): ProviderManifest | null {
  if (!isRecord(value)) return null;
  if (
    !hasOnlyKeys(value, [
      'schemaVersion',
      'protocolVersion',
      'packageName',
      'packageVersion',
      'providerPackageName',
      'platform',
      'binaryPath',
      'binarySha256',
    ])
  ) {
    return null;
  }
  const packageVersion = value['packageVersion'];
  const binaryPath = value['binaryPath'];
  const binarySha256 = value['binarySha256'];
  const fieldsAreValid = [
    value['schemaVersion'] === 1,
    value['protocolVersion'] === LocalArtifactProviderProtocolVersion,
    value['packageName'] === '@ocentra-parent/logging-domain',
    value['providerPackageName'] === 'ocentra-logging-local-artifact-provider',
    value['platform'] === 'win32',
    typeof packageVersion === 'string',
    typeof binaryPath === 'string',
    typeof binarySha256 === 'string',
  ].every(Boolean);
  if (
    !fieldsAreValid ||
    typeof packageVersion !== 'string' ||
    typeof binaryPath !== 'string' ||
    typeof binarySha256 !== 'string'
  ) {
    return null;
  }
  if (packageVersion.length === 0) return null;
  return {
    schemaVersion: 1,
    protocolVersion: LocalArtifactProviderProtocolVersion,
    packageName: '@ocentra-parent/logging-domain',
    packageVersion,
    providerPackageName: 'ocentra-logging-local-artifact-provider',
    platform: 'win32',
    binaryPath,
    binarySha256,
  };
}

export function loggingPackageRoot(): string {
  return path.dirname(fileURLToPath(new URL('../package.json', import.meta.url)));
}

export function readStableJsonFile(filePath: string, maximumBytes: number): unknown {
  const descriptor = fs.openSync(filePath, 'r');
  try {
    const opened = fs.fstatSync(descriptor);
    const boundPath = fs.lstatSync(filePath);
    const openIdentityChanged = [
      !opened.isFile(),
      opened.nlink !== 1,
      opened.size <= 0,
      opened.size > maximumBytes,
      !boundPath.isFile(),
      boundPath.isSymbolicLink(),
      boundPath.nlink !== 1,
      opened.dev !== boundPath.dev,
      opened.ino !== boundPath.ino,
      opened.size !== boundPath.size,
    ].some(Boolean);
    if (openIdentityChanged) {
      throw new Error('provider metadata file is not a bounded private regular file');
    }
    const content = fs.readFileSync(descriptor, 'utf8');
    const after = fs.fstatSync(descriptor);
    const pathAfter = fs.lstatSync(filePath);
    const readIdentityChanged = [
      after.dev !== opened.dev,
      after.ino !== opened.ino,
      after.size !== opened.size,
      after.mtimeMs !== opened.mtimeMs,
      after.nlink !== 1,
      !pathAfter.isFile(),
      pathAfter.isSymbolicLink(),
      pathAfter.nlink !== 1,
      pathAfter.dev !== after.dev,
      pathAfter.ino !== after.ino,
    ].some(Boolean);
    if (readIdentityChanged) {
      throw new Error('provider metadata identity changed while reading');
    }
    const parsed: unknown = JSON.parse(content);
    return parsed;
  } finally {
    fs.closeSync(descriptor);
  }
}

export function loggingPackageIdentity(): LoggingPackageIdentity | null {
  try {
    const packageJsonPath = fileURLToPath(new URL('../package.json', import.meta.url));
    const packageJson = readStableJsonFile(packageJsonPath, 256 * 1024);
    if (!isRecord(packageJson)) return null;
    const packageVersion = packageJson['version'];
    if (
      packageJson['name'] !== '@ocentra-parent/logging-domain' ||
      typeof packageVersion !== 'string' ||
      packageVersion.length === 0
    ) {
      return null;
    }
    return {
      name: '@ocentra-parent/logging-domain',
      version: packageVersion,
    };
  } catch {
    return null;
  }
}

export function providerManifestPaths(): readonly string[] {
  return [path.join(loggingPackageRoot(), 'dist', 'local-artifact-provider.manifest.json')];
}
