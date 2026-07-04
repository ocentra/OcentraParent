import type { GameControlCatalogSettingSeed } from './game-control-catalog-schema';
import { GameControlRuleIdSchema } from './game-control-catalog-schema';
import { slug } from './game-control-catalog-core';
import { effectStatusForSeed } from './game-control-catalog-state';

export function proofRequirementForSeed(seed: GameControlCatalogSettingSeed): string | null {
  if (/browserCloud/u.test(seed.settingId)) {
    return 'Browser-game title proof requires managed browser URL/title evidence or explicit platform integration.';
  }
  if (/nativeGames\.strictActions|rules\.allowedActions/u.test(seed.settingId)) {
    return 'Strict action proof requires current target recheck, adapter capability, audit, and rollback path.';
  }
  if (/evidence\.requiredProof|evidence\.durationCountingMode|budgets/u.test(seed.settingId)) {
    return 'Duration proof requires session id, process/package identity, observation gaps, and evidence refs.';
  }
  if (/launchers/u.test(seed.settingId)) {
    return 'Launcher proof must not treat launcher-only activity as active gameplay.';
  }
  if (/inventory/u.test(seed.settingId)) {
    return 'Unknown and possible-game evidence must stay labeled until deterministic proof exists.';
  }
  return null;
}

export function fallbackForSeed(seed: GameControlCatalogSettingSeed): string {
  const status = effectStatusForSeed(seed);
  if (status === 'manual-required') {
    return 'Disable strict behavior and show manual-required setup until platform proof exists.';
  }
  if (status === 'degraded') {
    return 'Keep lower-confidence or degraded state visible and compile observe, ask, or report-only behavior.';
  }
  if (status === 'proof-required') {
    return 'Require explicit proof before strict enforcement; otherwise use observe, ask, or audit-only fallback.';
  }
  if (status === 'already-represented') {
    return 'Render and validate parent intent without claiming new runtime enforcement.';
  }
  return 'Portal authors intent only; child-agent runtime owns compile, persistence, evaluation, and audit.';
}

export function enabledConditionsForSeed(seed: GameControlCatalogSettingSeed) {
  return [
    condition('A family, child, or device target must be selected before writing game policy intent.'),
    condition(`Capability state must support ${effectStatusForSeed(seed)} presentation for this control.`),
  ];
}

export function validationRulesForSeed(seed: GameControlCatalogSettingSeed) {
  const rules = [
    condition('Selected option ids must belong to this setting acceptedOptions list.'),
    condition('Portal writes only authored intent; child runtime owns compile, persistence, evaluation, and audit.'),
  ];
  const proofRequirement = proofRequirementForSeed(seed);
  if (proofRequirement !== null) {
    rules.push(condition(proofRequirement));
  }
  return rules;
}

export function condition(description: string) {
  return {
    ruleId: GameControlRuleIdSchema.parse(`game-control-rule-${slug(description)}`),
    description,
  };
}
