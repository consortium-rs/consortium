# Consortium landing page

The static site for [csti.dev](https://csti.dev), built with Vite+ and deployed
to Cloudflare Pages from `dist/` (see `wrangler.toml`).

```bash
vp install          # from the repository root
cd docs/landing
vp dev              # live preview at http://localhost:5173
vp run build        # type-check and write dist/
vp preview          # serve dist/
```

## Editing content

| What                                    | Where                  |
| --------------------------------------- | ---------------------- |
| Copy, cards, chips, targets, nav, links | `src/content/site.ts`  |
| Code samples and the log excerpt        | `src/snippets/*`       |
| Content schema                          | `src/content/types.ts` |
| Markup / layout                         | `src/render/page.ts`   |
| Styles and theme tokens                 | `src/styles.css`       |
| Theme toggle and code tabs              | `src/main.ts`          |

Text fields marked "rich" in `types.ts` accept `` `inline code` ``. Snippets are
plain `.rs`/`.toml`/`.log` files referenced by name from `site.ts`; adding a
tab to the manifest panel is one entry in `manifest.tabs`.

The page is rendered to static HTML at build time by the `landing-prerender`
plugin in `vite.config.ts`, so the shipped page needs no JavaScript to read.

Keep snippets in step with the crates: they mirror `examples/melt-pot` and the
current `Consortium.toml` schema (`docs/book/src/content/guide/manifest.md`).
When an API changes, update the snippet and any claim in `site.ts` with it.
