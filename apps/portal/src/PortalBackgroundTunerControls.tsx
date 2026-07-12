import { useState, type ReactElement } from 'react';
import type { PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import type { PortalBackgroundConfig } from './portal-background-config';
import { usePortalBackgroundTunerControlState } from './portal-background-tuner-control-state';
import { PortalBackgroundTunerEditor, PortalBackgroundTunerHeader } from './portal-background-tuner-controls-panels';

type PortalBackgroundTunerControlsProps = {
  readonly config: PortalBackgroundConfig;
  readonly dirty: boolean;
  readonly onConfigChange: (config: PortalBackgroundConfig) => void;
  readonly onPreviewThemeChange: (theme: PortalThemeValue) => void;
  readonly onResetAll: () => void;
  readonly onResetTheme: () => void;
  readonly onSave: () => void;
  readonly previewTheme: PortalThemeValue;
  readonly status: string;
};

const panelChrome = {
  border: 'rgba(148, 163, 184, 0.18)',
  subtext: '#93a4bb',
  text: '#e5eefb',
} as const;

export function PortalBackgroundTunerControls({
  config,
  dirty,
  onConfigChange,
  onPreviewThemeChange,
  onResetAll,
  onResetTheme,
  onSave,
  previewTheme,
  status,
}: PortalBackgroundTunerControlsProps): ReactElement {
  const [open, setOpen] = useState(true);
  const controls = usePortalBackgroundTunerControlState({ config, onConfigChange, previewTheme });
  return (
    <div
      style={{
        backdropFilter: 'blur(18px) saturate(145%)',
        background: 'rgba(5, 12, 24, 0.76)',
        border: `1px solid ${panelChrome.border}`,
        borderRadius: 12,
        boxShadow: '0 18px 50px rgba(0, 0, 0, 0.34), 0 0 0 1px rgba(85, 255, 246, 0.06)',
        color: panelChrome.text,
        display: 'grid',
        gap: 0,
        maxHeight: 'calc(100vh - 24px)',
        overflow: 'auto',
        padding: 10,
        position: 'fixed',
        right: 12,
        top: 12,
        width: 'min(390px, calc(100vw - 24px))',
        zIndex: 2,
      }}
    >
      <PortalBackgroundTunerHeader dirty={dirty} onOpenChange={setOpen} open={open} status={status} />
      {open ? (
        <PortalBackgroundTunerEditor
          controls={controls}
          dirty={dirty}
          onPreviewThemeChange={onPreviewThemeChange}
          onResetAll={onResetAll}
          onResetTheme={onResetTheme}
          onSave={onSave}
          previewTheme={previewTheme}
        />
      ) : null}
    </div>
  );
}
