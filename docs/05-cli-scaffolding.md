# CLI Scaffolding

`create-rust-wokrers` is the optional CLI that copies the boilerplate and walks through setup.

## Run

```bash
node cli/dist/index.js
```

Or build it first:

```bash
cd cli
node build.js
node dist/index.js
```

## What It Does

- copies the boilerplate
- replaces project names and domains
- helps create Cloudflare resources
- installs dependencies
- runs migrations
- helps set secrets
- runs build and deployment steps

## Best Use Case

Use the CLI when you are starting a brand-new project from the boilerplate. Skip it if you are already working inside an existing copy.
