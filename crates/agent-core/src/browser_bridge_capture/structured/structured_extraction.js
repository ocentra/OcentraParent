(() => {
  const limit = 480;
  const redacted = () => ({
    visibleText: '',
    visibleTextCharacterCount: 0,
    domOverflowRedacted: false,
    privateContentRedacted: true,
    protectedContentSkipped: true,
    metaValues: '',
    accessibilityValues: '',
  });
  try {
    if (!document || !document.body || typeof document.body.innerText !== 'string') {
      return redacted();
    }
    if (document.querySelector('iframe, frame, object, embed')) {
      return redacted();
    }
    if (typeof document.title !== 'string') {
      return redacted();
    }
    const pageIdentity = `${document.title} ${typeof location === 'object' && location ? location.href || '' : ''}`;
    if (/password|passcode|pin|security|secret|token|credential|one[- ]time|verification/i.test(pageIdentity)) {
      return redacted();
    }
    const controls = document.querySelectorAll(
      'input, textarea, select, form, button, summary, [contenteditable], [role="textbox"], [role="combobox"], [role="spinbutton"], [role="searchbox"], input[name], textarea[name], select[name], [contenteditable][name], input[autocomplete], textarea[autocomplete], select[autocomplete], [contenteditable][autocomplete]'
    );
    if (controls.length > 0) {
      return redacted();
    }
    const nodes = document.querySelectorAll('*');
    for (const node of nodes) {
      if (!node || typeof node.tagName !== 'string' || typeof node.getAttribute !== 'function') {
        return redacted();
      }
      const tagName = node.tagName.toLowerCase();
      if (tagName.includes('-') || node.shadowRoot) {
        return redacted();
      }
      if (tagName !== 'meta' && (node.hasAttribute('name') || node.hasAttribute('autocomplete'))) {
        return redacted();
      }
      const role = node.getAttribute('role') || '';
      const ariaLabel = node.getAttribute('aria-label') || '';
      const autocomplete = node.getAttribute('autocomplete') || '';
      const name = node.getAttribute('name') || '';
      const id = node.getAttribute('id') || '';
      const sensitiveAttributes = `${role} ${ariaLabel} ${autocomplete} ${name} ${id}`;
      if (/password|passcode|pin|security|secret|token|credential|one[- ]time|verification/i.test(sensitiveAttributes)) {
        return redacted();
      }
      if (/textbox|combobox|spinbutton|searchbox|listbox|slider|button|menuitem/i.test(role)) {
        return redacted();
      }
    }
    const bodyText = document.body.innerText;
    if (/password|passcode|pin|security code|credit card|cvv|ssn|secret|token|credential|one[- ]time|verification code/i.test(bodyText)) {
      return redacted();
    }
    const fullMetaValues = Array.from(document.querySelectorAll(
      'meta[name="description"], meta[property="og:title"], meta[property="og:description"]'
    )).slice(0, 6).map((node) => node.getAttribute('content') || '')
      .filter((value) => value.length > 0).join(' ');
    if (/password|passcode|pin|security|secret|token|credential|verification/i.test(fullMetaValues)) {
      return redacted();
    }
    const fullAccessibilityValues = Array.from(document.querySelectorAll(
      '[aria-label], [role]'
    )).slice(0, 32).map((node) => `${node.getAttribute('role') || ''}:${node.getAttribute('aria-label') || ''}`)
      .filter((value) => value !== ':').join(' ');
    if (/password|passcode|pin|security|secret|token|credential|verification/i.test(fullAccessibilityValues)) {
      return redacted();
    }
    const metaValues = fullMetaValues.slice(0, limit);
    const accessibilityValues = fullAccessibilityValues.slice(0, limit);
    const visibleText = bodyText.slice(0, limit);
    const visibleTextCharacterCount = Math.min(bodyText.length, limit);
    const domOverflowRedacted = bodyText.length > limit;
    return {
      visibleText,
      visibleTextCharacterCount,
      domOverflowRedacted,
      privateContentRedacted: false,
      protectedContentSkipped: false,
      metaValues,
      accessibilityValues,
    };
  } catch (_error) {
    return redacted();
  }
})()
