import { useRef, useState } from 'react';
import { PortalTheme, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
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
  readonly copyError: string;
  readonly copyJson: () => void;
  readonly jsonRef: React.RefObject<HTMLTextAreaElement | null>;
  readonly jsonText: string;
  readonly selectJson: () => void;
  readonly updateActivePalette: (patch: Partial<PortalBackgroundThemeColors>) => void;
  readonly updateBeamColor: (index: number, value: string) => void;
  readonly updateBlobColor: (index: number, value: string) => void;
  readonly updateConfig: (patch: Partial<PortalBackgroundConfig>) => void;
};

export function usePortalBackgroundTunerControlState({
  config,
  onConfigChange,
  previewTheme,
}: PortalBackgroundTunerControlStateProps): PortalBackgroundTunerControlState {
  const jsonRef = useRef<HTMLTextAreaElement | null>(null);
  const [copied, setCopied] = useState(false);
  const [copyError, setCopyError] = useState('');
  const activePalette = previewTheme === PortalTheme.Light ? config.themes.light : config.themes.dark;
  const jsonText = JSON.stringify(config, null, 2);

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
  value: string
): void {
  const nextColors = [...config.blobColors] as [string, string, string, string, string, string];
  nextColors[index] = value;
  updateConfigValue(config, onConfigChange, { blobColors: nextColors });
}

function updateBeamColorValue(
  config: PortalBackgroundConfig,
  onConfigChange: (config: PortalBackgroundConfig) => void,
  index: number,
  value: string
): void {
  const nextColors = [...config.beamColors] as [string, string, string];
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
  setCopyError: (value: string) => void
): void {
  try {
    selectJsonValue(jsonRef);
    const ok = document.execCommand('copy');
    if (!ok) {
      throw new Error('execCommand copy failed');
    }
    setCopyError('');
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1200);
  } catch {
    setCopied(false);
    setCopyError('Copy blocked. Select JSON, then Ctrl/Cmd+C.');
  }
}
