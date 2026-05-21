import { spawn } from 'node:child_process';
import { createServer } from 'node:net';
import { mkdir, rm, stat, writeFile } from 'node:fs/promises';
import { basename, join } from 'node:path';
import { tmpdir } from 'node:os';
import { setTimeout as delay } from 'node:timers/promises';

import { removeDirectoryWithRetry } from './agent-service-process.mjs';

const defaultUrls = ['https://example.com/', 'https://www.wikipedia.org/', 'https://www.youtube.com/'];
const defaultProfiles = ['managed-browser-profile-a', 'managed-browser-profile-b', 'managed-browser-profile-c'];
const runId = new Date().toISOString().replaceAll(':', '-').replaceAll('.', '-');
const evidenceDirectory = join(process.cwd(), 'test-results', 'managed-browser-profile-matrix');
const probeRoot = join(tmpdir(), `ocentra-parent-managed-browser-profile-matrix-${process.pid}`);

const urls = envList('OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_URLS', defaultUrls);
const profiles = envList('OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_PROFILES', defaultProfiles);
const timeoutMs = envNumber('OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_TIMEOUT_MS', 30_000);

const supportedBrowsers = await installedSupportedBrowsers();
const unsupportedBrowsers = await installedUnsupportedBrowsers();

if (supportedBrowsers.length === 0) {
  throw new Error('No installed Chrome or Edge executable found for managed browser matrix proof.');
}

await mkdir(evidenceDirectory, { recursive: true });
await mkdir(probeRoot, { recursive: true });

const browserResults = [];
try {
  for (const browser of supportedBrowsers) {
    browserResults.push(await runBrowserMatrix(browser));
  }
} finally {
  await stopWindowsProcessesByCommandLineFragment(probeRoot);
  await removeDirectoryWithRetry(probeRoot, { attempts: 20, delayMs: 250 });
}

const evidence = {
  schemaVersion: 1,
  generatedAt: new Date().toISOString(),
  urls,
  profiles,
  supportedBrowsers: browserResults,
  unsupportedInstalledBrowsers: unsupportedBrowsers,
  summary: summarize(browserResults, unsupportedBrowsers),
};

const evidencePath = join(evidenceDirectory, `${runId}.json`);
await writeFile(evidencePath, `${JSON.stringify(evidence, null, 2)}\n`);

printSummary(evidence, evidencePath);

if (evidence.summary.failures.length > 0) {
  process.exitCode = 1;
}

async function runBrowserMatrix(browser) {
  const profileRuns = await Promise.all(profiles.map((profileName) => launchProfile(browser, profileName)));
  try {
    const observations = [];
    for (const profileRun of profileRuns) {
      observations.push(await observeProfile(browser, profileRun));
    }
    return {
      browser,
      profiles: observations,
    };
  } finally {
    for (const profileRun of profileRuns) {
      await cleanupProfileRun(profileRun);
    }
  }
}

async function launchProfile(browser, profileName) {
  const port = await freePort();
  const profileDir = join(probeRoot, browser.id, profileName);
  await mkdir(profileDir, { recursive: true });

  const args = [
    '--remote-debugging-address=127.0.0.1',
    `--remote-debugging-port=${port}`,
    `--user-data-dir=${profileDir}`,
    '--profile-directory=OcentraManagedChild',
    '--no-first-run',
    '--no-default-browser-check',
    '--new-window',
    ...urls,
  ];
  const child = spawn(browser.executablePath, args, {
    stdio: 'ignore',
    windowsHide: true,
  });

  return {
    child,
    port,
    profileName,
    profileDir,
  };
}

async function observeProfile(browser, profileRun) {
  const version = await waitForJson(profileRun.port, '/json/version');
  const targets = await waitForTargets(profileRun.port);
  const pageTargets = targets
    .filter((target) => target.type === 'page')
    .map((target) => ({
      targetId: String(target.id ?? ''),
      type: String(target.type ?? ''),
      url: String(target.url ?? ''),
      title: target.title === undefined ? null : String(target.title),
      urlCapture: capturedUrl(target.url),
      titleCapture: capturedTitle(target.title),
    }));
  const siteEvidence = urls.map((url) => siteEvidenceForUrl(url, pageTargets));

  return {
    profileName: profileRun.profileName,
    profilePathContainsManagedPrefix: profileRun.profileDir.includes('managed-browser-profile'),
    browserVersion: String(version.Browser ?? ''),
    protocolVersion: version['Protocol-Version'] === undefined ? null : String(version['Protocol-Version']),
    devtoolsEndpoint: 'loopback-redacted',
    pageTargetCount: pageTargets.length,
    pageTargets,
    siteEvidence,
    assertions: {
      canConnectManagedProfile: Boolean(version.Browser),
      canSeeTabs: pageTargets.length >= urls.length,
      canSeeUrls: siteEvidence.every((site) => site.captured),
      canMatchRequestedSites: siteEvidence.every((site) => site.matchedRequestedHost),
      activeTabProof: 'not-proven-by-json-list',
      visitedHistoryProof: 'not-proven-by-json-list',
    },
    browserFamily: browser.family,
    browserChannel: browser.channel,
  };
}

