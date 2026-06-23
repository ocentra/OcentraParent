import crypto from 'node:crypto';
import fs from 'node:fs';

import { getArtifactFilePath, toPosixPath } from './agent-log-paths.mjs';

function sha256(content) {
  return crypto.createHash('sha256').update(content).digest('hex');
}

function lineCount(content) {
  if (content.length === 0) {
    return 0;
  }
  return content.split(/\r?\n/).length;
}

function artifactId(kind, digest) {
  return `${kind}-${digest.slice(0, 12)}`;
}

export function writeTextArtifact({ scope, runId, commandId, kind, fileName, content, createdAt }) {
  const filePath = getArtifactFilePath(runId, commandId, fileName, scope);
  fs.writeFileSync(filePath, content, 'utf8');
  const digest = sha256(content);
  return {
    schemaVersion: 1,
    eventType: 'artifact',
    artifactId: artifactId(kind, digest),
    runId,
    commandId,
    path: toPosixPath(filePath),
    kind,
    sha256: digest,
    byteLength: Buffer.byteLength(content, 'utf8'),
    lineCount: lineCount(content),
    createdAt,
  };
}

export function writeMetadataArtifact({ scope, runId, commandId, metadata, createdAt }) {
  return writeTextArtifact({
    scope,
    runId,
    commandId,
    kind: 'metadata',
    fileName: 'metadata.json',
    content: `${JSON.stringify(metadata, null, 2)}\n`,
    createdAt,
  });
}
