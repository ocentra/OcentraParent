import fs from 'node:fs';
import path from 'node:path';

import {
  getArtifact,
  getLatestFailures as getLatestFailureRuns,
  getRunEvidence,
  getStats as getAgentEvidenceStats,
} from './agent-evidence-db.mjs';
import { getEvidenceScope, getLogRoot, getWorkspaceRoot, sanitizePathSegment } from './agent-log-paths.mjs';

const DEFAULT_LIMIT = 50;
const MAX_LIMIT = 200;
const DEFAULT_ARTIFACT_LINES = 80;
const MAX_ARTIFACT_LINES = 200;
const DEFAULT_PROOF_TRACE_SCOPE = 'parent-portal';
const LOGGING_PLAN_NAME = 'logging-domain-parity';
const LOGGING_PLAN_PROOF_ROOT = 'output/logging-domain-parity-proof';

function getStructuredLogBaseRoot() {
  const configuredRoot = process.env['OCENTRA_PARENT_LOG_DIR'];
  if (configuredRoot != null && configuredRoot.trim().length > 0) {
    return path.resolve(configuredRoot);
  }
  return path.join(getWorkspaceRoot(), 'output', 'logging-domain');
}

function clampLimit(limit, fallback = DEFAULT_LIMIT) {
  const value = Number(limit ?? fallback);
  if (!Number.isFinite(value) || value <= 0) {
    return fallback;
  }
  return Math.min(Math.trunc(value), MAX_LIMIT);
}

function clampArtifactLines(value) {
  const parsed = Number(value ?? DEFAULT_ARTIFACT_LINES);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    return DEFAULT_ARTIFACT_LINES;
  }
  return Math.min(Math.trunc(parsed), MAX_ARTIFACT_LINES);
}

function parseIsoOrDuration(value) {
  if (value == null || String(value).trim().length === 0) {
    return null;
  }
  const text = String(value).trim();
  if (/^\d+[smhd]$/.test(text)) {
    const amount = Number(text.slice(0, -1));
    const unit = text.slice(-1);
    const multipliers = { s: 1000, m: 60_000, h: 3_600_000, d: 86_400_000 };
    return Date.now() - amount * multipliers[unit];
  }
  const timestamp = Date.parse(text);
  return Number.isNaN(timestamp) ? null : timestamp;
}

function normalizeEvidenceDiagnostic(diagnostic, run = {}) {
  return {
    recordType: 'diagnostic',
    scope: 'parent-codex',
    timestamp: null,
    level: diagnostic.severity === 'warning' ? 'warn' : 'error',
    source: null,
    context: diagnostic.kind,
    message: diagnostic.message,
    runId: run.runId ?? diagnostic.runId ?? null,
    commandId: run.commandId ?? diagnostic.commandId ?? null,
    file: diagnostic.file,
    filePath: diagnostic.file,
    line: diagnostic.line,
    column: diagnostic.column,
    correlationId: null,
    tags: [],
    data: diagnostic.signature,
    rawArtifact: diagnostic.rawArtifact,
    hitCount: diagnostic.hitCount ?? 1,
  };
}

function normalizeStoredTestLog(entry) {
  return {
    recordType: 'test-log',
    scope: entry.scope,
    timestamp: entry.timestamp,
    level: entry.level,
    source: entry.source,
    context: entry.context,
    message: entry.message,
    runId: entry.runId,
    commandId: null,
    file: entry.file,
    filePath: entry.filePath,
    line: entry.line,
    column: entry.column,
    correlationId: entry.correlationId,
    tags: entry.tags ?? [],
    data: entry.data,
    rawArtifact: null,
    hitCount: 1,
  };
}

function normalizeAppLog(entry) {
  return {
    recordType: 'app-log',
    scope: entry.scope,
    timestamp: entry.timestamp,
    level: entry.level,
    source: entry.source,
    context: entry.context,
    message: entry.message,
    runId: null,
    commandId: null,
    file: entry.file,
    filePath: entry.filePath,
    line: entry.line,
    column: entry.column,
    correlationId: entry.correlationId,
    tags: [],
    data: entry.data,
    rawArtifact: null,
    hitCount: 1,
  };
}

function fileNameFromPath(filePath) {
  if (typeof filePath !== 'string' || filePath.trim().length === 0) {
    return null;
  }
  const normalized = filePath.replace(/\\/g, '/');
  const lastSlash = normalized.lastIndexOf('/');
  return lastSlash >= 0 ? normalized.slice(lastSlash + 1) : normalized;
}

