import { spawn } from 'node:child_process';
import net from 'node:net';

export const expectedTrackingStates = [
  'Tracking off|Permission required|Stale last known|Offline last known|Low accuracy|Nearby place ambiguous',
  'Policy alert|Parent acknowledged|Exception active|Child check-in|Temporary live|Missing device|Retention deleted',
]
  .join('|')
  .split('|');

export async function runHostedTrackingUiBrowserProof({ route, screenshotPath }) {
  const { chromium } = await import('@playwright/test');
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1440, height: 1100 } });
  try {
    await page.goto(route, { waitUntil: 'domcontentloaded', timeout: 15_000 });
    await page.waitForSelector('.tracking-status-overlay-grid', { timeout: 20_000 });
    await page.waitForTimeout(750);

    const overlay = page.locator('.tracking-status-overlay');
    const overlayText = await overlay.innerText();
    assertTextIncludesAll(overlayText, expectedTrackingStates, 'tracking state');
    assertTextIncludesAll(
      overlayText,
      [
        'Service read model',
        'Generated at',
        'Retention tombstones',
        'Activity kind',
        'Subject',
        'Subject kind',
        'Subject ID',
        'Device',
        'Platform',
        'Observer',
        'Latest row evidence references',
        'Evidence references',
        'Runtime reference',
        'Missing proof',
        'Product claim',
      ],
      'tracking detail label'
    );
    assertSafetyCopy(overlayText);

    const proof = await collectAccessibilityProof(page, overlay);
    await page.screenshot({ path: screenshotPath, fullPage: true });
    return {
      route,
      viewport: '1440x1100',
      expectedStates: expectedTrackingStates,
      accessibility: proof,
      deletedEvidenceRendered: false,
    };
  } finally {
    await browser.close();
  }
}

export function startPortalServer({ repoRoot, host, port }) {
  const args = [
    'exec',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'vite',
    '--host',
    host,
    '--port',
    String(port),
    '--strictPort',
  ];
  const options = {
    cwd: repoRoot,
    env: { ...process.env, VITE_AGENT_WS_URL: process.env.VITE_AGENT_WS_URL ?? `ws://${host}:4577/api/dev/ws` },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  };
  if (process.platform === 'win32') {
    return {
      commandLine: ['npm', ...args].join(' '),
      child: spawn('cmd', ['/c', 'npm', ...args], options),
    };
  }
  return {
    commandLine: ['npm', ...args].join(' '),
    child: spawn('npm', args, options),
  };
}

export async function waitForHttp(url, timeoutMs) {
  const deadline = Date.now() + timeoutMs;
  let lastError;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return;
      }
      lastError = new Error(`${url} returned ${response.status}`);
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 300));
  }
  throw lastError ?? new Error(`${url} did not respond before timeout`);
}

export async function availablePort(preferredPort) {
  if (Number.isInteger(preferredPort) && preferredPort > 0 && (await canListen(preferredPort))) {
    return preferredPort;
  }
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.once('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const address = server.address();
      server.close(() => {
        if (typeof address === 'object' && address !== null) {
          resolve(address.port);
          return;
        }
        reject(new Error('Unable to allocate an available port'));
      });
    });
  });
}

export async function stopProcessTree({ repoRoot, child }) {
  if (child.exitCode !== null) {
    return;
  }
  if (process.platform === 'win32' && child.pid !== undefined) {
    await new Promise((resolve) => {
      const killer = spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], {
        cwd: repoRoot,
        stdio: 'ignore',
        windowsHide: true,
      });
      killer.once('exit', () => resolve());
      killer.once('error', () => resolve());
    });
    return;
  }
  child.kill('SIGTERM');
}

