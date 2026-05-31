import { defaultScopeToggleConfig } from './ScopeToggleConfig';
import { calculateScopeToggleMetrics } from './ScopeToggleMetrics';
import { getNextScopeToggleValue, normalizeScopeToggleOptions } from './ScopeToggleOptions';
import { parentNavIconAssetUrls } from '../../../Common/NavSvgIcons/ParentNavSvgIcons';

type AssertFn = (condition: boolean, message: string) => void;

export function runScopeToggleSelfTests(assert: AssertFn = console.assert): void {
  const config = defaultScopeToggleConfig;
  const defaultOptions = normalizeScopeToggleOptions(config);
  const renamedOptions = normalizeScopeToggleOptions(config, undefined, 'Household', 'Per Device');
  const defaultMetrics = calculateScopeToggleMetrics(config, config.text.title, defaultOptions);
  const fiveOptions = normalizeScopeToggleOptions(config, [
    { value: 'family', label: 'Family' },
    { value: 'device', label: 'Per Device' },
    { value: 'lan', label: 'LAN' },
    { value: 'remote', label: 'Remote' },
    { value: 'global', label: 'Global' },
  ]);
  const fiveMetrics = calculateScopeToggleMetrics(config, config.text.title, fiveOptions);
  const clampedOptions = normalizeScopeToggleOptions(config, [
    { value: 'a', label: 'A' },
    { value: 'b', label: 'B' },
    { value: 'c', label: 'C' },
    { value: 'd', label: 'D' },
    { value: 'e', label: 'E' },
    { value: 'f', label: 'F' },
  ]);
  const longOptions = normalizeScopeToggleOptions(config, [
    { value: 'family', label: 'Family Group Members' },
    { value: 'device', label: 'Per Device Override Mode' },
    { value: 'lan', label: 'LAN Network' },
  ]);
  const longMetrics = calculateScopeToggleMetrics(config, 'Very Long Scope Title', longOptions);

  assert(defaultMetrics.optionWidth > 0, 'Each scope option must have a positive width.');
  assert(
    defaultOptions[0]?.iconHref === parentNavIconAssetUrls.FamilyIcon,
    'Family option should carry the reusable family SVG asset.'
  );
  assert(
    defaultOptions[1]?.iconHref === parentNavIconAssetUrls.DevicesMultiScreenIcon,
    'Per-device option should carry the reusable device SVG asset.'
  );
  assert(
    renamedOptions[0]?.iconHref === parentNavIconAssetUrls.FamilyIcon &&
      renamedOptions[1]?.iconHref === parentNavIconAssetUrls.DevicesMultiScreenIcon,
    'Renamed default options should preserve their icon assets.'
  );
  assert(
    defaultMetrics.trackX === defaultMetrics.titleBoxX + defaultMetrics.titleBoxWidth,
    'Track should start immediately after the Scope box.'
  );
  assert(
    config.slider.inset + config.slider.gapFromDivider < defaultMetrics.optionWidth,
    'Slider inset and divider gap must leave a visible selected pill width.'
  );
  assert(
    config.slider.gapFromDivider > config.layout.dividerWidth,
    'Selected pill should leave space around the divider.'
  );
  assert(
    config.text.optionFontSize >= config.layout.trackHeight * 0.35,
    'Option text should visually fill the track height.'
  );
  assert(
    config.opacity.titleGlowIdle > config.opacity.trackGlowIdle,
    'Scope label glow should be stronger than the track idle glow.'
  );
  assert(
    config.layout.outerPadX > 0 && config.layout.outerPadY > 0,
    'Toggle-only outer edge should have positive padding around the track.'
  );
  assert(fiveOptions.length === 5, 'Toggle should support five options.');
  assert(fiveMetrics.dividerXs.length === 4, 'Five options should create four dividers.');
  assert(clampedOptions.length === config.layout.maxOptions, 'Options should be capped by maxOptions.');
  assert(longMetrics.titleBoxWidth > defaultMetrics.titleBoxWidth, 'Long title text should grow the title box.');
  assert(longMetrics.optionWidth > defaultMetrics.optionWidth, 'Long option labels should grow the option width.');
  assert(longMetrics.svgWidth > defaultMetrics.svgWidth, 'Long labels should grow the SVG width.');
  assert(getNextScopeToggleValue('family', defaultOptions) === 'device', 'Family should toggle to device.');
  assert(getNextScopeToggleValue('device', defaultOptions) === 'family', 'Device should toggle to family.');
  assert(
    getNextScopeToggleValue('remote', fiveOptions) === 'global',
    'N-way toggle should advance to the next option.'
  );
}