async function cleanupProfileRun(profileRun) {
  await stopWindowsProcessesByCommandLineFragment(profileRun.profileDir);
  if (profileRun.child.exitCode === null && profileRun.child.signalCode === null) {
    profileRun.child.kill();
  }
  await removeDirectoryWithRetry(profileRun.profileDir, { attempts: 20, delayMs: 250 });
}

function siteEvidenceForUrl(requestedUrl, pageTargets) {
  const requested = new URL(requestedUrl);
  const matchedTarget = pageTargets.find((target) => hostMatches(requested.hostname, target.url));
  const fallbackTarget = pageTargets.find((target) => target.url === requestedUrl) ?? null;
  const target = matchedTarget ?? fallbackTarget;
  return {
    requestedUrl,
    requestedHost: requested.hostname,
    captured: target !== undefined && target !== null && capturedUrl(target.url),
    matchedRequestedHost: matchedTarget !== undefined,
    observedUrl: target?.url ?? null,
    observedTitle: target?.title ?? null,
  };
}

async function waitForTargets(port) {
  const deadline = Date.now() + timeoutMs;
  let latestTargets = [];
  while (Date.now() < deadline) {
    latestTargets = await jsonRequest(port, '/json/list').catch(() => []);
    const pageTargets = latestTargets.filter((target) => target.type === 'page');
    const capturedCount = pageTargets.filter((target) => capturedUrl(target.url)).length;
    if (capturedCount >= urls.length) {
      return latestTargets;
    }
    await delay(500);
  }
  return latestTargets;
}

async function waitForJson(port, path) {
  const deadline = Date.now() + timeoutMs;
  let lastError;
  while (Date.now() < deadline) {
    try {
      return await jsonRequest(port, path);
    } catch (error) {
      lastError = error;
      await delay(500);
    }
  }
  throw new Error(`Timed out waiting for http://127.0.0.1:${port}${path}: ${lastError?.message ?? 'no response'}`);
}

async function jsonRequest(port, path) {
  const response = await fetch(`http://127.0.0.1:${port}${path}`);
  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }
  return response.json();
}

function capturedUrl(value) {
  return typeof value === 'string' && value.startsWith('http');
}

function capturedTitle(value) {
  return typeof value === 'string' && value.length > 0;
}

function hostMatches(expectedHost, observedUrl) {
  if (!capturedUrl(observedUrl)) {
    return false;
  }
  const observed = new URL(observedUrl);
  const expected = normalizeHost(expectedHost);
  return normalizeHost(observed.hostname) === expected || normalizeHost(observed.hostname).endsWith(`.${expected}`);
}

function normalizeHost(host) {
  return host.toLowerCase().replace(/^www\./, '');
}

async function installedSupportedBrowsers() {
  const candidates = browserCandidates().filter((candidate) => candidate.supported);
  const installed = [];
  for (const candidate of candidates) {
    if (await fileExists(candidate.executablePath)) {
      installed.push(candidate);
    }
  }
  return installed;
}

async function installedUnsupportedBrowsers() {
  const candidates = browserCandidates().filter((candidate) => !candidate.supported);
  const installed = [];
  for (const candidate of candidates) {
    if (await fileExists(candidate.executablePath)) {
      installed.push({
        browser: {
          id: candidate.id,
          family: candidate.family,
          channel: candidate.channel,
          executablePath: candidate.executablePath,
        },
        status: 'unsupported-by-current-chromium-devtools-bridge',
      });
    }
  }
  return installed;
}

function browserCandidates() {
  if (process.platform !== 'win32') {
    return [];
  }

  return [
    ...windowsRoots().flatMap((root) => [
      chromiumCandidate('edge-stable', 'edge', join(root, 'Microsoft', 'Edge', 'Application', 'msedge.exe')),
      chromiumCandidate('chrome-stable', 'chrome', join(root, 'Google', 'Chrome', 'Application', 'chrome.exe')),
      unsupportedCandidate('firefox-stable', 'firefox', join(root, 'Mozilla Firefox', 'firefox.exe')),
    ]),
    chromiumCandidate(
      'edge-local',
      'edge',
      join(process.env.LOCALAPPDATA ?? '', 'Microsoft', 'Edge', 'Application', 'msedge.exe')
    ),
    chromiumCandidate(
      'chrome-local',
      'chrome',
      join(process.env.LOCALAPPDATA ?? '', 'Google', 'Chrome', 'Application', 'chrome.exe')
    ),
    unsupportedCandidate(
      'firefox-local',
      'firefox',
      join(process.env.LOCALAPPDATA ?? '', 'Mozilla Firefox', 'firefox.exe')
    ),
  ];
}

