import { spawn } from 'node:child_process';
import { createServer } from 'node:net';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { setTimeout as delay } from 'node:timers/promises';

import { removeDirectoryWithRetry } from './agent-service-process.mjs';

const runId = new Date().toISOString().replaceAll(':', '-').replaceAll('.', '-');
const evidenceDirectory = join(process.cwd(), 'test-results', 'managed-browser-intervention-proof');
const screenshotDirectory = join(evidenceDirectory, `${runId}-screenshots`);
const probeRoot = join(tmpdir(), `ocentra-parent-managed-browser-intervention-${process.pid}`);

const timeoutMs = envNumber('OCENTRA_PARENT_MANAGED_BROWSER_INTERVENTION_TIMEOUT_MS', 45_000);
const commandTimeoutMs = envNumber('OCENTRA_PARENT_MANAGED_BROWSER_INTERVENTION_COMMAND_TIMEOUT_MS', 15_000);
const readyTimeoutMs = envNumber('OCENTRA_PARENT_MANAGED_BROWSER_INTERVENTION_READY_TIMEOUT_MS', 20_000);
const blockedSiteUrl =
  process.env.OCENTRA_PARENT_MANAGED_BROWSER_INTERVENTION_BLOCKED_SITE_URL ??
  'https://example.com/ocentra-parent-block-proof';
const blockedVideoUrl =
  process.env.OCENTRA_PARENT_MANAGED_BROWSER_INTERVENTION_BLOCKED_VIDEO_URL ??
  'https://www.youtube.com/watch?v=dQw4w9WgXcQ';
const allowedUrl = process.env.OCENTRA_PARENT_MANAGED_BROWSER_INTERVENTION_ALLOWED_URL ?? 'https://www.wikipedia.org/';
const blockMarker = 'OCENTRA_MANAGED_BROWSER_BLOCKED';

const ruleSet = [
  {
    id: 'blocked-site-host',
    url: blockedSiteUrl,
    label: 'Disallowed site',
    match: (url) => hostMatches('example.com', url),
  },
  {
    id: 'blocked-youtube-video-url',
    url: blockedVideoUrl,
    label: 'Disallowed YouTube video URL',
    match: (url) => hostMatches('www.youtube.com', url) && new URL(url).pathname === '/watch',
  },
];

async function main() {
  const browsers = await installedDirectBrowsers();
  if (browsers.length === 0) {
    throw new Error('No installed Chrome, Edge, or Firefox executable found for managed intervention proof.');
  }

  await mkdir(evidenceDirectory, { recursive: true });
  await mkdir(screenshotDirectory, { recursive: true });
  await mkdir(probeRoot, { recursive: true });

  const results = [];
  try {
    for (const browser of browsers) {
      if (browser.bridge === 'webdriver-bidi') {
        results.push(await runFirefoxInterventionProof(browser));
      } else {
        results.push(await runChromiumInterventionProof(browser));
      }
    }
  } finally {
    await stopWindowsProcessesByCommandLineFragment(probeRoot);
    await removeDirectoryWithRetry(probeRoot, { attempts: 20, delayMs: 250 });
  }

  const evidence = {
    schemaVersion: 1,
    generatedAt: new Date().toISOString(),
    blockedSiteUrl,
    blockedVideoUrl,
    allowedUrl,
    blockMarker,
    browsers: results,
    summary: summarize(results),
  };

  const evidencePath = join(evidenceDirectory, `${runId}.json`);
  await writeFile(evidencePath, `${JSON.stringify(evidence, null, 2)}\n`);
  printSummary(evidence, evidencePath);

  if (evidence.summary.failures.length > 0) {
    process.exitCode = 1;
  }
}

