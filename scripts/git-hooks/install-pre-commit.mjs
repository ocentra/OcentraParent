import { execFileSync } from 'node:child_process';
import { chmodSync, existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const hook = `#!/bin/sh
node scripts/enforcer/run-ocentra-enforcer.mjs check secrets --staged
if [ $? -ne 0 ]; then
  echo "[security] Pre-commit hook rejected this commit due to secret detection."
  exit 1
fi

if [ "$OCENTRA_PARENT_SKIP_LANE_GUARD" != "1" ]; then
  echo "[enforcer] Checking coordination lane ownership..."
  node scripts/enforcer/run-ocentra-enforcer.mjs coordination hub:guard
  if [ $? -ne 0 ]; then
    echo ""
    echo "[enforcer] Pre-commit hook rejected this commit because the checkout is not claimed correctly."
    echo "[ledger] Run npm run ledger:doctor, npm run hub:inbox, and npm run hub:lock for this branch, or set OCENTRA_PARENT_SKIP_LANE_GUARD=1 only for deliberate emergency bypass."
    exit 1
  fi
fi

if [ "$OCENTRA_PARENT_SKIP_HUB_GUARD" != "1" ]; then
  echo "[enforcer] Checking coordination inbox and file claims..."
  node scripts/enforcer/run-ocentra-enforcer.mjs coordination hub:guard
  if [ $? -ne 0 ]; then
    echo ""
    echo "[enforcer] Pre-commit hook rejected this commit because the lane has unread coordination messages or files outside its Enforcer claim."
    echo "[ledger] Run npm run hub:inbox, npm run hub:ack, and npm run hub:lock, or set OCENTRA_PARENT_SKIP_HUB_GUARD=1 only for deliberate emergency bypass."
    exit 1
  fi
fi

echo "[validation] Running Ocentra Parent pre-commit gate..."
node scripts/git-hooks/run-precommit-validation.mjs
if [ $? -ne 0 ]; then
  echo ""
  echo "[validation] Pre-commit hook rejected this commit due to errors."
  exit 1
fi

exit 0
`;

export function resolvePreCommitHookPath(repoRoot = process.cwd()) {
  const topLevel = git(repoRoot, ['rev-parse', '--show-toplevel']);
  const hookPath = git(topLevel, ['rev-parse', '--git-path', 'hooks/pre-commit']);
  return resolve(topLevel, hookPath);
}

export function installPreCommitHook(repoRoot = process.cwd()) {
  const hookPath = resolvePreCommitHookPath(repoRoot);
  const hookDir = dirname(hookPath);

  if (!existsSync(hookDir)) {
    mkdirSync(hookDir, { recursive: true });
  }

  writeFileSync(hookPath, hook, 'utf8');
  chmodSync(hookPath, 0o755);
  return hookPath;
}

function git(cwd, args) {
  return execFileSync('git', args, {
    cwd,
    encoding: 'utf8',
    env: cleanGitEnv(),
    stdio: ['ignore', 'pipe', 'pipe'],
  }).trim();
}

function cleanGitEnv() {
  return Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith('GIT_')));
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const hookPath = installPreCommitHook();
  console.log(`[validation] Installed ${hookPath}`);
}
