import fs from 'node:fs';
import { DuckDBInstance } from '@duckdb/node-api';

import { getDuckDbPath, getEvidenceScope, getManifestPath, listNdjsonFiles } from './agent-log-paths.mjs';

function openDatabase(filePath) {
  return DuckDBInstance.create(filePath);
}

function runAsync(connection, sql, ...params) {
  return connection.run(sql, params).then(() => undefined);
}

async function allAsync(connection, sql, ...params) {
  const reader = await connection.runAndReadAll(sql, params);
  return reader.getRowObjects();
}

function closeConnection(connection) {
  connection.disconnectSync();
}

function closeDatabase(database) {
  database.closeSync();
}

async function withConnection(scope, work) {
  const database = await openDatabase(getDuckDbPath(scope));
  const connection = await database.connect();
  try {
    await ensureSchema(connection);
    return await work(connection);
  } finally {
    closeConnection(connection);
    closeDatabase(database);
  }
}

async function ensureSchema(connection) {
  await runAsync(
    connection,
    `CREATE TABLE IF NOT EXISTS agent_runs (
      ndjson_file VARCHAR NOT NULL,
      run_id VARCHAR NOT NULL,
      command_id VARCHAR NOT NULL,
      lane_id VARCHAR,
      machine VARCHAR,
      workspace VARCHAR NOT NULL,
      cwd VARCHAR NOT NULL,
      command_json VARCHAR NOT NULL,
      started_at VARCHAR NOT NULL,
      ended_at VARCHAR NOT NULL,
      duration_ms BIGINT NOT NULL,
      status VARCHAR NOT NULL,
      exit_code BIGINT,
      stdout_artifact VARCHAR,
      stderr_artifact VARCHAR,
      summary VARCHAR
    )`
  );
  await runAsync(
    connection,
    `CREATE TABLE IF NOT EXISTS agent_diagnostics (
      ndjson_file VARCHAR NOT NULL,
      diagnostic_id VARCHAR NOT NULL,
      run_id VARCHAR NOT NULL,
      command_id VARCHAR NOT NULL,
      kind VARCHAR NOT NULL,
      severity VARCHAR NOT NULL,
      signature VARCHAR NOT NULL,
      file VARCHAR,
      line BIGINT,
      column_value BIGINT,
      message VARCHAR NOT NULL,
      raw_artifact VARCHAR,
      raw_start_line BIGINT,
      raw_end_line BIGINT,
      hit_count BIGINT NOT NULL
    )`
  );
  await runAsync(
    connection,
    `CREATE TABLE IF NOT EXISTS agent_artifacts (
      ndjson_file VARCHAR NOT NULL,
      artifact_id VARCHAR NOT NULL,
      run_id VARCHAR NOT NULL,
      command_id VARCHAR NOT NULL,
      path VARCHAR NOT NULL,
      kind VARCHAR NOT NULL,
      sha256 VARCHAR NOT NULL,
      byte_length BIGINT NOT NULL,
      line_count BIGINT NOT NULL,
      created_at VARCHAR NOT NULL
    )`
  );

  await runAsync(connection, 'CREATE INDEX IF NOT EXISTS idx_agent_runs_run_id ON agent_runs(run_id)');
  await runAsync(connection, 'CREATE INDEX IF NOT EXISTS idx_agent_runs_status ON agent_runs(status)');
  await runAsync(connection, 'CREATE INDEX IF NOT EXISTS idx_agent_runs_started_at ON agent_runs(started_at)');
  await runAsync(
    connection,
    'CREATE INDEX IF NOT EXISTS idx_agent_diagnostics_signature ON agent_diagnostics(signature)'
  );
  await runAsync(connection, 'CREATE INDEX IF NOT EXISTS idx_agent_diagnostics_file ON agent_diagnostics(file)');
  await runAsync(connection, 'CREATE INDEX IF NOT EXISTS idx_agent_diagnostics_kind ON agent_diagnostics(kind)');
  await runAsync(
    connection,
    'CREATE INDEX IF NOT EXISTS idx_agent_diagnostics_severity ON agent_diagnostics(severity)'
  );
  await runAsync(connection, 'CREATE INDEX IF NOT EXISTS idx_agent_diagnostics_run_id ON agent_diagnostics(run_id)');
  await runAsync(connection, 'CREATE INDEX IF NOT EXISTS idx_agent_artifacts_run_id ON agent_artifacts(run_id)');
}