function normalizeRustDevLog(scope, entry) {
  const timestamp = typeof entry?.timestamp === 'string' ? Date.parse(entry.timestamp) : null;
  const context = typeof entry?.fields?.context === 'string' ? entry.fields.context : null;
  return {
    recordType: 'rust-dev-log',
    scope,
    timestamp: Number.isNaN(timestamp) ? null : timestamp,
    level: entry?.level ?? null,
    source: entry?.source ?? null,
    context,
    message: entry?.message ?? null,
    runId: entry?.runId ?? null,
    commandId: entry?.commandId ?? null,
    file: entry?.file ?? fileNameFromPath(entry?.filePath ?? null),
    filePath: entry?.filePath ?? null,
    line: entry?.line ?? null,
    column: entry?.column ?? null,
    correlationId: entry?.correlationId ?? null,
    tags: [],
    data: entry?.fields != null ? JSON.stringify(entry.fields) : null,
    rawArtifact: null,
    hitCount: 1,
  };
}

function readNdjsonFile(filePath) {
  if (!fs.existsSync(filePath)) {
    return [];
  }
  const content = fs.readFileSync(filePath, 'utf8').trim();
  if (content.length === 0) {
    return [];
  }
  return content
    .split(/\r?\n/)
    .filter((line) => line.trim().length > 0)
    .map((line) => JSON.parse(line));
}

