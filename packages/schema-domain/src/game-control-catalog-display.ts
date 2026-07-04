import type { GameControlCatalogSettingSeed } from './game-control-catalog-schema';
import { condition } from './game-control-catalog-guidance';

export function helperTextForSeed(seed: GameControlCatalogSettingSeed): string {
  if (/browserCloud/u.test(seed.settingId)) {
    return 'Browser and cloud games keep their surface-specific proof boundary; network hints are not exact title proof.';
  }
  if (/launchers/u.test(seed.settingId)) {
    return 'Launcher activity is not automatically game play; manifests and child-process attribution remain separate.';
  }
  if (/nativeGames/u.test(seed.settingId)) {
    return 'Native game controls depend on process, package, foreground, and protected-process capability proof.';
  }
  return 'Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.';
}

export function visibilityConditionsForSeed(seed: GameControlCatalogSettingSeed) {
  const conditions = [condition(`Visible when Games side-panel category renders ${seed.sectionTitle}.`)];
  if (seed.settingId !== 'game.enabled') {
    conditions.push(condition('Visible when game management is enabled or when Portal previews disabled controls.'));
  }
  return conditions;
}
