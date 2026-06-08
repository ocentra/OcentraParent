import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', '05-cross-platform-inventory-matrix');
const resultDirectory = join(root, 'test-results', 'browser-platform-inventory-matrix-proof');
const proofPath = join(resultDirectory, 'proof.json');
const manifestPath = join(outputDirectory, '11-proof-gate-manifest.md');
const androidHostProofPath = join(outputDirectory, '11-android-host-device-proof.json');
const linuxHostProofPath = join(outputDirectory, '12-linux-host-package-proof.json');
const windowsHostProofPath = join(outputDirectory, '13-windows-host-browser-proof.json');
const windowsManagedCdpProofPath = join(outputDirectory, '14-windows-managed-cdp-proof.json');
const androidOwnedShellProofPath = join(outputDirectory, '15-android-owned-browser-shell-proof.json');
const androidOwnedShellRuntimeProofPath = join(outputDirectory, '16-android-owned-shell-runtime-proof.json');

const requiredProofFiles = [
  '00-source-snapshot.md',
  '01-contract-proof.log',
  '02-rust-protocol-proof.log',
  '03-runtime-evidence.json',
  '04-journal-sqlite-proof.json',
  '05-policy-action-proof.json',
  '06-ui-snapshots/ui-not-applicable.md',
  '07-playwright-ui-proof.log',
  '08-security-negative-proof.log',
  '09-manual-platform-proof.md',
  '10-validation-commands.log',
];

await main();

async function main() {
  buildWorkspace('@ocentra-parent/schema-domain');
  buildWorkspace('@ocentra-parent/activity-domain');

  const matrixModule = await import(
    pathToFileURL(join(root, 'packages', 'activity-domain', 'dist', 'browser-platform-inventory-matrix.js')).href
  );
  const matrix = matrixModule.BrowserInventoryPlatformMatrixSchema.parse(matrixModule.BrowserInventoryPlatformMatrix);
  const proofFiles = requiredProofFiles.map((path) => ({
    path: relativePath(join(outputDirectory, path)),
    exists: existsSync(join(outputDirectory, path)),
  }));
  const androidHostProof = await readAndroidHostProof();
  const linuxHostProof = await readLinuxHostProof();
  const windowsHostProof = await readWindowsHostProof();
  const windowsManagedCdpProof = await readWindowsManagedCdpProof();
  const androidOwnedShellProof = await readAndroidOwnedShellProof();
  const androidOwnedShellRuntimeProof = await readAndroidOwnedShellRuntimeProof();
  const failures = [
    ...validateMatrix(matrix.entries),
    ...proofFiles.filter((file) => !file.exists).map((file) => `missing proof artifact: ${file.path}`),
    ...validateLinuxHostProof(linuxHostProof),
    ...validateWindowsHostProof(windowsHostProof),
    ...validateWindowsManagedCdpProof(windowsManagedCdpProof),
    ...validateAndroidOwnedShellProof(androidOwnedShellProof),
    ...validateAndroidOwnedShellRuntimeProof(androidOwnedShellRuntimeProof),
  ];
  const proof = {
    schemaVersion: 1,
    proofMode: 'browser-platform-inventory-matrix-proof',
    generatedAt: new Date().toISOString(),
    proofContract: 'packages/activity-domain/src/browser-platform-inventory-matrix.ts',
    sourceWorkpack: 'docs/plans/browser-plan/workpacks/05-cross-platform-inventory-matrix.md',
    rows: matrix.entries.map((entry) => proofRow(entry)),
    summary: {
      totalRows: matrix.entries.length,
      platformCounts: countBy(matrix.entries.map((entry) => entry.platform)),
      proofStateCounts: countBy(matrix.entries.map((entry) => entry.proofState)),
      manualRequiredRows: matrix.entries.filter((entry) => entry.proofState === 'manual-required').length,
      unsupportedRows: matrix.entries.filter((entry) => entry.proofState === 'unsupported').length,
      fixtureBackedRows: matrix.entries.filter((entry) => entry.proofState === 'fixture-backed').length,
      hostObservedRows: matrix.entries.filter((entry) => entry.proofState === 'host-observed').length,
      productClaimed: false,
      checklistStatusChanged: false,
      failures: failures.length,
    },
    proofFiles,
    noClaimLabels: [
      'non-windows-managed-exact-url-not-claimed',
      'non-windows-known-active-tab-not-claimed',
      linuxHostProof?.browserLaunchObserved === true
        ? 'linux-wsl-headless-browser-launch-proof-present-desktop-adapter-still-manual-required'
        : linuxHostProof
          ? 'linux-wsl-package-inventory-boundary-proof-present-desktop-adapter-still-manual-required'
          : 'macos-linux-platform-adapters-manual-required',
      androidHostProof
        ? 'android-browser-package-visibility-proof-present'
        : 'android-owned-browser-shell-manual-required',
      androidOwnedShellProof
        ? 'android-owned-browser-shell-emulator-device-owner-policy-mutation-and-browser-role-routing-proof-present'
        : 'android-owned-browser-shell-build-install-launch-proof-required',
      androidOwnedShellRuntimeProof
        ? 'android-owned-shell-physical-visible-runtime-proof-present-no-physical-device-owner-or-exact-url-claim'
        : 'android-owned-shell-current-runtime-proof-required',
      windowsHostProof
        ? 'windows-host-browser-inventory-and-default-handler-boundary-proof-present-managed-exact-url-still-unclaimed'
        : 'windows-host-browser-inventory-proof-required',
      windowsManagedCdpProof
        ? 'windows-managed-cdp-exact-url-proof-present-final-enforcement-still-unclaimed'
        : 'windows-managed-cdp-proof-required',
      'ios-familycontrols-safari-extension-manual-required',
      'firefox-bidi-extension-later-adapter',
      'portal-ui-not-changed',
      'product-checklist-upgrade-not-claimed',
    ],
    androidHostProof,
    androidOwnedShellProof,
    androidOwnedShellRuntimeProof,
    linuxHostProof,
    windowsHostProof,
    windowsManagedCdpProof,
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Browser platform inventory matrix proof failed:\n${failures.join('\n')}`);
  }

  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(manifestPath, `${markdownFor(proof)}\n`);

  console.log('browser-platform-inventory-matrix-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(manifestPath)}`);
  console.log(
    `rows=${proof.summary.totalRows} manualRequired=${proof.summary.manualRequiredRows} unsupported=${proof.summary.unsupportedRows}`
  );
}

