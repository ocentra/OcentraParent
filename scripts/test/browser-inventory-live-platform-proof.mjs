import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync } from 'node:fs';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = process.cwd();
const resultDirectory = join(root, 'test-results', 'browser-inventory-live-platform-proof');
const proofPath = join(resultDirectory, 'proof.json');
const workpackRoots = [
  join(root, 'output', 'browser-plan-proof', '03-browser-inventory-model'),
  join(root, 'output', 'browser-plan-proof', '04-windows-browser-inventory-adapter'),
  join(root, 'output', 'browser-plan-proof', '05-cross-platform-inventory-matrix'),
];
const executableNames = [
  'msedge.exe',
  'chrome.exe',
  'brave.exe',
  'vivaldi.exe',
  'opera.exe',
  'opera_gx.exe',
  'chromium.exe',
  'firefox.exe',
  'tor.exe',
  'duckduckgo.exe',
  'arc.exe',
];

await main();

async function main() {
  buildWorkspace('@ocentra-parent/schema-domain');
  buildWorkspace('@ocentra-parent/activity-domain');

  const generatedAt = new Date().toISOString();
  const sourceEvidence = collectLiveEvidence();
  const rows = buildInventoryRows(generatedAt, sourceEvidence.candidates);
  const readModel = {
    schemaVersion: 1,
    generatedAt,
    limit: rows.length,
    returned: rows.length,
    latestObservedAt: rows.length > 0 ? generatedAt : null,
    capabilityStatus: rows.length > 0 ? 'managed-profile-missing' : null,
    custodyLabel: 'child-device-local',
    queryVisibility: 'live-local',
    rows,
  };

  const browserModule = await import(
    pathToFileURL(join(root, 'packages', 'activity-domain', 'dist', 'browser.js')).href
  );
  const parsedReadModel = browserModule.BrowserInventoryReadModelSchema.parse(readModel);
  const proof = {
    schemaVersion: 1,
    proofMode: 'browser-inventory-live-platform-proof',
    generatedAt,
    sourceWorkpacks: [
      'docs/plans/browser-plan/workpacks/03-browser-inventory-model.md',
      'docs/plans/browser-plan/workpacks/04-windows-browser-inventory-adapter.md',
      'docs/plans/browser-plan/workpacks/05-cross-platform-inventory-matrix.md',
    ],
    host: {
      platform: process.platform,
      arch: process.arch,
      windowsRequiredForLiveClaim: true,
    },
    liveSources: sourceEvidence.summary,
    readModel: parsedReadModel,
    privacy: {
      rawPathsPersisted: false,
      rawUrlsPersisted: false,
      rawRegistryKeysPersisted: false,
      executableRefsAreSha256: true,
      fileHashRefsAreSha256: true,
    },
    noClaimLabels: [
      'exact-url-not-claimed',
      'active-tab-not-claimed',
      'enforcement-not-claimed',
      'browser-control-policy-not-changed',
      'portal-ui-not-changed',
      'non-windows-platforms-remain-manual-required',
      'product-checklist-upgrade-not-claimed',
    ],
    summary: {
      candidates: sourceEvidence.candidates.length,
      schemaRows: parsedReadModel.rows.length,
      managedProfileMissingRows: parsedReadModel.rows.filter(
        (row) => row.capabilityStatus === 'managed-profile-missing'
      ).length,
      unmanagedProcessRows: parsedReadModel.rows.filter((row) => row.runningState === 'running-unmanaged').length,
      unsupportedRows: parsedReadModel.rows.filter((row) => row.supportTier === 'unsupported').length,
      manualRequiredRows: parsedReadModel.rows.filter((row) => row.managementTier === 'manual-required').length,
      productClaimed: false,
      checklistStatusChanged: false,
    },
  };
  const failures = validateProof(proof);
  if (failures.length > 0) {
    throw new Error(`Browser inventory live platform proof failed:\n${failures.join('\n')}`);
  }

  await mkdir(resultDirectory, { recursive: true });
  for (const workpackRoot of workpackRoots) {
    await mkdir(workpackRoot, { recursive: true });
    await writeFile(join(workpackRoot, '09-manual-platform-proof.md'), `${markdownFor(proof, workpackRoot)}\n`);
  }
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);

  console.log('browser-inventory-live-platform-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`rows=${proof.summary.schemaRows}`);
  console.log(`registryCandidates=${proof.liveSources.registry.browserCandidates}`);
  console.log(`shortcutCandidates=${proof.liveSources.shortcuts.browserCandidates}`);
  console.log(`appxCandidates=${proof.liveSources.appx.browserCandidates}`);
  console.log(`processCandidates=${proof.liveSources.processes.browserCandidates}`);
}

