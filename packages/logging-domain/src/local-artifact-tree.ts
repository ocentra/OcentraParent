import fs from 'node:fs';
import path from 'node:path';
import {
  assertExistingOwnedAncestors,
  assertExistingOwnedPath,
  assertNotFileSystemRoot,
  ensureOwnedDirectory,
  resolveLocalArtifactPath,
  syncOwnedDirectory,
} from './local-artifact-path';

const LoggingArtifactRootEntries = new Set(['test-logs', 'app-logs', 'db', 'manifests', '.bridge']);

export function assertLoggingArtifactRootLayout(rootDir: string): void {
  const invalidEntry = fs
    .readdirSync(rootDir, { withFileTypes: true })
    .find((entry) => !LoggingArtifactRootEntries.has(entry.name) || !entry.isDirectory());
  if (invalidEntry != null) {
    throw new Error('logging artifact root contains an unowned entry');
  }
}

function collectOwnedTree(
  rootDir: string,
  preserveRootEntries: ReadonlySet<string> = new Set()
): { readonly files: string[]; readonly directories: string[] } {
  const files: string[] = [];
  const directories: string[] = [];
  const visit = (dirPath: string): void => {
    assertExistingOwnedPath(dirPath, 'directory');
    for (const entry of fs.readdirSync(dirPath, { withFileTypes: true })) {
      const entryPath = path.join(dirPath, entry.name);
      if (entry.isDirectory()) {
        if (dirPath === rootDir && preserveRootEntries.has(entry.name)) {
          collectOwnedTree(entryPath);
          continue;
        }
        visit(entryPath);
        directories.push(entryPath);
        continue;
      }
      assertExistingOwnedPath(entryPath, 'file');
      files.push(entryPath);
    }
  };
  visit(rootDir);
  return { files, directories };
}

export function clearLoggingArtifactRoot(rootDir: string): void {
  const targetPath = resolveLocalArtifactPath(rootDir);
  assertNotFileSystemRoot(targetPath);
  if (!fs.existsSync(targetPath)) {
    ensureOwnedDirectory(targetPath);
    return;
  }
  assertExistingOwnedAncestors(path.dirname(targetPath));
  assertLoggingArtifactRootLayout(targetPath);
  const tree = collectOwnedTree(targetPath, new Set(['.bridge']));
  for (const filePath of tree.files) {
    fs.unlinkSync(filePath);
  }
  for (const dirPath of tree.directories) {
    fs.rmdirSync(dirPath);
  }
  syncOwnedDirectory(targetPath);
}
