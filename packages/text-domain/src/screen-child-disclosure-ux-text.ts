import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './contracts';

export const ScreenChildDisclosureUxTextToken = {
  Title: decodeTextTokenId('screen.childDisclosure.title'),
  Intro: decodeTextTokenId('screen.childDisclosure.intro'),
  DisabledTitle: decodeTextTokenId('screen.childDisclosure.disabled.title'),
  DisabledBody: decodeTextTokenId('screen.childDisclosure.disabled.body'),
  DisabledStatus: decodeTextTokenId('screen.childDisclosure.disabled.status'),
  PausedTitle: decodeTextTokenId('screen.childDisclosure.paused.title'),
  PausedBody: decodeTextTokenId('screen.childDisclosure.paused.body'),
  PausedStatus: decodeTextTokenId('screen.childDisclosure.paused.status'),
  ReadyTitle: decodeTextTokenId('screen.childDisclosure.ready.title'),
  ReadyBody: decodeTextTokenId('screen.childDisclosure.ready.body'),
  ReadyStatus: decodeTextTokenId('screen.childDisclosure.ready.status'),
  ActiveTitle: decodeTextTokenId('screen.childDisclosure.active.title'),
  ActiveBody: decodeTextTokenId('screen.childDisclosure.active.body'),
  ActiveStatus: decodeTextTokenId('screen.childDisclosure.active.status'),
  ProtectedTitle: decodeTextTokenId('screen.childDisclosure.protected.title'),
  ProtectedBody: decodeTextTokenId('screen.childDisclosure.protected.body'),
  ProtectedStatus: decodeTextTokenId('screen.childDisclosure.protected.status'),
  AskParentAction: decodeTextTokenId('screen.childDisclosure.action.askParent'),
} as const;

export type ScreenChildDisclosureUxTextTokenValue =
  (typeof ScreenChildDisclosureUxTextToken)[keyof typeof ScreenChildDisclosureUxTextToken];

export const ScreenChildDisclosureUxText: Record<ScreenChildDisclosureUxTextTokenValue, DisplayText> = {
  [ScreenChildDisclosureUxTextToken.Title]: decodeDisplayText('Screen check status'),
  [ScreenChildDisclosureUxTextToken.Intro]: decodeDisplayText(
    'When your parent turns on local screen checks, this device shows the current status.'
  ),
  [ScreenChildDisclosureUxTextToken.DisabledTitle]: decodeDisplayText('Screen checks are off'),
  [ScreenChildDisclosureUxTextToken.DisabledBody]: decodeDisplayText(
    'Your parent has not turned on local screen checks for this device.'
  ),
  [ScreenChildDisclosureUxTextToken.DisabledStatus]: decodeDisplayText('Off by parent setting'),
  [ScreenChildDisclosureUxTextToken.PausedTitle]: decodeDisplayText('Screen checks are paused'),
  [ScreenChildDisclosureUxTextToken.PausedBody]: decodeDisplayText(
    'Your parent paused local screen checks. No new screen check is running.'
  ),
  [ScreenChildDisclosureUxTextToken.PausedStatus]: decodeDisplayText('Paused'),
  [ScreenChildDisclosureUxTextToken.ReadyTitle]: decodeDisplayText('Screen checks are ready'),
  [ScreenChildDisclosureUxTextToken.ReadyBody]: decodeDisplayText(
    'This device can run a local screen check when a parent-approved trigger happens.'
  ),
  [ScreenChildDisclosureUxTextToken.ReadyStatus]: decodeDisplayText('Waiting for a trigger'),
  [ScreenChildDisclosureUxTextToken.ActiveTitle]: decodeDisplayText('Screen check is running'),
  [ScreenChildDisclosureUxTextToken.ActiveBody]: decodeDisplayText(
    'This device is running a local screen check and will delete temporary image data after processing.'
  ),
  [ScreenChildDisclosureUxTextToken.ActiveStatus]: decodeDisplayText('Running locally'),
  [ScreenChildDisclosureUxTextToken.ProtectedTitle]: decodeDisplayText('Screen check skipped'),
  [ScreenChildDisclosureUxTextToken.ProtectedBody]: decodeDisplayText(
    'This screen is protected, so Ocentra skipped the screen check.'
  ),
  [ScreenChildDisclosureUxTextToken.ProtectedStatus]: decodeDisplayText('Skipped protected screen'),
  [ScreenChildDisclosureUxTextToken.AskParentAction]: decodeDisplayText('Ask parent'),
};

const MissingScreenChildDisclosureTextTokenMessage = decodeDisplayText('Missing screen child disclosure text token.');

export function resolveScreenChildDisclosureUxText(token: unknown): DisplayText {
  const tokenId = decodeTextTokenId(token);
  const text = ScreenChildDisclosureUxText[tokenId as ScreenChildDisclosureUxTextTokenValue];
  if (text === undefined) {
    throw new Error(MissingScreenChildDisclosureTextTokenMessage);
  }
  return text;
}
