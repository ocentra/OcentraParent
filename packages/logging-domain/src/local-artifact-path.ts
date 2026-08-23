import fs from 'node:fs';
import path from 'node:path';

const IgnorableDirectorySyncErrors = new Set(['EBADF', 'EINVAL', 'EISDIR', 'EPERM']);
let directorySyncAvailable = process.platform !== 'win32';

export interface LocalArtifactIdentity {
  readonly device: number;
  readonly inode: number;
}

export type LocalArtifactDirectoryDurability = 'synced' | 'recovery-intent-only';

function requireOwnedPath(condition: boolean, message: string): void {
  if (!condition) {
    throw new Error(message);
  }
}

export function resolveLocalArtifactPath(filePath: string): string {
  return path.resolve(filePath);
}

export function resolveContainedLocalArtifactPath(rootDir: string, filePath: string): string {
  const rootPath = resolveLocalArtifactPath(rootDir);
  const targetPath = resolveLocalArtifactPath(filePath);
  const relativePath = path.relative(rootPath, targetPath);
  requireOwnedPath(
    relativePath.length > 0 &&
      !path.isAbsolute(relativePath) &&
      relativePath !== '..' &&
      !relativePath.startsWith(`..${path.sep}`),
    'local artifact target must remain inside its declared root'
  );
  return targetPath;
}

export function relativeLocalArtifactPath(rootDir: string, filePath: string): string {
  return path.relative(resolveLocalArtifactPath(rootDir), resolveContainedLocalArtifactPath(rootDir, filePath));
}

export function assertNotFileSystemRoot(targetPath: string): void {
  requireOwnedPath(targetPath !== path.parse(targetPath).root, 'local artifact root must not be a filesystem root');
}

export function assertExistingOwnedPath(targetPath: string, expected: 'file' | 'directory'): void {
  const stat = fs.lstatSync(targetPath);
  requireOwnedPath(!stat.isSymbolicLink(), 'local artifact paths must not use symbolic links');
  const expectedType = expected === 'file' ? stat.isFile() && stat.nlink === 1 : stat.isDirectory();
  requireOwnedPath(expectedType, `local artifact target must be an owned ${expected}`);
}

export function assertOwnedDirectoryTree(targetPath: string): void {
  assertExistingOwnedPath(targetPath, 'directory');
  for (const entry of fs.readdirSync(targetPath, { withFileTypes: true })) {
    const entryPath = path.join(targetPath, entry.name);
    if (entry.isDirectory()) {
      assertOwnedDirectoryTree(entryPath);
    } else {
      assertExistingOwnedPath(entryPath, 'file');
    }
  }
}

export function localArtifactIdentity(targetPath: string, expected: 'file' | 'directory'): LocalArtifactIdentity {
  assertExistingOwnedPath(targetPath, expected);
  const stat = fs.lstatSync(targetPath);
  return { device: stat.dev, inode: stat.ino };
}

export function assertLocalArtifactIdentity(
  targetPath: string,
  expected: 'file' | 'directory',
  identity: LocalArtifactIdentity
): void {
  const current = localArtifactIdentity(targetPath, expected);
  requireOwnedPath(
    current.device === identity.device && current.inode === identity.inode,
    'local artifact target identity changed during the operation'
  );
}

export function assertExistingOwnedAncestors(targetPath: string): void {
  const parsed = path.parse(targetPath);
  let current = parsed.root;
  for (const segment of path.relative(parsed.root, targetPath).split(path.sep).filter(Boolean)) {
    current = path.join(current, segment);
    if (fs.existsSync(current)) {
      assertExistingOwnedPath(current, 'directory');
    }
  }
}

export function ensureOwnedDirectory(dirPath: string): string {
  const targetPath = resolveLocalArtifactPath(dirPath);
  assertExistingOwnedAncestors(targetPath);
  fs.mkdirSync(targetPath, { recursive: true, mode: 0o700 });
  assertExistingOwnedPath(targetPath, 'directory');
  requireOwnedPath(
    normalizePathForComparison(fs.realpathSync(targetPath)) === normalizePathForComparison(targetPath),
    'local artifact parent must not redirect outside its declared path'
  );
  return targetPath;
}

function normalizePathForComparison(filePath: string): string {
  const resolved = resolveLocalArtifactPath(filePath);
  return process.platform === 'win32' ? resolved.toLowerCase() : resolved;
}

export function localArtifactDirectoryDurability(): LocalArtifactDirectoryDurability {
  return directorySyncAvailable ? 'synced' : 'recovery-intent-only';
}

export function ensureLocalArtifactRoot(rootDir: string): string {
  const targetPath = resolveLocalArtifactPath(rootDir);
  assertNotFileSystemRoot(targetPath);
  return ensureOwnedDirectory(targetPath);
}

export function syncOwnedDirectory(dirPath: string): boolean {
  if (process.platform === 'win32') {
    directorySyncAvailable = false;
    return false;
  }
  let descriptor: number | null = null;
  try {
    descriptor = fs.openSync(dirPath, 'r');
    fs.fsyncSync(descriptor);
  } catch (error) {
    const code = (error as NodeJS.ErrnoException).code ?? '';
    if (!IgnorableDirectorySyncErrors.has(code)) {
      throw error;
    }
    directorySyncAvailable = false;
    return false;
  } finally {
    if (descriptor != null) {
      fs.closeSync(descriptor);
    }
  }
  return true;
}

export function assertOpenedPrivateFile(descriptor: number): LocalArtifactIdentity {
  const stat = fs.fstatSync(descriptor);
  requireOwnedPath(stat.isFile() && stat.nlink === 1, 'local artifact target must remain a private regular file');
  return { device: stat.dev, inode: stat.ino };
}

export function assertOpenedFileMatchesPath(filePath: string, descriptor: number): void {
  const opened = assertOpenedPrivateFile(descriptor);
  assertLocalArtifactIdentity(filePath, 'file', opened);
}
