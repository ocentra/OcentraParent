import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

export interface ProviderBinary {
  readonly path: string;
  readonly sha256: string;
}

function isContained(root: string, target: string): boolean {
  const relative = path.relative(root, target);
  return [
    relative.length > 0,
    relative !== '..',
    !path.isAbsolute(relative),
    !relative.startsWith(`..${path.sep}`),
  ].every(Boolean);
}

function hashOpenedFile(descriptor: number): string {
  return crypto.createHash('sha256').update(fs.readFileSync(descriptor)).digest('hex');
}

function configuredBinaryPathIsValid(configuredPath: string, expectedSha256: string): boolean {
  const normalizedConfiguredPath = path.posix.normalize(configuredPath);
  return [
    !path.isAbsolute(configuredPath),
    configuredPath.length > 0,
    !configuredPath.includes('\\'),
    !configuredPath.includes(':'),
    normalizedConfiguredPath === configuredPath,
    normalizedConfiguredPath !== '..',
    !normalizedConfiguredPath.startsWith('../'),
    /^[0-9a-f]{64}$/u.test(expectedSha256),
  ].every(Boolean);
}

export function validateProviderBinary(
  manifestDirectory: string,
  configuredPath: string,
  expectedSha256: string
): ProviderBinary | null {
  if (!configuredBinaryPathIsValid(configuredPath, expectedSha256)) return null;
  const root = fs.realpathSync.native(manifestDirectory);
  const candidate = path.resolve(root, configuredPath);
  if (!isContained(root, candidate)) return null;
  const descriptor = fs.openSync(candidate, 'r');
  try {
    const opened = fs.fstatSync(descriptor);
    const resolved = fs.realpathSync.native(candidate);
    const boundPath = fs.lstatSync(candidate);
    const openIdentityChanged = [
      !isContained(root, resolved),
      !opened.isFile(),
      opened.nlink !== 1,
      !boundPath.isFile(),
      boundPath.isSymbolicLink(),
      boundPath.nlink !== 1,
      opened.dev !== boundPath.dev,
      opened.ino !== boundPath.ino,
    ].some(Boolean);
    if (openIdentityChanged) return null;
    const sha256 = hashOpenedFile(descriptor);
    const after = fs.fstatSync(descriptor);
    const readIdentityChanged = [
      after.dev !== opened.dev,
      after.ino !== opened.ino,
      after.size !== opened.size,
      after.mtimeMs !== opened.mtimeMs,
      after.nlink !== 1,
      sha256 !== expectedSha256,
    ].some(Boolean);
    return readIdentityChanged ? null : { path: resolved, sha256 };
  } finally {
    fs.closeSync(descriptor);
  }
}
