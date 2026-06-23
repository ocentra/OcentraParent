import { strict as assert } from 'node:assert';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));
const outputDir = join(repoRoot, 'test-results', 'screen-child-disclosure-proof');
const outputPath = join(outputDir, 'proof.json');
const planOutputDir = join(repoRoot, 'output', 'screen-plan-proof', 'screen-child-disclosure');
const planOutputPath = join(planOutputDir, 'proof-summary.json');
const screenshotDir = join(planOutputDir, 'screenshots');
const renderedHtmlPath = join(outputDir, 'screen-child-disclosure-page.html');
const desktopScreenshotPath = join(screenshotDir, 'screen-child-disclosure-desktop.png');
const mobileScreenshotPath = join(screenshotDir, 'screen-child-disclosure-mobile.png');

run('npx', [
  'vitest',
  'run',
  'packages/screen-domain/tests/unit/screen-child-disclosure.test.ts',
  'packages/screen-domain/tests/unit/screen-child-disclosure-page.test.ts',
]);
run('npm', ['run', 'build', '--workspace=@ocentra-parent/screen-domain']);

const { ScreenChildDisclosureSnapshotSchema, screenChildDisclosureProofSnapshots } =
  await import('@ocentra-parent/schema-domain/screen-child-disclosure');
const { createScreenChildDisclosurePageModel, renderScreenChildDisclosurePage } =
  await import('../../packages/screen-domain/dist/screen-child-disclosure-page.js');
const snapshots = screenChildDisclosureProofSnapshots();
const parsed = snapshots.map((snapshot) => ScreenChildDisclosureSnapshotSchema.parse(snapshot));
const pageModel = createScreenChildDisclosurePageModel(parsed);
const renderedHtml = renderScreenChildDisclosurePage(pageModel);

assert.deepEqual(
  parsed.map((snapshot) => snapshot.state),
  ['disabledByParent', 'pausedByParent', 'captureActive', 'protectedSurface', 'deletedSummaryReady']
);
assert.equal(parsed[2].surface, 'child-agent-capture-banner');
assert.equal(parsed[2].captureActive, true);
assert.equal(parsed[4].deletionState, 'deleted');
assert.equal(
  parsed.every((snapshot) => snapshot.visibleToChildRequired),
  true
);
assert.equal(
  parsed.every((snapshot) => !snapshot.hiddenCaptureClaimed),
  true
);
assert.equal(
  parsed.every((snapshot) => !snapshot.rawScreenshotShownToChild),
  true
);
assert.equal(pageModel.rawScreenshotRendered, false);
assert.equal(pageModel.hiddenCaptureClaimed, false);
assert.equal(pageModel.renderedChildAgentDeliveryClaimed, false);
assert.match(renderedHtml, /data-ocentra-screen-disclosure-state="captureActive"/u);

mkdirSync(outputDir, { recursive: true });
mkdirSync(screenshotDir, { recursive: true });
writeFileSync(renderedHtmlPath, renderedHtml);
await captureScreenshots(renderedHtmlPath);

const proof = {
  proofId: 'screen-child-disclosure-proof',
  generatedAt: '2026-06-06T21:55:00Z',
  source: '@ocentra-parent/screen-domain screen child disclosure contracts',
  assertions: [
    'child-visible screen disclosure states are schema-backed',
    'disabled-by-parent state cannot claim cadence, trigger, or active capture',
    'active capture requires ready capability, approved non-unsupported scope, and child-agent capture banner surface',
    'protected-surface state stays visible without queued raw image custody',
    'deleted-summary-ready state requires deleted local custody before summary display',
    'hidden capture, raw screenshot display, remote viewer, and policy-authority claims are rejected',
    'rendered child disclosure screenshots exist for desktop and mobile viewport inspection',
    'child-agent deployment/delivery remains unclaimed until a real child runtime surface serves the page',
  ],
  parsed: {
    states: parsed.map((snapshot) => snapshot.state),
    activeSurface: parsed[2].surface,
    summaryCustody: parsed[4].custodyState,
    rawScreenshotRendered: pageModel.rawScreenshotRendered,
    hiddenCaptureClaimed: pageModel.hiddenCaptureClaimed,
    renderedChildAgentDeliveryClaimed: pageModel.renderedChildAgentDeliveryClaimed,
  },
  screenshots: {
    desktop: 'output/screen-plan-proof/screen-child-disclosure/screenshots/screen-child-disclosure-desktop.png',
    mobile: 'output/screen-plan-proof/screen-child-disclosure/screenshots/screen-child-disclosure-mobile.png',
    html: 'test-results/screen-child-disclosure-proof/screen-child-disclosure-page.html',
  },
};

mkdirSync(planOutputDir, { recursive: true });
writeFileSync(outputPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(planOutputPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(`screen-child-disclosure-proof-ok: ${outputPath}`);

async function captureScreenshots(htmlPath) {
  const { chromium } = await import('playwright');
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    await page.goto(pathToFileURL(htmlPath).href);
    await page.screenshot({ fullPage: true, path: desktopScreenshotPath });
    await page.setViewportSize({ width: 390, height: 844 });
    await page.screenshot({ fullPage: true, path: mobileScreenshotPath });
  } finally {
    await browser.close();
  }
}

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    shell: process.platform === 'win32',
    stdio: 'inherit',
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${result.status}`);
  }
}
