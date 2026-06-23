import crypto from 'node:crypto';
import path from 'node:path';

function buildDiagnosticId(runId, commandId, key) {
  return `diag-${crypto.createHash('sha256').update(`${runId}:${commandId}:${key}`).digest('hex').slice(0, 16)}`;
}

function severityFromToken(token) {
  if (token === 'warning') {
    return 'warning';
  }
  if (token === 'critical') {
    return 'critical';
  }
  return 'error';
}

function addDiagnostic(store, diagnostic) {
  const filePart = diagnostic.file ?? '';
  const linePart = diagnostic.line ?? 0;
  const columnPart = diagnostic.column ?? 0;
  const key = `${diagnostic.kind}|${diagnostic.signature}|${filePart}|${linePart}|${columnPart}`;
  const existing = store.get(key);
  if (existing == null) {
    store.set(key, {
      ...diagnostic,
      diagnosticId: buildDiagnosticId(diagnostic.runId, diagnostic.commandId, key),
      hitCount: 1,
    });
    return;
  }

  existing.hitCount += 1;
  existing.rawEndLine = Math.max(existing.rawEndLine ?? existing.rawStartLine ?? 0, diagnostic.rawEndLine ?? 0);
}

function parseTypescriptDiagnostics(store, context) {
  const regex = /^(.+?)\((\d+),(\d+)\):\s+(error|warning)\s+(TS\d+):\s+(.+)$/;
  for (const item of context.lines) {
    const match = item.text.match(regex);
    if (match == null) {
      continue;
    }
    const [, file, line, column, severity, code, message] = match;
    addDiagnostic(store, {
      schemaVersion: 1,
      eventType: 'diagnostic',
      runId: context.runId,
      commandId: context.commandId,
      kind: 'typescript',
      severity: severityFromToken(severity),
      signature: code,
      file: normalizeFile(file),
      line: Number(line),
      column: Number(column),
      message,
      rawArtifact: context.rawArtifact,
      rawStartLine: item.lineNumber,
      rawEndLine: item.lineNumber,
    });
  }
}

function parseRustDiagnostics(store, context) {
  const headerRegex = /^(error|warning)(?:\[(E\d+)\])?:\s+(.+)$/;
  const locationRegex = /^\s*-->\s+(.+?):(\d+):(\d+)$/;

  for (let index = 0; index < context.lines.length; index += 1) {
    const line = context.lines[index];
    const match = line.text.match(headerRegex);
    if (match == null) {
      continue;
    }

    const [, severity, code, message] = match;
    let file = null;
    let row = null;
    let column = null;
    let endLine = line.lineNumber;

    for (let lookahead = index + 1; lookahead < context.lines.length; lookahead += 1) {
      const candidate = context.lines[lookahead];
      const location = candidate.text.match(locationRegex);
      if (location != null) {
        file = normalizeFile(location[1]);
        row = Number(location[2]);
        column = Number(location[3]);
        endLine = candidate.lineNumber;
        break;
      }
      if (candidate.text.trim().length === 0) {
        break;
      }
    }

    addDiagnostic(store, {
      schemaVersion: 1,
      eventType: 'diagnostic',
      runId: context.runId,
      commandId: context.commandId,
      kind: inferRustKind(context.command, code),
      severity: severityFromToken(severity),
      signature: code ?? `rust-${message.slice(0, 80)}`,
      file,
      line: row,
      column,
      message,
      rawArtifact: context.rawArtifact,
      rawStartLine: line.lineNumber,
      rawEndLine: endLine,
    });
  }
}

function parseCargoTestFailures(store, context) {
  const testFailureRegex = /^test\s+(.+?)\s+\.\.\.\s+FAILED$/;
  for (const item of context.lines) {
    const match = item.text.match(testFailureRegex);
    if (match == null) {
      continue;
    }
    addDiagnostic(store, {
      schemaVersion: 1,
      eventType: 'diagnostic',
      runId: context.runId,
      commandId: context.commandId,
      kind: 'cargo-test',
      severity: 'error',
      signature: `cargo-test:${match[1]}`,
      file: null,
      line: null,
      column: null,
      message: `cargo test failure: ${match[1]}`,
      rawArtifact: context.rawArtifact,
      rawStartLine: item.lineNumber,
      rawEndLine: item.lineNumber,
    });
  }
}

function parseEslintDiagnostics(store, context) {
  let currentFile = null;
  const issueRegex = /^\s*(\d+):(\d+)\s+(error|warning)\s+(.+?)\s{2,}([@a-zA-Z0-9/_-]+)$/;

  for (const item of context.lines) {
    const trimmed = item.text.trim();
    if (trimmed.length === 0) {
      continue;
    }

    if (!item.text.startsWith(' ') && !item.text.startsWith('\t') && /\.(?:[cm]?[jt]sx?)$/i.test(trimmed)) {
      currentFile = normalizeFile(trimmed);
      continue;
    }

    const match = item.text.match(issueRegex);
    if (match == null || currentFile == null) {
      continue;
    }

    const [, line, column, severity, message, rule] = match;
    addDiagnostic(store, {
      schemaVersion: 1,
      eventType: 'diagnostic',
      runId: context.runId,
      commandId: context.commandId,
      kind: 'eslint',
      severity: severityFromToken(severity),
      signature: rule,
      file: currentFile,
      line: Number(line),
      column: Number(column),
      message,
      rawArtifact: context.rawArtifact,
      rawStartLine: item.lineNumber,
      rawEndLine: item.lineNumber,
    });
  }
}