function collectLiveEvidence() {
  const knownPathCandidates = knownPathEvidence();
  const registry = powershellJson(registryScript());
  const shortcuts = powershellJson(shortcutScript());
  const appx = powershellJson(appxScript());
  const processes = powershellJson(processScript());
  const sourceRows = [
    ...knownPathCandidates,
    ...registryCandidates(registry.rows),
    ...shortcutCandidates(shortcuts.rows),
    ...appxCandidates(appx.rows),
    ...processCandidates(processes.rows),
  ];
  const candidates = dedupeCandidates(sourceRows);
  return {
    candidates,
    summary: {
      knownPaths: sourceSummary('completed', knownPathCandidates.length, knownPathCandidates.length),
      registry: sourceSummary(
        registry.status,
        registry.rows.length,
        registryCandidates(registry.rows).length,
        registry.error
      ),
      shortcuts: sourceSummary(
        shortcuts.status,
        shortcuts.rows.length,
        shortcutCandidates(shortcuts.rows).length,
        shortcuts.error
      ),
      appx: sourceSummary(appx.status, appx.rows.length, appxCandidates(appx.rows).length, appx.error),
      processes: sourceSummary(
        processes.status,
        processes.rows.length,
        processCandidates(processes.rows).length,
        processes.error
      ),
    },
  };
}

function knownPathEvidence() {
  const roots = [process.env.PROGRAMFILES, process.env['PROGRAMFILES(X86)'], process.env.LOCALAPPDATA].filter(Boolean);
  const candidates = [];
  for (const rootPath of roots) {
    for (const target of knownRelativeTargets()) {
      const candidatePath = join(rootPath, ...target.parts);
      if (existsSync(candidatePath)) {
        candidates.push(candidateFromPath('known-path', candidatePath));
      }
    }
  }
  return candidates;
}

function registryCandidates(rows) {
  return rows.flatMap((row) => {
    const targets = [row.DisplayIcon, row.InstallLocation, row.DisplayName].filter(Boolean);
    return targets.flatMap((target) => {
      const executablePath = executableTargetPath(String(target));
      const candidate =
        executablePath === null
          ? candidateFromText('registry', String(target), row.DisplayVersion)
          : candidateFromPath('registry', executablePath, row.DisplayVersion);
      if (candidate !== null && row.Publisher) {
        candidate.publisherSignatureRef = opaqueRef('browser-live-publisher-ref', row.Publisher);
      }
      return candidate === null ? [] : [candidate];
    });
  });
}

function shortcutCandidates(rows) {
  return rows
    .map((row) => executableTargetPath(String(row.TargetPath ?? '')))
    .filter((target) => target !== null)
    .map((target) => candidateFromPath('shortcut', target));
}

function appxCandidates(rows) {
  return rows.flatMap((row) => {
    const text = [row.Name, row.PackageFullName].filter(Boolean).join(' ');
    const candidate = candidateFromText('appx', text, null);
    return candidate === null
      ? []
      : [
          {
            ...candidate,
            installState: 'packaged',
            executablePathRef: row.InstallLocation
              ? opaqueRef('browser-live-appx-ref', row.InstallLocation)
              : candidate.executablePathRef,
          },
        ];
  });
}

function processCandidates(rows) {
  return rows.flatMap((row) => {
    const executablePath = row.ExecutablePath ? String(row.ExecutablePath) : String(row.Name ?? '');
    const candidate = candidateFromPath('process', executablePath);
    return candidate === null
      ? []
      : [
          {
            ...candidate,
            processId: Number(row.ProcessId),
            installState: 'candidate-running',
            runningState: candidate.supportKind === 'unsupported' ? 'running-unknown' : 'running-unmanaged',
          },
        ];
  });
}