function listNdjsonFiles(rootPath) {
  if (!fs.existsSync(rootPath)) {
    return [];
  }
  const files = [];
  const stack = [rootPath];
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

function getStructuredLogRoots(scope) {
  const safeScope = sanitizePathSegment(scope);
  const structuredRoot = getStructuredLogBaseRoot();
  return {
    testLogs: path.join(structuredRoot, 'test-logs', safeScope),
    appLogs: path.join(structuredRoot, 'app-logs', safeScope),
  };
}

function parseStructuredData(value) {
  if (value == null || typeof value !== 'string' || value.trim().length === 0) {
    return null;
  }
  try {
    const parsed = JSON.parse(value);
    return parsed != null && typeof parsed === 'object' && !Array.isArray(parsed) ? parsed : null;
  } catch {
    return null;
  }
}

function getProofTraceMetadata(log) {
  const data = parseStructuredData(log.data);
  if (data == null) {
    return null;
  }

  const proofId =
    typeof data.proofId === 'string' ? data.proofId : typeof data.proof_id === 'string' ? data.proof_id : null;
  if (proofId == null || proofId.trim().length === 0) {
    return null;
  }

  return {
    proofId,
    testId: asOptionalString(data.testId ?? data.test_id),
    causationId: asOptionalString(data.causationId ?? data.causation_id),
    traceStep: asOptionalString(data.traceStep ?? data.trace_step),
    eventType: asOptionalString(data.eventType ?? data.event_type),
    action: asOptionalString(data.action),
    command: asOptionalString(data.command),
    status: asOptionalString(data.status),
    expectedNext: asOptionalString(data.expectedNext ?? data.expected_next),
    artifactRef: asOptionalString(data.artifactRef ?? data.artifact_ref),
  };
}

function asOptionalString(value) {
  return typeof value === 'string' && value.trim().length > 0 ? value : null;
}

function compareNullableStrings(left, right) {
  if (left == null && right == null) {
    return 0;
  }
  if (left == null) {
    return 1;
  }
  if (right == null) {
    return -1;
  }
  return left.localeCompare(right);
}

function normalizeProofTraceRow(log) {
  const metadata = getProofTraceMetadata(log);
  if (metadata == null) {
    return null;
  }

  return {
    proofId: metadata.proofId,
    testId: metadata.testId,
    causationId: metadata.causationId,
    traceStep: metadata.traceStep,
    eventType: metadata.eventType,
    action: metadata.action,
    command: metadata.command,
    status: metadata.status,
    expectedNext: metadata.expectedNext,
    artifactRef: metadata.artifactRef,
    scope: log.scope,
    timestamp: log.timestamp,
    level: log.level,
    source: log.source,
    context: log.context,
    message: log.message,
    runId: log.runId,
    commandId: log.commandId,
    file: log.file,
    filePath: log.filePath,
    line: log.line,
    column: log.column,
    correlationId: log.correlationId,
    tags: log.tags,
    data: log.data,
    rawArtifact: log.rawArtifact,
    hitCount: log.hitCount,
  };
}

function proofTraceRowsForScope(scope, proofId) {
  const logs = readLocalLogs(scope);
  if (logs.length === 0) {
    throw missingStructuredScopeError(scope);
  }

  return logs
    .map(normalizeProofTraceRow)
    .filter((row) => row != null && row.proofId === proofId)
    .sort((left, right) => {
      const byTimestamp = (left.timestamp ?? 0) - (right.timestamp ?? 0);
      if (byTimestamp !== 0) {
        return byTimestamp;
      }
      return compareNullableStrings(left.traceStep, right.traceStep);
    });
}

function parseExpectedSteps(value) {
  if (Array.isArray(value)) {
    return value;
  }
  if (typeof value === 'string' && value.trim().length > 0) {
    return JSON.parse(value);
  }
  return [];
}

function proofTraceStepMatches(row, expectedStep) {
  if (typeof expectedStep === 'string') {
    return row.traceStep === expectedStep;
  }
  if (expectedStep == null || typeof expectedStep !== 'object' || Array.isArray(expectedStep)) {
    return false;
  }

  const contains = typeof expectedStep.contains === 'string' ? expectedStep.contains.toLowerCase() : null;
  if (expectedStep.traceStep != null && row.traceStep !== expectedStep.traceStep) {
    return false;
  }
  if (expectedStep.source != null && row.source !== expectedStep.source) {
    return false;
  }
  if (expectedStep.context != null && row.context !== expectedStep.context) {
    return false;
  }
  if (expectedStep.eventType != null && row.eventType !== expectedStep.eventType) {
    return false;
  }
  if (expectedStep.action != null && row.action !== expectedStep.action) {
    return false;
  }
  if (expectedStep.command != null && row.command !== expectedStep.command) {
    return false;
  }
  if (expectedStep.status != null && row.status !== expectedStep.status) {
    return false;
  }
  if (contains != null && !`${row.message} ${row.data ?? ''}`.toLowerCase().includes(contains)) {
    return false;
  }
  return true;
}

function findLatestProofTraceId(scope) {
  const logs = readStructuredLogs(scope);
  const rows = logs
    .map(normalizeProofTraceRow)
    .filter((row) => row != null)
    .sort((left, right) => (right.timestamp ?? 0) - (left.timestamp ?? 0));
  return rows[0]?.proofId ?? null;
}

function readStructuredLogs(scope) {
  const { testLogs, appLogs } = getStructuredLogRoots(scope);
  const logs = [];
  for (const filePath of listNdjsonFiles(testLogs)) {
    for (const entry of readNdjsonFile(filePath)) {
      logs.push(normalizeStoredTestLog(entry));
    }
  }
  for (const filePath of listNdjsonFiles(appLogs)) {
    for (const entry of readNdjsonFile(filePath)) {
      logs.push(normalizeAppLog(entry));
    }
  }
  return logs.sort((left, right) => (right.timestamp ?? 0) - (left.timestamp ?? 0));
}

function readRustDevLogs(scope) {
  const rustRoot = path.join(getLogRoot(), sanitizePathSegment(scope), 'ndjson', 'dev-log');
  const logs = [];
  for (const filePath of listNdjsonFiles(rustRoot)) {
    for (const entry of readNdjsonFile(filePath)) {
      logs.push(normalizeRustDevLog(scope, entry));
    }
  }
  return logs.sort((left, right) => (right.timestamp ?? 0) - (left.timestamp ?? 0));
}

function readLocalLogs(scope) {
  return [...readStructuredLogs(scope), ...readRustDevLogs(scope)].sort(
    (left, right) => (right.timestamp ?? 0) - (left.timestamp ?? 0)
  );
}

function filterLogs(logs, filters) {
  return logs.filter((log) => {
    if (filters.level != null && log.level !== filters.level) {
      return false;
    }
    if (filters.source != null && log.source !== filters.source) {
      return false;
    }
    if (filters.context != null && log.context !== filters.context) {
      return false;
    }
    if (filters.runId != null && log.runId !== filters.runId) {
      return false;
    }
    if (filters.contains != null && filters.contains.trim().length > 0) {
      const haystack = `${log.message} ${log.context ?? ''} ${log.data ?? ''}`.toLowerCase();
      if (!haystack.includes(filters.contains.toLowerCase())) {
        return false;
      }
    }
    if (filters.fromTs != null && (log.timestamp == null || log.timestamp < filters.fromTs)) {
      return false;
    }
    if (filters.toTs != null && (log.timestamp == null || log.timestamp > filters.toTs)) {
      return false;
    }
    return true;
  });
}

function missingStructuredScopeError(scope) {
  return new Error(
    `No structured logs found for scope "${scope}". Populate output/logging-domain/test-logs or app-logs for that scope before using this query.`
  );
}

function readTextIfExists(filePath) {
  return fs.existsSync(filePath) ? fs.readFileSync(filePath, 'utf8') : null;
}

function listFilesRecursive(rootPath) {
  if (!fs.existsSync(rootPath)) {
    return [];
  }
  const files = [];
  const stack = [rootPath];
  while (stack.length > 0) {
    const current = stack.pop();
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        stack.push(fullPath);
        continue;
      }
      files.push(fullPath);
    }
  }
  return files.sort((left, right) => left.localeCompare(right));
}

