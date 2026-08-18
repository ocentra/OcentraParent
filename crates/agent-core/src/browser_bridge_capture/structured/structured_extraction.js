(async () => {
  const maxNodes = 256;
  const maxTextWork = 4096;
  const maxDocumentUrl = 4096;
  const protectedDigest = 'managed-browser-sensitivity-protected-v1';
  const unknownDigest = 'managed-browser-sensitivity-unknown-v1';
  const protectedBodyDigest = 'protected-content-redacted-v1';
  const hasSensitiveMarker = (value) =>
    typeof value === 'string' &&
    /\b(password|passcode|pin|security|secret|credential)\b/i.test(value);

  const documentUrl = () => {
    try {
      if (typeof location !== 'object' || !location || typeof location.href !== 'string') {
        return '';
      }
      return location.href.length <= maxDocumentUrl ? location.href : '';
    } catch (_error) {
      return '';
    }
  };

  const result = (protectedContentSkipped, captureSafe, sensitivityDigest, overflow, bodyDigest) => ({
    visibleText: '',
    visibleTextCharacterCount: 0,
    domOverflowRedacted: Boolean(overflow),
    privateContentRedacted: true,
    protectedContentSkipped,
    metaValues: '',
    accessibilityValues: '',
    documentUrl: documentUrl(),
    captureSafe,
    sensitivityDigest,
    bodyDigest,
  });

  const protectedResult = (overflow = false) =>
    result(true, false, protectedDigest, overflow, protectedBodyDigest);

  const safeAttribute = (node, name) => {
    const value = node.getAttribute(name);
    return value === null || typeof value === 'string' ? value || '' : null;
  };

  const bodyDigestFor = async (text) => {
    try {
      if (
        typeof crypto !== 'object' ||
        !crypto ||
        !crypto.subtle ||
        typeof TextEncoder !== 'function'
      ) {
        return '';
      }
      const bytes = await crypto.subtle.digest(
        'SHA-256',
        new TextEncoder().encode(text),
      );
      return `managed-browser-body-sha256-v1-${Array.from(
        new Uint8Array(bytes),
        (byte) => byte.toString(16).padStart(2, '0'),
      ).join('')}`;
    } catch (_error) {
      return '';
    }
  };

  try {
    if (
      typeof document !== 'object' ||
      !document ||
      !document.body ||
      typeof document.createTreeWalker !== 'function' ||
      !documentUrl() ||
      document.body.shadowRoot
    ) {
      return protectedResult();
    }
    const walker = document.createTreeWalker(document.body, 1 | 4);
    let nodeCount = 0;
    let textWork = 0;
    let bodyText = '';
    let node = walker.nextNode();
    while (node) {
      nodeCount += 1;
      if (nodeCount > maxNodes) {
        return protectedResult(true);
      }
      if (node.nodeType === 3) {
        if (typeof node.nodeValue !== 'string') {
          return protectedResult();
        }
        if (node.nodeValue.length > maxTextWork - textWork) {
          return protectedResult(true);
        }
        if (hasSensitiveMarker(node.nodeValue)) {
          return protectedResult();
        }
        textWork += node.nodeValue.length;
        bodyText += `${node.nodeValue}\u0000`;
      } else if (node.nodeType === 1) {
        if (
          typeof node.tagName !== 'string' ||
          typeof node.getAttribute !== 'function' ||
          typeof node.hasAttribute !== 'function'
        ) {
          return protectedResult();
        }
        const tagName = node.tagName.toLowerCase();
        if (
          tagName.includes('-') ||
          tagName === 'iframe' ||
          tagName === 'frame' ||
          tagName === 'object' ||
          tagName === 'embed' ||
          tagName === 'input' ||
          tagName === 'textarea' ||
          tagName === 'select' ||
          tagName === 'form' ||
          tagName === 'button' ||
          tagName === 'summary' ||
          node.shadowRoot
        ) {
          return protectedResult();
        }
        const role = safeAttribute(node, 'role');
        const autocomplete = safeAttribute(node, 'autocomplete');
        const name = safeAttribute(node, 'name');
        const id = safeAttribute(node, 'id');
        const placeholder = safeAttribute(node, 'placeholder');
        if ([role, autocomplete, name, id, placeholder].some((value) => value === null)) {
          return protectedResult();
        }
        if (
          hasSensitiveMarker(role) ||
          hasSensitiveMarker(autocomplete) ||
          hasSensitiveMarker(name) ||
          hasSensitiveMarker(id) ||
          hasSensitiveMarker(placeholder) ||
          node.hasAttribute('aria-label') ||
          node.hasAttribute('aria-labelledby') ||
          node.hasAttribute('aria-describedby')
        ) {
          return protectedResult();
        }
        if (
          node.hasAttribute('contenteditable') ||
          autocomplete !== '' ||
          name !== '' ||
          /textbox|combobox|spinbutton|searchbox|listbox|slider|button|menuitem/i.test(role)
        ) {
          return protectedResult();
        }
      } else {
        return protectedResult();
      }
      node = walker.nextNode();
    }
    // No affirmative owner-safe classification exists in this producer. The
    // body, metadata, and accessibility values therefore remain redacted even
    // when the bounded structural probe permits a frozen screenshot.
    const bodyDigest = await bodyDigestFor(bodyText);
    return bodyDigest
      ? result(false, false, unknownDigest, false, bodyDigest)
      : protectedResult();
  } catch (_error) {
    return protectedResult();
  }
})()
