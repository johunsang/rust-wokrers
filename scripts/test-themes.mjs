#!/usr/bin/env node

import { execFileSync } from 'node:child_process'
import { rmSync, readdirSync, readFileSync, writeFileSync } from 'node:fs'
import { basename, join } from 'node:path'

const themeDir = '_templates/themes'
const landingStyles = 'apps/landing/src/styles.css'
const adminStyles = 'apps/admin/src/styles.css'
const expectedCount = 30

const themes = readdirSync(themeDir)
  .filter((file) => file.endsWith('.css'))
  .sort()

if (themes.length !== expectedCount) {
  console.error(`Expected ${expectedCount} themes, found ${themes.length}.`)
  process.exit(1)
}

const landingOriginal = readFileSync(landingStyles, 'utf8')
const adminOriginal = readFileSync(adminStyles, 'utf8')
const requiredTokens = [
  '--bg-base',
  '--text-primary',
  '--accent-warm',
  '--surface',
  '--border',
  '--admin-bg',
  '--sidebar-bg',
  '--content-bg',
]

function run(command, args) {
  execFileSync(command, args, {
    cwd: process.cwd(),
    stdio: 'pipe',
    env: process.env,
  })
}

function validateTheme(name, css) {
  const missing = requiredTokens.filter((token) => !css.includes(token))
  if (missing.length > 0) {
    throw new Error(`${name}: missing ${missing.join(', ')}`)
  }

  const opens = (css.match(/{/g) ?? []).length
  const closes = (css.match(/}/g) ?? []).length
  if (opens !== closes) {
    throw new Error(`${name}: mismatched braces (${opens} open, ${closes} close)`)
  }
}

try {
  for (const theme of themes) {
    const themePath = join(themeDir, theme)
    const css = readFileSync(themePath, 'utf8')
    const name = basename(theme, '.css')
    validateTheme(name, css)

    const injected = `\n\n/* Theme under test: ${name} */\n${css}\n`
    writeFileSync(landingStyles, landingOriginal + injected)
    writeFileSync(adminStyles, adminOriginal + injected)

    run('pnpm', ['--filter', '@rust-wokrers/landing', 'build'])
    run('pnpm', ['--filter', '@rust-wokrers/admin', 'build'])
    rmSync('apps/landing/dist', { recursive: true, force: true })
    rmSync('apps/admin/dist', { recursive: true, force: true })

    console.log(`ok ${name}`)
  }
} finally {
  writeFileSync(landingStyles, landingOriginal)
  writeFileSync(adminStyles, adminOriginal)
  rmSync('apps/landing/dist', { recursive: true, force: true })
  rmSync('apps/admin/dist', { recursive: true, force: true })
}

console.log(`All ${themes.length} design themes passed landing/admin build tests.`)
