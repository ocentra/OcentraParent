export function getNpmCommand(args) {
  return process.platform === 'win32' ? ['cmd', ['/c', 'npm', ...args]] : ['npm', args];
}

export function runNpmCommand(runner, args) {
  const [command, commandArgs] = getNpmCommand(args);
  return runner(command, commandArgs);
}
