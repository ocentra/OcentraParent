import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { chromium } from 'playwright';

const outputDir = join('output', 'screen-plan-proof', 'real-capture', 'trigger-matrix');
const fixtureTitle = 'Ocentra Screen Trigger Matrix Proof';

rmSync(outputDir, { recursive: true, force: true });
mkdirSync(outputDir, { recursive: true });

const fixturePath = writeFixture();
let browser;

try {
  browser = await chromium.launch({
    headless: false,
    args: ['--window-size=920,620', '--window-position=120,120'],
  });
  const page = await browser.newPage({ viewport: { width: 920, height: 620 } });
  await page.goto(pathToFileURL(resolve(fixturePath)).href);
  await page.bringToFront();
  await page.waitForTimeout(1000);

  const browserUse = runCaptureScenario('browser-use-active-window', {
    requestedTrigger: 'managed_browser_url_change',
    productTriggerWired: false,
    proofHarnessTrigger: 'headed-playwright-window-title-match',
    targetTitle: fixtureTitle,
  });

  await page.locator('#state').evaluate((node) => {
    node.textContent = 'Timed cadence frame 1';
  });
  await page.waitForTimeout(750);

  const timedFirst = runCaptureScenario('timed-cadence-frame-1', {
    requestedTrigger: 'timed_cadence',
    productTriggerWired: false,
    proofHarnessTrigger: 'bounded-two-frame-cadence-proof',
    targetTitle: fixtureTitle,
  });

  await page.locator('#state').evaluate((node) => {
    node.textContent = 'Timed cadence frame 2';
  });
  await page.waitForTimeout(1250);

  const timedSecond = runCaptureScenario('timed-cadence-frame-2', {
    requestedTrigger: 'timed_cadence',
    productTriggerWired: false,
    proofHarnessTrigger: 'bounded-two-frame-cadence-proof',
    targetTitle: fixtureTitle,
  });

  const scenarios = [browserUse, timedFirst, timedSecond];
  const imageDigests = scenarios.map((scenario) => scenario.imageDigest).filter((imageDigest) => imageDigest !== null);
  const summary = {
    proof: 'screen-capture-trigger-matrix-proof',
    outputDir,
    platform: process.platform,
    realCaptureRuns: scenarios.length,
    capturedRuns: scenarios.filter((scenario) => scenario.captured).length,
    allRawImagesDeleted: scenarios.every((scenario) => scenario.rawImageDeleted),
    distinctCapturedFrames: new Set(imageDigests).size === imageDigests.length,
    selectedWindowScopeMatched: scenarios.every((scenario) => scenario.actualScope === 'selectedWindow'),
    productSchedulerImplemented: false,
    productForegroundTriggerImplemented: false,
    degradedIsCaptureProof: false,
    scenarios,
    nonClaims: [
      'This proof fires the real capture adapter from a harness; it does not claim service scheduler wiring.',
      'This proof uses a controlled headed browser window; it does not claim managed browser-plan URL integration.',
    ],
  };
  writeJson(join(outputDir, 'proof-summary.json'), summary);
  if (
    process.platform === 'win32' &&
    (summary.capturedRuns !== scenarios.length ||
      !summary.allRawImagesDeleted ||
      !summary.distinctCapturedFrames ||
      !summary.selectedWindowScopeMatched)
  ) {
    throw new Error(`Windows trigger matrix proof incomplete: ${JSON.stringify(summary, null, 2)}`);
  }
  console.log(JSON.stringify(summary, null, 2));
} finally {
  if (browser !== undefined) {
    await browser.close();
  }
}

function runCaptureScenario(scenarioId, options) {
  const scenarioDir = join(outputDir, scenarioId);
  mkdirSync(scenarioDir, { recursive: true });
  writeJson(join(scenarioDir, '00-trigger-request.json'), {
    scenarioId,
    requestedTrigger: options.requestedTrigger,
    productTriggerWired: options.productTriggerWired,
    proofHarnessTrigger: options.proofHarnessTrigger,
    targetTitle: options.targetTitle,
    expectedScope: 'selectedWindow',
  });
  const result = spawnSync(
    'cargo',
    ['run', '-p', 'ocentra-parent-screen-capture-adapter', '--example', 'screen_capture_real_proof', '--', scenarioDir],
    {
      cwd: process.cwd(),
      encoding: 'utf8',
      shell: process.platform === 'win32',
      env: {
        ...process.env,
        OCENTRA_SCREEN_CAPTURE_WINDOW_TITLE_CONTAINS: options.targetTitle,
      },
    }
  );
  writeFileSync(join(scenarioDir, 'cargo-stdout.log'), result.stdout ?? '');
  writeFileSync(join(scenarioDir, 'cargo-stderr.log'), result.stderr ?? '');
  if (result.status !== 0) {
    throw new Error(`capture scenario ${scenarioId} failed with status ${result.status}`);
  }
  const metadata = readJson(join(scenarioDir, '02-capture-metadata.json'));
  const deletion = metadata.captured ? readJson(join(scenarioDir, '04-deletion-proof.json')) : null;
  const scenarioSummary = {
    scenarioId,
    requestedTrigger: options.requestedTrigger,
    productTriggerWired: options.productTriggerWired,
    proofHarnessTrigger: options.proofHarnessTrigger,
    captured: metadata.captured === true,
    status: metadata.status,
    actualScope: metadata.actualScope ?? null,
    imageDigest: metadata.imageDigest ?? null,
    imageByteSize: metadata.imageByteSize ?? null,
    rawImageDeleted: deletion?.rawImageDeleted === true,
    encryptedQueueOmitsRawDigest: deletion?.encryptedQueueContainsRawDigest === false,
  };
  writeJson(join(scenarioDir, '06-scenario-summary.json'), scenarioSummary);
  return scenarioSummary;
}

function writeFixture() {
  const fixturePath = join(outputDir, 'controlled-trigger-fixture.html');
  writeFileSync(
    fixturePath,
    `<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>${fixtureTitle}</title>
    <style>
      body { margin: 0; font-family: Arial, sans-serif; background: #07101c; color: #e6fbff; }
      main { min-height: 100vh; display: grid; place-items: center; }
      section { border: 6px solid #38bdf8; padding: 42px; width: 720px; background: #0f172a; }
      h1 { font-size: 46px; margin: 0 0 20px; }
      p { font-size: 32px; margin: 12px 0; }
    </style>
  </head>
  <body>
    <main>
      <section>
        <h1>Screen Trigger Proof</h1>
        <p id="state">Browser use active window</p>
      </section>
    </main>
  </body>
</html>
`
  );
  return fixturePath;
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}