export function sanitizeServerOutput(value) {
  return value.replace(/\u001b\[[0-9;]*m/gu, '').replace(/[^\x09\x0a\x0d\x20-\x7e]/gu, '');
}

async function collectAccessibilityProof(page, overlay) {
  const cardTexts = await page
    .locator('.tracking-status-overlay-grid > article')
    .evaluateAll((nodes) => nodes.map((node) => node.textContent ?? ''));
  const cardBoxes = await page.locator('.tracking-status-overlay-grid > article').evaluateAll((nodes) =>
    nodes.map((node) => {
      const rect = node.getBoundingClientRect();
      return {
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
        left: rect.left,
        width: rect.width,
        height: rect.height,
      };
    })
  );
  const cardCount = cardTexts.length;
  const proof = {
    sectionLabel: await overlay.getAttribute('aria-label'),
    firstHeading: await page
      .getByRole('heading', { name: /Tracking status/iu })
      .first()
      .innerText(),
    buttonName: await page.getByRole('button', { name: /tracking status/iu }).innerText(),
    cardCount,
    headingCount: await page.locator('.tracking-status-overlay-grid > article h2').count(),
    definitionListCount: await page.locator('.tracking-status-overlay-grid > article dl').count(),
    productClaimCount: cardTexts.filter((text) => text.includes('No product claim')).length,
    visibleCardCount: cardBoxes.filter((box) => box.width > 0 && box.height > 0).length,
    overlappingCards: countOverlappingBoxes(cardBoxes),
  };
  const completeProof = {
    ...proof,
    allCardsHaveHeadings: proof.headingCount === cardCount,
    allCardsHaveDefinitionLists: proof.definitionListCount === cardCount,
    allCardsHaveNoClaimCopy: proof.productClaimCount === cardCount,
    noVisibleOverlap: proof.overlappingCards === 0,
  };
  assertAccessibility(completeProof);
  return completeProof;
}

function assertSafetyCopy(overlayText) {
  if (!overlayText.includes('No product claim')) {
    throw new Error('Hosted UI proof must keep no-product-claim copy visible.');
  }
  if (!overlayText.includes('Deleted history hidden') || !overlayText.includes('Deleted evidence not rendered')) {
    throw new Error('Hosted UI proof must keep retention-deleted safety copy visible.');
  }
  if (overlayText.includes('location-evidence-1')) {
    throw new Error('Hosted UI proof rendered deleted evidence id.');
  }
}

function assertAccessibility(proof) {
  const failures = [
    ['tracking section has an accessible label', proof.sectionLabel !== null && proof.sectionLabel.length > 0],
    ['route has a tracking heading', proof.firstHeading.length > 0],
    ['tracking refresh button has accessible text', proof.buttonName.length > 0],
    ['all cards are visible', proof.visibleCardCount === proof.cardCount],
    ['all cards have headings', proof.allCardsHaveHeadings],
    ['all cards have definition lists', proof.allCardsHaveDefinitionLists],
    ['all cards carry no-product-claim copy', proof.allCardsHaveNoClaimCopy],
    ['cards do not visibly overlap', proof.noVisibleOverlap],
  ].filter(([, passed]) => !passed);
  if (failures.length > 0) {
    throw new Error(`Hosted UI accessibility proof failed: ${failures.map(([label]) => label).join(', ')}`);
  }
}

function assertTextIncludesAll(text, expectedValues, label) {
  const normalizedText = text.toLocaleLowerCase('en-US');
  const missing = expectedValues.find((value) => !normalizedText.includes(value.toLocaleLowerCase('en-US')));
  if (missing !== undefined) {
    throw new Error(`Hosted UI proof missing ${label}: ${missing}. Rendered text: ${text.slice(0, 600)}`);
  }
}

function countOverlappingBoxes(boxes) {
  let overlaps = 0;
  for (let outerIndex = 0; outerIndex < boxes.length; outerIndex += 1) {
    for (let innerIndex = outerIndex + 1; innerIndex < boxes.length; innerIndex += 1) {
      if (boxesOverlap(boxes[outerIndex], boxes[innerIndex])) {
        overlaps += 1;
      }
    }
  }
  return overlaps;
}

function boxesOverlap(first, second) {
  return !(
    first.right <= second.left + 1 ||
    second.right <= first.left + 1 ||
    first.bottom <= second.top + 1 ||
    second.bottom <= first.top + 1
  );
}

async function canListen(port) {
  return new Promise((resolve) => {
    const server = net.createServer();
    server.once('error', () => resolve(false));
    server.listen(port, '127.0.0.1', () => {
      server.close(() => resolve(true));
    });
  });
}
