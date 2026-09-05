import { Buffer } from 'node:buffer';
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';

const MaximumProviderBinaryBytes = 64 * 1024 * 1024;
const DirectorySyncUnsupportedCodes = new Set(['EACCES', 'EBADF', 'EINVAL', 'EISDIR', 'ENOTSUP', 'EPERM']);

function fileDigest(content) {
  return crypto.createHash('sha256').update(content).digest('hex');
}

function assertPrivateRegularFile(fileStat, label, allowHardLinks) {
  if (!fileStat.isFile() || fileStat.isSymbolicLink()) {
    throw new Error(`${label} is not a regular file`);
  }
  if (fileStat.nlink < 1n || (!allowHardLinks && fileStat.nlink !== 1n)) {
    throw new Error(`${label} is not a private regular file`);
  }
  if (fileStat.size > BigInt(MaximumProviderBinaryBytes)) {
    throw new Error(`${label} exceeds the provider binary size limit`);
  }
}

function sameStableFile(left, right) {
  return left.dev === right.dev && left.ino === right.ino && left.size === right.size && left.mtimeNs === right.mtimeNs;
}

function readStableRegularFile(filePath, label, allowHardLinks) {
  const descriptor = fs.openSync(filePath, 'r');
  try {
    const descriptorBefore = fs.fstatSync(descriptor, { bigint: true });
    const pathBefore = fs.lstatSync(filePath, { bigint: true });
    assertPrivateRegularFile(descriptorBefore, label, allowHardLinks);
    assertPrivateRegularFile(pathBefore, label, allowHardLinks);
    if (!sameStableFile(pathBefore, descriptorBefore)) {
      throw new Error(`${label} changed while it was opened`);
    }
    const content = fs.readFileSync(descriptor);
    const descriptorAfter = fs.fstatSync(descriptor, { bigint: true });
    const pathAfter = fs.lstatSync(filePath, { bigint: true });
    assertPrivateRegularFile(descriptorAfter, label, allowHardLinks);
    assertPrivateRegularFile(pathAfter, label, allowHardLinks);
    if (
      !sameStableFile(descriptorBefore, descriptorAfter) ||
      !sameStableFile(descriptorAfter, pathAfter) ||
      BigInt(content.byteLength) !== descriptorAfter.size
    ) {
      throw new Error(`${label} changed while it was read`);
    }
    return content;
  } finally {
    fs.closeSync(descriptor);
  }
}

function temporarySibling(targetPath, label) {
  return `${targetPath}.${process.pid}-${crypto.randomBytes(12).toString('hex')}.${label}`;
}

function writePrivateTemporaryFile(targetPath, content, label, mode) {
  const temporaryPath = temporarySibling(targetPath, label);
  try {
    const descriptor = fs.openSync(temporaryPath, 'wx', mode);
    try {
      fs.writeFileSync(descriptor, content);
      fs.fsyncSync(descriptor);
    } finally {
      fs.closeSync(descriptor);
    }
    const stored = readStableRegularFile(temporaryPath, `temporary ${label}`, false);
    if (!stored.equals(content)) {
      throw new Error(`temporary ${label} content changed before publication`);
    }
    return temporaryPath;
  } catch (error) {
    fs.rmSync(temporaryPath, { force: true });
    throw error;
  }
}

function syncDirectoryIfSupported(directoryPath) {
  let descriptor;
  try {
    descriptor = fs.openSync(directoryPath, 'r');
    fs.fsyncSync(descriptor);
  } catch (error) {
    if (!(error instanceof Error) || !('code' in error) || !DirectorySyncUnsupportedCodes.has(error.code)) {
      throw error;
    }
  } finally {
    if (descriptor !== undefined) fs.closeSync(descriptor);
  }
}

function ensureUnlinkedDirectoryChain(directoryPath) {
  const absoluteDirectory = path.resolve(directoryPath);
  fs.mkdirSync(absoluteDirectory, { mode: 0o700, recursive: true });
  const parsed = path.parse(absoluteDirectory);
  const segments = absoluteDirectory.slice(parsed.root.length).split(path.sep).filter(Boolean);
  let current = parsed.root;
  for (const segment of segments) {
    current = path.join(current, segment);
    const currentStat = fs.lstatSync(current, { bigint: true });
    if (!currentStat.isDirectory() || currentStat.isSymbolicLink()) {
      throw new Error(`provider staging directory is linked or not a directory: ${current}`);
    }
  }
}

function assertContentAddressedBinary(binaryPath, expectedContent, expectedDigest) {
  const stagedContent = readStableRegularFile(binaryPath, 'staged local artifact provider', false);
  if (!stagedContent.equals(expectedContent) || fileDigest(stagedContent) !== expectedDigest) {
    throw new Error('staged local artifact provider digest does not match the built binary');
  }
}

function publishContentAddressedBinary(binaryPath, content, digest) {
  try {
    assertContentAddressedBinary(binaryPath, content, digest);
    return;
  } catch (error) {
    if (!(error instanceof Error) || !('code' in error) || error.code !== 'ENOENT') throw error;
  }

  const temporaryPath = writePrivateTemporaryFile(binaryPath, content, 'provider', 0o700);
  try {
    try {
      fs.linkSync(temporaryPath, binaryPath);
    } catch (error) {
      if (!(error instanceof Error) || !('code' in error) || error.code !== 'EEXIST') throw error;
    }
  } finally {
    fs.rmSync(temporaryPath, { force: true });
  }
  syncDirectoryIfSupported(path.dirname(binaryPath));
  assertContentAddressedBinary(binaryPath, content, digest);
}

function publishManifest(manifestPath, content) {
  const temporaryPath = writePrivateTemporaryFile(manifestPath, Buffer.from(content, 'utf8'), 'manifest', 0o600);
  try {
    fs.renameSync(temporaryPath, manifestPath);
  } finally {
    fs.rmSync(temporaryPath, { force: true });
  }
  syncDirectoryIfSupported(path.dirname(manifestPath));
  const stored = readStableRegularFile(manifestPath, 'local artifact provider manifest', false);
  if (stored.toString('utf8') !== content) {
    throw new Error('local artifact provider manifest changed during publication');
  }
}

export function stageLocalArtifactProvider({
  sourceBinary,
  providerDirectory,
  manifestPath,
  packageName,
  packageVersion,
}) {
  ensureUnlinkedDirectoryChain(providerDirectory);
  ensureUnlinkedDirectoryChain(path.dirname(manifestPath));
  const sourceContent = readStableRegularFile(sourceBinary, 'built local artifact provider', true);
  const binarySha256 = fileDigest(sourceContent);
  const stagedExecutableName = `ocentra-logging-local-artifact-provider-${binarySha256}.exe`;
  const outputBinary = path.join(providerDirectory, stagedExecutableName);
  publishContentAddressedBinary(outputBinary, sourceContent, binarySha256);
  const manifest = {
    schemaVersion: 1,
    protocolVersion: 1,
    packageName,
    packageVersion,
    providerPackageName: 'ocentra-logging-local-artifact-provider',
    platform: 'win32',
    binaryPath: `local-artifact-provider/${stagedExecutableName}`,
    binarySha256,
  };
  publishManifest(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`);
  return { binarySha256, manifest, outputBinary };
}