async function runChromiumInterventionProof(browser) {
  const profileRun = await launchChromiumProfile(browser);
  try {
    const version = await waitForJson(profileRun.port, '/json/version');
    const target = await waitForFirstPageTarget(profileRun.port);
    const client = await DevToolsClient.connect(target.webSocketDebuggerUrl);
    const interventions = [];
    try {
      await client.command('Page.enable', {});
      await client.command('Runtime.enable', {});
      await client.command('Fetch.enable', {
        patterns: [{ urlPattern: '*', resourceType: 'Document', requestStage: 'Request' }],
      });
      client.onEvent((event) => {
        if (event.method === 'Fetch.requestPaused') {
          void handleChromiumRequestPaused(client, event, interventions);
        }
      });

      const cases = [];
      for (const rule of ruleSet) {
        cases.push(await observeChromiumNavigation(browser, client, rule.url, true, rule.id));
      }
      cases.push(await observeChromiumNavigation(browser, client, allowedUrl, false, 'allowed-control'));

      return {
        browser,
        profileName: 'managed-browser-intervention-proof',
        profilePathContainsManagedPrefix: profileRun.profileDir.includes('managed-browser-intervention'),
        browserVersion: version.Browser,
        bridge: browser.bridge,
        devtoolsEndpoint: 'loopback-redacted',
        interventions,
        cases,
        assertions: assertionsForCases(cases),
      };
    } finally {
      client.close();
    }
  } finally {
    await cleanupProfileRun(profileRun);
  }
}

async function handleChromiumRequestPaused(client, event, interventions) {
  const requestUrl = event.params.request.url;
  const rule = ruleForUrl(requestUrl);
  if (rule === null) {
    await client.command('Fetch.continueRequest', { requestId: event.params.requestId });
    return;
  }
  const html = blockPageHtml(rule, requestUrl, 'chromium-devtools-protocol');
  interventions.push({
    bridge: 'chromium-devtools-protocol',
    method: 'Fetch.fulfillRequest',
    ruleId: rule.id,
    requestedUrl: requestUrl,
    observedAt: new Date().toISOString(),
  });
  await client.command('Fetch.fulfillRequest', {
    requestId: event.params.requestId,
    responseCode: 451,
    responsePhrase: 'Blocked by Ocentra Managed Browser',
    responseHeaders: [
      { name: 'content-type', value: 'text/html; charset=utf-8' },
      { name: 'cache-control', value: 'no-store' },
    ],
    body: Buffer.from(html, 'utf8').toString('base64'),
  });
}

async function observeChromiumNavigation(browser, client, url, expectedBlocked, ruleId) {
  await client.command('Page.navigate', { url });
  await waitForChromiumReady(client);
  const markerPresent = await evaluateChromiumValue(
    client,
    `document.body.textContent.includes(${JSON.stringify(blockMarker)})`
  );
  const title = await evaluateChromiumValue(client, 'document.title');
  const locationHref = await evaluateChromiumValue(client, 'location.href');
  const screenshotPath = await captureChromiumScreenshot(browser, client, ruleId);
  return {
    ruleId,
    requestedUrl: url,
    observedUrl: locationHref,
    title,
    expectedBlocked,
    blockMarkerPresent: markerPresent === true,
    interventionProof:
      expectedBlocked && markerPresent === true
        ? 'blocked-page-rendered-before-target-document'
        : !expectedBlocked && markerPresent === false
          ? 'allowed-navigation-did-not-render-block-page'
          : 'unexpected-page-state',
    screenshotPath,
  };
}