function candidateFromPath(sourceKind, targetPath, browserVersion = null) {
  const identity = browserIdentity(targetPath);
  if (identity === null) {
    return null;
  }
  return {
    ...identity,
    sourceKinds: [sourceKind],
    browserVersion,
    installState: installStateFromPath(targetPath),
    runningState: 'not-running',
    executablePathRef: opaqueRef('browser-live-path-ref', targetPath),
    publisherSignatureRef: null,
    fileHashRef: fileHashRef(targetPath),
    processId: null,
  };
}

function candidateFromText(sourceKind, text, browserVersion) {
  const identity = browserIdentity(text);
  if (identity === null) {
    return null;
  }
  return {
    ...identity,
    sourceKinds: [sourceKind],
    browserVersion,
    installState: sourceKind === 'appx' ? 'packaged' : 'unknown',
    runningState: 'unknown',
    executablePathRef: opaqueRef('browser-live-source-ref', text),
    publisherSignatureRef: null,
    fileHashRef: null,
    processId: null,
  };
}

function browserIdentity(value) {
  const text = value.toLowerCase();
  const channel = text.includes('beta')
    ? 'beta'
    : text.includes('dev')
      ? 'dev'
      : text.includes('sxs') || text.includes('canary')
        ? 'canary'
        : 'stable';
  if (text.includes('msedge.exe') || (text.includes('microsoft edge') && !text.includes('webview'))) {
    return { browserFamily: 'edge', browserChannel: channel, productName: 'Microsoft Edge', supportKind: 'managed' };
  }
  if (text.includes('chrome.exe') || text.includes('google chrome') || text.includes('chrome for testing')) {
    return { browserFamily: 'chrome', browserChannel: channel, productName: 'Google Chrome', supportKind: 'managed' };
  }
  if (text.includes('brave.exe') || text.includes('brave browser')) {
    return { browserFamily: 'brave', browserChannel: 'stable', productName: 'Brave Browser', supportKind: 'manual' };
  }
  if (text.includes('vivaldi.exe') || text.includes('vivaldi')) {
    return {
      browserFamily: 'unknown-chromium',
      browserChannel: 'stable',
      productName: 'Vivaldi Browser',
      supportKind: 'manual',
    };
  }
  if (text.includes('opera')) {
    return { browserFamily: 'opera', browserChannel: 'stable', productName: 'Opera Browser', supportKind: 'manual' };
  }
  if (text.includes('chromium.exe') || text.includes('chromium')) {
    return {
      browserFamily: 'unknown-chromium',
      browserChannel: 'unknown',
      productName: 'Chromium',
      supportKind: 'manual',
    };
  }
  if (text.includes('firefox.exe') || text.includes('mozilla firefox')) {
    return {
      browserFamily: 'firefox',
      browserChannel: channel,
      productName: 'Mozilla Firefox',
      supportKind: 'unsupported',
    };
  }
  if (text.includes('tor browser') || text.includes('tor.exe')) {
    return {
      browserFamily: 'unknown',
      browserChannel: 'unknown',
      productName: 'Tor Browser',
      supportKind: 'unsupported',
    };
  }
  if (text.includes('duckduckgo')) {
    return {
      browserFamily: 'unknown',
      browserChannel: 'unknown',
      productName: 'DuckDuckGo Browser',
      supportKind: 'unsupported',
    };
  }
  if (text.includes('arc.exe') || text.includes('arc browser')) {
    return {
      browserFamily: 'unknown-chromium',
      browserChannel: 'unknown',
      productName: 'Arc Browser',
      supportKind: 'unsupported',
    };
  }
  return null;
}

