export function formatRunSummary({ run, diagnostics, artifacts, stdout, stderr, includeStdout, includeStderr, raw }) {
  const lines = [
    `run_id: ${run.runId}`,
    `command_id: ${run.commandId}`,
    `status: ${run.status}`,
    `exit_code: ${run.exitCode ?? 'null'}`,
    `duration_ms: ${run.durationMs}`,
    `command: ${run.command.join(' ')}`,
    `unique_diagnostics: ${diagnostics.length}`,
  ];

  if (diagnostics.length > 0) {
    lines.push('');
    lines.push('diagnostics:');
    for (const diagnostic of diagnostics.slice(0, 10)) {
      lines.push(`- [${diagnostic.signature}] ${formatLocation(diagnostic)} ${diagnostic.message}`);
    }
  }

  lines.push('');
  lines.push('artifacts:');
  for (const artifact of artifacts) {
    lines.push(`- ${artifact.kind}: ${artifact.path}`);
  }

  lines.push('');
  lines.push(`next_query: npm run agent:query -- by-run ${run.runId}`);

  if (raw || includeStdout) {
    lines.push('');
    lines.push('stdout:');
    lines.push(stdout.length > 0 ? stdout : '<empty>');
  }

  if (raw || includeStderr) {
    lines.push('');
    lines.push('stderr:');
    lines.push(stderr.length > 0 ? stderr : '<empty>');
  }

  return `${lines.join('\n')}\n`;
}

export function formatLatestFailures(runs) {
  if (runs.length === 0) {
    return 'No failed runs recorded.\n';
  }

  const lines = [];
  for (const item of runs) {
    lines.push(`run_id: ${item.runId}`);
    lines.push(`command_id: ${item.commandId}`);
    lines.push(`status: ${item.status}`);
    lines.push(`exit_code: ${item.exitCode ?? 'null'}`);
    lines.push(`duration_ms: ${item.durationMs}`);
    lines.push(`command: ${item.command.join(' ')}`);
    lines.push(`unique_diagnostics: ${item.diagnostics.length}`);
    if (item.diagnostics.length > 0) {
      for (const diagnostic of item.diagnostics.slice(0, 5)) {
        lines.push(`- [${diagnostic.signature}] ${formatLocation(diagnostic)} ${diagnostic.message}`);
      }
    }
    const stderrArtifact = item.artifacts.find((artifact) => artifact.kind === 'stderr');
    if (stderrArtifact != null) {
      lines.push(`stderr: ${stderrArtifact.path}`);
    }
    lines.push(`next_query: npm run agent:query -- by-run ${item.runId}`);
    lines.push('');
  }

  return `${lines.join('\n').trimEnd()}\n`;
}

export function formatRunEvidence(evidence) {
  if (evidence == null) {
    return 'Run not found.\n';
  }

  const { run, diagnostics, artifacts } = evidence;
  const lines = [
    `run_id: ${run.runId}`,
    `command_id: ${run.commandId}`,
    `status: ${run.status}`,
    `exit_code: ${run.exitCode ?? 'null'}`,
    `duration_ms: ${run.durationMs}`,
    `command: ${run.command.join(' ')}`,
    '',
    'diagnostics:',
  ];

  if (diagnostics.length === 0) {
    lines.push('- none');
  } else {
    for (const diagnostic of diagnostics) {
      lines.push(`- [${diagnostic.signature}] ${formatLocation(diagnostic)} ${diagnostic.message}`);
    }
  }

  lines.push('');
  lines.push('artifacts:');
  for (const artifact of artifacts) {
    lines.push(`- ${artifact.kind}: ${artifact.path}`);
  }
  lines.push('');
  lines.push(`next_query: npm run agent:query -- diagnostics --run-id ${run.runId}`);
  return `${lines.join('\n')}\n`;
}

export function formatStats(stats) {
  return `${[
    `total_runs: ${stats.totalRuns}`,
    `failed_runs: ${stats.failedRuns}`,
    `passed_runs: ${stats.passedRuns}`,
    `newest_started_at: ${stats.newestStartedAt ?? 'null'}`,
    `total_diagnostics: ${stats.totalDiagnostics}`,
    `unique_diagnostic_signatures: ${stats.uniqueDiagnosticSignatures}`,
  ].join('\n')}\n`;
}

