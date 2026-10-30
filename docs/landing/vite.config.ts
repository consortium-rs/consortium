import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'

import { defineConfig, type Plugin } from 'vite-plus'

import { site } from './src/content/site.ts'
import { renderBody, renderHead } from './src/render/page.ts'

const snippetsDir = fileURLToPath(new URL('./src/snippets/', import.meta.url))

/**
 * Renders `src/content/site.ts` and `src/snippets/*` into index.html, so the
 * built page is static HTML. Snippets are read on every request in dev; edits
 * to the content or renderer modules restart the dev server.
 */
function prerender(): Plugin {
  const load = (file: string) => readFileSync(snippetsDir + file, 'utf8')
  return {
    name: 'landing-prerender',
    configureServer(server) {
      server.watcher.add(snippetsDir)
      server.watcher.on('change', (file) => {
        if (file.startsWith(snippetsDir)) server.ws.send({ type: 'full-reload' })
      })
    },
    transformIndexHtml(html) {
      return html
        .replace('<!-- landing:head -->', renderHead(site))
        .replace('<!-- landing:body -->', renderBody(site, load))
    },
  }
}

export default defineConfig({
  plugins: [prerender()],
})
