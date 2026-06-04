import { spawnSync } from 'node:child_process';
import { mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { basename, dirname, extname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const resultPath = join(root, 'test-results', 'windows-browser-inventory-source-proof', 'proof.json');
const browserNameHints = ['edge', 'chrome', 'brave', 'firefox', 'opera', 'vivaldi'];
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
const liveHostEvidence = captureLiveHostEvidence();

const proof = {
  proofId: 'windows-browser-inventory-source-proof',
  generatedAt: new Date().toISOString(),
  platform: process.platform,
  passed,
  commands: results,
  sourceChecks: checks,
  liveHostEvidence,
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

function captureLiveHostEvidence() {
  if (process.platform !== 'win32') {
    return {
      platform: process.platform,
      captured: false,
      reason: 'non-windows-host',
    };
  }
  return {
    platform: process.platform,
    captured: true,
    registry: captureRegistryEvidence(),
    startMenu: captureStartMenuEvidence(),
    privacyBoundary: 'counts and booleans only; executable and shortcut paths are not written',
  };
}

function captureRegistryEvidence() {
  const script = `
$roots = @(
  'HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*',
  'HKLM:\\SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*',
  'HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*'
)
$items = @()
$rootsReadable = 0
foreach ($root in $roots) {
  $batch = @(Get-ItemProperty -Path $root -ErrorAction SilentlyContinue)
  if ($batch.Count -gt 0) { $rootsReadable += 1 }
  $items += $batch
}
$browserPattern = '(?i)(msedge|chrome|brave|firefox|opera|vivaldi)\\.exe|microsoft edge|google chrome|brave|firefox|opera|vivaldi'
$browserItems = @($items | Where-Object {
  (@($_.DisplayName, $_.DisplayIcon, $_.InstallLocation) -join ' ') -match $browserPattern
})
$displayIconItems = @($browserItems | Where-Object { $_.DisplayIcon })
$installLocationItems = @($browserItems | Where-Object { $_.InstallLocation })
[pscustomobject]@{
  rootsChecked = $roots.Count
  rootsReadable = $rootsReadable
  entriesScanned = $items.Count
  browserLikeEntries = $browserItems.Count
  browserLikeDisplayIconEntries = $displayIconItems.Count
  browserLikeInstallLocationEntries = $installLocationItems.Count
  rawPathsRedacted = $true
} | ConvertTo-Json -Compress
`;
  return runPowerShellJson(script);
}

function captureStartMenuEvidence() {
  const roots = [process.env.PROGRAMDATA, process.env.APPDATA]
    .filter(Boolean)
    .map((envRoot) => join(envRoot, 'Microsoft', 'Windows', 'Start Menu', 'Programs'));
  const summary = {
    rootsChecked: roots.length,
    rootsReadable: 0,
    shortcutFiles: 0,
    browserNamedShortcutFiles: 0,
    rawPathsRedacted: true,
  };
  for (const rootPath of roots) {
    const rootCounts = countShortcutFiles(rootPath);
    if (rootCounts.readable) {
      summary.rootsReadable += 1;
    }
    summary.shortcutFiles += rootCounts.shortcutFiles;
    summary.browserNamedShortcutFiles += rootCounts.browserNamedShortcutFiles;
  }
  return summary;
}

function countShortcutFiles(rootPath) {
  const counts = {
    readable: false,
    shortcutFiles: 0,
    browserNamedShortcutFiles: 0,
  };
  try {
    for (const entry of readdirSync(rootPath)) {
      const path = join(rootPath, entry);
      const stats = statSync(path);
      if (stats.isDirectory()) {
        const nested = countShortcutFiles(path);
        counts.shortcutFiles += nested.shortcutFiles;
        counts.browserNamedShortcutFiles += nested.browserNamedShortcutFiles;
      } else if (extname(entry).toLowerCase() === '.lnk') {
        counts.shortcutFiles += 1;
        if (browserNameHints.some((hint) => basename(entry).toLowerCase().includes(hint))) {
          counts.browserNamedShortcutFiles += 1;
        }
      }
    }
    counts.readable = true;
  } catch {
    counts.readable = false;
  }
  return counts;
}

function runPowerShellJson(script) {
  const result = spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', script], {
    cwd: root,
    encoding: 'utf8',
  });
  if (result.status !== 0) {
    return {
      ok: false,
      status: result.status,
      stderrLineCount: tail(result.stderr).length,
      rawPathsRedacted: true,
    };
  }
  try {
    return {
      ok: true,
      ...JSON.parse(result.stdout),
    };
  } catch {
    return {
      ok: false,
      status: result.status,
      parseFailed: true,
      rawPathsRedacted: true,
    };
  }
}

function tail(value) {
  return value.split(/\r?\n/u).filter(Boolean).slice(-12);
}
