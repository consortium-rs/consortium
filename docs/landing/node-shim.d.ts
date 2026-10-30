// The two Node APIs vite.config.ts needs. Declared here instead of pulling in
// @types/node, which would force the workspace lockfile to re-resolve.
declare module 'node:fs' {
  export function readFileSync(path: string, encoding: 'utf8'): string
}
declare module 'node:url' {
  export function fileURLToPath(url: URL): string
}