function buildInventoryRows(scannedAt, candidates) {
  return candidates.map((candidate, index) => {
    const capabilities = capabilitiesFor(candidate);
    return {
      schemaVersion: 1,
      inventoryRowId: `browser-live-inventory-${String(index + 1).padStart(3, '0')}-${shortHash(candidate.executablePathRef)}`,
      scannedAt,
      deviceId: 'local-windows-host-redacted',
      browserFamily: candidate.browserFamily,
      browserChannel: candidate.browserChannel,
      productName: candidate.productName,
      browserVersion: candidate.browserVersion,
      installState: candidate.installState,
      runningState: candidate.runningState,
      ...capabilities,
      executablePathRef: candidate.executablePathRef,
      publisherSignatureRef: candidate.publisherSignatureRef,
      fileHashRef: candidate.fileHashRef,
      profileId: null,
      processId: candidate.processId,
      custodyLabel: 'child-device-local',
      queryVisibility: capabilities.managementTier === 'unmanaged' ? 'unavailable' : 'live-local',
    };
  });
}

function capabilitiesFor(candidate) {
  if (candidate.runningState === 'running-unmanaged' && candidate.supportKind !== 'unsupported') {
    return {
      managementTier: 'unmanaged',
      supportTier: 'unmanaged-process-only',
      exactUrlCapability: 'not-claimed',
      activeTabCapability: 'not-claimed',
      managedProfileState: 'not-applicable',
      unmanagedFallbackCapability: 'report-only',
      capabilityStatus: 'unmanaged-browser',
      reasonCode: 'unmanaged-browser-process-only',
    };
  }
  if (candidate.supportKind === 'managed') {
    return {
      managementTier: 'managed',
      supportTier: 'candidate',
      exactUrlCapability: 'unavailable',
      activeTabCapability: 'unavailable',
      managedProfileState: 'missing',
      unmanagedFallbackCapability: 'os-block-manual-required',
      capabilityStatus: 'managed-profile-missing',
      reasonCode: 'windows-managed-profile-required',
    };
  }
  if (candidate.supportKind === 'manual') {
    return {
      managementTier: 'manual-required',
      supportTier: 'candidate',
      exactUrlCapability: 'manual-required',
      activeTabCapability: 'manual-required',
      managedProfileState: 'manual-required',
      unmanagedFallbackCapability: 'report-only',
      capabilityStatus: 'permission-limited',
      reasonCode: 'windows-chromium-fork-manual-required',
    };
  }
  return {
    managementTier: 'unsupported',
    supportTier: 'unsupported',
    exactUrlCapability: 'unsupported',
    activeTabCapability: 'unsupported',
    managedProfileState: 'not-applicable',
    unmanagedFallbackCapability: 'unsupported',
    capabilityStatus: 'unsupported-browser',
    reasonCode:
      candidate.runningState === 'running-unknown'
        ? 'windows-browser-process-unsupported'
        : 'windows-unsupported-later-adapter',
  };
}

function knownRelativeTargets() {
  return [
    ['Microsoft', 'Edge', 'Application', 'msedge.exe'],
    ['Microsoft', 'Edge Beta', 'Application', 'msedge.exe'],
    ['Microsoft', 'Edge Dev', 'Application', 'msedge.exe'],
    ['Microsoft', 'Edge SxS', 'Application', 'msedge.exe'],
    ['Google', 'Chrome', 'Application', 'chrome.exe'],
    ['Google', 'Chrome for Testing', 'Application', 'chrome.exe'],
    ['BraveSoftware', 'Brave-Browser', 'Application', 'brave.exe'],
    ['Vivaldi', 'Application', 'vivaldi.exe'],
    ['Opera Software', 'Opera Stable', 'Application', 'opera.exe'],
    ['Opera Software', 'Opera GX Stable', 'Application', 'opera.exe'],
    ['Chromium', 'Application', 'chrome.exe'],
    ['Mozilla Firefox', 'Application', 'firefox.exe'],
    ['Firefox Developer Edition', 'Application', 'firefox.exe'],
    ['Firefox Nightly', 'Application', 'firefox.exe'],
    ['Tor Browser', 'Browser', 'firefox.exe'],
    ['DuckDuckGo', 'duckduckgo.exe'],
    ['Arc', 'arc.exe'],
  ].map((parts) => ({ parts }));
}

