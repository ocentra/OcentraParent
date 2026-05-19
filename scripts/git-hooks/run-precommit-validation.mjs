import { spawnSync } from 'node:child_process';

const validations = [
  ['npm', ['run', 'format:check']],
  ['npm', ['run', 'validate']],
  ['npm', ['run', 'build']],
];

function runCommand(command, args) {
  if (process.platform === 'win32') {
    return spawnSync('cmd.exe', ['/d', '/s', '/c', `${command} ${args.join(' ')}`], {
      cwd: process.cwd(),
      stdio: 'inherit',
    });
  }

  return spawnSync(command, args, {
    cwd: process.cwd(),
    stdio: 'inherit',
  });
}

for (const [command, args] of validations) {
  const result = runCommand(command, args);
  if (result.error) {
    console.error(`[validation] ${result.error.message}`);
    process.exit(1);
  }
  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}
