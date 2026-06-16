# Bottle: superinstance-website Deploy — 2026-06-15

## Summary
Deployed [SuperInstance/superinstance-website](https://github.com/SuperInstance/superinstance-website) to **GitHub Pages** (not Cloudflare Pages — repo uses GitHub Actions for deploy).

## Why not Cloudflare Pages
- The stored CLOUDFLARE_API_KEY (`cfk_...`) is invalid → returns "Invalid API Token" / 10502 rate-limited
- Repo already has GitHub Pages enabled with a `pages.yml` workflow (build_type: workflow)
- No wrangler.toml present; this is a pure static HTML site

## What was done
1. Cloned the repo
2. Added `_headers` file for proper `Content-Type: text/markdown` on `.md` files
3. Committed and pushed to `master` branch
4. Triggered the existing `.github/workflows/pages.yml` GitHub Actions workflow

## Deployed URLs
- **Homepage**: https://superinstance.github.io/superinstance-website/
- **Education**: https://superinstance.github.io/superinstance-website/education.html
- **Papers**: https://superinstance.github.io/superinstance-website/papers/
- **Markdown files**: served as `text/markdown; charset=utf-8` (via `_headers` equivalent on GitHub Pages)

## Verification
All 9 pages tested return HTTP 200. The `_headers` file correctly sets Content-Type for `.md` files.

## Custom Domain
The Cloudflare credentials need a working API token before DNS can be configured there. Once valid tokens exist, a CNAME to `superinstance.github.io` or direct CF Pages deployment can be set up.
