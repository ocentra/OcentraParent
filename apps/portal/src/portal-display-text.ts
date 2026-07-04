import type { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';

type PortalFrameText = typeof PortalFrameTuner.Text;

export type PortalDisplayText = PortalFrameText[keyof PortalFrameText];

export function decodeDisplayText(value: string): PortalDisplayText {
  if (value.trim().length === 0) {
    throw new TypeError('DisplayText must be non-empty');
  }
  return value as PortalDisplayText;
}