async function runFirefoxInterventionProof(browser) {
  const profileRun = await launchFirefoxProfile(browser);
  try {
    const bidi = await FirefoxBidiClient.connect(`ws://127.0.0.1:${profileRun.port}/session`, timeoutMs);
    const interventions = [];
    try {
      await bidi.command('session.new', { capabilities: {} });
      const created = await bidi.command('browsingContext.create', { type: 'tab' });
      const context = created.context;
      bidi.onEvent((event) => {
        if (event.method === 'network.beforeRequestSent' && event.params.isBlocked === true) {
          void handleFirefoxBeforeRequest(bidi, event, interventions);
        }
      });
      await bidi.command('session.subscribe', {
        events: ['network.beforeRequestSent', 'browsingContext.load'],
        contexts: [context],
      });
      await bidi.command('network.addIntercept', {
        phases: ['beforeRequestSent'],
        contexts: [context],
        urlPatterns: [
          { type: 'pattern', protocol: 'https', hostname: 'example.com' },
          { type: 'pattern', protocol: 'https', hostname: 'www.youtube.com', pathname: '/watch' },
        ],
      });

      const cases = [];
      for (const rule of ruleSet) {
        cases.push(await observeFirefoxNavigation(browser, bidi, context, rule.url, true, rule.id));
      }
      cases.push(await observeFirefoxNavigation(browser, bidi, context, allowedUrl, false, 'allowed-control'));

      return {
        browser,
        profileName: 'managed-browser-intervention-proof',
        profilePathContainsManagedPrefix: profileRun.profileDir.includes('managed-browser-intervention'),
        bridge: browser.bridge,
        devtoolsEndpoint: 'loopback-redacted',
        interventions,
        cases,
        assertions: assertionsForCases(cases),
      };
    } finally {
      bidi.close();
    }
  } finally {
    await cleanupProfileRun(profileRun);
  }
}

async function handleFirefoxBeforeRequest(bidi, event, interventions) {
  const requestUrl = event.params.request.url;
  const rule = ruleForUrl(requestUrl);
  if (rule === null) {
    await bidi.command('network.continueRequest', { request: event.params.request.request });
    return;
  }
  const html = blockPageHtml(rule, requestUrl, 'webdriver-bidi');
  interventions.push({
    bridge: 'webdriver-bidi',
    method: 'network.provideResponse',
    ruleId: rule.id,
    requestedUrl: requestUrl,
    observedAt: new Date().toISOString(),
  });
  await bidi.command('network.provideResponse', {
    request: event.params.request.request,
    statusCode: 451,
    reasonPhrase: 'Blocked by Ocentra Managed Browser',
    headers: [
      { name: 'content-type', value: { type: 'string', value: 'text/html; charset=utf-8' } },
      { name: 'cache-control', value: { type: 'string', value: 'no-store' } },
    ],
    body: { type: 'string', value: html },
  });
}

async function observeFirefoxNavigation(browser, bidi, context, url, expectedBlocked, ruleId) {
  await bidi.command('browsingContext.navigate', { context, url, wait: 'complete' }).catch(() => ({}));
  await delay(750);
  const markerPresent = await evaluateFirefoxValue(
    bidi,
    context,
    `document.body.textContent.includes(${JSON.stringify(blockMarker)})`
  );
  const title = await evaluateFirefoxValue(bidi, context, 'document.title');
  const locationHref = await evaluateFirefoxValue(bidi, context, 'location.href');
  const screenshotPath = await captureFirefoxScreenshot(browser, bidi, context, ruleId);
  return {
    ruleId,
    requestedUrl: url,
    observedUrl: locationHref,
    title,
    expectedBlocked,
    blockMarkerPresent: markerPresent === true,
    interventionProof:
      expectedBlocked && markerPresent === true
        ? 'blocked-page-rendered-before-target-document'
        : !expectedBlocked && markerPresent === false
          ? 'allowed-navigation-did-not-render-block-page'
          : 'unexpected-page-state',
    screenshotPath,
  };
}

async function launchChromiumProfile(browser) {
  const port = await freePort();
  const profileDir = join(probeRoot, browser.id, 'managed-browser-intervention-proof');
  await mkdir(profileDir, { recursive: true });
  const child = spawn(
    browser.executablePath,
    [
      '--remote-debugging-address=127.0.0.1',
      `--remote-debugging-port=${port}`,
      `--user-data-dir=${profileDir}`,
      '--profile-directory=OcentraManagedIntervention',
      '--no-first-run',
      '--no-default-browser-check',
      '--new-window',
      'about:blank',
    ],
    { stdio: 'ignore', windowsHide: true }
  );
  return { child, port, profileDir };
}