function readManifest(scope) {
  const manifestPath = getManifestPath(scope);
  if (!fs.existsSync(manifestPath)) {
    return {};
  }

  try {
    return JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
  } catch {
    return {};
  }
}

function writeManifest(scope, manifest) {
  fs.writeFileSync(getManifestPath(scope), `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');
}

function fileMeta(filePath) {
  const stat = fs.statSync(filePath);
  return {
    size: stat.size,
    modifiedMs: stat.mtimeMs,
  };
}

function changedFiles(scope, force) {
  const files = listNdjsonFiles(scope);
  if (force) {
    return { files, manifest: {} };
  }

  const manifest = readManifest(scope);
  const changed = files.filter((filePath) => {
    const existing = manifest[filePath];
    const current = fileMeta(filePath);
    return existing == null || existing.size !== current.size || existing.modifiedMs !== current.modifiedMs;
  });

  return { files: changed, manifest };
}

function updateManifest(scope) {
  const next = {};
  for (const filePath of listNdjsonFiles(scope)) {
    next[filePath] = fileMeta(filePath);
  }
  writeManifest(scope, next);
}

function readNdjson(filePath) {
  const content = fs.readFileSync(filePath, 'utf8').trim();
  if (content.length === 0) {
    return [];
  }
  return content
    .split(/\r?\n/)
    .filter((line) => line.trim().length > 0)
    .map((line) => JSON.parse(line));
}

async function ingestFile(connection, filePath) {
  const events = readNdjson(filePath);
  if (events.length === 0) {
    return 0;
  }

  const eventType = events[0]?.eventType;
  if (eventType === 'agent-run') {
    await runAsync(connection, 'DELETE FROM agent_runs WHERE ndjson_file = ?', filePath);
    for (const event of events) {
      await runAsync(
        connection,
        `INSERT INTO agent_runs (
          ndjson_file, run_id, command_id, lane_id, machine, workspace, cwd, command_json,
          started_at, ended_at, duration_ms, status, exit_code, stdout_artifact, stderr_artifact, summary
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
        filePath,
        event.runId,
        event.commandId,
        event.laneId,
        event.machine,
        event.workspace,
        event.cwd,
        JSON.stringify(event.command),
        event.startedAt,
        event.endedAt,
        event.durationMs,
        event.status,
        event.exitCode,
        event.stdoutArtifact,
        event.stderrArtifact,
        event.summary
      );
    }
    return events.length;
  }

  if (eventType === 'diagnostic') {
    await runAsync(connection, 'DELETE FROM agent_diagnostics WHERE ndjson_file = ?', filePath);
    for (const event of events) {
      await runAsync(
        connection,
        `INSERT INTO agent_diagnostics (
          ndjson_file, diagnostic_id, run_id, command_id, kind, severity, signature, file,
          line, column_value, message, raw_artifact, raw_start_line, raw_end_line, hit_count
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
        filePath,
        event.diagnosticId,
        event.runId,
        event.commandId,
        event.kind,
        event.severity,
        event.signature,
        event.file,
        event.line,
        event.column,
        event.message,
        event.rawArtifact,
        event.rawStartLine,
        event.rawEndLine,
        event.hitCount ?? 1
      );
    }
    return events.length;
  }

  if (eventType === 'artifact') {
    await runAsync(connection, 'DELETE FROM agent_artifacts WHERE ndjson_file = ?', filePath);
    for (const event of events) {
      await runAsync(
        connection,
        `INSERT INTO agent_artifacts (
          ndjson_file, artifact_id, run_id, command_id, path, kind, sha256, byte_length, line_count, created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
        filePath,
        event.artifactId,
        event.runId,
        event.commandId,
        event.path,
        event.kind,
        event.sha256,
        event.byteLength,
        event.lineCount,
        event.createdAt
      );
    }
    return events.length;
  }

  return 0;
}

export async function ingestAgentEvidence(scope = getEvidenceScope(), force = false) {
  return withConnection(scope, async (connection) => {
    return ingestAgentEvidenceWithConnection(connection, scope, force);
  });
}