function parseNpmFailures(store, context) {
  for (const item of context.lines) {
    if (!item.text.startsWith('npm error ')) {
      continue;
    }

    const message = item.text.slice('npm error '.length).trim();
    const signature = message.startsWith('code ') ? `npm:${message}` : `npm:${message.slice(0, 80)}`;
    addDiagnostic(store, {
      schemaVersion: 1,
      eventType: 'diagnostic',
      runId: context.runId,
      commandId: context.commandId,
      kind: 'npm-script',
      severity: 'error',
      signature,
      file: null,
      line: null,
      column: null,
      message,
      rawArtifact: context.rawArtifact,
      rawStartLine: item.lineNumber,
      rawEndLine: item.lineNumber,
    });
  }
}

function parsePolicyDiagnostics(store, context) {
  for (const item of context.lines) {
    const lower = item.text.toLowerCase();
    const isReexport = lower.includes('re-export') || lower.includes('no-reexport');
    const isPolicy =
      lower.includes('guard failed') || lower.includes('validation bypass') || lower.includes('import boundary');

    if (!isReexport && !isPolicy) {
      continue;
    }

    addDiagnostic(store, {
      schemaVersion: 1,
      eventType: 'diagnostic',
      runId: context.runId,
      commandId: context.commandId,
      kind: isReexport ? 'no-reexport-policy' : 'architecture-policy',
      severity: lower.includes('warning') ? 'warning' : 'error',
      signature: isReexport ? 'no-reexport-policy' : 'architecture-policy',
      file: normalizePolicyFile(item.text),
      line: null,
      column: null,
      message: item.text.trim(),
      rawArtifact: context.rawArtifact,
      rawStartLine: item.lineNumber,
      rawEndLine: item.lineNumber,
    });
  }
}

function normalizePolicyFile(text) {
  const match =
    text.match(/\bfor\s+([A-Za-z]:\\[^ ]+|\/[^ ]+|[\w./-]+\.[A-Za-z]+)\b/) ??
    text.match(/([A-Za-z]:\\[^ :]+|\/[^ :]+|[\w./-]+\.[A-Za-z]+)/);
  if (match == null) {
    return null;
  }
  return normalizeFile(match[1]);
}

function normalizeFile(filePath) {
  const trimmed = String(filePath ?? '').trim();
  if (trimmed.length === 0) {
    return null;
  }
  return path.resolve(trimmed).replace(/\\/g, '/');
}

function inferFallbackKind(command) {
  if (command[0] === 'npm') {
    return 'npm-script';
  }
  if (command[0] === 'cargo') {
    return 'cargo-test';
  }
  if (command[0] === 'eslint') {
    return 'eslint';
  }
  if (command[0] === 'tsc') {
    return 'typescript';
  }
  return 'unknown';
}

function inferRustKind(command, code) {
  if (command.includes('clippy')) {
    return 'clippy';
  }
  if (code != null) {
    return 'rustc';
  }
  return 'cargo-test';
}

function toLineObjects(text) {
  return text.split(/\r?\n/).map((line, index) => ({
    lineNumber: index + 1,
    text: line,
  }));
}

export function parseDiagnostics({
  runId,
  commandId,
  command,
  stdout,
  stderr,
  stdoutArtifactPath,
  stderrArtifactPath,
  exitCode,
}) {
  const store = new Map();
  const contexts = [
    { rawArtifact: stdoutArtifactPath, lines: toLineObjects(stdout) },
    { rawArtifact: stderrArtifactPath, lines: toLineObjects(stderr) },
  ];

  for (const raw of contexts) {
    const context = { ...raw, runId, commandId, command };
    parseTypescriptDiagnostics(store, context);
    parseRustDiagnostics(store, context);
    parseCargoTestFailures(store, context);
    parseEslintDiagnostics(store, context);
    parseNpmFailures(store, context);
    parsePolicyDiagnostics(store, context);
  }

  if (exitCode !== 0 && store.size === 0) {
    addDiagnostic(store, {
      schemaVersion: 1,
      eventType: 'diagnostic',
      runId,
      commandId,
      kind: inferFallbackKind(command),
      severity: 'error',
      signature: `exit-code:${exitCode}`,
      file: null,
      line: null,
      column: null,
      message: `Command exited with code ${exitCode}.`,
      rawArtifact: stderrArtifactPath,
      rawStartLine: null,
      rawEndLine: null,
    });
  }

  return [...store.values()].sort((left, right) => {
    if (left.kind !== right.kind) {
      return left.kind.localeCompare(right.kind);
    }
    return left.signature.localeCompare(right.signature);
  });
}
