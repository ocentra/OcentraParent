import { useEffect, useMemo, useState, type ReactElement } from 'react';
import { PortalTheme, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import { PortalBackgroundTunerControls } from './PortalBackgroundTunerControls';
import { PortalBackgroundSvg } from './PortalBackgroundSvg';
import {
  DEFAULT_PORTAL_BACKGROUND_CONFIG,
  DEFAULT_PORTAL_BACKGROUND_DARK_COLORS,
  DEFAULT_PORTAL_BACKGROUND_LIGHT_COLORS,
  loadPortalBackgroundConfig,
  portalBackgroundRenderConfig,
  readDefaultPortalBackgroundConfig,
  savePortalBackgroundConfig,
  subscribePortalBackgroundConfig,
  type PortalBackgroundConfig,
} from './portal-background-config';

type BackgroundDevToolProps = {
  readonly initialTheme: PortalThemeValue;
};

type PortalBackgroundDevToolState = {
  readonly dirty: boolean;
  readonly draftConfig: PortalBackgroundConfig;
  readonly previewTheme: PortalThemeValue;
  readonly renderConfig: ReturnType<typeof portalBackgroundRenderConfig>;
  readonly resetAll: () => void;
  readonly resetTheme: () => void;
  readonly saveDraft: () => Promise<void>;
  readonly setDraftConfig: (config: PortalBackgroundConfig) => void;
  readonly setPreviewTheme: (theme: PortalThemeValue) => void;
  readonly status: string;
};

export function PortalBackgroundDevTool({ initialTheme }: BackgroundDevToolProps): ReactElement {
  const {
    dirty,
    draftConfig,
    previewTheme,
    renderConfig,
    resetAll,
    resetTheme,
    saveDraft,
    setDraftConfig,
    setPreviewTheme,
    status,
  } = usePortalBackgroundDevToolState(initialTheme);

  return (
    <div
      style={{
        background: 'transparent',
        color: '#e5eefb',
        fontFamily: 'Inter, ui-sans-serif, system-ui, sans-serif',
        inset: 0,
        minHeight: '100vh',
        overflow: 'hidden',
        padding: 0,
        position: 'fixed',
      }}
    >
      <PortalBackgroundSvg
        {...renderConfig}
        preserveAspectRatio="xMidYMid slice"
        style={{
          height: '100%',
          inset: 0,
          position: 'absolute',
          width: '100%',
        }}
      />
      <PortalBackgroundTunerControls
        config={draftConfig}
        dirty={dirty}
        onConfigChange={setDraftConfig}
        onPreviewThemeChange={setPreviewTheme}
        onResetAll={resetAll}
        onResetTheme={resetTheme}
        onSave={saveDraft}
        previewTheme={previewTheme}
        status={status}
      />
    </div>
  );
}

function usePortalBackgroundDevToolState(initialTheme: PortalThemeValue): PortalBackgroundDevToolState {
  const [savedConfig, setSavedConfig] = useState<PortalBackgroundConfig>(() => readDefaultPortalBackgroundConfig());
  const [draftConfig, setDraftConfig] = useState<PortalBackgroundConfig>(savedConfig);
  const [previewTheme, setPreviewTheme] = useState<PortalThemeValue>(initialTheme);
  const [status, setStatus] = useState('Ready');
  const dirty = useMemo(() => JSON.stringify(draftConfig) !== JSON.stringify(savedConfig), [draftConfig, savedConfig]);
  const renderConfig = useMemo(
    () => portalBackgroundRenderConfig(draftConfig, previewTheme),
    [draftConfig, previewTheme]
  );

  useEffect(() => {
    let active = true;
    const applyLoadedConfig = (nextConfig: PortalBackgroundConfig): void => {
      if (!active) {
        return;
      }
      setSavedConfig(nextConfig);
      setDraftConfig(nextConfig);
    };
    void loadPortalBackgroundConfig().then(applyLoadedConfig);
    const unsubscribe = subscribePortalBackgroundConfig(applyLoadedConfig);
    return () => {
      active = false;
      unsubscribe();
    };
  }, []);

  const saveDraft = async (): Promise<void> => {
    setStatus('Saving JSON');
    const nextSavedConfig = await savePortalBackgroundConfig(draftConfig);
    if (nextSavedConfig === undefined) {
      setStatus('Save failed');
      return;
    }
    setSavedConfig(nextSavedConfig);
    setDraftConfig(nextSavedConfig);
    setStatus('Saved JSON');
  };
  const resetAll = (): void => {
    setDraftConfig(DEFAULT_PORTAL_BACKGROUND_CONFIG);
    setStatus('Reset draft');
  };
  const resetTheme = (): void => {
    setDraftConfig((currentConfig) => ({
      ...currentConfig,
      themes: {
        ...currentConfig.themes,
        [previewTheme]:
          previewTheme === PortalTheme.Light
            ? DEFAULT_PORTAL_BACKGROUND_LIGHT_COLORS
            : DEFAULT_PORTAL_BACKGROUND_DARK_COLORS,
      },
    }));
    setStatus(`Reset ${previewTheme} draft`);
  };

  return {
    dirty,
    draftConfig,
    previewTheme,
    renderConfig,
    resetAll,
    resetTheme,
    saveDraft,
    setDraftConfig,
    setPreviewTheme,
    status,
  };
}