function buildWorkspace(workspace) {
  const command = workspaceBuildCommand(workspace);

  execFileSync(command.executable, command.args, {
    cwd: root,
    stdio: 'inherit',
  });
}

function workspaceBuildCommand(workspace) {
  if (process.platform === 'win32') {
    return { executable: 'cmd', args: ['/c', 'npm', 'run', 'build', '--workspace', workspace] };
  }

  return { executable: 'npm', args: ['run', 'build', '--workspace', workspace] };
}

function validateMatrix(entries) {
  const failures = [];
  const keys = new Set();

  for (const entry of entries) {
    const key = [entry.platform, entry.browserFamily, entry.browserChannel, entry.productName].join('|');
    if (keys.has(key)) {
      failures.push(`duplicate matrix entry: ${key}`);
    }
    keys.add(key);

    if (entry.platform !== 'windows' && entry.exactUrlCapability === 'managed-exact-url-available') {
      failures.push(`${key} claims managed exact URL on non-Windows platform`);
    }
    if (entry.platform !== 'windows' && entry.activeTabCapability === 'known-active-supported') {
      failures.push(`${key} claims known active tab on non-Windows platform`);
    }
    if (entry.proofState === 'manual-required' && entry.proofRequirement === null) {
      failures.push(`${key} is manual-required without proofRequirement`);
    }
    if (entry.supportTier === 'unsupported' && entry.exactUrlCapability !== 'unsupported') {
      failures.push(`${key} is unsupported without unsupported exactUrlCapability`);
    }
    if (
      (entry.proofState === 'host-observed' || entry.proofState === 'fixture-backed') &&
      entry.platform !== 'windows' &&
      !linuxHostLaunchRowIsAllowed(entry) &&
      !androidOwnedShellRoutingRowIsAllowed(entry)
    ) {
      failures.push(`${key} is host-observed/fixture-backed outside Windows/Linux/Android proof allowance`);
    }
    if (entry.platform === 'ios' && entry.managementTier !== 'unsupported') {
      failures.push(`${key} upgrades iOS browser management before platform proof`);
    }
  }

  const platformCounts = countBy(entries.map((entry) => entry.platform));
  for (const platform of ['windows', 'macos', 'linux', 'android', 'ios']) {
    if ((platformCounts[platform] ?? 0) === 0) {
      failures.push(`missing platform row for ${platform}`);
    }
  }

  return failures;
}

