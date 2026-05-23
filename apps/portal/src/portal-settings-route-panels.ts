import {
  PortalDom,
  PortalText,
  PortalTextToken,
  PortalTheme,
  type PortalThemeValue,
} from '@ocentra-parent/portal-domain/contracts';
import { applyTheme, saveTheme } from './portal-theme';

export function renderSettingsRulesRoute(container: HTMLElement, theme: PortalThemeValue, rerender: () => void): void {
  renderThemeSettingsPanel(container, theme, rerender);
}

function renderThemeSettingsPanel(container: HTMLElement, theme: PortalThemeValue, rerender: () => void): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = [PortalDom.Classes.Summary, PortalDom.Classes.SettingsThemePanel].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.DisplayTheme);

  const toggle = document.createElement(PortalDom.Tags.Division);
  toggle.className = PortalDom.Classes.ThemeToggle;
  toggle.setAttribute(PortalDom.Attributes.Role, PortalDom.Attributes.TabList);
  toggle.append(themeButton(PortalTheme.Light, theme, rerender));
  toggle.append(themeButton(PortalTheme.Dark, theme, rerender));

  const note = document.createElement(PortalDom.Tags.Paragraph);
  note.className = PortalDom.Classes.ProductPanelNote;
  note.textContent = PortalText.Resolve(PortalTextToken.SettingsRulesDescription);

  panel.append(title, toggle, note);
  container.append(panel);
}

function themeButton(option: PortalThemeValue, theme: PortalThemeValue, rerender: () => void): HTMLButtonElement {
  const button = document.createElement(PortalDom.Tags.Button);
  const active = option === theme;
  button.type = PortalDom.ButtonType.Button;
  button.className = themeButtonClass(active);
  button.setAttribute(
    PortalDom.Attributes.AriaSelected,
    active ? PortalDom.Attributes.True : PortalDom.Attributes.False
  );
  button.textContent =
    option === PortalTheme.Light
      ? PortalText.Resolve(PortalTextToken.ThemeLight)
      : PortalText.Resolve(PortalTextToken.ThemeDark);
  button.addEventListener(PortalDom.Events.Click, () => {
    saveTheme(option);
    applyTheme(option);
    rerender();
  });
  return button;
}

function themeButtonClass(active: boolean) {
  if (!active) {
    return PortalDom.Classes.ThemeToggleButton;
  }
  return [PortalDom.Classes.ThemeToggleButton, PortalDom.Classes.ThemeToggleButtonActive].join(
    PortalDom.Classes.ClassNameSeparator
  );
}
