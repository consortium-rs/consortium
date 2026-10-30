/**
 * Renders `site` to static HTML. Runs at build time (and on every dev request)
 * from the `landing-prerender` plugin in `vite.config.ts`, so the shipped page
 * is plain HTML and the only client script is `src/main.ts`.
 *
 * Layout and markup live here; words live in `src/content/site.ts`.
 */

import type { Accent, ArchNode, CodePanel, Link, SectionHead, Site } from '../content/types.ts'
import { escapeHtml as esc, highlight } from './highlight.ts'

export type SnippetLoader = (file: string) => string

/** Escapes text and turns `backticks` into inline code. */
function rich(text: string): string {
  return esc(text).replace(/`([^`]+)`/g, '<code class="inline-code">$1</code>')
}

function isExternal(href: string): boolean {
  return /^https?:\/\//.test(href)
}

function link({ label, href }: Link, cls = ''): string {
  const attrs = isExternal(href) ? ' rel="noopener"' : ''
  return `<a href="${esc(href)}"${cls ? ` class="${cls}"` : ''}${attrs}>${esc(label)}</a>`
}

// Lucide-style 24×24 stroke icons. CSS shows the one for the current theme.
const MOON =
  '<svg class="icon icon-moon" viewBox="0 0 24 24" aria-hidden="true"><path d="M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401"/></svg>'
const SUN =
  '<svg class="icon icon-sun" viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"/></svg>'

const accentClass = (accent: Accent): string => `tone-${accent}`

function logo(): string {
  return '<span class="logo-mark" aria-hidden="true"><span class="a"></span><span class="b"></span></span>'
}

function code(panel: CodePanel, load: SnippetLoader, extraClass = ''): string {
  const source = load(panel.snippet).replace(/\s+$/, '')
  return `<pre class="code ${extraClass}"><code>${highlight(source, panel.lang)}</code></pre>`
}

function windowPanel(panel: CodePanel, load: SnippetLoader, cls = ''): string {
  return `
  <div class="panel ${cls}">
    <div class="panel-bar">
      <span class="dot"></span><span class="dot"></span><span class="dot"></span>
      <span class="panel-title">${esc(panel.title)}</span>
    </div>
    ${code(panel, load)}
  </div>`
}

function sectionHead(head: SectionHead, small = false): string {
  return `
  <header class="section-head">
    <h2${small ? ' class="sm"' : ''}>${esc(head.title)}</h2>
    <p>${rich(head.lede)}</p>
  </header>`
}

// ── Sections ────────────────────────────────────────────────────────────────

function nav(site: Site): string {
  return `
<nav class="top">
  <a href="#" class="brand">${logo()}<span>Consortium</span></a>
  <div class="nav-right">
    <div class="nav-links">${site.nav.map((l) => link(l)).join('')}</div>
    <button class="theme-toggle" id="theme-toggle" type="button" aria-label="Switch to light theme" title="Switch to light theme">
      ${MOON}${SUN}
    </button>
    ${link({ label: 'GitHub →', href: site.links.github }, 'nav-github')}
  </div>
</nav>`
}

function hero(site: Site, load: SnippetLoader): string {
  const h = site.hero
  return `
<section class="hero">
  <div>
    <div class="eyebrow">${esc(h.eyebrow)}</div>
    <h1>${esc(h.title)}</h1>
    <p>${rich(h.lede)}</p>
    <div class="hero-cta">
      ${link(h.primary, 'btn-primary')}
      ${link(h.secondary, 'btn-ghost')}
    </div>
  </div>
  ${windowPanel(h.code, load, 'shadow')}
