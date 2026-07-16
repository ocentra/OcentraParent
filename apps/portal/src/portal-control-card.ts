import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { attachPortalGoldenCardFrame } from './portal-golden-card-frame';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';

type ControlCardAccent =
  | typeof PortalDom.Classes.ControlCardAccentPrimary
  | typeof PortalDom.Classes.ControlCardAccentPrivacy
  | typeof PortalDom.Classes.ControlCardAccentWarn;

type ControlCardSpec = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly status: PortalDisplayText;
  readonly tipTitle: PortalDisplayText;
  readonly tipBody: PortalDisplayText;
  readonly accent: ControlCardAccent;
};

export function renderControlDeck(
  container: HTMLElement,
  titleText: PortalDisplayText,
  _introText: PortalDisplayText,
  cards: readonly ControlCardSpec[]
): void {
  const section = document.createElement(PortalDom.Tags.Section);
  section.className = [PortalDom.Classes.ControlDeck, PortalDom.Classes.ControlCarouselFrame].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const header = document.createElement(PortalDom.Tags.Division);
  header.className = PortalDom.Classes.ControlDeckHeader;

  const title = document.createElement(PortalDom.Tags.Division);
  title.className = PortalDom.Classes.ControlCarouselTitle;

  const count = document.createElement(PortalDom.Tags.Span);
  count.className = PortalDom.Classes.ControlCarouselCount;
  count.textContent = String(cards.length);

  const label = document.createElement(PortalDom.Tags.HeadingTwo);
  label.className = PortalDom.Classes.ControlCarouselLabel;
  label.textContent = titleText;

  title.append(count, label);

  header.append(title);

  const stage = document.createElement(PortalDom.Tags.Division);
  stage.className = PortalDom.Classes.ControlCarouselStage;

  const leftHandle = document.createElement(PortalDom.Tags.Division);
  leftHandle.className = [PortalDom.Classes.ControlCarouselHandle, PortalDom.Classes.ControlCarouselHandleLeft].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const rightHandle = document.createElement(PortalDom.Tags.Division);
  rightHandle.className = [PortalDom.Classes.ControlCarouselHandle, PortalDom.Classes.ControlCarouselHandleRight].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const grid = document.createElement(PortalDom.Tags.Division);
  grid.className = [PortalDom.Classes.CapabilityGrid, PortalDom.Classes.ControlCarouselRail].join(
    PortalDom.Classes.ClassNameSeparator
  );
  for (const [index, card] of cards.entries()) {
    grid.append(controlCard(card, index + 1));
  }

  const pager = document.createElement(PortalDom.Tags.Division);
  pager.className = PortalDom.Classes.ControlCarouselPager;
  for (let index = 0; index < Math.min(cards.length, 5); index += 1) {
    const pill = document.createElement(PortalDom.Tags.Span);
    pill.className =
      index === 0
        ? [PortalDom.Classes.ControlCarouselPill, PortalDom.Classes.ControlCarouselPillActive].join(
            PortalDom.Classes.ClassNameSeparator
          )
        : PortalDom.Classes.ControlCarouselPill;
    pager.append(pill);
  }

  stage.append(leftHandle, grid, rightHandle, pager);
  section.append(header, stage);
  container.append(section);
}

function controlCard(card: ControlCardSpec, rank: number): HTMLElement {
  const article = document.createElement(PortalDom.Tags.Section);
  article.className = [PortalDom.Classes.ControlCard, card.accent].join(PortalDom.Classes.ClassNameSeparator);
  article.tabIndex = 0;

  const art = document.createElement(PortalDom.Tags.Image);
  art.className = PortalDom.Classes.ControlCardGoldenArt;
  art.setAttribute(PortalDom.Attributes.AriaHidden, PortalDom.Attributes.True);
  attachPortalGoldenCardFrame(art, card, rank);

  const content = document.createElement(PortalDom.Tags.Division);
  content.className = PortalDom.Classes.ControlCardContent;

  const header = document.createElement(PortalDom.Tags.Division);
  header.className = PortalDom.Classes.ControlCardHeader;

  const glyph = document.createElement(PortalDom.Tags.Span);
  glyph.className = PortalDom.Classes.ControlCardGlyph;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = card.title;

  const status = document.createElement(PortalDom.Tags.Span);
  status.className = PortalDom.Classes.ControlCardStatus;
  status.textContent = card.status;

  header.append(glyph, title, status);

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.ControlCardBody;
  body.textContent = card.body;

  const tip = document.createElement(PortalDom.Tags.Division);
  tip.className = PortalDom.Classes.ControlCardTip;

  const tipTitle = document.createElement(PortalDom.Tags.Strong);
  tipTitle.className = PortalDom.Classes.ControlCardTipTitle;
  tipTitle.textContent = card.tipTitle;

  const tipBody = document.createElement(PortalDom.Tags.Paragraph);
  tipBody.className = PortalDom.Classes.ControlCardTipBody;
  tipBody.textContent = card.tipBody;

  tip.append(tipTitle, tipBody);
  content.append(header, body, tip);
  article.append(art, content);
  return article;
}
