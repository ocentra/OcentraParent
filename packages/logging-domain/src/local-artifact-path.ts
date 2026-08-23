import fs from 'node:fs';
import path from 'node:path';

const IgnorableDirectorySyncErrors = new Set(['EBADF', 'EINVAL', 'EISDIR', 'EPERM']);

function requireOwnedPath(condition: boolean, message: string): void {
  if (!condition) {
    throw new Error(message);
  }
}

export function resolveLocalArtifactPath(filePath: string): string {
  return path.resolve(filePath);
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
  fs.mkdirSync(targetPath, { recursive: true });
  assertExistingOwnedPath(targetPath, 'directory');
  requireOwnedPath(
    resolveLocalArtifactPath(fs.realpathSync(targetPath)).toLowerCase() === targetPath.toLowerCase(),
    'local artifact parent must not redirect outside its declared path'
  );
  return targetPath;
}

export function ensureLocalArtifactRoot(rootDir: string): string {
  const targetPath = resolveLocalArtifactPath(rootDir);
  assertNotFileSystemRoot(targetPath);
  return ensureOwnedDirectory(targetPath);
}

export function syncOwnedDirectory(dirPath: string): void {
  if (process.platform === 'win32') {
    return;
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
  } finally {
    if (descriptor != null) {
      fs.closeSync(descriptor);
    }
  }
}

export function assertOpenedPrivateFile(descriptor: number): void {
  const stat = fs.fstatSync(descriptor);
  requireOwnedPath(stat.isFile() && stat.nlink === 1, 'local artifact target must remain a private regular file');
}
