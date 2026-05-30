import { mkdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

import { appControlFullCatalogSettings } from '../packages/parent-domain/dist/app-control-catalog.js';
import {
  BaselineGameControlAuthoringManifest,
  gameControlCatalogSettings,
} from '../packages/parent-domain/dist/game-control-catalog.js';
import { networkControlCatalogSettings } from '../packages/parent-domain/dist/network-control-catalog.js';
import { screenControlCatalogSettings } from '../packages/parent-domain/dist/screen-control-catalog.js';
import { trackingControlCatalogSettings } from '../packages/parent-domain/dist/tracking-control-catalog.js';

const catalogSpecs = [
  {
    title: 'App Control Settings Inventory',
    sourceName: 'BaselineAppControlFullCatalog',
    outputPath: '.codex/app-control-settings-inventory.md',
    settings: appControlFullCatalogSettings(),
  },
  {
    title: 'Game Control Settings Inventory',
    sourceName: 'BaselineGameControlAuthoringManifest',
    outputPath: '.codex/game-control-settings-inventory.md',
    settings: gameControlCatalogSettings(),
    notes: [
      'Games currently has 33 formal authoring settings from the schema proposal.',
      'The capability guide is represented as capability truths and a capability registry, not as separate parent-facing settings yet.',
      'Those guide-derived constraints are listed after the settings so the grouping pass can decide whether any should become explicit questions.',
    ],
    capabilityTruths: BaselineGameControlAuthoringManifest.capabilityTruths,
    capabilities: BaselineGameControlAuthoringManifest.capabilityRegistry.capabilities,
  },
  {
    title: 'Screen Control Settings Inventory',
    sourceName: 'BaselineScreenControlCatalog',
    outputPath: '.codex/screen-control-settings-inventory.md',
    settings: screenControlCatalogSettings(),
  },
  {
    title: 'Network Control Settings Inventory',
    sourceName: 'BaselineNetworkControlCatalog',
    outputPath: '.codex/network-control-settings-inventory.md',
    settings: networkControlCatalogSettings(),
  },
  {
    title: 'Tracking Control Settings Inventory',
    sourceName: 'BaselineTrackingControlCatalog',
    outputPath: '.codex/tracking-control-settings-inventory.md',
    settings: trackingControlCatalogSettings(),
  },
];

mkdirSync(resolve('.codex'), { recursive: true });

for (const spec of catalogSpecs) {
  writeFileSync(resolve(spec.outputPath), `${renderInventory(spec)}\n`, 'utf8');
  console.log(spec.outputPath);
}

function renderInventory(spec) {
  const lines = [];
  lines.push(`# ${spec.title}`);
  lines.push('');
  lines.push(`Generated from \`${spec.sourceName}\`.`);
  lines.push(`Total settings: ${spec.settings.length}`);
  if (spec.notes !== undefined) {
    for (const note of spec.notes) {
      lines.push(`Note: ${note}`);
    }
  }
  lines.push('');
  lines.push('Use this as the raw review list for deciding parent-facing grouping.');
  lines.push('');

  let currentLane = '';
  let currentSection = '';
  let currentGroup = '';

  spec.settings.forEach((setting, index) => {
    const lane = String(setting.policyLane ?? setting.uiTab ?? 'catalog');
    if (lane !== currentLane) {
      currentLane = lane;
      currentSection = '';
      currentGroup = '';
      lines.push(`## Tab: ${lane}`);
      lines.push('');
    }

    const sectionTitle = String(setting.sourceHeadingPath?.[0] ?? setting.sectionId);
    if (sectionTitle !== currentSection) {
      currentSection = sectionTitle;
      currentGroup = '';
      lines.push(`### ${sectionTitle}`);
      lines.push('');
    }

    const groupTitle = String(setting.sourceHeadingPath?.[1] ?? setting.groupId);
    if (groupTitle !== currentGroup) {
      currentGroup = groupTitle;
      lines.push(`#### ${groupTitle}`);
      lines.push('');
    }

    const prefix = `${String(index + 1).padStart(4, '0')}.`;
    const sourceText = String(setting.sourceText ?? setting.originalSourceText ?? setting.uiQuestionText);
    const optionLabels = (setting.acceptedOptions ?? [])
      .map((option) => String(option.label ?? option.value))
      .join(' | ');
    lines.push(`${prefix} ${sourceText}`);
    lines.push(`   - settingId: \`${setting.settingId}\``);
    lines.push(
      `   - policyLane: \`${lane}\`; cardKind: \`${setting.cardKind ?? setting.uiCardType}\`; selectionMode: \`${setting.selectionMode ?? 'derived'}\`; controlKind: \`${setting.controlKind ?? setting.controlType}\``
    );
    lines.push(
      `   - effectStatus: \`${setting.effectStatus}\`; runtimeOwner: \`${setting.runtimeOwner}\`; capabilityState: \`${setting.capabilityState}\``
    );
    lines.push(`   - proofRequirement: ${setting.proofRequirement ?? 'none'}`);
    lines.push(`   - sourceDocument: \`${setting.sourceDocument}\`; sourceLine: ${setting.sourceLine ?? 'n/a'}`);
    if (optionLabels.length > 0) {
      lines.push(`   - acceptedOptions: ${optionLabels}`);
    }
    if (setting.helperText !== null && setting.helperText !== undefined) {
      lines.push(`   - helperText: ${setting.helperText}`);
    }
    lines.push('');
  });

  if (spec.capabilityTruths !== undefined) {
    lines.push('## Capability truth coverage');
    lines.push('');
    lines.push(`Total capability truths: ${spec.capabilityTruths.length}`);
    lines.push('');
    spec.capabilityTruths.forEach((truth, index) => {
      const prefix = `${String(index + 1).padStart(4, '0')}.`;
      lines.push(`${prefix} ${truth.originalSourceText}`);
      lines.push(`   - truthId: \`${truth.truthId}\``);
      lines.push(`   - capabilityState: \`${truth.capabilityState}\``);
      lines.push(`   - sourceDocument: \`${truth.sourceDocument}\``);
      lines.push(`   - sourceHeadingPath: ${truth.sourceHeadingPath.join(' > ')}`);
      lines.push(
        `   - appliesToSettingIds: ${truth.appliesToSettingIds.map((settingId) => `\`${settingId}\``).join(', ')}`
      );
      lines.push('');
    });
  }

  if (spec.capabilities !== undefined) {
    lines.push('## Capability registry');
    lines.push('');
    lines.push(`Total capabilities: ${spec.capabilities.length}`);
    lines.push('');
    spec.capabilities.forEach((capability, index) => {
      const prefix = `${String(index + 1).padStart(4, '0')}.`;
      lines.push(`${prefix} \`${capability.capabilityId}\``);
      lines.push(`   - state: \`${capability.state}\``);
      lines.push(`   - proofRequirement: ${capability.proofRequirement}`);
      lines.push(
        `   - affectsSettingIds: ${capability.affectsSettingIds.map((settingId) => `\`${settingId}\``).join(', ')}`
      );
      lines.push('');
    });
  }

  return lines.join('\n');
}
