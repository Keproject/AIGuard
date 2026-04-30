# Deployment & CI/CD Configuration

## GitHub Pages Deployment
- **URL**: [https://keproject.github.io/AIGuard/](https://keproject.github.io/AIGuard/)
- **Bypass**: Added `.nojekyll` to disable Jekyll processing and allow underscore-prefixed directories or raw HTML/CSS assets.
- **Workflow**: `.github/workflows/deploy-pages.yml`
  - Triggers on every push to `main`.
  - Automatically builds and deploys the static site.

## Rust Windows Build
- **Workflow**: `.github/workflows/rust-windows.yml`
  - Targets: `x86_64-pc-windows-msvc`.
  - Performs linting, testing, and artifact generation for the core engine.

## Environment Requirements
- **OS**: Windows 10/11 (for the Core Engine).
- **Web**: Modern browser with JavaScript enabled (for the landing page).
- **Payment**: Live PayPal/SumUp integration (requires valid merchant configuration).
