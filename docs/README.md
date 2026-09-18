# Kavita Import Tool Docs 

Based on [Nextra](https://nextra.site).

Not looking to contribute? [Read the latest docs here](https://kavitaimporttool.mackenly.com/).

## Local Development

First, run `pnpm i` to install the dependencies.

Then, run `pnpm dev` to start the development server and visit localhost:3000.

## Cloudflare Pages

This is a static Next.js export. In the Pages project, use:

- **Root directory:** `docs`
- **Build command:** `pnpm run pages:build`
- **Output directory:** `out` (from `wrangler.toml`)
- **Environment variables:** `NODE_VERSION=20` and `PNPM_VERSION=9`

Node 20 is also set via `.nvmrc`. The v2 Pages image defaults to pnpm 8, which cannot read this repo's pnpm 9 lockfile, so `PNPM_VERSION=9` is required.

## License
Nextra is licensed under the MIT License, however, any modifications and the content are licensed under this project's license, found [here](../LICENSE).