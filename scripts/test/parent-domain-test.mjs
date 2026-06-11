import { spawnSync } from 'node:child_process';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const parentDomainRoot = join(repoRoot, 'packages', 'parent-domain');
const forwardedArgs = sanitizeForwardedVitestArgs(process.argv.slice(2));

if (process.env.OCENTRA_PARENT_DOMAIN_TEST_SKIP_PROOF_CHAIN === '1') {
  runNpmBin('vitest', ['run', ...forwardedArgs]);
  process.exit(0);
}

runNpm(['run', 'build']);
runNodeScript(join(repoRoot, 'scripts', 'test', 'app-game-timer-proof-chain.mjs'), {
  OCENTRA_PARENT_DOMAIN_TEST_SKIP_PROOF_CHAIN: '1',
});
runNpmBin('vitest', ['run', ...forwardedArgs]);

function runNpm(args) {
  const npmCliPath = process.env.npm_execpath;
  if (npmCliPath) {
    run(process.execPath, [npmCliPath, ...args], parentDomainRoot);
    return;
  }

  if (process.platform === 'win32') {
    throw new Error('npm_execpath is required to run parent-domain tests on Windows without shell dispatch.');
  }

  runCommand(['npm', ...args], parentDomainRoot);
}

function runNpmBin(binary, args) {
  runNpm(['exec', '--', binary, ...args]);
}

function runNodeScript(scriptPath, env = {}) {
  run(process.execPath, [scriptPath], repoRoot, env);
}

function run(command, args, cwd, env = {}) {
  const result = spawnSync(command, args, {
    cwd,
    env: { ...process.env, ...env },
    stdio: 'inherit',
    windowsHide: true,
  });

  if (result.error) {
    console.error(`failed to run ${command}: ${result.error.message}`);
    process.exit(1);
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function runCommand(commandParts, cwd) {
  const [command, ...args] = commandParts;
  run(command, args, cwd);
}

function sanitizeForwardedVitestArgs(args) {
  return args.map((arg) => {
    if (!/^[A-Za-z0-9._/@:=,+-]+$/.test(arg)) {
      throw new Error(`Unsupported parent-domain test argument: ${arg}`);
    }
    return arg;
  });
}
