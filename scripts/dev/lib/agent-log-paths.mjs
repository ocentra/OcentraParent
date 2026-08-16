import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const DEFAULT_SCOPE = 'parent-codex';
const LOG_ROOT_ENV = 'OCENTRA_PARENT_LOG_ROOT';
const LEGACY_LOG_ROOT_ENV = 'OCENTRA_PARENT_LOG_DIR';
const WORKSPACE_ROOT_ENV = 'OCENTRA_PARENT_WORKSPACE_ROOT';

function scriptRoot() {
  return path.dirname(fileURLToPath(import.meta.url));
}

export function getWorkspaceRoot() {
  const configured = process.env[WORKSPACE_ROOT_ENV];
  if (configured != null && configured.trim().length > 0) {
    return path.resolve(configured);
  }
  return path.resolve(scriptRoot(), '..', '..', '..');
}

export function ensureDirectory(dirPath) {
  fs.mkdirSync(dirPath, { recursive: true });
  return dirPath;
}

export function sanitizePathSegment(value, fallback = 'default') {
  const normalized = String(value ?? '')
    .trim()
    .replace(/[^a-zA-Z0-9._-]+/g, '-')
    .replace(/^-+/, '')
    .replace(/-+$/, '');
  return normalized.length > 0 ? normalized : fallback;
}

export function getLogRoot() {
  const configured = process.env[LOG_ROOT_ENV] ?? process.env[LEGACY_LOG_ROOT_ENV];
  if (configured != null && configured.trim().length > 0) {
    return path.resolve(configured);
  }
  return path.join(getWorkspaceRoot(), '.logs');
}

export function getEvidenceScope() {
  return DEFAULT_SCOPE;
}

export function getScopeRoot(scope = getEvidenceScope()) {
  return ensureDirectory(path.join(getLogRoot(), sanitizePathSegment(scope, DEFAULT_SCOPE)));
}

export function getNdjsonStreamDir(stream, scope = getEvidenceScope()) {
  return ensureDirectory(path.join(getScopeRoot(scope), 'ndjson', sanitizePathSegment(stream)));
}

export function getNdjsonFilePath(stream, scope = getEvidenceScope(), date = new Date()) {
  const day = date.toISOString().slice(0, 10);
  return path.join(getNdjsonStreamDir(stream, scope), `${day}.ndjson`);
}

export function getArtifactDir(runId, commandId, scope = getEvidenceScope()) {
  return ensureDirectory(
    path.join(getScopeRoot(scope), 'artifacts', sanitizePathSegment(runId), sanitizePathSegment(commandId))
  );
}

export function getArtifactFilePath(runId, commandId, fileName, scope = getEvidenceScope()) {
  return path.join(getArtifactDir(runId, commandId, scope), fileName);
}

export function getDuckDbDir(scope = getEvidenceScope()) {
  return ensureDirectory(path.join(getScopeRoot(scope), 'db'));
}

export function getDuckDbPath(scope = getEvidenceScope()) {
  return path.join(getDuckDbDir(scope), 'agent-evidence.duckdb');
}

export function getManifestPath(scope = getEvidenceScope()) {
  return path.join(ensureDirectory(path.join(getScopeRoot(scope), 'manifests')), 'agent-evidence-ingest.json');
}

export function listNdjsonFiles(scope = getEvidenceScope()) {
  const ndjsonRoot = path.join(getScopeRoot(scope), 'ndjson');
  if (!fs.existsSync(ndjsonRoot)) {
    return [];
  }

  const files = [];
  const stack = [ndjsonRoot];
  while (stack.length > 0) {
    const current = stack.pop();
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        stack.push(fullPath);
        continue;
      }
      if (entry.name.endsWith('.ndjson')) {
        files.push(fullPath);
      }
    }
  }

  return files.sort((left, right) => left.localeCompare(right));
}

export function appendNdjson(stream, event, scope = getEvidenceScope(), date = new Date()) {
  const filePath = getNdjsonFilePath(stream, scope, date);
  fs.appendFileSync(filePath, `${JSON.stringify(event)}\n`, 'utf8');
  return filePath;
}

export function detectLaneId() {
  const candidates = [
    process.env.LEDGER_LANE,
    process.env.OCENTRA_PARENT_CODEX_LANE_ID,
    process.env.OCENTRA_PARENT_LANE_ID,
  ];

  for (const candidate of candidates) {
    if (candidate != null && candidate.trim().length > 0) {
      return candidate.trim();
    }
  }

  return null;
}

export function detectMachineName() {
  return os.hostname();
}

export function toPosixPath(filePath) {
  return path.resolve(filePath).replace(/\\/g, '/');
}
