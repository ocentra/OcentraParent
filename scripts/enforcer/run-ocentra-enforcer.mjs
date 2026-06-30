#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const defaultProfile = 'ocentra-parent';
const targetConfig = path.join(repoRoot, 'ocentra-enforcer.config.json');

function candidateRoots() {
  return [
    process.env.OCENTRA_ENFORCER_HOME,
    path.join(repoRoot, 'node_modules', 'ocentra-enforcer'),
    path.resolve(repoRoot, '..', 'ocentra-enforcer'),
    'E:\\ocentra-enforcer',
  ].filter(Boolean);
}

function resolveEnforcerFile(relativePath) {
  for (const candidate of candidateRoots()) {
    const fullPath = path.join(candidate, relativePath);
    if (fs.existsSync(fullPath)) return fullPath;
  }
  const searched = candidateRoots().map((entry) => path.join(entry, relativePath)).join('\n  ');
  throw new Error(`Could not find Ocentra Enforcer ${relativePath}. Searched:\n  ${searched}`);
}

function hasOption(args, names) {
  return args.some((arg) => names.includes(arg) || names.some((name) => arg.startsWith(`${name}=`)));
}

function main() {
  const cliPath = resolveEnforcerFile(path.join('scripts', 'rust-rules.mjs'));
  const args = process.argv.slice(2);
  if (args.length === 0) args.push('doctor');
  if (!hasOption(args, ['--root'])) args.push('--root', repoRoot);
  if (!hasOption(args, ['--profile'])) args.push('--profile', defaultProfile);
  if (!hasOption(args, ['--config']) && fs.existsSync(targetConfig)) args.push('--config', targetConfig);

  const result = spawnSync(process.execPath, [cliPath, ...args], {
    cwd: repoRoot,
    env: process.env,
    stdio: 'inherit',
  });
  if (result.error) throw result.error;
  process.exit(result.status ?? 1);
}

main();