async function launchFirefoxProfile(browser) {
  const port = await freePort();
  const profileDir = join(probeRoot, browser.id, 'managed-browser-intervention-proof');
  await mkdir(profileDir, { recursive: true });
  const child = spawn(
    browser.executablePath,
    ['--no-remote', '--new-instance', '-profile', profileDir, '--remote-debugging-port', String(port), 'about:blank'],
    { stdio: 'ignore', windowsHide: true }
  );
  return { child, port, profileDir };
}

function blockPageHtml(rule, requestedUrl, bridge) {
  return `<!doctype html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Ocentra managed browser blocked</title>
  </head>
  <body>
    <main>
      <h1>${blockMarker}</h1>
      <p>Rule: ${escapeHtml(rule.id)}</p>
      <p>Label: ${escapeHtml(rule.label)}</p>
      <p>Bridge: ${escapeHtml(bridge)}</p>
      <p>Blocked URL: ${escapeHtml(requestedUrl)}</p>
    </main>
  </body>
</html>`;
}

function ruleForUrl(url) {
  if (!capturedUrl(url)) {
    return null;
  }
  return ruleSet.find((rule) => rule.match(url)) ?? null;
}

function assertionsForCases(cases) {
  return {
    blockedSiteBlocked: casePassed(cases, 'blocked-site-host', true),
    youtubeVideoBlocked: casePassed(cases, 'blocked-youtube-video-url', true),
    allowedControlNotBlocked: casePassed(cases, 'allowed-control', false),
  };
}

function casePassed(cases, ruleId, expectedBlocked) {
  const item = cases.find((testCase) => testCase.ruleId === ruleId);
  return item !== undefined && item.blockMarkerPresent === expectedBlocked;
}

function summarize(results) {
  const failures = [];
  for (const result of results) {
    if (!result.assertions.blockedSiteBlocked) {
      failures.push(`${result.browser.id}: blocked site did not render managed block page`);
    }
    if (!result.assertions.youtubeVideoBlocked) {
      failures.push(`${result.browser.id}: YouTube video URL did not render managed block page`);
    }
    if (!result.assertions.allowedControlNotBlocked) {
      failures.push(`${result.browser.id}: allowed control rendered block page`);
    }
  }
  return {
    supportedBrowserCount: results.length,
    blockedSiteProofs: results.filter((result) => result.assertions.blockedSiteBlocked).length,
    youtubeVideoProofs: results.filter((result) => result.assertions.youtubeVideoBlocked).length,
    allowedControlProofs: results.filter((result) => result.assertions.allowedControlNotBlocked).length,
    failures,
  };
}

function printSummary(evidence, evidencePath) {
  console.log(`managed-browser-intervention-proof-ok=${evidence.summary.failures.length === 0}`);
  console.log(`evidence=${evidencePath}`);
  console.log(
    `supportedBrowsers=${evidence.summary.supportedBrowserCount} blockedSiteProofs=${evidence.summary.blockedSiteProofs} youtubeVideoProofs=${evidence.summary.youtubeVideoProofs} allowedControlProofs=${evidence.summary.allowedControlProofs}`
  );
  for (const browser of evidence.browsers) {
    console.log(`${browser.browser.id} ${browser.browser.bridge} ${browser.browser.executablePath}`);
    for (const testCase of browser.cases) {
      console.log(
        `  ${testCase.ruleId}: requested=${testCase.requestedUrl} observed=${testCase.observedUrl} proof=${testCase.interventionProof} screenshot=${testCase.screenshotPath}`
      );
    }
  }
  for (const failure of evidence.summary.failures) {
    console.error(`failure=${failure}`);
  }
}

