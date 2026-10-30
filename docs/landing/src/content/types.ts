/**
 * Shape of the landing page content. `site.ts` fills it in; the renderer in
 * `src/render/page.ts` turns it into static HTML at build time.
 *
 * Any `string` field documented as "rich" accepts `inline code` in backticks,
 * which renders as monospace. Everything else is plain text and is escaped.
 */

export type Accent = 'accent' | 'accent2' | 'neutral'

/** Syntax highlighters available to code panels. */
export type Lang = 'rust' | 'toml' | 'log' | 'text'

export interface Link {
  label: string
  href: string
}

export interface CodePanel {
  /** Tab or window title shown above the code. */
  title: string
  /** File name under `src/snippets/`. */
  snippet: string
  lang: Lang
}

export interface SectionHead {
  /** Anchor id used by the nav, if the section is linked. */
  id?: string
  title: string
  /** Rich text. */
  lede: string
}

export interface ArchNode {
  tag: string
  title: string
  sub: string
  accent: Accent
  /** Optional maturity marker shown on the node, e.g. "untested". */
  badge?: string
}

export interface Architecture extends SectionHead {
  nodes: {
    secure: ArchNode
    interface: ArchNode
    host: ArchNode
    ecosystem: ArchNode
    controller: ArchNode
    wired: ArchNode
  }
  /** Labels on the vertical connectors. */
  links: {
    secure: string
    controller: string
    wired: string
  }
  /** Rich text. */
  footnote: string
}

export interface Command {
  cmd: string
  /** Rich text. */
  desc: string
}

export interface Manifest {
  id: string
  title: string
  /** Rich text paragraphs. */
  body: string[]
  commands: Command[]
  tabs: CodePanel[]
}

export interface Transport {
  name: string
  crate: string
  /** What wakes the receiver. */
  signal: string
  /** Rich text. */
  desc: string
  /** Rich text; how you get it today. */
  status: string
  accent: Accent
}

export interface Transports extends SectionHead {
  items: Transport[]
  /** Rich text. */
  footnote: string
}

export interface Crate {
  category: string
  /** crates.io name; also used for the docs.rs link. */
  name: string
  /** Rich text. */
  desc: string
  accent: Accent
  /** Optional maturity marker, e.g. "untested". */
  badge?: string
}

/**
 * - `official`: supported and verified on hardware.
 * - `derived`: reuses another chip's support without formal verification.
 * - `in-progress`: support is still being brought up.
 */
export type Support = 'official' | 'derived' | 'in-progress'

export interface Chip {
  name: string
  cores: string
  /** Core the generated firmware runs on. */
  controller: string
  doorbells: string
  support: Support
  /** Label for the support badge, e.g. "Derived from MP25". */
  supportLabel: string
  note: string
}

export interface TargetGroup {
  title: string
  accent: Accent
  triples: string[]
}

export interface Site {
  meta: {
    title: string
    description: string
    url: string
  }
  links: {
    github: string
    docs: string
  }
  nav: Link[]
  hero: {
    eyebrow: string
    title: string
    /** Rich text. */
    lede: string
    primary: Link
    secondary: Link
    code: CodePanel
  }
  architecture: Architecture
  manifest: Manifest
  transports: Transports
  logs: SectionHead & { code: CodePanel }
  crates: SectionHead & { items: Crate[] }
  targets: SectionHead & { chips: Chip[]; groups: TargetGroup[] }
  footer: {
    copyright: string
    links: Link[]
  }
}
