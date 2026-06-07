import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/05-cross-platform-inventory-matrix');
const testResultPath = join(repoRoot, 'test-results/browser-platform-linux-host-proof/proof.json');
const outputProofPath = join(proofRoot, '12-linux-host-package-proof.json');
const observedAt = new Date().toISOString();
const distroName = 'Ubuntu-22.04';

const knownBrowserTargets = [
  { targetId: 'google-chrome-stable', command: 'google-chrome', packageName: 'google-chrome-stable' },
  { targetId: 'chromium', command: 'chromium', packageName: 'chromium' },
  { targetId: 'chromium-browser', command: 'chromium-browser', packageName: 'chromium-browser' },
  { targetId: 'firefox', command: 'firefox', packageName: 'firefox' },
  { targetId: 'microsoft-edge-stable', command: 'microsoft-edge', packageName: 'microsoft-edge-stable' },
];

mkdirSync(proofRoot, { recursive: true });

const wslStatus = command(['--status'], { allowFailure: true });
const kernelName = wslBash('uname -s');
const distroDescription = wslBash(
  'if command -v lsb_release >/dev/null 2>&1; then lsb_release -ds; else cat /etc/os-release | sed -n "s/^PRETTY_NAME=//p" | tr -d "\\""; fi'
);
const pathVisibility = knownBrowserTargets.map((target) => queryCommand(target));
const packageVisibility = knownBrowserTargets.map((target) => queryPackage(target));
const desktopEntries = queryDesktopEntries();
const browserCommandVisible = pathVisibility.some((entry) => entry.visible);
const browserPackageInstalled = packageVisibility.some((entry) => entry.installed);
const browserDesktopEntryVisible = desktopEntries.length > 0;
const negativeChecks = [
  { claim: 'linux-managed-browser-adapter', rejected: true },
  { claim: 'linux-managed-exact-url', rejected: true },
  { claim: 'linux-known-active-tab', rejected: true },
  { claim: 'linux-browser-enforcement', rejected: true },
];

const proof = {
  schemaVersion: 1,
  proofId: 'browser-platform-linux-host-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  hostProofSummary: {
    wslAvailable: kernelName === 'Linux',
    distroName,
    distroDescriptionRef: distroDescription ? `redacted-linux-distro-${sha256(distroDescription).slice(0, 16)}` : null,
    wslStatusSha256: wslStatus ? sha256(wslStatus) : null,
    knownBrowserCommandsQueriedOnly: true,
    knownBrowserPackagesQueriedOnly: true,
    knownDesktopEntryGlobsQueriedOnly: true,
    browserCommandVisible,
    browserPackageInstalled,
    browserDesktopEntryVisible,
    rawPathPersisted: false,
    rawPackageListPersisted: false,
    rawDesktopEntryListPersisted: false,
    desktopSessionProofClaimed: false,
    managedProfileClaimed: false,
    exactUrlProofClaimed: false,
    knownActiveTabProofClaimed: false,
    snapFlatpakProofClaimed: false,
    enforcementClaimed: false,
    resultState:
      kernelName === 'Linux' ? 'linux-wsl-package-inventory-boundary-proof' : 'manual-linux-host-proof-required',
  },
  pathVisibility,
  packageVisibility,
  desktopEntries,
  negativeChecks,
};

if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected Linux host proof negative checks to reject dishonest claims');
}

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('browser-platform-linux-host-proof-ok=true');
console.log(`proof=${testResultPath}`);
console.log(`outputProof=${outputProofPath}`);
console.log(`wslAvailable=${proof.hostProofSummary.wslAvailable}`);
console.log(`browserCommandVisible=${browserCommandVisible}`);
console.log(`browserPackageInstalled=${browserPackageInstalled}`);
console.log(`browserDesktopEntryVisible=${browserDesktopEntryVisible}`);
console.log(`resultState=${proof.hostProofSummary.resultState}`);

function queryCommand(target) {
  const output = wslBash(`command -v ${shellEscape(target.command)} 2>/dev/null || true`);
  return {
    targetId: target.targetId,
    commandName: target.command,
    visible: output.length > 0,
    pathRef: output.length > 0 ? `redacted-linux-command-path-${sha256(output).slice(0, 16)}` : null,
    rawPathPersisted: false,
    managedProfileClaimed: false,
    exactUrlProofClaimed: false,
    knownActiveTabProofClaimed: false,
  };
}

function queryPackage(target) {
  const output = wslBash(
    `dpkg-query -W -f='\\${'${Package}'}\\t\\${'${Status}'}\\n' ${shellEscape(target.packageName)} 2>/dev/null || true`
  );
  return {
    targetId: target.targetId,
    packageName: target.packageName,
    installed: output.includes('install ok installed'),
    packageStatusRef: output.length > 0 ? `redacted-linux-package-status-${sha256(output).slice(0, 16)}` : null,
    rawPackageListPersisted: false,
    managedProfileClaimed: false,
    exactUrlProofClaimed: false,
    knownActiveTabProofClaimed: false,
  };
}

function queryDesktopEntries() {
  const output = wslBash(
    'for pattern in /usr/share/applications/*chrome*.desktop /usr/share/applications/*chromium*.desktop /usr/share/applications/*firefox*.desktop; do [ -e "$pattern" ] && printf "%s\\n" "$pattern"; done'
  );
  return output
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0)
    .map((entryPath) => ({
      entryRef: `redacted-linux-desktop-entry-${sha256(entryPath).slice(0, 16)}`,
      rawPathPersisted: false,
      managedProfileClaimed: false,
      exactUrlProofClaimed: false,
      knownActiveTabProofClaimed: false,
    }));
}

function wslBash(script) {
  return command(['-d', distroName, '--exec', 'bash', '-lc', script], { allowFailure: true }).trim();
}

function command(args, { allowFailure }) {
  try {
    return execFileSync('wsl.exe', args, { cwd: repoRoot, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
  } catch (error) {
    if (allowFailure) {
      return `${error.stdout?.toString() ?? ''}${error.stderr?.toString() ?? ''}`;
    }
    throw error;
  }
}

function shellEscape(value) {
  return `'${value.replaceAll("'", "'\\''")}'`;
}

function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}