export async function getLatestFailures(scope = getEvidenceScope(), limit = 5) {
  return withConnection(scope, async (connection) => {
    await ingestAgentEvidenceWithConnection(connection, scope, false);
    const runs = await allAsync(
      connection,
      `SELECT
        run_id,
        command_id,
        lane_id,
        machine,
        workspace,
        cwd,
        command_json,
        started_at,
        ended_at,
        duration_ms,
        status,
        exit_code,
        stdout_artifact,
        stderr_artifact,
        summary
      FROM agent_runs
      WHERE status = 'failed'
      ORDER BY started_at DESC
      LIMIT ?`,
      limit
    );

    const enriched = [];
    for (const run of runs) {
      const diagnostics = await allAsync(
        connection,
        `SELECT diagnostic_id, kind, severity, signature, file, line, column_value, message, raw_artifact, hit_count
        FROM agent_diagnostics
        WHERE run_id = ?
        ORDER BY severity DESC, file ASC NULLS LAST, line ASC NULLS LAST`,
        run.run_id
      );
      const artifacts = await allAsync(
        connection,
        `SELECT artifact_id, kind, path
        FROM agent_artifacts
        WHERE run_id = ?
        ORDER BY kind ASC`,
        run.run_id
      );
      enriched.push({
        ...mapRunRow(run),
        diagnostics: diagnostics.map(mapDiagnosticRow),
        artifacts: artifacts.map(mapArtifactRow),
      });
    }

    return enriched;
  });
}

export async function getRunEvidence(runId, scope = getEvidenceScope()) {
  return withConnection(scope, async (connection) => {
    await ingestAgentEvidenceWithConnection(connection, scope, false);
    const runs = await allAsync(
      connection,
      `SELECT
        run_id,
        command_id,
        lane_id,
        machine,
        workspace,
        cwd,
        command_json,
        started_at,
        ended_at,
        duration_ms,
        status,
        exit_code,
        stdout_artifact,
        stderr_artifact,
        summary
      FROM agent_runs
      WHERE run_id = ?
      LIMIT 1`,
      runId
    );
    const run = runs[0];
    if (run == null) {
      return null;
    }
    const diagnostics = await allAsync(
      connection,
      `SELECT diagnostic_id, kind, severity, signature, file, line, column_value, message, raw_artifact, raw_start_line, raw_end_line, hit_count
      FROM agent_diagnostics
      WHERE run_id = ?
      ORDER BY severity DESC, file ASC NULLS LAST, line ASC NULLS LAST`,
      runId
    );
    const artifacts = await allAsync(
      connection,
      `SELECT artifact_id, kind, path, sha256, byte_length, line_count, created_at
      FROM agent_artifacts
      WHERE run_id = ?
      ORDER BY kind ASC`,
      runId
    );
    return {
      run: mapRunRow(run),
      diagnostics: diagnostics.map(mapDiagnosticRow),
      artifacts: artifacts.map(mapArtifactRow),
    };
  });
}

export async function getStats(scope = getEvidenceScope()) {
  return withConnection(scope, async (connection) => {
    await ingestAgentEvidenceWithConnection(connection, scope, false);
    const rows = await allAsync(
      connection,
      `SELECT
        COUNT(*)::BIGINT AS total_runs,
        SUM(CASE WHEN status = 'failed' THEN 1 ELSE 0 END)::BIGINT AS failed_runs,
        SUM(CASE WHEN status = 'passed' THEN 1 ELSE 0 END)::BIGINT AS passed_runs,
        MAX(started_at) AS newest_started_at
      FROM agent_runs`
    );
    const diagnosticRows = await allAsync(
      connection,
      `SELECT
        COUNT(*)::BIGINT AS total_diagnostics,
        COUNT(DISTINCT signature)::BIGINT AS unique_signatures
      FROM agent_diagnostics`
    );
    return {
      totalRuns: Number(rows[0]?.total_runs ?? 0),
      failedRuns: Number(rows[0]?.failed_runs ?? 0),
      passedRuns: Number(rows[0]?.passed_runs ?? 0),
      newestStartedAt: rows[0]?.newest_started_at ?? null,
      totalDiagnostics: Number(diagnosticRows[0]?.total_diagnostics ?? 0),
      uniqueDiagnosticSignatures: Number(diagnosticRows[0]?.unique_signatures ?? 0),
    };
  });
}

export async function getArtifact(artifactId, scope = getEvidenceScope()) {
  return withConnection(scope, async (connection) => {
    await ingestAgentEvidenceWithConnection(connection, scope, false);
    const rows = await allAsync(
      connection,
      `SELECT artifact_id, run_id, command_id, path, kind, sha256, byte_length, line_count, created_at
      FROM agent_artifacts
      WHERE artifact_id = ?
      LIMIT 1`,
      artifactId
    );
    const row = rows[0];
    return row == null ? null : mapArtifactRow(row);
  });
}