</section>`
}

function node(n: ArchNode, cls = '', style = ''): string {
  const preview = n.badge ? `<span class="badge">${esc(n.badge)}</span>` : ''
  return `
    <div class="node ${cls}"${style ? ` style="${style}"` : ''}>
      <div class="node-tag ${accentClass(n.accent)}">${esc(n.tag)}</div>
      <div class="node-title">${esc(n.title)}</div>
      <div class="node-sub">${esc(n.sub)}</div>
      ${preview}
    </div>`
}

function architecture(site: Site): string {
  const a = site.architecture
  const { nodes: n, links: l } = a
  const at = (col: number, row: number) => `grid-column: ${col}; grid-row: ${row};`
  const vlabel = (label: string, labelFirst: boolean, double = false) => {
    const line = double
      ? '<div class="vline-pair"><div class="vline"></div><div class="vline"></div></div>'
      : '<div class="vline grow"></div>'
    const text = `<span class="conn-label">${esc(label)}</span>`
    return labelFirst ? text + line : line + text
  }

  const desktop = `
  <div class="diagram-desktop" aria-hidden="true">
    ${node(n.secure, '', at(3, 1))}
    <div class="conn-v" style="${at(3, 2)}">${vlabel(l.secure, true)}</div>
    ${node(n.interface, '', at(1, 3))}
    <div class="conn-h" style="${at(2, 3)}"><div class="hline"></div></div>
    ${node(n.host, 'host', at(3, 3))}
    <div class="conn-h" style="${at(4, 3)}"><div class="hline"></div></div>
    ${node(n.ecosystem, '', at(5, 3))}
    <div class="conn-v" style="${at(3, 4)}">${vlabel(l.controller, false)}</div>
    ${node(n.controller, '', at(3, 5))}
    <div class="conn-v" style="${at(3, 6)}">${vlabel(l.wired, false, true)}</div>
    ${node(n.wired, '', at(3, 7))}
  </div>`

  const mobile = `
  <div class="diagram-mobile">
    ${node(n.secure, 'full')}
    <div class="conn-v mobile">${vlabel(l.secure, false)}</div>
    ${node(n.host, 'host full')}
    <div class="side-pair">${node(n.interface, 'compact')}${node(n.ecosystem, 'compact')}</div>
    <div class="conn-v mobile">${vlabel(l.controller, true)}</div>
    ${node(n.controller, 'full')}
    <div class="conn-v mobile">${vlabel(l.wired, false, true)}</div>
    ${node(n.wired, 'full')}
  </div>`

  return `
<section id="${a.id}" class="wrap pad-sm">
  ${sectionHead(a)}
  ${desktop}
  ${mobile}
  <p class="footnote">${rich(a.footnote)}</p>
</section>`
}

function manifest(site: Site, load: SnippetLoader): string {
  const m = site.manifest
  const tabs = m.tabs
    .map(
      (t, i) =>
        `<button class="tab${i === 0 ? ' active' : ''}" type="button" role="tab" id="tab-${i}" aria-controls="tabpanel-${i}" aria-selected="${i === 0}">${esc(t.title)}</button>`,
    )
    .join('')
  const panels = m.tabs
    .map(
      (t, i) =>
        `<div role="tabpanel" id="tabpanel-${i}" aria-labelledby="tab-${i}"${i === 0 ? '' : ' hidden'}>${code(t, load, 'wrap-code')}</div>`,
    )
    .join('')
  const commands = m.commands
    .map((c) => `<li><code class="cmd">${esc(c.cmd)}</code><span>${rich(c.desc)}</span></li>`)
    .join('')

  return `
<section id="${m.id}" class="manifest-band">
  <div class="manifest">
    <div>
      <h2 class="sm">${esc(m.title)}</h2>
      ${m.body.map((p) => `<p>${rich(p)}</p>`).join('')}
      <ul class="commands">${commands}</ul>
    </div>
    <div class="panel tabbed" data-tabs>
      <div class="tabs" role="tablist">${tabs}</div>
      ${panels}
    </div>
  </div>