function executableTargetPath(target) {
  const trimmed = target
    .trim()
    .replace(/^"([^"]+)".*$/u, '$1')
    .split(',')[0]
    .trim();
  if (trimmed.length === 0) {
    return null;
  }
  const lower = trimmed.toLowerCase();
  for (const executable of executableNames) {
    const index = lower.indexOf(executable);
    if (index >= 0) {
      return trimmed.slice(0, index + executable.length);
    }
  }
  return null;
}

function installStateFromPath(targetPath) {
  const lower = targetPath.toLowerCase();
  if (lower.includes('\\windowsapps\\')) {
    return 'packaged';
  }
  if (lower.includes('portable')) {
    return 'portable';
  }
  return 'installed';
}

function dedupeCandidates(candidates) {
  const byKey = new Map();
  for (const candidate of candidates.filter(Boolean)) {
    const key = `${candidate.executablePathRef}|${candidate.productName}`;
    const existing = byKey.get(key);
    if (existing === undefined) {
      byKey.set(key, candidate);
      continue;
    }
    existing.sourceKinds = [...new Set([...existing.sourceKinds, ...candidate.sourceKinds])].sort();
    existing.fileHashRef ??= candidate.fileHashRef;
    existing.publisherSignatureRef ??= candidate.publisherSignatureRef;
    existing.browserVersion ??= candidate.browserVersion;
    existing.processId ??= candidate.processId;
    if (existing.installState === 'unknown' || existing.installState === 'candidate-running') {
      existing.installState =
        candidate.installState === 'candidate-running' ? existing.installState : candidate.installState;
    }
    if (candidate.runningState !== 'not-running' && candidate.runningState !== 'unknown') {
      existing.runningState = candidate.runningState;
    }
  }
  return [...byKey.values()].sort((left, right) => left.productName.localeCompare(right.productName));
}

function validateProof(proof) {
  const failures = [];
  if (process.platform !== 'win32') {
    failures.push(`live Windows proof must run on win32; actual=${process.platform}`);
  }
  if (proof.summary.schemaRows === 0) {
    failures.push('no live browser inventory rows were validated');
  }
  for (const [name, source] of Object.entries(proof.liveSources)) {
    if (source.status !== 'completed') {
      failures.push(`${name} live source did not complete: ${source.error ?? 'unknown error'}`);
    }
  }
  const proofText = JSON.stringify(proof);
  if (/[A-Za-z]:\\\\/.test(proofText) || /[A-Za-z]:\\/.test(proofText)) {
    failures.push('proof persisted a raw Windows path');
  }
  if (proof.readModel.rows.some((row) => row.exactUrlCapability === 'managed-exact-url-available')) {
    failures.push('live inventory proof claimed managed exact URL support');
  }
  return failures;
}

function powershellJson(script) {
  try {
    const output = execFileSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', script], {
      cwd: root,
      encoding: 'utf8',
      maxBuffer: 1024 * 1024 * 8,
      windowsHide: true,
    }).trim();
    return { status: 'completed', rows: output.length === 0 ? [] : normalizeJsonArray(JSON.parse(output)) };
  } catch (error) {
    return { status: 'failed', rows: [], error: redactText(error.message) };
  }
}

function registryScript() {
  return `
$paths = @(
  'HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*',
  'HKLM:\\Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*',
  'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*'
)
$rows = foreach ($path in $paths) {
  Get-ItemProperty -Path $path -ErrorAction SilentlyContinue | ForEach-Object {
    [pscustomobject]@{
      DisplayName = $_.DisplayName
      DisplayVersion = $_.DisplayVersion
      Publisher = $_.Publisher
      DisplayIcon = $_.DisplayIcon
      InstallLocation = $_.InstallLocation
    }
  }
}
@($rows) | ConvertTo-Json -Compress -Depth 4
`;
}

function shortcutScript() {
  return `
$roots = @([Environment]::GetFolderPath('CommonPrograms'), [Environment]::GetFolderPath('Programs')) | Where-Object { $_ -and (Test-Path $_) }
$shell = New-Object -ComObject WScript.Shell
$rows = foreach ($root in $roots) {
  Get-ChildItem -Path $root -Filter '*.lnk' -Recurse -ErrorAction SilentlyContinue | Select-Object -First 250 | ForEach-Object {
    try {
      $shortcut = $shell.CreateShortcut($_.FullName)
      [pscustomobject]@{ Name = $_.BaseName; TargetPath = $shortcut.TargetPath; Arguments = $shortcut.Arguments }
    } catch {}
  }
}
@($rows) | ConvertTo-Json -Compress -Depth 4
`;
}

