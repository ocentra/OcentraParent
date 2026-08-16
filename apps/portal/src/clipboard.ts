import { PortalClipboard, PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { type ParentPortalClipboardText } from '../generated/parent-ui-bridge';
export async function writeClipboardText(text: ParentPortalClipboardText): Promise<boolean> {
  if (navigator.clipboard !== undefined) {
    try {
      await navigator.clipboard.writeText(text);
      return true;
    } catch {
      return writeClipboardTextWithSelection(text);
    }
  }
  return writeClipboardTextWithSelection(text);
}

function writeClipboardTextWithSelection(text: ParentPortalClipboardText): boolean {
  const buffer = document.createElement(PortalDom.Tags.TextArea);
  buffer.className = PortalDom.Classes.ClipboardBuffer;
  buffer.setAttribute(PortalDom.Attributes.ReadOnly, PortalDom.Attributes.ReadOnly);
  buffer.value = text;
  document.body.append(buffer);
  buffer.focus();
  buffer.select();
  const copied = document.execCommand(PortalClipboard.CommandCopy);
  buffer.remove();
  return copied;
}
