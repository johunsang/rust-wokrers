# Design Theme Presets (30)
Just tell the AI ​​the name of your theme and the entire design will change.
## How to use
```
“Change the theme to synthwave”“Apply and distribute the cloud-dancer theme”```

## Validation

Run every preset against both Vite apps:

```bash
pnpm test:themes
```

The test injects each theme into `apps/landing` and `apps/admin`, builds both apps, removes generated `dist` folders, and restores the original styles.

## Dark theme (11)
| theme | Style | Note ||------|--------|------|
| dark-glass | Glasmorphism (default) | Linear, Raycast || stripe-gradient | Gradient Vibrant | Stripe, Framer || terminal-mono | Terminal/Monospace | GitHub CLI, Warp || neon-cyber | neon cyberpunk | PlanetScale, Railway || glow-aurora | Aurora Glow | Supabase, Resend || bento-clean | Bento Grid Dark | shadcn/ui, Cal.com || synthwave | 80s synthwave | Synthwave aesthetic || mesh-gradient | Multicolor mesh | Apple Music || infrared-purple | Purple+Red Energy | Creative Tools || jewel-tone | Jewel Tone Luxury | Premium SaaS || grayscale-yellow | Monochrome + Yellow | IKEA, developer blog |
## Light Theme (19)
| theme | Style | Note ||------|--------|------|
| linear-minimal | Bright minimalist | Notion, Vercel || soft-pastel | Pastel gradient | Canva, Monday || corporate-blue | Corporate Blue | Salesforce || earth-organic | Earth Tone | Basecamp || neo-brutalism | Brutalism | Gumroad || retro-digital | Retro Digital | Clerk, Deno || bold-serif | Large serif | Pitch, Webflow || split-contrast | Split High Contrast | Loom || organic-blob | Blob shape | Notion AI || neumorphism | Soft UI 3D | Apple Settings || claymorphism | Clay 3D | Figma, Pitch || outline-skeletal | Outline only | wireframe || mint-fresh | Mint Fresh | Wellness App || fresh-modern | Turquoise+Pink | Social SaaS || sorbet | Peach/Cream | Lifestyle || cloud-dancer | Pantone 2026 | Apple Clean || green-eco | Eco-friendly green | ESG platform || dopamine | high saturation energy | Miro, Figma || frost-ui | Frost/Ice Blue | Windows Fluent |