function appxScript() {
  return `
$pattern = 'edge|chrome|firefox|opera|brave|duckduckgo|arc|browser'
$rows = Get-AppxPackage -ErrorAction SilentlyContinue | Where-Object { $_.Name -match $pattern -or $_.PackageFullName -match $pattern } | ForEach-Object {
  [pscustomobject]@{ Name = $_.Name; PackageFullName = $_.PackageFullName; Publisher = $_.Publisher; InstallLocation = $_.InstallLocation }
}
@($rows) | ConvertTo-Json -Compress -Depth 4
`;
}

function processScript() {
  return `
$names = @(${executableNames.map((name) => `'${name}'`).join(', ')})
$rows = Get-CimInstance Win32_Process -ErrorAction SilentlyContinue | Where-Object { $names -contains $_.Name.ToLowerInvariant() } | ForEach-Object {
  [pscustomobject]@{ Name = $_.Name; ProcessId = $_.ProcessId; ExecutablePath = $_.ExecutablePath }
}
@($rows) | ConvertTo-Json -Compress -Depth 4
`;
}

function fileHashRef(targetPath) {
  try {
    if (!existsSync(targetPath)) {
      return null;
    }
    const digest = createHash('sha256').update(readFileSync(targetPath)).digest('hex');
    return `browser-live-file-sha256-${digest.slice(0, 32)}`;
  } catch {
    return null;
  }
}

function opaqueRef(prefix, value) {
  return `${prefix}-${shortHash(value)}`;
}

function shortHash(value) {
  return createHash('sha256').update(String(value)).digest('hex').slice(0, 24);
}

function sourceSummary(status, scanned, browserCandidates, error) {
  return { status, scanned, browserCandidates, error: error ?? null };
}

function normalizeJsonArray(value) {
  if (Array.isArray(value)) {
    return value;
  }
  return value === null || value === undefined ? [] : [value];
}

function redactText(value) {
  return String(value).replace(/[A-Za-z]:\\[^\r\n"]+/gu, '<redacted-path>');
}

function markdownFor(proof, workpackRoot) {
  const rows = proof.readModel.rows
    .map(
      (row) =>
        `| ${row.productName} | ${row.browserFamily} | ${row.installState} | ${row.runningState} | ${row.exactUrlCapability} | ${row.activeTabCapability} | ${row.reasonCode} |`
    )
    .join('\n');
  return [
    '# Browser Inventory Live Platform Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    `Proof JSON: ${relativePath(proofPath)}`,
    `Workpack proof root: ${relativePath(workpackRoot)}`,
    '',
    '## Live Sources',
    '',
    '| Source | Status | Scanned | Browser Candidates |',
    '| --- | --- | ---: | ---: |',
    ...Object.entries(proof.liveSources).map(
      ([name, source]) => `| ${name} | ${source.status} | ${source.scanned} | ${source.browserCandidates} |`
    ),
    '',
    '## Schema-Validated Rows',
    '',
    '| Product | Family | Install | Running | Exact URL | Active Tab | Reason |',
    '| --- | --- | --- | --- | --- | --- | --- |',
    rows,
    '',
    'Raw executable paths, registry keys, shortcut paths, URLs, page titles, cookies, tokens, and browser payloads are not persisted. Executable and package evidence uses hashed refs only.',
    '',
    'No product checklist upgrade is claimed. This proof captures live Windows/manual platform evidence for browser inventory rows while exact URL, active tab proof, enforcement, portal UI, and non-Windows/mobile adapters remain unclaimed.',
  ].join('\n');
}

function buildWorkspace(workspace) {
  const command =
    process.platform === 'win32'
      ? { executable: 'cmd', args: ['/c', 'npm', 'run', 'build', '--workspace', workspace] }
      : { executable: 'npm', args: ['run', 'build', '--workspace', workspace] };
  execFileSync(command.executable, command.args, { cwd: root, stdio: 'inherit' });
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
