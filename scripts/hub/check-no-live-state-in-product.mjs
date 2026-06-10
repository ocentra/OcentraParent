import { spawnSync } from 'node:child_process';

const result = spawnSync('git', ['diff', '--cached', '--name-status', '--', '.hub/state'], {
  cwd: process.cwd(),
  encoding: 'utf8',
});

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

if ((result.status ?? 1) !== 0) {
  process.stderr.write(result.stderr ?? '');
  process.exit(result.status ?? 1);
}

const violations = result.stdout
  .split(/\r?\n/)
  .filter(Boolean)
  .filter((line) => !line.startsWith('D\t'));

if (violations.length > 0) {
  console.error('Live hub state must not be committed to OcentraParent.');
  console.error('Move live hub state to OcentraHub and keep .hub/state ignored.');
  for (const violation of violations) {
    console.error(`- ${violation}`);
  }
  process.exit(1);
}

console.log('live-hub-state-guard-ok');