</section>`
}

function transports(site: Site): string {
  const t = site.transports
  const cards = t.items
    .map(
      (it) => `
    <article class="transport-card">
      <div class="transport-head">
        <h3>${esc(it.name)}</h3>
        <span class="signal">${esc(it.signal)}</span>
      </div>
      <a class="crate-link ${accentClass(it.accent)}" href="https://docs.rs/${esc(it.crate)}" rel="noopener">${esc(it.crate)}</a>
      <p>${rich(it.desc)}</p>
      <div class="status">${rich(it.status)}</div>
    </article>`,
    )
    .join('')
  return `
<section id="${t.id}" class="wrap pad-sm">
  ${sectionHead(t)}
  <div class="transport-grid">${cards}</div>
  <p class="footnote">${rich(t.footnote)}</p>
</section>`
}

function logs(site: Site, load: SnippetLoader): string {
  const l = site.logs
  return `
<section class="wrap pad-sm">
  ${sectionHead(l, true)}
  ${windowPanel(l.code, load, 'log-panel')}
</section>`
}

function crates(site: Site): string {
  const c = site.crates
  const cards = c.items
    .map(
      (it) => `
    <article class="crate-card">
      <div class="crate-cat ${accentClass(it.accent)}">${esc(it.category)}${it.badge ? `<span class="pill pill-warn">${esc(it.badge)}</span>` : ''}</div>
      <a class="crate-name" href="https://crates.io/crates/${esc(it.name)}" rel="noopener">${esc(it.name)}</a>
      <p class="crate-desc">${rich(it.desc)}</p>
      <a class="crate-docs" href="https://docs.rs/${esc(it.name)}" rel="noopener">Docs →</a>
    </article>`,
    )
    .join('')
  return `
<section id="${c.id}" class="wrap pad-sm">
  ${sectionHead(c)}
  <div class="crates-grid">${cards}</div>
</section>`
}

function targets(site: Site): string {
  const t = site.targets
  const chips = t.chips
    .map(
      (c) => `
      <article class="chip-card support-${c.support}-card">
        <div class="chip-head">
          <div class="chip-name">${esc(c.name)}</div>
          <span class="pill support-${c.support}">${esc(c.supportLabel)}</span>
        </div>
        <div class="chip-cores">${esc(c.cores)}</div>
        <dl class="chip-meta">
          <div><dt>firmware</dt><dd>${esc(c.controller)}</dd></div>
          <div><dt>doorbell</dt><dd>${esc(c.doorbells)}</dd></div>
        </dl>
        <div class="chip-note">${esc(c.note)}</div>
      </article>`,
    )
    .join('')
  const groups = t.groups
    .map(
      (g) => `
      <div>
        <div class="target-col-head ${accentClass(g.accent)}">${esc(g.title)}</div>
        <div class="target-list">${g.triples.map((x) => `<span>${esc(x)}</span>`).join('')}</div>
      </div>`,
    )
    .join('')
  return `
<section id="${t.id}" class="targets-band">
  <div class="wrap pad-lg">
    ${sectionHead(t, true)}
    <div class="chips-grid">${chips}</div>
    <div class="targets-grid">${groups}</div>
  </div>
</section>`
}

function footer(site: Site): string {
  return `
<footer>
  <div class="footer-inner">
    <div class="footer-brand">${logo()}<span>${esc(site.footer.copyright)}</span></div>
    <div class="footer-links">${site.footer.links.map((l) => link(l)).join('')}</div>
  </div>
</footer>`
}

export function renderBody(site: Site, load: SnippetLoader): string {
  return [
    nav(site),
    '<main>',
    hero(site, load),
    architecture(site),
    manifest(site, load),
    transports(site),
    logs(site, load),
    crates(site),
    targets(site),
    '</main>',
    footer(site),
  ].join('\n')
}

export function renderHead(site: Site): string {
  const { title, description, url } = site.meta
  return `
    <title>${esc(title)}</title>
    <meta name="description" content="${esc(description)}">
    <meta property="og:title" content="${esc(title)}">
    <meta property="og:description" content="${esc(description)}">
    <meta property="og:url" content="${esc(url)}">
    <meta property="og:type" content="website">`
}
