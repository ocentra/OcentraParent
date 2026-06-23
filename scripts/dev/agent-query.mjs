#!/usr/bin/env node

import fs from 'node:fs';

import {
  getArtifactSlice,
  getLatestFailures,
  getLogStats,
  getProofInventoryStatus,
  getProofTrace,
  getProofTraceGaps,
  getRunDiagnostics,
} from './lib/log-query-service.mjs';
import {
  formatArtifact,
  formatLatestFailures,
  formatProofTrace,
  formatProofTraceGaps,
  formatRunEvidence,
  formatStats,
} from './lib/agent-summary-format.mjs';

function parseFlag(argv, name) {
  const prefix = `--${name}=`;
  const entry = argv.find((value) => value.startsWith(prefix));
  return entry == null ? null : entry.slice(prefix.length);
}

async function main() {
  const argv = process.argv.slice(2);
  const command = argv[0] ?? 'latest-failures';
  const runId = parseFlag(argv, 'run-id');
  const scope = parseFlag(argv, 'scope');
  const proofId = parseFlag(argv, 'proof-id');
  const raw = argv.includes('--raw');

  if (command === 'latest-failures') {
    process.stdout.write(formatLatestFailures(await getLatestFailures()));
    return;
  }

  if (command === 'by-run') {
    const result = await getRunDiagnostics({
      runId: argv[1] ?? '',
      includeArtifactRefs: true,
      limit: 100,
    });
    process.stdout.write(
      formatRunEvidence({
        run: result.run,
        diagnostics: result.diagnostics,
        artifacts: result.artifacts,
      })
    );
    return;
  }

  if (command === 'diagnostics') {
    const diagnostics = await getRunDiagnostics({
      runId: runId ?? argv[1] ?? '',
      includeArtifactRefs: false,
      limit: 100,
    });
    if (diagnostics.length === 0) {
      process.stdout.write('No diagnostics found.\n');
      return;
    }
    const lines = diagnostics.map(
      (diagnostic) =>
        `[${diagnostic.signature}] ${diagnostic.file ?? '<no-file>'}${diagnostic.line == null ? '' : `:${diagnostic.line}${diagnostic.column == null ? '' : `:${diagnostic.column}`}`} ${diagnostic.message}`
    );
    process.stdout.write(`${lines.join('\n')}\n`);
    return;
  }

  if (command === 'artifact') {
    const slice = await getArtifactSlice({
      artifactId: argv[1] ?? '',
      startLine: Number(parseFlag(argv, 'start-line') ?? 1),
      endLine: parseFlag(argv, 'end-line') == null ? null : Number(parseFlag(argv, 'end-line')),
      maxLines: Number(parseFlag(argv, 'max-lines') ?? 80),
    });
    let content = null;
    if (raw && fs.existsSync(slice.path)) {
      content = fs.readFileSync(slice.path, 'utf8');
    }
    process.stdout.write(
      formatArtifact(
        {
          artifactId: argv[1] ?? '',
          runId: null,
          commandId: null,
          kind: 'artifact-slice',
          path: slice.path,
          sha256: null,
          byteLength: null,
          lineCount: slice.lineCount,
          createdAt: null,
        },
        content ?? slice.lines.join('\n')
      )
    );
    return;
  }

  if (command === 'stats') {
    const stats = await getLogStats({ scope: 'parent-codex' });
    process.stdout.write(formatStats(stats.agentEvidence ?? stats));
    return;
  }

  if (command === 'proof-inventory') {
    process.stdout.write(`${JSON.stringify(await getProofInventoryStatus(), null, 2)}\n`);
    return;
  }

  if (command === 'proof-trace') {
    const trace = await getProofTrace({
      scope: scope ?? undefined,
      proofId: proofId ?? undefined,
      limit: Number(parseFlag(argv, 'limit') ?? 100),
    });
    process.stdout.write(formatProofTrace(trace));
    return;
  }

  if (command === 'proof-trace-gaps') {
    const result = await getProofTraceGaps({
      scope: scope ?? undefined,
      proofId: proofId ?? undefined,
      expectedSteps: parseFlag(argv, 'expected-steps-json') ?? '[]',
      limit: Number(parseFlag(argv, 'limit') ?? 100),
    });
    process.stdout.write(formatProofTraceGaps(result));
    return;
  }

  process.stderr.write(`Unknown command: ${command}\n`);
  process.exitCode = 1;
}

void main();
