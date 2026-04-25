# Security Policy

## Supported Versions

The `main` branch is the supported development line.

## Reporting A Vulnerability

Please do not open a public issue for a suspected vulnerability.

Report security issues by email to the project maintainer or through the private security advisory feature of the hosting platform. Include:

- A short description of the issue
- Affected endpoint, module, or file
- Reproduction steps
- Impact and suggested fix, if known

## Secret Handling

- Never commit `.dev.vars`, `.env`, certificates, Cloudflare API tokens, OAuth secrets, or production IDs.
- Use `rust-workers/.dev.vars.example` as the local template.
- Store production secrets with `wrangler secret put` or your deployment platform secret store.
- Keep public config values generic in open-source examples.