function proofRow(entry) {
  return {
    platform: entry.platform,
    browserFamily: entry.browserFamily,
    browserChannel: entry.browserChannel,
    productName: entry.productName,
    supportTier: entry.supportTier,
    proofState: entry.proofState,
    exactUrlCapability: entry.exactUrlCapability,
    activeTabCapability: entry.activeTabCapability,
    managementTier: entry.managementTier,
    capabilityStatus: entry.capabilityStatus,
    reasonCode: entry.reasonCode,
    proofRequirement: entry.proofRequirement,
  };
}

function markdownFor(proof) {
  const rows = proof.rows
    .map(
      (row) =>
        `| ${row.platform} | ${row.browserFamily} | ${row.productName} | ${row.proofState} | ${row.exactUrlCapability} | ${row.activeTabCapability} | ${row.reasonCode} |`
    )
    .join('\n');

  return [
    '# WP05 Platform Inventory Matrix Proof Gate',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Rows checked: ${proof.summary.totalRows}`,
    `Host-observed rows: ${proof.summary.hostObservedRows}`,
    `Fixture-backed rows: ${proof.summary.fixtureBackedRows}`,
    `Manual-required rows: ${proof.summary.manualRequiredRows}`,
    `Unsupported rows: ${proof.summary.unsupportedRows}`,
    `Product claimed: ${proof.summary.productClaimed}`,
    `Android host proof: ${proof.androidHostProof?.resultState ?? 'not-present'}`,
    `Android owned shell proof: ${proof.androidOwnedShellProof?.resultState ?? 'not-present'}`,
    `Android owned shell runtime proof: ${proof.androidOwnedShellRuntimeProof?.resultState ?? 'not-present'}`,
    `Linux host proof: ${proof.linuxHostProof?.resultState ?? 'not-present'}`,
    `Windows host proof: ${proof.windowsHostProof?.resultState ?? 'not-present'}`,
    `Windows managed CDP proof: ${proof.windowsManagedCdpProof?.resultState ?? 'not-present'}`,
    '',
    '| Platform | Browser | Product | Proof State | Exact URL | Active Tab | Reason |',
    '| --- | --- | --- | --- | --- | --- | --- |',
    rows,
    '',
    'No product checklist upgrade is claimed.',
    'Non-Windows managed exact URL and known-active tab support remain manual-required or unsupported until separate real platform proof exists.',
    proof.androidHostProof
      ? 'Android emulator package-visibility proof is present; exact URL, active tab, device-owner policy, and enforcement remain unclaimed.'
      : 'Android owned browser shell/device proof remains manual-required.',
    proof.androidOwnedShellProof
      ? 'Android owned browser shell build/install/launch proof plus proof-launched emulator Device Owner enrollment, persistent HTTP/HTTPS routing policy mutation evidence, and browser-role implicit routing proof is present, but exact URL policy, known active tab, VPN/DNS, UsageStats, Accessibility, physical-device behavior, final policy execution, and broad enforcement remain unclaimed.'
      : 'Android owned browser shell build/install/launch proof remains manual-required.',
    proof.androidOwnedShellRuntimeProof
      ? 'Android owned-shell current runtime proof projects the physical visible owned-shell launch and screenshot evidence into a typed row, while physical Device Owner, Browser Role routing, exact URL policy, active tab, VPN/DNS, UsageStats, Accessibility, final policy execution, and enforcement remain unclaimed.'
      : 'Android owned-shell current runtime projection proof remains required.',
    proof.linuxHostProof
      ? 'Linux WSL package/PATH/desktop-entry evidence plus a real headless Linux browser launch and screenshot proof are present, but Linux desktop adapter, managed profile, exact URL, active tab, and enforcement remain unclaimed.'
      : 'Linux desktop package and adapter proof remains manual-required.',
    proof.windowsHostProof
      ? 'Windows host browser executable proof and default URL handler association boundary evidence are present, but managed launch, bridge custody, exact URL, active tab, and enforcement remain unclaimed.'
      : 'Windows host browser inventory proof remains required.',
    proof.windowsManagedCdpProof
      ? 'Windows managed CDP proof is present for an Ocentra-launched managed browser profile reaching the exact local proof URL and capturing a CDP screenshot, but active-tab enforcement, final policy execution, browser blocking, and non-Windows support remain unclaimed.'
      : 'Windows managed CDP proof remains required.',
  ].join('\n');
}

async function readAndroidHostProof() {
  if (!existsSync(androidHostProofPath)) {
    return null;
  }

  const proof = JSON.parse(await readFile(androidHostProofPath, 'utf8'));
  return {
    path: relativePath(androidHostProofPath),
    proofId: proof.proofId,
    resultState: proof.hostProofSummary?.resultState ?? 'unknown',
    attachedDeviceCount: proof.hostProofSummary?.attachedDeviceCount ?? 0,
    bootedDeviceCount: proof.hostProofSummary?.bootedDeviceCount ?? 0,
    browserPackageVisible: proof.hostProofSummary?.browserPackageVisible === true,
    ownedBrowserShellVisible: proof.hostProofSummary?.ownedBrowserShellVisible === true,
    exactUrlProofClaimed: proof.hostProofSummary?.exactUrlProofClaimed === true,
    knownActiveTabProofClaimed: proof.hostProofSummary?.knownActiveTabProofClaimed === true,
    enforcementClaimed: proof.hostProofSummary?.enforcementClaimed === true,
  };
}

async function readAndroidOwnedShellProof() {
  if (!existsSync(androidOwnedShellProofPath)) {
    return null;
  }

  const proof = JSON.parse(await readFile(androidOwnedShellProofPath, 'utf8'));
  return {
    path: relativePath(androidOwnedShellProofPath),
    proofId: proof.proofId,
    resultState: proof.hostProofSummary?.resultState ?? 'unknown',
    attachedDeviceCount: proof.hostProofSummary?.attachedDeviceCount ?? 0,
    bootedDeviceCount: proof.hostProofSummary?.bootedDeviceCount ?? 0,
    ownedBrowserShellPackageInstalled: proof.hostProofSummary?.ownedBrowserShellPackageInstalled === true,
    ownedBrowserShellSourceDeclared: proof.hostProofSummary?.ownedBrowserShellSourceDeclared === true,
    webViewDeclared: proof.hostProofSummary?.webViewDeclared === true,
    browsableViewIntentDeclared: proof.hostProofSummary?.browsableViewIntentDeclared === true,
    deviceAdminReceiverDeclared: proof.hostProofSummary?.deviceAdminReceiverDeclared === true,
    deviceAdminMetadataDeclared: proof.hostProofSummary?.deviceAdminMetadataDeclared === true,
    deviceAdminPoliciesDeclared: proof.hostProofSummary?.deviceAdminPoliciesDeclared === true,
    launchObserved: proof.hostProofSummary?.launchObserved === true,
    localProofPageObserved: proof.hostProofSummary?.localProofPageObserved === true,
    deviceOwnerEnrollmentAttempted: proof.hostProofSummary?.deviceOwnerEnrollmentAttempted === true,
    deviceOwnerEnrollmentObserved: proof.hostProofSummary?.deviceOwnerEnrollmentObserved === true,
    deviceOwnerProofLimitedToProofLaunchedEmulator:
      proof.hostProofSummary?.deviceOwnerProofLimitedToProofLaunchedEmulator === true,
    deviceOwnerPolicyMutationAttempted: proof.hostProofSummary?.deviceOwnerPolicyMutationAttempted === true,
    deviceOwnerPolicyMutationObserved: proof.hostProofSummary?.deviceOwnerPolicyMutationObserved === true,
    deviceOwnerPolicyMutationLimitedToProofLaunchedEmulator:
      proof.hostProofSummary?.deviceOwnerPolicyMutationLimitedToProofLaunchedEmulator === true,
    androidOwnedBrowserRoutingEnforcementObserved:
      proof.hostProofSummary?.androidOwnedBrowserRoutingEnforcementObserved === true,
    androidBrowserRoleRoutingObserved: proof.hostProofSummary?.androidBrowserRoleRoutingObserved === true,
    deviceOwnerPolicyMutationClaimed: proof.hostProofSummary?.deviceOwnerPolicyMutationClaimed === true,
    androidOwnedBrowserRoutingEnforcementClaimed:
      proof.hostProofSummary?.androidOwnedBrowserRoutingEnforcementClaimed === true,
    exactUrlPolicyClaimed: proof.hostProofSummary?.exactUrlPolicyClaimed === true,
    knownActiveTabProofClaimed: proof.hostProofSummary?.knownActiveTabProofClaimed === true,
    deviceOwnerEnrollmentClaimed: proof.hostProofSummary?.deviceOwnerEnrollmentClaimed === true,
    vpnDnsBrowserProofClaimed: proof.hostProofSummary?.vpnDnsBrowserProofClaimed === true,
    usageStatsRouteProofClaimed: proof.hostProofSummary?.usageStatsRouteProofClaimed === true,
    accessibilityRouteProofClaimed: proof.hostProofSummary?.accessibilityRouteProofClaimed === true,
    enforcementClaimed: proof.hostProofSummary?.enforcementClaimed === true,
    rawDpmOutputPersisted: proof.hostProofSummary?.rawDpmOutputPersisted === true,
    rawUrlPersisted: proof.hostProofSummary?.rawUrlPersisted === true,
    rawPageContentPersisted: proof.hostProofSummary?.rawPageContentPersisted === true,
  };
}

async function readAndroidOwnedShellRuntimeProof() {
  if (!existsSync(androidOwnedShellRuntimeProofPath)) {
    return null;
  }

  const proof = JSON.parse(await readFile(androidOwnedShellRuntimeProofPath, 'utf8'));
  return {
    path: relativePath(androidOwnedShellRuntimeProofPath),
    proofMode: proof.proofMode,
    resultState:
      proof.summary?.physicalVisibleRows === 1
        ? 'android-owned-shell-physical-visible-runtime-proof'
        : 'manual-required',
    physicalVisibleRows: proof.summary?.physicalVisibleRows ?? 0,
    manualRequiredRows: proof.summary?.manualRequiredRows ?? 0,
    productClaimed: proof.summary?.productClaimed === true,
    physicalDeviceOwnerClaimed: proof.summary?.physicalDeviceOwnerClaimed === true,
    physicalBrowserRoleRoutingClaimed: proof.summary?.physicalBrowserRoleRoutingClaimed === true,
    exactUrlPolicyClaimed: proof.summary?.exactUrlPolicyClaimed === true,
    knownActiveTabProofClaimed: proof.summary?.knownActiveTabProofClaimed === true,
    enforcementClaimed: proof.summary?.enforcementClaimed === true,
    failures: proof.summary?.failures ?? proof.failures?.length ?? 0,
  };
}

async function readLinuxHostProof() {
  if (!existsSync(linuxHostProofPath)) {
    return null;
  }

  const proof = JSON.parse(await readFile(linuxHostProofPath, 'utf8'));
  return {
    path: relativePath(linuxHostProofPath),
    proofId: proof.proofId,
    resultState: proof.hostProofSummary?.resultState ?? 'unknown',
    wslAvailable: proof.hostProofSummary?.wslAvailable === true,
    browserCommandVisible: proof.hostProofSummary?.browserCommandVisible === true,
    browserPackageInstalled: proof.hostProofSummary?.browserPackageInstalled === true,
    browserDesktopEntryVisible: proof.hostProofSummary?.browserDesktopEntryVisible === true,
    browserLaunchAttempted: proof.hostProofSummary?.browserLaunchAttempted === true,
    browserLaunchObserved: proof.hostProofSummary?.browserLaunchObserved === true,
    browserLaunchScreenshotCaptured: proof.hostProofSummary?.browserLaunchScreenshotCaptured === true,
    browserLaunchScreenshotPersisted: proof.hostProofSummary?.browserLaunchScreenshotPersisted === true,
    rawBrowserLaunchDomPersisted: proof.hostProofSummary?.rawBrowserLaunchDomPersisted === true,
    rawBrowserLaunchUrlPersisted: proof.hostProofSummary?.rawBrowserLaunchUrlPersisted === true,
    exactUrlProofClaimed: proof.hostProofSummary?.exactUrlProofClaimed === true,
    knownActiveTabProofClaimed: proof.hostProofSummary?.knownActiveTabProofClaimed === true,
    enforcementClaimed: proof.hostProofSummary?.enforcementClaimed === true,
  };
}

async function readWindowsHostProof() {
  if (!existsSync(windowsHostProofPath)) {
    return null;
  }

  const proof = JSON.parse(await readFile(windowsHostProofPath, 'utf8'));
  return {
    path: relativePath(windowsHostProofPath),
    proofId: proof.proofId,
    resultState: proof.hostProofSummary?.resultState ?? 'unknown',
    windowsHost: proof.hostProofSummary?.windowsHost === true,
    executableVisible: proof.hostProofSummary?.executableVisible === true,
    defaultUrlHandlerVisible: proof.hostProofSummary?.defaultUrlHandlerVisible === true,
    defaultUrlHandlerAssociationVisible: proof.hostProofSummary?.defaultUrlHandlerAssociationVisible === true,
    knownDefaultBrowserHandlerVisible: proof.hostProofSummary?.knownDefaultBrowserHandlerVisible === true,
    managedLaunchClaimed: proof.hostProofSummary?.managedLaunchClaimed === true,
    exactUrlProofClaimed: proof.hostProofSummary?.exactUrlProofClaimed === true,
    knownActiveTabProofClaimed: proof.hostProofSummary?.knownActiveTabProofClaimed === true,
    enforcementClaimed: proof.hostProofSummary?.enforcementClaimed === true,
  };
}

async function readWindowsManagedCdpProof() {
  if (!existsSync(windowsManagedCdpProofPath)) {
    return null;
  }

  const proof = JSON.parse(await readFile(windowsManagedCdpProofPath, 'utf8'));
  return {
    path: relativePath(windowsManagedCdpProofPath),
    proofId: proof.proofId,
    resultState: proof.hostProofSummary?.resultState ?? 'unknown',
    windowsHost: proof.hostProofSummary?.windowsHost === true,
    realManagedBrowserLaunched: proof.hostProofSummary?.realManagedBrowserLaunched === true,
    loopbackCdpEndpointResponded: proof.hostProofSummary?.loopbackCdpEndpointResponded === true,
    cdpVersionEndpointResponded: proof.hostProofSummary?.cdpVersionEndpointResponded === true,
    cdpTabListEndpointResponded: proof.hostProofSummary?.cdpTabListEndpointResponded === true,
    exactManagedUrlObserved: proof.hostProofSummary?.exactManagedUrlObserved === true,
    activeTabKnownByTargetSelection: proof.hostProofSummary?.activeTabKnownByTargetSelection === true,
    cdpScreenshotCaptured: proof.hostProofSummary?.cdpScreenshotCaptured === true,
    managedProfileCreated: proof.hostProofSummary?.managedProfileCreated === true,
    managedProfileDeletedAfterProof: proof.hostProofSummary?.managedProfileDeletedAfterProof === true,
    rawExecutablePathPersisted: proof.hostProofSummary?.rawExecutablePathPersisted === true,
    rawProfilePathPersisted: proof.hostProofSummary?.rawProfilePathPersisted === true,
    rawCdpPayloadPersisted: proof.hostProofSummary?.rawCdpPayloadPersisted === true,
    rawPageContentPersisted: proof.hostProofSummary?.rawPageContentPersisted === true,
    activeTabEnforcementClaimed: proof.hostProofSummary?.activeTabEnforcementClaimed === true,
    finalPolicyExecutionClaimed: proof.hostProofSummary?.finalPolicyExecutionClaimed === true,
    enforcementClaimed: proof.hostProofSummary?.enforcementClaimed === true,
  };
}

function validateLinuxHostProof(proof) {
  if (proof === null) {
    return [
      'missing Linux host proof artifact: output/browser-plan-proof/05-cross-platform-inventory-matrix/12-linux-host-package-proof.json',
    ];
  }

  const failures = [];
  if (proof.resultState !== 'linux-wsl-headless-browser-launch-proof') {
    failures.push(`Linux host proof has unexpected resultState: ${proof.resultState}`);
  }
  if (!proof.wslAvailable) {
    failures.push('Linux host proof did not observe WSL Linux availability');
  }
  if (!proof.browserCommandVisible || !proof.browserPackageInstalled || !proof.browserDesktopEntryVisible) {
    failures.push('Linux host proof lacks command, package, and desktop-entry browser evidence');
  }
  if (!proof.browserLaunchAttempted || !proof.browserLaunchObserved || !proof.browserLaunchScreenshotCaptured) {
    failures.push('Linux host proof did not observe a real headless browser launch with screenshot evidence');
  }
  if (proof.rawBrowserLaunchDomPersisted || proof.rawBrowserLaunchUrlPersisted) {
    failures.push('Linux host proof persisted raw launch DOM or raw launch URL evidence');
  }
  if (proof.exactUrlProofClaimed || proof.knownActiveTabProofClaimed || proof.enforcementClaimed) {
    failures.push('Linux host proof made exact URL, active tab, or enforcement claims');
  }
  return failures;
}

function validateWindowsHostProof(proof) {
  if (proof === null) {
    return [
      'missing Windows host proof artifact: output/browser-plan-proof/05-cross-platform-inventory-matrix/13-windows-host-browser-proof.json',
    ];
  }

  const failures = [];
  if (!proof.windowsHost) {
    failures.push('Windows host proof was not captured on a Windows host');
  }
  if (!proof.executableVisible && !proof.defaultUrlHandlerVisible) {
    failures.push('Windows host proof lacks browser executable and default URL handler evidence');
  }
  if (proof.defaultUrlHandlerVisible && !proof.defaultUrlHandlerAssociationVisible) {
    failures.push('Windows host proof saw a handler without marking the default-handler association boundary visible');
  }
  if (
    proof.managedLaunchClaimed ||
    proof.exactUrlProofClaimed ||
    proof.knownActiveTabProofClaimed ||
    proof.enforcementClaimed
  ) {
    failures.push('Windows host proof made managed launch, exact URL, active tab, or enforcement claims');
  }
  return failures;
}

function linuxHostLaunchRowIsAllowed(entry) {
  return (
    entry.platform === 'linux' &&
    entry.browserFamily === 'chrome' &&
    entry.proofState === 'host-observed' &&
    entry.reasonCode === 'linux-chrome-host-observed-launch-proof' &&
    entry.exactUrlCapability === 'manual-required' &&
    entry.activeTabCapability === 'manual-required' &&
    entry.managementTier === 'manual-required'
  );
}

function androidOwnedShellRoutingRowIsAllowed(entry) {
  return (
    entry.platform === 'android' &&
    entry.browserFamily === 'unknown-chromium' &&
    entry.proofState === 'host-observed' &&
    entry.reasonCode === 'android-owned-browser-shell-browser-role-routing-proof' &&
    entry.exactUrlCapability === 'manual-required' &&
    entry.activeTabCapability === 'manual-required' &&
    entry.managementTier === 'owned-shell'
  );
}

function validateWindowsManagedCdpProof(proof) {
  if (proof === null) {
    return [
      'missing Windows managed CDP proof artifact: output/browser-plan-proof/05-cross-platform-inventory-matrix/14-windows-managed-cdp-proof.json',
    ];
  }

  const failures = [];
  if (!proof.windowsHost) {
    failures.push('Windows managed CDP proof was not captured on a Windows host');
  }
  if (proof.resultState !== 'windows-managed-cdp-exact-url-proof') {
    failures.push(`Windows managed CDP proof has unexpected resultState: ${proof.resultState}`);
  }
  if (!proof.realManagedBrowserLaunched || !proof.loopbackCdpEndpointResponded) {
    failures.push('Windows managed CDP proof did not launch a real managed browser with a loopback CDP endpoint');
  }
  if (!proof.cdpVersionEndpointResponded || !proof.cdpTabListEndpointResponded) {
    failures.push('Windows managed CDP proof lacks CDP version/list endpoint evidence');
  }
  if (!proof.exactManagedUrlObserved || !proof.activeTabKnownByTargetSelection) {
    failures.push('Windows managed CDP proof lacks exact managed URL and target-selection evidence');
  }
  if (!proof.cdpScreenshotCaptured) {
    failures.push('Windows managed CDP proof did not capture a CDP screenshot');
  }
  if (!proof.managedProfileCreated || !proof.managedProfileDeletedAfterProof) {
    failures.push('Windows managed CDP proof did not create and clean up the temporary managed profile');
  }
  if (
    proof.rawExecutablePathPersisted ||
    proof.rawProfilePathPersisted ||
    proof.rawCdpPayloadPersisted ||
    proof.rawPageContentPersisted
  ) {
    failures.push('Windows managed CDP proof persisted raw executable/profile/CDP/page content data');
  }
  if (proof.activeTabEnforcementClaimed || proof.finalPolicyExecutionClaimed || proof.enforcementClaimed) {
    failures.push('Windows managed CDP proof made active-tab enforcement, final policy, or enforcement claims');
  }
  return failures;
}

function validateAndroidOwnedShellProof(proof) {
  if (proof === null) {
    return [
      'missing Android owned shell proof artifact: output/browser-plan-proof/05-cross-platform-inventory-matrix/15-android-owned-browser-shell-proof.json',
    ];
  }

  const failures = [];
  if (proof.resultState !== 'android-owned-browser-shell-browser-role-routing-proof') {
    failures.push(`Android owned shell proof has unexpected resultState: ${proof.resultState}`);
  }
  if (!proof.ownedBrowserShellPackageInstalled) {
    failures.push('Android owned shell proof did not install the owned browser shell package');
  }
  if (!proof.ownedBrowserShellSourceDeclared || !proof.webViewDeclared || !proof.browsableViewIntentDeclared) {
    failures.push('Android owned shell proof lacks source-backed package/WebView/BROWSABLE VIEW evidence');
  }
  if (!proof.deviceAdminReceiverDeclared || !proof.deviceAdminMetadataDeclared || !proof.deviceAdminPoliciesDeclared) {
    failures.push('Android owned shell proof lacks source-backed DeviceAdmin receiver metadata evidence');
  }
  if (!proof.launchObserved || !proof.localProofPageObserved) {
    failures.push('Android owned shell proof did not observe launched proof UI');
  }
  if (
    !proof.deviceOwnerEnrollmentAttempted ||
    !proof.deviceOwnerEnrollmentObserved ||
    !proof.deviceOwnerProofLimitedToProofLaunchedEmulator
  ) {
    failures.push('Android owned shell proof did not observe proof-launched emulator Device Owner enrollment evidence');
  }
  if (
    !proof.deviceOwnerPolicyMutationAttempted ||
    !proof.deviceOwnerPolicyMutationObserved ||
    !proof.deviceOwnerPolicyMutationLimitedToProofLaunchedEmulator
  ) {
    failures.push(
      'Android owned shell proof did not observe proof-launched emulator Device Owner persistent browser routing policy mutation evidence'
    );
  }
  if (
    !proof.androidOwnedBrowserRoutingEnforcementObserved ||
    !proof.androidOwnedBrowserRoutingEnforcementClaimed ||
    !proof.androidBrowserRoleRoutingObserved
  ) {
    failures.push('Android owned shell proof did not observe browser-role implicit VIEW routing to the owned shell');
  }
  if (
    proof.exactUrlPolicyClaimed ||
    proof.knownActiveTabProofClaimed ||
    !proof.deviceOwnerPolicyMutationClaimed ||
    proof.vpnDnsBrowserProofClaimed ||
    proof.usageStatsRouteProofClaimed ||
    proof.accessibilityRouteProofClaimed ||
    proof.enforcementClaimed
  ) {
    failures.push(
      'Android owned shell proof made dishonest exact URL/active tab/VPN/UsageStats/Accessibility/broad enforcement claims or failed to claim the narrow policy mutation proof'
    );
  }
  if (proof.rawUrlPersisted || proof.rawPageContentPersisted) {
    failures.push('Android owned shell proof persisted raw URL or raw page content');
  }
  if (proof.rawDpmOutputPersisted) {
    failures.push('Android owned shell proof persisted raw Device Policy Manager output');
  }
  return failures;
}

function validateAndroidOwnedShellRuntimeProof(proof) {
  if (proof === null) {
    return [
      'missing Android owned-shell runtime proof artifact: output/browser-plan-proof/05-cross-platform-inventory-matrix/16-android-owned-shell-runtime-proof.json',
    ];
  }

  const failures = [];
  if (proof.proofMode !== 'browser-platform-android-owned-shell-runtime-proof') {
    failures.push(`Android owned-shell runtime proof has unexpected proofMode: ${proof.proofMode}`);
  }
  if (proof.physicalVisibleRows !== 1 || proof.manualRequiredRows !== 1) {
    failures.push('Android owned-shell runtime proof lacks one physical-visible row and one manual-required row');
  }
  if (
    proof.productClaimed ||
    proof.physicalDeviceOwnerClaimed ||
    proof.physicalBrowserRoleRoutingClaimed ||
    proof.exactUrlPolicyClaimed ||
    proof.knownActiveTabProofClaimed ||
    proof.enforcementClaimed
  ) {
    failures.push(
      'Android owned-shell runtime proof made product, physical Device Owner, physical Browser Role, exact URL, active-tab, or enforcement claims'
    );
  }
  if (proof.failures !== 0) {
    failures.push(`Android owned-shell runtime proof recorded ${proof.failures} failures`);
  }
  return failures;
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
