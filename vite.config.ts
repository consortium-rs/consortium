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
    ignorePatterns: ['submodules/**', 'docs/landing/**', '**/*.toml', '*.toml'],
  },
  lint: {
    ignorePatterns: ['submodules/**'],
    jsPlugins: [{ name: 'vite-plus', specifier: 'vite-plus/oxlint-plugin' }],
    rules: { 'vite-plus/prefer-vite-plus-imports': 'error' },
    options: { typeAware: true, typeCheck: true },
  },
})