export function formatArtifact(artifact, rawContent = null) {
  if (artifact == null) {
    return 'Artifact not found.\n';
  }

  const lines = [
    `artifact_id: ${artifact.artifactId}`,
    `run_id: ${artifact.runId ?? 'null'}`,
    `command_id: ${artifact.commandId ?? 'null'}`,
    `kind: ${artifact.kind}`,
    `path: ${artifact.path}`,
    `sha256: ${artifact.sha256 ?? 'null'}`,
    `byte_length: ${artifact.byteLength ?? 'null'}`,
    `line_count: ${artifact.lineCount ?? 'null'}`,
    `created_at: ${artifact.createdAt ?? 'null'}`,
  ];

  if (rawContent != null) {
    lines.push('');
    lines.push(rawContent);
  }

  return `${lines.join('\n')}\n`;
}

export function formatProofTrace(trace) {
  const lines = [`proof_id: ${trace.proofId}`, `scope: ${trace.scope}`, `row_count: ${trace.rows.length}`, '', 'rows:'];

  if (trace.rows.length === 0) {
    lines.push('- none');
  } else {
    for (const row of trace.rows) {
      const label = row.traceStep ?? row.eventType ?? row.action ?? row.message;
      lines.push(
        `- [${label}] ${row.source ?? '<no-source>'}/${row.context ?? '<no-context>'} ${row.status ?? ''}`.trim()
      );
    }
  }

  return `${lines.join('\n')}\n`;
}

export function formatProofTraceGaps(result) {
  const lines = [
    `proof_id: ${result.proofId}`,
    `scope: ${result.scope}`,
    `matched_steps: ${result.matchedSteps.length}`,
    `missing_steps: ${result.missingSteps.length}`,
    `out_of_order_steps: ${result.outOfOrderSteps.length}`,
    `unexpected_warn_or_error_rows: ${result.unexpectedErrorRows.length}`,
    '',
    'matched:',
  ];

  if (result.matchedSteps.length === 0) {
    lines.push('- none');
  } else {
    for (const match of result.matchedSteps) {
      const row = match.matchedRow;
      lines.push(`- ${row.traceStep ?? row.eventType ?? row.message}`);
    }
  }

  lines.push('');
  lines.push('missing:');
  if (result.missingSteps.length === 0) {
    lines.push('- none');
  } else {
    for (const step of result.missingSteps) {
      lines.push(`- ${typeof step === 'string' ? step : JSON.stringify(step)}`);
    }
  }

  lines.push('');
  lines.push('out_of_order:');
  if (result.outOfOrderSteps.length === 0) {
    lines.push('- none');
  } else {
    for (const item of result.outOfOrderSteps) {
      lines.push(`- ${typeof item.expected === 'string' ? item.expected : JSON.stringify(item.expected)}`);
    }
  }

  return `${lines.join('\n')}\n`;
}

export function formatEvidencePacket(evidence, laneId = null) {
  if (evidence == null) {
    return laneId == null
      ? '# Evidence Packet\nNo failed runs recorded.\n'
      : `# Evidence Packet\nlane: ${laneId}\nNo failed runs recorded for this lane.\n`;
  }

  const { run, diagnostics, artifacts } = evidence;
  const lines = [
    '# Evidence Packet',
    `run_id: ${run.runId}`,
    `command_id: ${run.commandId}`,
    `status: ${run.status}`,
    `command: ${run.command.join(' ')}`,
  ];

  if (laneId != null) {
    lines.push(`lane: ${laneId}`);
  }

  lines.push('');
  lines.push('## Diagnostics');
  if (diagnostics.length === 0) {
    lines.push('- none');
  } else {
    for (const diagnostic of diagnostics.slice(0, 10)) {
      lines.push(`- [${diagnostic.signature}] ${formatLocation(diagnostic)} ${diagnostic.message}`);
    }
  }

  lines.push('');
  lines.push('## Local artifacts');
  for (const artifact of artifacts) {
    lines.push(`- ${artifact.kind}: ${artifact.path}`);
  }

  lines.push('');
  lines.push('## Next action');
  lines.push('Use listed diagnostics first. Query local raw artifacts only when compact diagnostics are insufficient.');
  return `${lines.join('\n')}\n`;
}

function formatLocation(diagnostic) {
  const file = diagnostic.file ?? '<no-file>';
  if (diagnostic.line == null) {
    return file;
  }
  if (diagnostic.column == null) {
    return `${file}:${diagnostic.line}`;
  }
  return `${file}:${diagnostic.line}:${diagnostic.column}`;
}
