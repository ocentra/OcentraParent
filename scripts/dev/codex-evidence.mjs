#!/usr/bin/env node

import { detectLaneId } from './lib/agent-log-paths.mjs';
import { formatEvidencePacket } from './lib/agent-summary-format.mjs';
import { getLatestFailures, getRunDiagnostics } from './lib/log-query-service.mjs';

async function main() {
  const argv = process.argv.slice(2);
  const command = argv[0] ?? 'latest-failures';

  if (command === 'latest-failures') {
    const failures = await getLatestFailures();
    const latest = failures[0];
    process.stdout.write(
      formatEvidencePacket(
        latest == null
          ? null
          : {
              run: {
                runId: latest.runId,
                commandId: latest.commandId,
                laneId: latest.laneId,
                machine: latest.machine,
                workspace: latest.workspace,
                cwd: latest.cwd,
                command: latest.command,
                startedAt: latest.startedAt,
                endedAt: latest.endedAt,
                durationMs: latest.durationMs,
                status: latest.status,
                exitCode: latest.exitCode,
                stdoutArtifact: latest.stdoutArtifact,
                stderrArtifact: latest.stderrArtifact,
                summary: latest.summary,
              },
              diagnostics: latest.diagnostics,
              artifacts: latest.artifacts,
            }
      )
    );
    return;
  }

  if (command === 'by-run') {
    const result = await getRunDiagnostics({
      runId: argv[1] ?? '',
      includeArtifactRefs: true,
      limit: 100,
    });
    process.stdout.write(
      formatEvidencePacket({
        run: result.run,
        diagnostics: result.diagnostics,
        artifacts: result.artifacts,
      })
    );
    return;
  }

  if (command === 'current-lane') {
    const laneId = detectLaneId();
    if (laneId == null) {
      process.stdout.write('# Evidence Packet\nNo lane id detected in environment.\n');
      return;
    }
    const latest = (await getLatestFailures({ limit: 20 })).find((failure) => failure.laneId === laneId);
    process.stdout.write(
      formatEvidencePacket(
        latest == null
          ? null
          : {
              run: {
                runId: latest.runId,
                commandId: latest.commandId,
                laneId: latest.laneId,
                machine: latest.machine,
                workspace: latest.workspace,
                cwd: latest.cwd,
                command: latest.command,
                startedAt: latest.startedAt,
                endedAt: latest.endedAt,
                durationMs: latest.durationMs,
                status: latest.status,
                exitCode: latest.exitCode,
                stdoutArtifact: latest.stdoutArtifact,
                stderrArtifact: latest.stderrArtifact,
                summary: latest.summary,
              },
              diagnostics: latest.diagnostics,
              artifacts: latest.artifacts,
            },
        laneId
      )
    );
    return;
  }

  process.stderr.write(`Unknown command: ${command}\n`);
  process.exitCode = 1;
}

void main();
