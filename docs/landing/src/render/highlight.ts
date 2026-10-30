/**
 * A deliberately small build-time highlighter for the handful of languages the
 * landing page shows. It only needs to look right on our own snippets, not to
 * parse arbitrary code.
 */

import type { Lang } from '../content/types.ts'

export function escapeHtml(text: string): string {
  return text
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
}

type Rule = [cls: string, pattern: string]

const RUST_KEYWORDS = [
  'as',
  'async',
  'await',
  'break',
  'const',
  'continue',
  'crate',
  'else',
  'enum',
  'false',
  'fn',
  'for',
  'if',
  'impl',
  'in',
  'let',
  'loop',
  'match',
  'mod',
  'move',
  'mut',
  'pub',
  'ref',
  'return',
  'self',
  'Self',
  'static',
  'struct',
  'trait',
  'true',
  'type',
  'unsafe',
  'use',
  'where',
  'while',
]

const RUST: Rule[] = [
  ['cmt', String.raw`//[^\n]*`],
  ['str', String.raw`"(?:\\.|[^"\\\n])*"`],
  ['attr', String.raw`#!?\[[^\]\n]*\]`],
  ['kw', String.raw`\b(?:${RUST_KEYWORDS.join('|')})\b`],
  ['num', String.raw`\b(?:0x[\da-fA-F_]+|\d[\d_]*(?:\.\d+)?)(?:[iu](?:8|16|32|64|size))?\b`],
  ['mac', String.raw`\b[a-z_][a-z0-9_]*!`],
  ['ty', String.raw`\b[A-Z][A-Za-z0-9_]*\b`],
]

const TOML: Rule[] = [
  ['cmt', String.raw`#[^\n]*`],
  ['hdr', String.raw`^[ \t]*\[\[?[^\]\n]+\]\]?`],
  ['str', String.raw`"(?:\\.|[^"\\\n])*"`],
  ['kw', String.raw`\b(?:true|false)\b`],
  ['num', String.raw`\b(?:0x[\da-fA-F_]+|\d[\d_]*)\b`],
  ['key', String.raw`[A-Za-z_][\w.-]*(?=\s*=)`],
]

const LOG: Rule[] = [
  ['cmt', String.raw`^\d{4}-\d\d-\d\dT[\d:.]+Z`],
  ['kw', String.raw`\b(?:TRACE|DEBUG|INFO|WARN|ERROR)\b`],
  ['str', String.raw`(?<=(?:TRACE|DEBUG|INFO|WARN|ERROR) )[\w:]+(?=:)`],
  ['num', String.raw`\b(?:0x[\da-fA-F]+|\d+)\b`],
]

const RULES: Record<Lang, Rule[]> = { rust: RUST, toml: TOML, log: LOG, text: [] }

const compiled = new Map<Lang, RegExp>()

function regexFor(lang: Lang): RegExp | undefined {
  const rules = RULES[lang]
  if (rules.length === 0) return undefined
  let re = compiled.get(lang)
  if (!re) {
    re = new RegExp(rules.map(([cls, p]) => `(?<${cls}>${p})`).join('|'), 'gm')
    compiled.set(lang, re)
  }
  return re
}

/** Returns escaped HTML with `<span class="…">` around recognised tokens. */
export function highlight(source: string, lang: Lang): string {
  const re = regexFor(lang)
  if (!re) return escapeHtml(source)

  let out = ''
  let last = 0
  for (const match of source.matchAll(re)) {
    const index = match.index
    out += escapeHtml(source.slice(last, index))
    const cls = Object.entries(match.groups ?? {}).find(([, v]) => v !== undefined)?.[0]
    out += `<span class="${cls}">${escapeHtml(match[0])}</span>`
    last = index + match[0].length
  }
  return out + escapeHtml(source.slice(last))
}
