import type {
  ScreenChildDisclosureSnapshot,
  ScreenChildDisclosureState,
} from '@ocentra-parent/schema-domain/screen-child-disclosure';

export type ScreenChildDisclosurePageModel = {
  readonly title: string;
  readonly summary: string;
  readonly snapshots: readonly ScreenChildDisclosureSnapshot[];
  readonly renderedChildAgentDeliveryClaimed: false;
  readonly rawScreenshotRendered: false;
  readonly hiddenCaptureClaimed: false;
};

const PageTitle = 'Screen activity is visible on this device';
const PageSummary =
  'Ocentra shows when screen activity is disabled, paused, protected, active, analyzing, or summarized. Raw screenshots are not shown here.';

export function createScreenChildDisclosurePageModel(
  snapshots: readonly ScreenChildDisclosureSnapshot[]
): ScreenChildDisclosurePageModel {
  return {
    title: PageTitle,
    summary: PageSummary,
    snapshots,
    renderedChildAgentDeliveryClaimed: false,
    rawScreenshotRendered: false,
    hiddenCaptureClaimed: false,
  };
}

export function renderScreenChildDisclosurePage(model: ScreenChildDisclosurePageModel): string {
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="color-scheme" content="dark light" />
    <title>${escapeHtml(model.title)}</title>
    <style>${screenChildDisclosurePageStyle()}</style>
  </head>
  <body>
    <main class="screen-child-page" aria-labelledby="screen-child-title">
      <section class="screen-child-hero">
        <p class="screen-child-kicker">Child device disclosure</p>
        <h1 id="screen-child-title">${escapeHtml(model.title)}</h1>
        <p>${escapeHtml(model.summary)}</p>
      </section>
      <section class="screen-child-grid" aria-label="Screen disclosure states">
        ${model.snapshots.map(renderSnapshotCard).join('')}
      </section>
      <section class="screen-child-boundary" aria-label="Proof boundary">
        <dl>
          ${detailRow('Raw screenshot shown', String(model.rawScreenshotRendered))}
          ${detailRow('Hidden capture claimed', String(model.hiddenCaptureClaimed))}
          ${detailRow('Child-agent delivery claimed', String(model.renderedChildAgentDeliveryClaimed))}
        </dl>
      </section>
    </main>
  </body>
</html>`;
}

function renderSnapshotCard(snapshot: ScreenChildDisclosureSnapshot): string {
  return `<article class="screen-child-card" data-ocentra-screen-disclosure-state="${escapeAttribute(snapshot.state)}">
    <p class="screen-child-state">${escapeHtml(copyForState(snapshot.state).eyebrow)}</p>
    <h2>${escapeHtml(copyForState(snapshot.state).title)}</h2>
    <p>${escapeHtml(copyForState(snapshot.state).body)}</p>
    <dl>
      ${detailRow('State', snapshot.state)}
      ${detailRow('Surface', snapshot.surface)}
      ${detailRow('Capability', snapshot.capabilityStatus)}
      ${detailRow('Capture active', String(snapshot.captureActive))}
      ${detailRow('Queue', snapshot.queueStatus ?? 'none')}
      ${detailRow('Deletion', snapshot.deletionState ?? 'none')}
      ${detailRow('Custody', snapshot.custodyState)}
    </dl>
  </article>`;
}

function copyForState(state: ScreenChildDisclosureState): {
  readonly body: string;
  readonly eyebrow: string;
  readonly title: string;
} {
  switch (state) {
    case 'disabledByParent':
      return {
        body: 'Screen activity analysis is turned off by your parent, so this device is not capturing screen evidence.',
        eyebrow: 'Off',
        title: 'Screen activity is off',
      };
    case 'pausedByParent':
      return {
        body: 'Screen activity analysis is paused. Ocentra will show this status before it resumes.',
        eyebrow: 'Paused',
        title: 'Screen activity is paused',
      };
    case 'captureActive':
      return {
        body: 'A screen check is running now. This notice is visible while the local device captures allowed screen evidence.',
        eyebrow: 'Visible check',
        title: 'Screen activity is being checked',
      };
    case 'localAnalysisRunning':
      return {
        body: 'Local analysis is reading screen evidence on this device without showing raw screenshots.',
        eyebrow: 'Local analysis',
        title: 'Ocentra is analyzing activity',
      };
    case 'deletedSummaryReady':
      return {
        body: 'The raw image has been deleted. Only the local summary and evidence references remain.',
        eyebrow: 'Summary ready',
        title: 'A screen activity summary is ready',
      };
    case 'permissionRequired':
      return {
        body: 'The device needs a platform permission before screen activity checks can run.',
        eyebrow: 'Permission',
        title: 'Permission is needed',
      };
    case 'protectedSurface':
      return {
        body: 'This screen is protected by the platform, so Ocentra records the limitation instead of capturing pixels.',
        eyebrow: 'Protected',
        title: 'This screen cannot be captured',
      };
    case 'unavailable':
      return {
        body: 'Screen activity status is not available right now. Ocentra will keep showing the latest safe state.',
        eyebrow: 'Unavailable',
        title: 'Screen activity is unavailable',
      };
  }
}

function detailRow(label: string, value: string): string {
  return `<div><dt>${escapeHtml(label)}</dt><dd>${escapeHtml(value)}</dd></div>`;
}

function screenChildDisclosurePageStyle(): string {
  return `:root{color-scheme:dark;--bg:#06111d;--panel:#0a1f2f;--line:#27d7e8;--text:#f3fbff;--muted:#a5c8d6;--ok:#7ef0c1;--warn:#ffd166}*{box-sizing:border-box}body{margin:0;background:radial-gradient(circle at 18% 0%,#103a4d 0,#06111d 36rem),var(--bg);color:var(--text);font-family:Inter,Arial,sans-serif;letter-spacing:0}.screen-child-page{min-height:100vh;padding:28px}.screen-child-hero,.screen-child-boundary,.screen-child-card{background:color-mix(in srgb,var(--panel) 86%,black);border:1px solid color-mix(in srgb,var(--line) 65%,transparent);border-radius:8px;box-shadow:0 18px 48px rgba(0,0,0,.28)}.screen-child-hero{margin:0 auto 18px;max-width:1180px;padding:24px}.screen-child-kicker,.screen-child-state{color:var(--ok);font-size:12px;font-weight:800;margin:0 0 8px;text-transform:uppercase}.screen-child-hero h1{font-size:32px;line-height:1.12;margin:0 0 10px}.screen-child-hero p{color:var(--muted);font-size:16px;line-height:1.45;margin:0;max-width:820px}.screen-child-grid{display:grid;gap:14px;grid-template-columns:repeat(5,minmax(0,1fr));margin:0 auto;max-width:1180px}.screen-child-card{min-height:292px;padding:18px}.screen-child-card h2{font-size:20px;line-height:1.18;margin:0 0 10px}.screen-child-card p{color:var(--muted);font-size:14px;line-height:1.42;margin:0 0 14px}.screen-child-card dl,.screen-child-boundary dl{display:grid;gap:9px;margin:0}.screen-child-card div,.screen-child-boundary div{display:grid;gap:3px}.screen-child-card dt,.screen-child-boundary dt{color:var(--muted);font-size:11px;font-weight:800;text-transform:uppercase}.screen-child-card dd,.screen-child-boundary dd{font-size:13px;font-weight:700;margin:0;overflow-wrap:anywhere}.screen-child-boundary{margin:18px auto 0;max-width:1180px;padding:16px}.screen-child-boundary dl{grid-template-columns:repeat(3,minmax(0,1fr))}@media (max-width:900px){.screen-child-page{padding:14px}.screen-child-grid{grid-template-columns:1fr}.screen-child-card{min-height:auto}.screen-child-boundary dl{grid-template-columns:1fr}.screen-child-hero h1{font-size:26px}}`;
}

function escapeAttribute(value: string): string {
  return escapeHtml(value);
}

function escapeHtml(value: string): string {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}
