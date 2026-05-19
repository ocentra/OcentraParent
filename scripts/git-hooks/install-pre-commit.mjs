import { chmodSync, existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const hookDir = join(process.cwd(), '.git', 'hooks');
const hookPath = join(hookDir, 'pre-commit');
const hook = `#!/bin/sh
node scripts/security/scan-staged-secrets.mjs
if [ $? -ne 0 ]; then
  echo "[security] Pre-commit hook rejected this commit due to secret detection."
  exit 1
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

if (!existsSync(hookDir)) {
  mkdirSync(hookDir, { recursive: true });
}

writeFileSync(hookPath, hook, 'utf8');
chmodSync(hookPath, 0o755);
console.log(`[validation] Installed ${hookPath}`);
