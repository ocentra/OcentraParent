import { BrowserControlAuthoringManifestSchema } from './browser-control-manifest';
import { GeneratedBaselineBrowserControlAuthoringManifest } from './generated/browser-policy-control-catalog-helpers';

export const BaselineBrowserControlAuthoringManifest = BrowserControlAuthoringManifestSchema.parse(
  GeneratedBaselineBrowserControlAuthoringManifest
);
