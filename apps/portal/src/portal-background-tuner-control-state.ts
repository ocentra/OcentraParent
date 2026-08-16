import { useRef, useState } from 'react';
import {
  PortalClipboard,
  PortalTheme,
  PortalTiming,
  type PortalThemeValue,
} from '@ocentra-parent/portal-domain/contracts';
import {
  PortalDevTextToken,
  resolvePortalDevText,
  type DisplayText as PortalDisplayText,
} from '@ocentra-parent/portal-domain/display-text';
import { decodeParentPortalClipboardText, type ParentPortalClipboardText } from '../generated/parent-ui-bridge';
import type { PortalBackgroundConfig, PortalBackgroundThemeColors } from './portal-background-config';

type PortalBackgroundTunerControlStateProps = {
  readonly config: PortalBackgroundConfig;
  readonly onConfigChange: (config: PortalBackgroundConfig) => void;
  readonly previewTheme: PortalThemeValue;
};

export type PortalBackgroundTunerControlState = {
  readonly activePalette: PortalBackgroundThemeColors;
  readonly config: PortalBackgroundConfig;
  readonly copied: boolean;
  readonly copyError: PortalDisplayText | null;
  readonly copyJson: () => void;
  readonly jsonRef: React.RefObject<HTMLTextAreaElement | null>;
  readonly jsonText: ParentPortalClipboardText;
  readonly selectJson: () => void;
  readonly updateActivePalette: (patch: Partial<PortalBackgroundThemeColors>) => void;
  readonly updateBeamColor: (index: number, value: PortalBackgroundColor) => void;
  readonly updateBlobColor: (index: number, value: PortalBackgroundColor) => void;
  readonly updateConfig: (patch: Partial<PortalBackgroundConfig>) => void;
};

type PortalBackgroundColor = PortalBackgroundThemeColors[keyof PortalBackgroundThemeColors];

export function usePortalBackgroundTunerControlState({
  config,
  onConfigChange,
  previewTheme,
}: PortalBackgroundTunerControlStateProps): PortalBackgroundTunerControlState {
  const jsonRef = useRef<HTMLTextAreaElement | null>(null);
  const [copied, setCopied] = useState(false);
  const [copyError, setCopyError] = useState<PortalDisplayText | null>(null);
  const activePalette = previewTheme === PortalTheme.Light ? config.themes.light : config.themes.dark;
  const jsonText = decodeParentPortalClipboardText(JSON.stringify(config, null, 2));

  return {
    activePalette,
    config,
    copied,
    copyError,
    copyJson: () => copyJsonValue(jsonRef, setCopied, setCopyError),
    jsonRef,
    jsonText,
    selectJson: () => selectJsonValue(jsonRef),
    updateActivePalette: (patch) => updateActivePaletteValue(config, previewTheme, onConfigChange, patch),
    updateBeamColor: (index, value) => updateBeamColorValue(config, onConfigChange, index, value),
    updateBlobColor: (index, value) => updateBlobColorValue(config, onConfigChange, index, value),
    updateConfig: (patch) => updateConfigValue(config, onConfigChange, patch),
  };
}

function updateConfigValue(
  config: PortalBackgroundConfig,
  onConfigChange: (config: PortalBackgroundConfig) => void,
  patch: Partial<PortalBackgroundConfig>
): void {
  onConfigChange({ ...config, ...patch });
}

function updateActivePaletteValue(
  config: PortalBackgroundConfig,
  previewTheme: PortalThemeValue,
  onConfigChange: (config: PortalBackgroundConfig) => void,
  patch: Partial<PortalBackgroundThemeColors>
): void {
  const themeKey = previewTheme === PortalTheme.Light ? PortalTheme.Light : PortalTheme.Dark;
  onConfigChange({
    ...config,
    themes: {
      ...config.themes,
      [themeKey]: {
        ...config.themes[themeKey],
        ...patch,
      },
    },
  });
}

function updateBlobColorValue(
  config: PortalBackgroundConfig,
  onConfigChange: (config: PortalBackgroundConfig) => void,
  index: number,
  value: PortalBackgroundColor
): void {
  const nextColors = [...config.blobColors] as [...typeof config.blobColors];
  nextColors[index] = value;
  updateConfigValue(config, onConfigChange, { blobColors: nextColors });
}

function updateBeamColorValue(
  config: PortalBackgroundConfig,
  onConfigChange: (config: PortalBackgroundConfig) => void,
  index: number,
  value: PortalBackgroundColor
): void {
  const nextColors = [...config.beamColors] as [...typeof config.beamColors];
  nextColors[index] = value;
  updateConfigValue(config, onConfigChange, { beamColors: nextColors });
}

function selectJsonValue(jsonRef: React.RefObject<HTMLTextAreaElement | null>): void {
  const textArea = jsonRef.current;
  if (textArea === null) {
    return;
  }
  textArea.focus();
  textArea.select();
  textArea.setSelectionRange(0, textArea.value.length);
}

function copyJsonValue(
  jsonRef: React.RefObject<HTMLTextAreaElement | null>,
  setCopied: (value: boolean) => void,
  setCopyError: (value: PortalDisplayText | null) => void
): void {
  try {
    selectJsonValue(jsonRef);
    const ok = document.execCommand(PortalClipboard.CommandCopy);
    if (!ok) {
      throw new Error(resolvePortalDevText(PortalDevTextToken.CopyResultFailed));
    }
    setCopyError(null);
    setCopied(true);
    window.setTimeout(() => setCopied(false), PortalTiming.CopyFeedbackMs);
  } catch {
    setCopied(false);
    setCopyError(resolvePortalDevText(PortalDevTextToken.CopyResultFailed));
  }
}
