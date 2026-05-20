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

if [ "$OCENTRA_PARENT_SKIP_LANE_GUARD" != "1" ]; then
  echo "[lanes] Checking Ocentra Parent worktree lane ownership..."
  node scripts/dev/worktree-lanes.mjs guard
  if [ $? -ne 0 ]; then
    echo ""
    echo "[lanes] Pre-commit hook rejected this commit because the checkout is not claimed correctly."
    echo "[lanes] Run npm run lanes:status and npm run lanes:claim for this branch, or set OCENTRA_PARENT_SKIP_LANE_GUARD=1 only for deliberate emergency bypass."
    exit 1
  fi
fi

if [ "$OCENTRA_PARENT_SKIP_HUB_GUARD" != "1" ]; then
  echo "[hub] Checking Ocentra Parent hub mailbox and file locks..."
  node scripts/dev/hub-mailbox.mjs guard
  if [ $? -ne 0 ]; then
    echo ""
    echo "[hub] Pre-commit hook rejected this commit because the lane has unread hub messages or files outside its hub lock."
    echo "[hub] Run npm run hub:inbox, npm run hub:ack, and npm run hub:lock, or set OCENTRA_PARENT_SKIP_HUB_GUARD=1 only for deliberate emergency bypass."
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

if (!existsSync(hookDir)) {
  mkdirSync(hookDir, { recursive: true });
}

writeFileSync(hookPath, hook, 'utf8');
chmodSync(hookPath, 0o755);
console.log(`[validation] Installed ${hookPath}`);
