import { PortalDom, type PortalDisplayText } from '@ocentra-parent/portal-domain/contracts';

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
  introText: PortalDisplayText,
  cards: readonly ControlCardSpec[]
): void {
  const section = document.createElement(PortalDom.Tags.Section);
  section.className = PortalDom.Classes.ControlDeck;

  const header = document.createElement(PortalDom.Tags.Division);
  header.className = PortalDom.Classes.ControlDeckHeader;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = titleText;

  const intro = document.createElement(PortalDom.Tags.Paragraph);
  intro.className = PortalDom.Classes.ControlDeckIntro;
  intro.textContent = introText;

  header.append(title, intro);
  const grid = document.createElement(PortalDom.Tags.Division);
  grid.className = PortalDom.Classes.CapabilityGrid;
  for (const card of cards) {
    grid.append(controlCard(card));
  }
  section.append(header, grid);
  container.append(section);
}

function controlCard(card: ControlCardSpec): HTMLElement {
  const article = document.createElement(PortalDom.Tags.Section);
  article.className = [PortalDom.Classes.ControlCard, card.accent].join(PortalDom.Classes.ClassNameSeparator);
  article.tabIndex = 0;

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
  article.append(header, body, tip);
  return article;
}
