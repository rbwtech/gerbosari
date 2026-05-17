/**
 * Svelte action turning an <a href="/path"> into a hash-router internal link.
 * Falls through to default browser behaviour for external URLs, modifier-key
 * clicks (open-in-new-tab), and explicit hash-only links.
 */
export function link(node: HTMLAnchorElement) {
  function onClick(event: MouseEvent) {
    if (event.defaultPrevented) return;
    if (event.button !== 0) return;
    if (event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;

    const href = node.getAttribute('href');
    if (!href) return;
    if (/^[a-z]+:\/\//i.test(href)) return; // absolute external
    if (href.startsWith('mailto:') || href.startsWith('tel:')) return;
    if (href.startsWith('#')) return; // already hash-targeting

    event.preventDefault();
    const path = href.startsWith('/') ? href : `/${href}`;
    if (window.location.hash === `#${path}`) {
      window.dispatchEvent(new HashChangeEvent('hashchange'));
    } else {
      window.location.hash = `#${path}`;
    }
  }

  node.addEventListener('click', onClick);
  return {
    destroy() {
      node.removeEventListener('click', onClick);
    },
  };
}
