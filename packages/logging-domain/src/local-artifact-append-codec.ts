import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import {
  assertExistingOwnedPath,
  assertOpenedFileMatchesPath,
  ensureOwnedDirectory,
  resolveContainedLocalArtifactPath,
  syncOwnedDirectory,
  type LocalArtifactIdentity,
} from './local-artifact-path';

export interface AppendIntent {
  readonly schemaVersion: 1;
  readonly relativePath: string;
  readonly offset: number;
  readonly payloadLength: number;
  readonly payloadSha256: string;
  readonly payloadBase64: string;
  readonly targetIdentity: LocalArtifactIdentity | null;
}

export interface LoadedAppendIntent {
  readonly intent: AppendIntent;
  readonly identity: LocalArtifactIdentity;
}

export const MaximumAppendBytes = 1024 * 1024;

const MaximumAppendIntentBytes = 2 * 1024 * 1024;
const AppendIntentNamePattern = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\.json$/u;

function parseIdentity(value: unknown): LocalArtifactIdentity | null {
  if (value === null) {
    return null;
  }
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid local artifact append intent');
  }
  const input = value as Record<string, unknown>;
  if (!Number.isSafeInteger(input['device']) || !Number.isSafeInteger(input['inode'])) {
    throw new Error('invalid local artifact append intent');
  }
  return { device: input['device'] as number, inode: input['inode'] as number };
}

function isAppendIntentMetadata(input: Record<string, unknown>): boolean {
  return (
    input['schemaVersion'] === 1 &&
    typeof input['relativePath'] === 'string' &&
    Number.isSafeInteger(input['offset']) &&
    (input['offset'] as number) >= 0 &&
    Number.isSafeInteger(input['payloadLength']) &&
    (input['payloadLength'] as number) > 0 &&
    (input['payloadLength'] as number) <= MaximumAppendBytes &&
    typeof input['payloadSha256'] === 'string' &&
    /^[0-9a-f]{64}$/u.test(input['payloadSha256'] as string) &&
    typeof input['payloadBase64'] === 'string'
  );
}

function appendPayloadMatchesMetadata(input: Record<string, unknown>, payload: Buffer): boolean {
  return (
    payload.byteLength === input['payloadLength'] &&
    crypto.createHash('sha256').update(payload).digest('hex') === input['payloadSha256']
  );
}

function parseAppendIntent(value: unknown): AppendIntent {
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid local artifact append intent');
  }
  const input = value as Record<string, unknown>;
  if (!isAppendIntentMetadata(input)) {
    throw new Error('invalid local artifact append intent');
  }
  const payload = Buffer.from(input['payloadBase64'] as string, 'base64');
  if (!appendPayloadMatchesMetadata(input, payload)) {
    throw new Error('invalid local artifact append intent');
  }
  return {
    schemaVersion: 1,
    relativePath: input['relativePath'] as string,
    offset: input['offset'] as number,
    payloadLength: input['payloadLength'] as number,
    payloadSha256: input['payloadSha256'] as string,
    payloadBase64: input['payloadBase64'] as string,
    targetIdentity: parseIdentity(input['targetIdentity']),
  };
}

export function appendIntentDirectory(rootDir: string): string {
  return ensureOwnedDirectory(path.join(rootDir, '.bridge', 'append-intents'));
}

export function appendTargetPath(rootDir: string, relativePath: string): string {
  const normalized = path.posix.normalize(relativePath);
  const rootEntry = normalized.split('/', 1)[0];
  if (
    normalized !== relativePath ||
    normalized === '..' ||
    normalized.startsWith('../') ||
    (rootEntry !== 'test-logs' && rootEntry !== 'app-logs')
  ) {
    throw new Error('invalid local artifact append target');
  }
  return resolveContainedLocalArtifactPath(rootDir, path.join(rootDir, ...relativePath.split('/')));
}

export function readAppendIntent(intentPath: string): LoadedAppendIntent {
  assertExistingOwnedPath(intentPath, 'file');
  const descriptor = fs.openSync(intentPath, 'r');
  try {
    assertOpenedFileMatchesPath(intentPath, descriptor);
    const stat = fs.fstatSync(descriptor);
    if (stat.size > MaximumAppendIntentBytes) {
      throw new Error('invalid local artifact append intent');
    }
    const intent = parseAppendIntent(JSON.parse(fs.readFileSync(descriptor, 'utf8')) as unknown);
    assertOpenedFileMatchesPath(intentPath, descriptor);
    return { intent, identity: { device: stat.dev, inode: stat.ino } };
  } finally {
    fs.closeSync(descriptor);
  }
}

export function writeAppendIntent(rootDir: string, intent: AppendIntent): string {
  const intentDir = appendIntentDirectory(rootDir);
  const intentPath = path.join(intentDir, `${crypto.randomUUID()}.json`);
  const descriptor = fs.openSync(intentPath, 'wx', 0o600);
  try {
    assertOpenedFileMatchesPath(intentPath, descriptor);
    fs.writeFileSync(descriptor, `${JSON.stringify(intent)}\n`, 'utf8');
    fs.fsyncSync(descriptor);
    assertOpenedFileMatchesPath(intentPath, descriptor);
  } finally {
    fs.closeSync(descriptor);
  }
  syncOwnedDirectory(intentDir);
  return intentPath;
}

export function isAppendIntentName(value: string): boolean {
  return AppendIntentNamePattern.test(value);
}