async function installedDirectBrowsers() {
  const candidates = browserCandidates();
  const installed = [];
  const seen = new Set();
  for (const candidate of candidates) {
    const key = `${candidate.id}:${candidate.executablePath.toLowerCase()}`;
    if (!seen.has(key) && (await fileExists(candidate.executablePath))) {
      installed.push(candidate);
      seen.add(key);
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
      chromiumCandidate('chrome-stable', 'chrome', join(root, 'Google', 'Chrome', 'Application', 'chrome.exe')),
      chromiumCandidate('edge-stable', 'edge', join(root, 'Microsoft', 'Edge', 'Application', 'msedge.exe')),
      firefoxCandidate('firefox-stable', join(root, 'Mozilla Firefox', 'firefox.exe')),
    ]),
  ];
}

function chromiumCandidate(id, family, executablePath) {
  return {
    id,
    family,
    channel: 'stable',
    executablePath,
    bridge: 'chromium-devtools-protocol',
  };
}

function firefoxCandidate(id, executablePath) {
  return {
    id,
    family: 'firefox',
    channel: 'stable',
    executablePath,
    bridge: 'webdriver-bidi',
  };
}

function windowsRoots() {
  return [process.env.ProgramFiles, process.env['ProgramFiles(x86)'], process.env.LOCALAPPDATA].filter(Boolean);
}

async function cleanupProfileRun(profileRun) {
  profileRun.child.kill();
  await new Promise((resolve) => {
    profileRun.child.once('exit', resolve);
    setTimeout(resolve, 3000).unref();
  });
}

async function waitForFirstPageTarget(port) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const targets = await waitForJson(port, '/json/list');
    const pageTarget = targets.find((target) => target.type === 'page' && target.webSocketDebuggerUrl !== undefined);
    if (pageTarget !== undefined) {
      return pageTarget;
    }
    await delay(250);
  }
  throw new Error(`Timed out waiting for page target on ${port}`);
}

async function waitForChromiumReady(client) {
  const deadline = Date.now() + readyTimeoutMs;
  while (Date.now() < deadline) {
    const state = await evaluateChromiumValue(client, 'document.readyState').catch(() => '');
    if (state === 'complete') {
      return;
    }
    await delay(250);
  }
}

async function evaluateChromiumValue(client, expression) {
  const result = await client.command('Runtime.evaluate', {
    expression,
    returnByValue: true,
  });
  return result.result?.value;
}

async function evaluateFirefoxValue(bidi, context, expression) {
  const result = await bidi.command('script.evaluate', {
    expression,
    target: { context },
    awaitPromise: false,
    resultOwnership: 'none',
  });
  return result.result?.value;
}

async function captureChromiumScreenshot(browser, client, ruleId) {
  const screenshot = await client.command('Page.captureScreenshot', { format: 'png' });
  const path = join(screenshotDirectory, `${safeFileName(`${browser.id}-${ruleId}`)}.png`);
  await writeFile(path, Buffer.from(screenshot.data, 'base64'));
  return path;
}

async function captureFirefoxScreenshot(browser, bidi, context, ruleId) {
  const screenshot = await bidi.command('browsingContext.captureScreenshot', { context });
  const path = join(screenshotDirectory, `${safeFileName(`${browser.id}-${ruleId}`)}.png`);
  await writeFile(path, Buffer.from(screenshot.data, 'base64'));
  return path;
}

function safeFileName(value) {
  return value.replace(/[^a-zA-Z0-9_.-]/g, '-');
}

function hostMatches(expectedHost, observedUrl) {
  if (!capturedUrl(observedUrl)) {
    return false;
  }
  const observed = new URL(observedUrl);
  return observed.hostname.toLowerCase() === expectedHost.toLowerCase();
}

function capturedUrl(value) {
  return value.startsWith('http://') || value.startsWith('https://');
}

