import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const resultPath = join(root, 'test-results', 'windows-browser-inventory-source-proof', 'proof.json');
const proofPackPath = join(
  root,
  'output',
  'browser-plan-proof',
  '04-windows-browser-inventory-adapter',
  '11-live-source-proof.json'
);

const commands = [
  {
    id: 'agent-core-browser-windows-inventory',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'browser_windows_inventory', '--quiet'],
  },
  {
    id: 'agent-service-browser-inventory-read-model',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'browser_inventory_read_model', '--quiet'],
  },
];

const sourceChecks = [
  {
    id: 'live-source-module',
    path: 'crates/agent-core/src/browser_windows_inventory_sources.rs',
    required: [
      'windows_browser_inventory_live_candidate_paths',
      'live_registry_install_sources',
      'live_start_menu_shortcut_targets',
    ],
  },
  {
    id: 'registry-source-module',
    path: 'crates/agent-core/src/browser_windows_inventory_registry_sources.rs',
    required: [
      'WINDOWS_REGISTRY_UNINSTALL_PATH',
      'WINDOWS_REGISTRY_WOW6432_UNINSTALL_PATH',
      'WINDOWS_REGISTRY_DISPLAY_ICON_VALUE',
      'WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE',
    ],
  },
  {
    id: 'start-menu-shortcut-source-module',
    path: 'crates/agent-core/src/browser_windows_inventory_shortcut_sources.rs',
    required: ['WINDOWS_PATH_START_MENU', 'WINDOWS_SHORTCUT_EXTENSION', 'windows_browser_shortcut_targets_from_roots'],
  },
  {
    id: 'service-default-source-wiring',
    path: 'crates/agent-service/src/browser_runtime_paths.rs',
    required: ['windows_browser_inventory_live_candidate_paths'],
  },
  {
    id: 'no-claim-service-test',
    path: 'crates/agent-service/src/browser_inventory_read_model_tests.rs',
    required: ['claim_boundary_is_honest', 'BrowserExactUrlCapability::Unavailable'],
  },
];

const results = commands.map(runCommand);
const checks = sourceChecks.map(checkSource);
const passed = results.every((result) => result.status === 0) && checks.every((check) => check.ok);

const proof = {
  proofId: 'windows-browser-inventory-source-proof',
  generatedAt: new Date().toISOString(),
  platform: process.platform,
  passed,
  commands: results,
  sourceChecks: checks,
  claims: {
    registrySource:
      'bounded Windows Uninstall registry source feeds known browser executable candidates when available',
    startMenuSource:
      'bounded Start Menu shortcut target extraction feeds known browser executable candidates when available',
    serviceReadModel: 'service default browser inventory includes live source candidates plus process observations',
  },
  noClaimBoundaries: [
    'no exact URL or active-tab support is inferred from registry entries',
    'no exact URL or active-tab support is inferred from Start Menu shortcuts',
    'no browser launch prevention, AppLocker policy write, WDAC policy write, or enforcement action is claimed',
    'non-Windows live registry and Start Menu sources remain empty unless a separate platform adapter proves them',
  ],
};

mkdirSync(dirname(resultPath), { recursive: true });
mkdirSync(dirname(proofPackPath), { recursive: true });
writeFileSync(resultPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(proofPackPath, `${JSON.stringify(proof, null, 2)}\n`);

if (!passed) {
  console.error(JSON.stringify(proof, null, 2));
  process.exit(1);
}

console.log(`windows-browser-inventory-source-proof=${resultPath}`);

function runCommand(entry) {
  const startedAt = new Date().toISOString();
  const result = spawnSync(entry.command, entry.args, {
    cwd: root,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  return {
    id: entry.id,
    command: [entry.command, ...entry.args].join(' '),
    status: result.status,
    signal: result.signal,
    startedAt,
    completedAt: new Date().toISOString(),
    stdoutTail: tail(result.stdout),
    stderrTail: tail(result.stderr),
  };
}

function checkSource(entry) {
  const absolutePath = join(root, entry.path);
  const content = readFileSync(absolutePath, 'utf8');
  const missing = entry.required.filter((needle) => !content.includes(needle));
  return {
    id: entry.id,
    path: entry.path,
    ok: missing.length === 0,
    missing,
  };
}

function tail(value) {
  return value.split(/\r?\n/u).filter(Boolean).slice(-12);
}
