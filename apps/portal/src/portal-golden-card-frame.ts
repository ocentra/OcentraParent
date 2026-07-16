import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import {
  createGoldenFrameFrameOnlySvgDataUri,
  createGoldenFrameVariantConfig,
} from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalGoldenFrameForeignObject';

type ControlCardAccent =
  | typeof PortalDom.Classes.ControlCardAccentPrimary
  | typeof PortalDom.Classes.ControlCardAccentPrivacy
  | typeof PortalDom.Classes.ControlCardAccentWarn;

type PortalGoldenCardContent = {
  readonly title: PortalDisplayText;
  readonly status: PortalDisplayText;
  readonly accent: ControlCardAccent;
};

type PortalGoldenCardEntry = {
  readonly image: HTMLImageElement;
  readonly card: PortalGoldenCardContent;
  readonly rank: number;
};

const portalGoldenCardEntries = new Set<PortalGoldenCardEntry>();
let portalGoldenFrameChannel: BroadcastChannel | undefined;
let portalGoldenFrameStorageListener = false;

export function attachPortalGoldenCardFrame(
  image: HTMLImageElement,
  card: PortalGoldenCardContent,
  rank: number
): void {
  const entry = { image, card, rank };
  portalGoldenCardEntries.add(entry);
  updatePortalGoldenCardImage(entry, readSharedGoldenFrameConfig());
  ensurePortalGoldenFrameListeners();
}

function ensurePortalGoldenFrameListeners(): void {
  if (typeof window === PortalFrameTuner.GoldenFrame.ValueType.Undefined) {
    return;
  }
  if (!portalGoldenFrameStorageListener) {
    window.addEventListener(PortalDom.Events.Storage, (event) => {
      if (event.key === PortalFrameTuner.GoldenFrame.StorageKey) {
        updatePortalGoldenCardImages(readSharedGoldenFrameConfig());
      }
    });
    portalGoldenFrameStorageListener = true;
  }
  if (portalGoldenFrameChannel !== undefined) {
    return;
  }
  try {
    portalGoldenFrameChannel = new BroadcastChannel(PortalFrameTuner.GoldenFrame.Channel);
    portalGoldenFrameChannel.onmessage = (event) => updatePortalGoldenCardImages(event.data);
  } catch {
    portalGoldenFrameChannel = undefined;
  }
}

function updatePortalGoldenCardImages(sharedConfig: unknown): void {
  for (const entry of portalGoldenCardEntries) {
    if (!entry.image.isConnected) {
      portalGoldenCardEntries.delete(entry);
      continue;
    }
    updatePortalGoldenCardImage(entry, sharedConfig);
  }
}

function updatePortalGoldenCardImage(entry: PortalGoldenCardEntry, sharedConfig: unknown): void {
  const cfg = portalGoldenFrameConfig(entry.card, entry.rank, sharedConfig);
  entry.image.src = createGoldenFrameFrameOnlySvgDataUri(cfg);
}

function portalGoldenFrameConfig(card: PortalGoldenCardContent, rank: number, sharedConfig: unknown) {
  const cfg = isGoldenFrameConfig(sharedConfig)
    ? structuredClone(sharedConfig)
    : createGoldenFrameVariantConfig({
        name: card.title,
        rank: String(rank),
        statName: card.status,
        statValue: String(rank),
        tone: PortalFrameTuner.GoldenFrame.ToneGold,
      });

  cfg.sideHexBadge.label = String(rank);
  cfg.winnerName.text = card.title;
  cfg.rightStats.name = card.status;
  cfg.rightStats.value = String(rank);
  return cfg;
}

function readSharedGoldenFrameConfig(): unknown {
  if (typeof window === PortalFrameTuner.GoldenFrame.ValueType.Undefined) {
    return undefined;
  }
  try {
    const stored = window.localStorage.getItem(PortalFrameTuner.GoldenFrame.StorageKey);
    return stored === null ? undefined : JSON.parse(stored);
  } catch {
    return undefined;
  }
}

function isGoldenFrameConfig(value: unknown): value is ReturnType<typeof createGoldenFrameVariantConfig> {
  if (typeof value !== PortalFrameTuner.GoldenFrame.ValueType.Object || value === null) {
    return false;
  }
  const record = value as Record<PropertyKey, unknown>;
  return (
    typeof record[PortalFrameTuner.GoldenFrame.ConfigKey.OuterFrame] ===
      PortalFrameTuner.GoldenFrame.ValueType.Object &&
    typeof record[PortalFrameTuner.GoldenFrame.ConfigKey.InnerFrame] ===
      PortalFrameTuner.GoldenFrame.ValueType.Object &&
    typeof record[PortalFrameTuner.GoldenFrame.ConfigKey.SideHexBadge] ===
      PortalFrameTuner.GoldenFrame.ValueType.Object &&
    typeof record[PortalFrameTuner.GoldenFrame.ConfigKey.WinnerName] ===
      PortalFrameTuner.GoldenFrame.ValueType.Object &&
    typeof record[PortalFrameTuner.GoldenFrame.ConfigKey.RightStats] === PortalFrameTuner.GoldenFrame.ValueType.Object
  );
}