function getLoggingPlanRoot(workspaceRoot = getWorkspaceRoot()) {
  return path.join(workspaceRoot, 'docs', 'plans', LOGGING_PLAN_NAME);
}

function getLoggingPlanFile(relativePath, workspaceRoot = getWorkspaceRoot()) {
  return path.join(getLoggingPlanRoot(workspaceRoot), relativePath);
}

function parseWorkpackIndexStatuses(text) {
  const statuses = new Map();
  const pattern = /^\|\s*([a-z-]+)\s*\|\s*\[WP(\d{2})\s+([^\]]+)\]\([^)]+\)\s*\|\s*([^|]+?)\s*\|\s*`([^`]+)`\s*\|$/gmu;
  for (const match of text.matchAll(pattern)) {
    statuses.set(match[2], {
      workpackId: match[2],
      status: match[1],
      title: match[3].trim(),
      boxes: match[4].trim(),
      sourceDoc: match[5].trim(),
    });
  }
  return statuses;
}

function parseChecklistProofState(text) {
  const checklist = new Map();
  const pattern = /^##\s+WP(\d{2})[^\n]*\n([\s\S]*?)(?=^##\s+WP\d{2}|$(?![\r\n]))/gmu;
  for (const match of text.matchAll(pattern)) {
    const body = match[2];
    const checkedLines = body
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter((line) => /^-\s+\[x\]\s+/iu.test(line));
    const combinedRowChecked = checkedLines.some((line) =>
      /Proof root and workpack completion section filled\./iu.test(line)
    );
    checklist.set(match[1], {
      workpackId: match[1],
      proofRowChecked: combinedRowChecked || checkedLines.some((line) => /Proof root written\./iu.test(line)),
      workpackCompletionChecked:
        combinedRowChecked || checkedLines.some((line) => /Workpack completion section filled\./iu.test(line)),
    });
  }
  return checklist;
}

function parseProofRootDefinitions(text) {
  const roots = new Map();
  const pattern = /output\/logging-domain-parity-proof\/(\d{2})-([a-z0-9-]+)\//giu;
  for (const match of text.matchAll(pattern)) {
    const workpackId = match[1];
    if (roots.has(workpackId)) {
      continue;
    }
    roots.set(workpackId, {
      workpackId,
      slug: match[2],
      relativePath: `${LOGGING_PLAN_PROOF_ROOT}/${workpackId}-${match[2]}`,
    });
  }
  return [...roots.values()].sort((left, right) => left.workpackId.localeCompare(right.workpackId));
}

function parsePlanStateRestoredRootsClaim(text) {
  for (const line of text.split(/\r?\n/)) {
    if (!/roots are restored so far/iu.test(line)) {
      continue;
    }
    const workpackIds = [...line.matchAll(/WP(\d{2})/gu)].map((match) => match[1]);
    if (workpackIds.length === 0) {
      continue;
    }
    return {
      line: line.trim(),
      workpackIds: [...new Set(workpackIds)].sort(),
    };
  }
  return null;
}

function sameStringSet(left, right) {
  if (left.length !== right.length) {
    return false;
  }
  return left.every((value, index) => value === right[index]);
}

function readAgentEvidenceStream(stream) {
  const streamRoot = path.join(getLogRoot(), getEvidenceScope(), 'ndjson', stream);
  const events = [];
  for (const filePath of listNdjsonFiles(streamRoot)) {
    events.push(...readNdjsonFile(filePath));
  }
  return events;
}

function readAgentEvidenceFromNdjson() {
  return {
    runs: readAgentEvidenceStream('agent-run'),
    diagnostics: readAgentEvidenceStream('diagnostics'),
    artifacts: readAgentEvidenceStream('artifacts'),
  };
}

function getAgentEvidenceDbPath() {
  return path.join(getLogRoot(), getEvidenceScope(), 'db', 'agent-evidence.duckdb');
}

function mapArtifactEvent(artifact) {
  return {
    artifactId: artifact.artifactId,
    runId: artifact.runId ?? null,
    commandId: artifact.commandId ?? null,
    path: artifact.path,
    kind: artifact.kind,
    sha256: artifact.sha256 ?? null,
    byteLength: artifact.byteLength ?? null,
    lineCount: artifact.lineCount ?? null,
    createdAt: artifact.createdAt ?? null,
  };
}

function mapDiagnosticEvent(diagnostic) {
  return {
    diagnosticId: diagnostic.diagnosticId,
    kind: diagnostic.kind,
    severity: diagnostic.severity,
    signature: diagnostic.signature,
    file: diagnostic.file ?? null,
    line: diagnostic.line ?? null,
    column: diagnostic.column ?? null,
    message: diagnostic.message,
    rawArtifact: diagnostic.rawArtifact ?? null,
    rawStartLine: diagnostic.rawStartLine ?? null,
    rawEndLine: diagnostic.rawEndLine ?? null,
    hitCount: diagnostic.hitCount ?? 1,
  };
}

function assembleRunEvidenceFromNdjson(runId) {
  const data = readAgentEvidenceFromNdjson();
  const run = data.runs.find((entry) => entry.runId === runId);
  if (run == null) {
    return null;
  }
  const diagnostics = data.diagnostics.filter((entry) => entry.runId === runId).map(mapDiagnosticEvent);
  const artifacts = data.artifacts.filter((entry) => entry.runId === runId).map(mapArtifactEvent);
  return {
    run: {
      runId: run.runId,
      commandId: run.commandId,
      laneId: run.laneId ?? null,
      machine: run.machine,
      workspace: run.workspace,
      cwd: run.cwd,
      command: run.command,
      startedAt: run.startedAt,
      endedAt: run.endedAt,
      durationMs: run.durationMs,
      status: run.status,
      exitCode: run.exitCode,
      stdoutArtifact: run.stdoutArtifact,
      stderrArtifact: run.stderrArtifact,
      summary: run.summary ?? null,
    },
    diagnostics,
    artifacts,
  };
}

function assembleLatestFailuresFromNdjson(limit) {
  const data = readAgentEvidenceFromNdjson();
  return data.runs
    .filter((entry) => entry.status === 'failed')
    .sort((left, right) => Date.parse(right.startedAt) - Date.parse(left.startedAt))
    .slice(0, limit)
    .map((run) => {
      const diagnostics = data.diagnostics.filter((entry) => entry.runId === run.runId).map(mapDiagnosticEvent);
      const artifacts = data.artifacts.filter((entry) => entry.runId === run.runId).map(mapArtifactEvent);
      return {
        runId: run.runId,
        commandId: run.commandId,
        laneId: run.laneId ?? null,
        machine: run.machine,
        workspace: run.workspace,
        cwd: run.cwd,
        command: run.command,
        startedAt: run.startedAt,
        endedAt: run.endedAt,
        durationMs: run.durationMs,
        status: run.status,
        exitCode: run.exitCode,
        stdoutArtifact: run.stdoutArtifact,
        stderrArtifact: run.stderrArtifact,
        summary: run.summary ?? null,
        diagnostics,
        artifacts,
      };
    });
}

function agentEvidenceStatsFromNdjson() {
  const data = readAgentEvidenceFromNdjson();
  const failedRuns = data.runs.filter((entry) => entry.status === 'failed').length;
  const passedRuns = data.runs.filter((entry) => entry.status === 'passed').length;
  const newestRun =
    data.runs.map((entry) => entry.startedAt).sort((left, right) => Date.parse(right) - Date.parse(left))[0] ?? null;
  return {
    totalRuns: data.runs.length,
    failedRuns,
    passedRuns,
    newestStartedAt: newestRun,
    totalDiagnostics: data.diagnostics.length,
    uniqueDiagnosticSignatures: new Set(data.diagnostics.map((entry) => entry.signature)).size,
  };
}

async function tryDb(work) {
  if (!fs.existsSync(getAgentEvidenceDbPath())) {
    return null;
  }
  try {
    return await work();
  } catch {
    return null;
  }
}

function ensureLocalPath(candidatePath) {
  const absolute = path.resolve(candidatePath);
  const allowedRoots = [getLogRoot(), getStructuredLogBaseRoot()].map((entry) => path.resolve(entry));

  const allowed = allowedRoots.some((root) => absolute.startsWith(root));
  if (!allowed) {
    throw new Error('Artifact path must stay inside local logging roots.');
  }
  return absolute;
}

export async function getLatestFailures(options = {}) {
  const limit = clampLimit(options.limit, 10);
  const fromDb = await tryDb(() => getLatestFailureRuns('parent-codex', limit));
  if (fromDb != null && fromDb.length > 0) {
    return fromDb;
  }
  return assembleLatestFailuresFromNdjson(limit);
}

export async function getRunDiagnostics(options) {
  const fromDb = await tryDb(() => getRunEvidence(options.runId, 'parent-codex'));
  const evidence = fromDb ?? assembleRunEvidenceFromNdjson(options.runId);
  if (evidence == null) {
    throw new Error(`Run not found: ${options.runId}`);
  }
  const limit = clampLimit(options.limit, 100);
  const diagnostics = evidence.diagnostics.slice(0, limit);
  if (options.includeArtifactRefs) {
    return {
      run: evidence.run,
      diagnostics,
      artifacts: evidence.artifacts,
    };
  }
  return diagnostics;
}

export async function getArtifactSlice(options) {
  const maxLines = clampArtifactLines(options.maxLines);
  const startLine = Math.max(Number(options.startLine ?? 1), 1);
  let endLine = options.endLine == null ? startLine + maxLines - 1 : Number(options.endLine);
  if (!Number.isFinite(endLine) || endLine < startLine) {
    endLine = startLine + maxLines - 1;
  }
  if (endLine - startLine + 1 > maxLines) {
    endLine = startLine + maxLines - 1;
  }

  let resolvedPath = null;
  if (options.artifactId != null) {
    const artifact =
      (await tryDb(() => getArtifact(options.artifactId, 'parent-codex'))) ??
      readAgentEvidenceFromNdjson().artifacts.find((entry) => entry.artifactId === options.artifactId);
    if (artifact == null) {
      throw new Error(`Artifact not found: ${options.artifactId}`);
    }
    resolvedPath = artifact.path;
  } else if (options.path != null) {
    resolvedPath = options.path;
  }

  if (resolvedPath == null) {
    throw new Error('artifactId or path is required.');
  }

  const artifactPath = ensureLocalPath(resolvedPath);
  if (!fs.existsSync(artifactPath)) {
    throw new Error(`Artifact file missing: ${artifactPath}`);
  }

  const lines = fs.readFileSync(artifactPath, 'utf8').split(/\r?\n/);
  const slice = lines.slice(startLine - 1, endLine);
  return {
    path: artifactPath.replace(/\\/g, '/'),
    startLine,
    endLine: startLine + slice.length - 1,
    lineCount: slice.length,
    lines: slice,
  };
}

export async function getErrors(options = {}) {
  const scope = options.scope ?? 'parent-codex';
  const limit = clampLimit(options.limit);

  if (scope === 'parent-codex') {
    const failures = await getLatestFailures({ limit });
    return failures.flatMap((failure) =>
      failure.diagnostics.map((diagnostic) => ({
        ...normalizeEvidenceDiagnostic(diagnostic, failure),
      }))
    );
  }

  const logs = readLocalLogs(scope);
  if (logs.length === 0) {
    throw missingStructuredScopeError(scope);
  }
  return filterLogs(logs, {
    level: 'error',
    fromTs: parseIsoOrDuration(options.since),
    toTs: null,
  }).slice(0, limit);
}

export async function getRecentLogs(options = {}) {
  const scope = options.scope ?? 'parent-codex';
  const limit = clampLimit(options.limit);
  const level = options.level ?? null;

  if (scope === 'parent-codex') {
    const failures = await getLatestFailures({ limit });
    const logs = failures.flatMap((failure) =>
      failure.diagnostics.map((diagnostic) => ({
        ...normalizeEvidenceDiagnostic(diagnostic, failure),
      }))
    );
    return level == null ? logs.slice(0, limit) : logs.filter((log) => log.level === level).slice(0, limit);
  }

  const logs = readLocalLogs(scope);
  if (logs.length === 0) {
    throw missingStructuredScopeError(scope);
  }
  return filterLogs(logs, {
    level,
    fromTs: parseIsoOrDuration(options.since),
    toTs: null,
  }).slice(0, limit);
}

export async function getLogsBySource(options) {
  const scope = options.scope ?? 'parent-test';
  const logs = readLocalLogs(scope);
  if (logs.length === 0) {
    throw missingStructuredScopeError(scope);
  }
  return filterLogs(logs, {
    source: options.source,
    level: options.level ?? null,
    fromTs: null,
    toTs: null,
  }).slice(0, clampLimit(options.limit));
}

export async function getLogsByContext(options) {
  const scope = options.scope ?? 'parent-test';
  const logs = readLocalLogs(scope);
  if (logs.length === 0) {
    throw missingStructuredScopeError(scope);
  }
  return filterLogs(logs, {
    context: options.context,
    level: options.level ?? null,
    fromTs: null,
    toTs: null,
  }).slice(0, clampLimit(options.limit));
}

export async function queryLogs(options = {}) {
  const scope = options.scope ?? 'parent-codex';
  const limit = clampLimit(options.limit);

  if (scope === 'parent-codex' && options.source == null && options.context == null) {
    const failures = await getLatestFailures({ limit });
    const logs = failures.flatMap((failure) =>
      failure.diagnostics.map((diagnostic) => ({
        ...normalizeEvidenceDiagnostic(diagnostic, failure),
      }))
    );
    return filterLogs(logs, {
      level: options.level ?? null,
      contains: options.contains ?? null,
      runId: options.runId ?? null,
      fromTs: parseIsoOrDuration(options.from),
      toTs: parseIsoOrDuration(options.to),
    }).slice(0, limit);
  }

  const logs = readLocalLogs(scope);
  if (logs.length === 0) {
    throw missingStructuredScopeError(scope);
  }
  return filterLogs(logs, {
    level: options.level ?? null,
    source: options.source ?? null,
    context: options.context ?? null,
    runId: options.runId ?? null,
    contains: options.contains ?? null,
    fromTs: parseIsoOrDuration(options.from),
    toTs: parseIsoOrDuration(options.to),
  }).slice(0, limit);
}

export async function getLogStats(options = {}) {
  const scope = options.scope ?? 'parent-codex';
  const logs = readLocalLogs(scope);
  const filtered =
    logs.length === 0
      ? []
      : filterLogs(logs, {
          fromTs: parseIsoOrDuration(options.from),
          toTs: parseIsoOrDuration(options.to),
        });

  const levelCounts = {};
  const sourceCounts = {};
  const contextCounts = {};
  for (const log of filtered) {
    levelCounts[log.level] = (levelCounts[log.level] ?? 0) + 1;
    if (log.source != null) {
      sourceCounts[log.source] = (sourceCounts[log.source] ?? 0) + 1;
    }
    if (log.context != null) {
      contextCounts[log.context] = (contextCounts[log.context] ?? 0) + 1;
    }
  }

  const evidenceStats =
    scope === 'parent-codex'
      ? ((await tryDb(() => getAgentEvidenceStats('parent-codex'))) ?? agentEvidenceStatsFromNdjson())
      : null;
  return {
    scope,
    logLevels: levelCounts,
    sources: sourceCounts,
    contexts: contextCounts,
    agentEvidence: evidenceStats,
  };
}

export async function getProofInventoryStatus(options = {}) {
  const workspaceRoot = path.resolve(options.workspaceRoot ?? getWorkspaceRoot());
  const proofIndexPath = getLoggingPlanFile('PROOF_INDEX.md', workspaceRoot);
  const workpackIndexPath = getLoggingPlanFile('WORKPACK_INDEX.md', workspaceRoot);
  const checklistPath = getLoggingPlanFile('CHECKLIST_INDEX.md', workspaceRoot);
  const planStatePath = getLoggingPlanFile('PLAN_STATE.md', workspaceRoot);

  const proofIndexText = readTextIfExists(proofIndexPath);
  const workpackIndexText = readTextIfExists(workpackIndexPath);
  const checklistText = readTextIfExists(checklistPath);
  const planStateText = readTextIfExists(planStatePath);

  if (proofIndexText == null || workpackIndexText == null || checklistText == null || planStateText == null) {
    throw new Error(
      `Logging proof inventory docs are incomplete under ${getLoggingPlanRoot(workspaceRoot).replace(/\\/g, '/')}.`
    );
  }

  const proofRoots = parseProofRootDefinitions(proofIndexText);
  const workpackStatuses = parseWorkpackIndexStatuses(workpackIndexText);
  const checklistState = parseChecklistProofState(checklistText);
  const restoredRootsClaim = parsePlanStateRestoredRootsClaim(planStateText);

  const workpacks = [];
  const gaps = [];
  const actualPresentWorkpackIds = [];
  const actualMissingWorkpackIds = [];

  for (const proofRoot of proofRoots) {
    const absoluteRoot = path.join(workspaceRoot, proofRoot.relativePath);
    const artifactFiles = listFilesRecursive(absoluteRoot);
    const status = workpackStatuses.get(proofRoot.workpackId) ?? null;
    const checklist = checklistState.get(proofRoot.workpackId) ?? {
      workpackId: proofRoot.workpackId,
      proofRowChecked: false,
      workpackCompletionChecked: false,
    };
    const exists = artifactFiles.length > 0;
    if (exists) {
      actualPresentWorkpackIds.push(proofRoot.workpackId);
    } else {
      actualMissingWorkpackIds.push(proofRoot.workpackId);
    }

    if (!exists && status?.status === 'partial-proof') {
      gaps.push({
        kind: 'status-claims-proof-root-but-root-missing',
        severity: 'error',
        workpackId: proofRoot.workpackId,
        message: `WORKPACK_INDEX marks WP${proofRoot.workpackId} as partial-proof, but ${proofRoot.relativePath} has no artifacts.`,
      });
    }

    if (!exists && checklist.proofRowChecked) {
      gaps.push({
        kind: 'checklist-claims-proof-root-written-but-root-missing',
        severity: 'error',
        workpackId: proofRoot.workpackId,
        message: `CHECKLIST_INDEX says the proof root is written for WP${proofRoot.workpackId}, but ${proofRoot.relativePath} is missing.`,
      });
    }

    if (!exists && checklist.workpackCompletionChecked) {
      gaps.push({
        kind: 'checklist-claims-workpack-completion-without-proof-root',
        severity: 'error',
        workpackId: proofRoot.workpackId,
        message: `CHECKLIST_INDEX says the workpack completion section is filled for WP${proofRoot.workpackId}, but ${proofRoot.relativePath} is missing.`,
      });
    }

    if (exists && (status?.status === 'source-present' || status?.status === 'audit-open')) {
      gaps.push({
        kind: 'status-underclaims-existing-proof-root',
        severity: 'warning',
        workpackId: proofRoot.workpackId,
        message: `WORKPACK_INDEX still says ${status.status} for WP${proofRoot.workpackId}, but ${proofRoot.relativePath} has artifacts on disk.`,
      });
    }

    workpacks.push({
      workpackId: proofRoot.workpackId,
      title: status?.title ?? null,
      status: status?.status ?? null,
      boxes: status?.boxes ?? null,
      proofRoot: proofRoot.relativePath,
      proofArtifacts: artifactFiles.length,
      proofRootExists: exists,
      checklist,
      artifacts: artifactFiles.map((filePath) => path.relative(workspaceRoot, filePath).replace(/\\/g, '/')),
    });
  }

  actualPresentWorkpackIds.sort();
  actualMissingWorkpackIds.sort();

  if (restoredRootsClaim != null && !sameStringSet(restoredRootsClaim.workpackIds, actualPresentWorkpackIds)) {
    gaps.push({
      kind: 'plan-state-restored-roots-drift',
      severity: 'warning',
      claimedWorkpackIds: restoredRootsClaim.workpackIds,
      actualWorkpackIds: actualPresentWorkpackIds,
      message: `PLAN_STATE restored-root summary is stale. Claimed: ${restoredRootsClaim.workpackIds.join(', ')}. Actual: ${actualPresentWorkpackIds.join(', ') || 'none'}.`,
      line: restoredRootsClaim.line,
    });
  }

  return {
    plan: LOGGING_PLAN_NAME,
    workspaceRoot: workspaceRoot.replace(/\\/g, '/'),
    proofRoot: LOGGING_PLAN_PROOF_ROOT,
    actualPresentWorkpackIds,
    actualMissingWorkpackIds,
    summary: {
      totalWorkpacks: workpacks.length,
      presentProofRoots: actualPresentWorkpackIds.length,
      missingProofRoots: actualMissingWorkpackIds.length,
      blockingGapCount: gaps.filter((gap) => gap.severity === 'error').length,
      warningGapCount: gaps.filter((gap) => gap.severity === 'warning').length,
    },
    workpacks,
    gaps,
  };
}

export async function getProofTrace(options = {}) {
  const scope = options.scope ?? DEFAULT_PROOF_TRACE_SCOPE;
  const proofId = options.proofId ?? options.proof_id ?? findLatestProofTraceId(scope);
  if (proofId == null || String(proofId).trim().length === 0) {
    throw new Error(`No proof trace rows found for scope "${scope}".`);
  }

  const rows = proofTraceRowsForScope(scope, String(proofId)).slice(0, clampLimit(options.limit, 100));
  if (rows.length === 0) {
    throw new Error(`Proof trace not found: ${proofId}`);
  }

  return {
    proofId: String(proofId),
    scope,
    rows,
  };
}

export async function getProofTraceGaps(options = {}) {
  const trace = await getProofTrace(options);
  const expectedSteps = parseExpectedSteps(options.expectedSteps ?? options.expected_steps ?? []);
  let rowIndex = 0;
  const matchedSteps = [];
  const missingSteps = [];
  const outOfOrderSteps = [];

  for (const expectedStep of expectedSteps) {
    const foundIndex = trace.rows.findIndex(
      (row, index) => index >= rowIndex && proofTraceStepMatches(row, expectedStep)
    );
    if (foundIndex === -1) {
      const anywhereIndex = trace.rows.findIndex((row) => proofTraceStepMatches(row, expectedStep));
      if (anywhereIndex !== -1) {
        outOfOrderSteps.push({
          expected: expectedStep,
          matchedRow: trace.rows[anywhereIndex],
        });
      } else {
        missingSteps.push(expectedStep);
      }
      continue;
    }
    matchedSteps.push({
      expected: expectedStep,
      matchedRow: trace.rows[foundIndex],
    });
    rowIndex = foundIndex + 1;
  }

  const unexpectedErrorRows = trace.rows.filter((row) => row.level === 'error' || row.level === 'warn');

  return {
    proofId: trace.proofId,
    scope: trace.scope,
    matchedSteps,
    missingSteps,
    outOfOrderSteps,
    unexpectedErrorRows,
    rows: trace.rows,
  };
}

export async function queryProofTrace(options = {}) {
  const trace = await getProofTrace(options);
  if (options.expectedSteps == null && options.expected_steps == null) {
    return trace;
  }
  return getProofTraceGaps(options);
}
