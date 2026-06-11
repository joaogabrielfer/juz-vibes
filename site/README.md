# Juz Language Site

Static documentation site for the Juz programming language, built with Astro and Starlight.

## Source layout

- `docs/` contains the authored markdown copied from Obsidian.
- `scripts/prepare-docs.mjs` converts Obsidian-style wikilinks and generates the Starlight content tree.
- `src/content/docs/` is generated at build time and is ignored by Git.
- `src/generated/` is generated at build time and is ignored by Git.

## Local development

```bash
npm install
npm run dev
```

The dev and build scripts regenerate the site content from `docs-src/` before Astro starts.

## Updating docs

1. Create or edit the highest `MAJOR.MINOR` source folder in `docs/`, usually with a `v` prefix like `docs/v0.12/`.
2. Keep that version's `index.md` as the canonical table of contents and sidebar order.
3. Run `npm run dev` for local work or `npm run prepare-docs` if you only want to refresh generated content.

The build picks the highest version directory matching `vMAJOR.MINOR` or `MAJOR.MINOR`, so pushing a new folder such as `docs/v0.13/` to `main` makes that version the deployed site.

## CI

The monorepo CI builds this site only when files under `site/` change.
