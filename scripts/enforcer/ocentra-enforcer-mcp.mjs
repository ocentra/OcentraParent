#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function candidateRoots() {
  return [
    process.env.OCENTRA_ENFORCER_HOME,
    path.join(repoRoot, 'node_modules', 'ocentra-enforcer'),
    path.resolve(repoRoot, '..', 'ocentra-enforcer'),
    'E:\\ocentra-enforcer',
  ].filter(Boolean);
}

function resolveMcpServer() {
  for (const candidate of candidateRoots()) {
    const fullPath = path.join(candidate, 'mcp', 'rust-rules-mcp.mjs');
    if (fs.existsSync(fullPath)) return fullPath;
  }
  const searched = candidateRoots()
    .map((entry) => path.join(entry, 'mcp', 'rust-rules-mcp.mjs'))
    .join('\n  ');
  throw new Error(`Could not find Ocentra Enforcer MCP server. Searched:\n  ${searched}`);
}

const result = spawnSync(process.execPath, [resolveMcpServer()], {
  cwd: repoRoot,
  env: {
    ...process.env,
    OCENTRA_ENFORCER_TARGET_ROOT: repoRoot,
  },
  stdio: 'inherit',
});

if (result.error) throw result.error;
process.exit(result.status ?? 1);
