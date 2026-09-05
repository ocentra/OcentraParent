import path from 'node:path';

const screenshotDirectory = path.join(
  'output',
  'tracking-plan-proof',
  '30-parent-and-child-ui-ux-surfaces',
  '11-ui-snapshots'
);

export type HostedTrackingScreenshotPaths = {
  readonly proofUnavailable: string;
  readonly productUnavailable: string;
  readonly productUnavailableMobile: string;
  readonly parentOverview: string;
  readonly parentDevices: string;
};

export function hostedTrackingScreenshotPaths(repoRoot: string): HostedTrackingScreenshotPaths {
  return {
    proofUnavailable: relativeScreenshot(repoRoot, 'hosted-proof-panels-tracking-unavailable.png'),
    productUnavailable: relativeScreenshot(repoRoot, 'hosted-policy-tracking-unavailable.png'),
    productUnavailableMobile: relativeScreenshot(repoRoot, 'hosted-policy-tracking-unavailable-mobile.png'),
    parentOverview: relativeScreenshot(repoRoot, 'hosted-parent-overview-shell.png'),
    parentDevices: relativeScreenshot(repoRoot, 'hosted-parent-devices-shell.png'),
  };
}

function relativeScreenshot(repoRoot: string, fileName: string): string {
  return path.relative(repoRoot, path.join(repoRoot, screenshotDirectory, fileName)).replace(/\\/gu, '/');
}