function escapeHtml(value) {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
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

function envNumber(name, fallback) {
  const parsed = Number(process.env[name]);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback;
}

async function waitForJson(port, path) {
  const deadline = Date.now() + timeoutMs;
  let lastError;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(`http://127.0.0.1:${port}${path}`);
      if (response.ok) {
        return await response.json();
      }
    } catch (error) {
      lastError = error;
    }
    await delay(250);
  }
  throw lastError ?? new Error(`Timed out waiting for ${path}`);
}

class DevToolsClient {
  static async connect(url) {
    const webSocket = await connectWebSocket(url, timeoutMs);
    return new DevToolsClient(webSocket);
  }

  constructor(webSocket) {
    this.webSocket = webSocket;
    this.commandId = 0;
    this.pending = new Map();
    this.eventHandlers = [];
    this.webSocket.addEventListener('message', (message) => this.handleMessage(message));
  }

  command(method, params) {
    const id = ++this.commandId;
    const promise = new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      setTimeout(() => {
        if (this.pending.delete(id)) {
          reject(new Error(`Timed out waiting for ${method}`));
        }
      }, commandTimeoutMs).unref();
    });
    this.webSocket.send(JSON.stringify({ id, method, params }));
    return promise;
  }

  onEvent(handler) {
    this.eventHandlers.push(handler);
  }

  close() {
    this.webSocket.close();
  }

  handleMessage(message) {
    const data = JSON.parse(message.data);
    if (data.id !== undefined) {
      const pending = this.pending.get(data.id);
      if (pending === undefined) {
        return;
      }
      this.pending.delete(data.id);
      if (data.error !== undefined) {
        pending.reject(new Error(JSON.stringify(data.error)));
      } else {
        pending.resolve(data.result ?? {});
      }
      return;
    }
    for (const handler of this.eventHandlers) {
      handler(data);
    }
  }
}

class FirefoxBidiClient {
  static async connect(url, timeout) {
    const webSocket = await connectWebSocket(url, timeout);
    return new FirefoxBidiClient(webSocket);
  }

  constructor(webSocket) {
    this.webSocket = webSocket;
    this.commandId = 0;
    this.pending = new Map();
    this.eventHandlers = [];
    this.webSocket.addEventListener('message', (message) => this.handleMessage(message));
  }

  command(method, params) {
    const id = ++this.commandId;
    const promise = new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      setTimeout(() => {
        if (this.pending.delete(id)) {
          reject(new Error(`Timed out waiting for ${method}`));
        }
      }, commandTimeoutMs).unref();
    });
    this.webSocket.send(JSON.stringify({ id, method, params }));
    return promise;
  }

  onEvent(handler) {
    this.eventHandlers.push(handler);
  }

  close() {
    this.webSocket.close();
  }

  handleMessage(message) {
    const data = JSON.parse(message.data);
    if (data.type === 'event') {
      for (const handler of this.eventHandlers) {
        handler(data);
      }
      return;
    }
    const pending = this.pending.get(data.id);
    if (pending === undefined) {
      return;
    }
    this.pending.delete(data.id);
    if (data.type === 'success') {
      pending.resolve(data.result ?? {});
    } else {
      pending.reject(new Error(JSON.stringify(data)));
    }
  }
}

async function connectWebSocket(url, timeout) {
  const deadline = Date.now() + timeout;
  let lastError;
  while (Date.now() < deadline) {
    try {
      return await openWebSocket(url);
    } catch (error) {
      lastError = error;
      await delay(250);
    }
  }
  throw lastError;
}

function openWebSocket(url) {
  return new Promise((resolve, reject) => {
    const webSocket = new WebSocket(url);
    const timer = setTimeout(() => {
      webSocket.close();
      reject(new Error(`Timed out connecting to ${url}`));
    }, 3000);
    webSocket.addEventListener(
      'open',
      () => {
        clearTimeout(timer);
        resolve(webSocket);
      },
      { once: true }
    );
    webSocket.addEventListener(
      'error',
      () => {
        clearTimeout(timer);
        reject(new Error(`WebSocket error for ${url}`));
      },
      { once: true }
    );
  });
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
