import { defineConfig } from 'vite-plus'

export default defineConfig({
  staged: {
    '*': 'vp check --fix',
  },
  fmt: {
    singleQuote: true,
    semi: false,
    sortImports: true,
    sortTailwindcss: true,
    sortPackageJson: true,
    ignorePatterns: ['submodules/**', '**/*.toml', '*.toml'],
  },
  lint: {
    // Bun-managed project outside the pnpm workspace: its deps are not installed here.
    ignorePatterns: ['submodules/**', 'examples/pocket-hmi/**'],
    jsPlugins: [{ name: 'vite-plus', specifier: 'vite-plus/oxlint-plugin' }],
    rules: { 'vite-plus/prefer-vite-plus-imports': 'error' },
    options: { typeAware: true, typeCheck: true },
  },
})