function chromiumCandidate(id, family, executablePath) {
  return {
    id,
    family,
    channel: channelFromPath(executablePath),
    executablePath,
    supported: true,
    bridge: 'chromium-devtools-protocol',
  };
}

function unsupportedCandidate(id, family, executablePath) {
  return {
    id,
    family,
    channel: 'stable',
    executablePath,
    supported: false,
    bridge: 'unsupported',
  };
}

function windowsRoots() {
  return [process.env.ProgramFiles, process.env['ProgramFiles(x86)'], process.env.LOCALAPPDATA].filter(Boolean);
}

function channelFromPath(pathValue) {
  const normalized = pathValue.toLowerCase();
  if (normalized.includes('beta')) {
    return 'beta';
  }
  if (normalized.includes('dev')) {
    return 'dev';
  }
  if (normalized.includes('sxs') || normalized.includes('canary')) {
    return 'canary';
  }
  return 'stable';
}

async function fileExists(pathValue) {
  if (pathValue.length === 0) {
    return false;
  }
  try {
    return (await stat(pathValue)).isFile();
  } catch {
    return false;
  }
}

async function freePort() {
  const server = createServer();
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', resolve);
  });
  const address = server.address();
  await new Promise((resolve) => server.close(resolve));
  return address.port;
}

async function stopWindowsProcessesByCommandLineFragment(fragment) {
  if (process.platform !== 'win32') {
    return;
  }
  const script = [
    "$fragment = [Environment]::GetEnvironmentVariable('OCENTRA_PARENT_STOP_FRAGMENT')",
    'Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -and $_.CommandLine.Contains($fragment) } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }',
  ].join('; ');
  await new Promise((resolve) => {
    const child = spawn('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', script], {
      env: { ...process.env, OCENTRA_PARENT_STOP_FRAGMENT: fragment },
      stdio: 'ignore',
      windowsHide: true,
    });
    child.once('exit', resolve);
  });
}

function summarize(browserResults, unsupported) {
  const failures = [];
  const profileCount = browserResults.reduce((count, browser) => count + browser.profiles.length, 0);
  const capturedUrls = browserResults.reduce(
    (count, browser) =>
      count +
      browser.profiles.reduce(
        (profileCountForBrowser, profile) =>
          profileCountForBrowser + profile.siteEvidence.filter((site) => site.captured).length,
        0
      ),
    0
  );
  for (const browser of browserResults) {
    for (const profile of browser.profiles) {
      if (!profile.assertions.canConnectManagedProfile) {
        failures.push(`${browser.browser.id}/${profile.profileName}: managed profile did not connect`);
      }
      if (!profile.assertions.canSeeTabs) {
        failures.push(`${browser.browser.id}/${profile.profileName}: expected ${urls.length} page targets`);
      }
      if (!profile.assertions.canSeeUrls) {
        failures.push(`${browser.browser.id}/${profile.profileName}: one or more tab URLs were not captured`);
      }
    }
  }

  return {
    supportedBrowserCount: browserResults.length,
    unsupportedInstalledBrowserCount: unsupported.length,
    managedProfileCount: profileCount,
    requestedUrlCount: urls.length,
    capturedUrlCount: capturedUrls,
    failures,
  };
}

function printSummary(evidence, evidencePath) {
  console.log(`managed-browser-profile-matrix-ok=${evidence.summary.failures.length === 0}`);
  console.log(`evidence=${evidencePath}`);
  console.log(
    `supportedBrowsers=${evidence.summary.supportedBrowserCount} managedProfiles=${evidence.summary.managedProfileCount} capturedUrls=${evidence.summary.capturedUrlCount}`
  );
  for (const browser of evidence.supportedBrowsers) {
    console.log(`${browser.browser.id} ${browser.browser.executablePath}`);
    for (const profile of browser.profiles) {
      const urlsForProfile = profile.siteEvidence
        .map((site) => `${site.requestedHost}=>${site.observedUrl}`)
        .join(' | ');
      console.log(`  ${profile.profileName}: tabs=${profile.pageTargetCount} urls=${urlsForProfile}`);
    }
  }
  for (const item of evidence.unsupportedInstalledBrowsers) {
    console.log(`unsupported=${item.browser.family} path=${item.browser.executablePath} status=${item.status}`);
  }
  for (const failure of evidence.summary.failures) {
    console.error(`failure=${failure}`);
  }
}

function envList(name, fallback) {
  const value = process.env[name];
  if (value === undefined || value.trim().length === 0) {
    return fallback;
  }
  return value
    .split(',')
    .map((item) => item.trim())
    .filter(Boolean);
}

function envNumber(name, fallback) {
  const parsed = Number(process.env[name]);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback;
}