export async function getDiagnosticsForRun(runId, scope = getEvidenceScope()) {
  return withConnection(scope, async (connection) => {
    await ingestAgentEvidenceWithConnection(connection, scope, false);
    const rows = await allAsync(
      connection,
      `SELECT diagnostic_id, kind, severity, signature, file, line, column_value, message, raw_artifact, raw_start_line, raw_end_line, hit_count
      FROM agent_diagnostics
      WHERE run_id = ?
      ORDER BY severity DESC, file ASC NULLS LAST, line ASC NULLS LAST`,
      runId
    );
    return rows.map(mapDiagnosticRow);
  });
}

export async function getLatestFailureForLane(laneId, scope = getEvidenceScope()) {
  return withConnection(scope, async (connection) => {
    await ingestAgentEvidenceWithConnection(connection, scope, false);
    const runs = await allAsync(
      connection,
      `SELECT
        run_id,
        command_id,
        lane_id,
        machine,
        workspace,
        cwd,
        command_json,
        started_at,
        ended_at,
        duration_ms,
        status,
        exit_code,
        stdout_artifact,
        stderr_artifact,
        summary
      FROM agent_runs
      WHERE status = 'failed' AND lane_id = ?
      ORDER BY started_at DESC
      LIMIT 1`,
      laneId
    );
    const run = runs[0];
    if (run == null) {
      return null;
    }
    const diagnostics = await allAsync(
      connection,
      `SELECT diagnostic_id, kind, severity, signature, file, line, column_value, message, raw_artifact, raw_start_line, raw_end_line, hit_count
      FROM agent_diagnostics
      WHERE run_id = ?
      ORDER BY severity DESC, file ASC NULLS LAST, line ASC NULLS LAST`,
      run.run_id
    );
    const artifacts = await allAsync(
      connection,
      `SELECT artifact_id, kind, path, sha256, byte_length, line_count, created_at
      FROM agent_artifacts
      WHERE run_id = ?
      ORDER BY kind ASC`,
      run.run_id
    );
    return {
      run: mapRunRow(run),
      diagnostics: diagnostics.map(mapDiagnosticRow),
      artifacts: artifacts.map(mapArtifactRow),
    };
  });
}

async function ingestAgentEvidenceWithConnection(connection, scope, force) {
  const { files } = changedFiles(scope, force);
  let eventsIngested = 0;
  for (const filePath of files) {
    eventsIngested += await ingestFile(connection, filePath);
  }
  updateManifest(scope);
  return {
    filesProcessed: files.length,
    eventsIngested,
  };
}

function mapRunRow(row) {
  return {
    runId: row.run_id,
    commandId: row.command_id,
    laneId: row.lane_id,
    machine: row.machine,
    workspace: row.workspace,
    cwd: row.cwd,
    command: JSON.parse(row.command_json),
    startedAt: row.started_at,
    endedAt: row.ended_at,
    durationMs: Number(row.duration_ms),
    status: row.status,
    exitCode: row.exit_code == null ? null : Number(row.exit_code),
    stdoutArtifact: row.stdout_artifact,
    stderrArtifact: row.stderr_artifact,
    summary: row.summary,
  };
}

function mapDiagnosticRow(row) {
  return {
    diagnosticId: row.diagnostic_id,
    kind: row.kind,
    severity: row.severity,
    signature: row.signature,
    file: row.file,
    line: row.line == null ? null : Number(row.line),
    column: row.column_value == null ? null : Number(row.column_value),
    message: row.message,
    rawArtifact: row.raw_artifact,
    rawStartLine: row.raw_start_line == null ? null : Number(row.raw_start_line),
    rawEndLine: row.raw_end_line == null ? null : Number(row.raw_end_line),
    hitCount: Number(row.hit_count ?? 1),
  };
}

function mapArtifactRow(row) {
  return {
    artifactId: row.artifact_id,
    runId: row.run_id ?? null,
    commandId: row.command_id ?? null,
    path: row.path,
    kind: row.kind,
    sha256: row.sha256 ?? null,
    byteLength: row.byte_length == null ? null : Number(row.byte_length),
    lineCount: row.line_count == null ? null : Number(row.line_count),
    createdAt: row.created_at ?? null,
  };
}
