import { mkdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

import { browserControlFullCatalogSettings } from '../packages/parent-domain/dist/browser-control-full-catalog.js';

const settings = browserControlFullCatalogSettings();
const outputPath = resolve('.codex/browser-control-1057-settings-inventory.md');
mkdirSync(resolve('.codex'), { recursive: true });

const lines = [];
lines.push('# Browser Control Full Settings Inventory');
lines.push('');
lines.push(`Generated from \`BaselineBrowserControlFullCatalog\`.`);
lines.push(`Total settings: ${settings.length}`);
lines.push('');
lines.push('Use this as the raw review list for deciding parent-facing grouping.');
lines.push('');

let currentTab = '';
let currentSection = '';
let currentGroup = '';

for (const [index, setting] of settings.entries()) {
  if (setting.uiTab !== currentTab) {
    currentTab = setting.uiTab;
    currentSection = '';
    currentGroup = '';
    lines.push(`## Tab: ${setting.uiTab}`);
    lines.push('');
  }

  const sectionTitle = setting.sourceHeadingPath?.[0] ?? setting.sectionId;
  if (sectionTitle !== currentSection) {
    currentSection = sectionTitle;
    currentGroup = '';
    lines.push(`### ${sectionTitle}`);
    lines.push('');
  }

  const groupTitle = setting.sourceHeadingPath?.[1] ?? setting.groupId;
  if (groupTitle !== currentGroup) {
    currentGroup = groupTitle;
    lines.push(`#### ${groupTitle}`);
    lines.push('');
  }

  const optionLabels = setting.acceptedOptions.map((option) => option.label).join(' | ');
  const prefix = `${String(index + 1).padStart(4, '0')}.`;
  lines.push(`${prefix} ${setting.sourceText}`);
  lines.push(`   - settingId: \`${setting.settingId}\``);
  lines.push(
    `   - uiTab: \`${setting.uiTab}\`; cardKind: \`${setting.cardKind}\`; selectionMode: \`${setting.selectionMode}\`; controlKind: \`${setting.controlKind}\``
  );
  lines.push(
    `   - effectStatus: \`${setting.effectStatus}\`; runtimeOwner: \`${setting.runtimeOwner}\`; capabilityState: \`${setting.capabilityState}\``
  );
  lines.push(`   - proofRequirement: ${setting.proofRequirement ?? 'none'}`);
  lines.push(`   - sourceLine: ${setting.sourceLine}; sourceText: ${setting.originalSourceText}`);
  if (optionLabels.length > 0) {
    lines.push(`   - acceptedOptions: ${optionLabels}`);
  }
  if (setting.helperText !== null) {
    lines.push(`   - helperText: ${setting.helperText}`);
  }
  lines.push('');
}

writeFileSync(outputPath, `${lines.join('\n')}\n`, 'utf8');
console.log(outputPath);
