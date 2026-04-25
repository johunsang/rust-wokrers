-- Make GitHub the primary landing-page destination for existing databases.
UPDATE site_settings
SET
  brand = 'rust-wokrers',
  hero_label = 'GitHub-first SaaS boilerplate',
  hero_title = 'Build from the repository, deploy at the edge.',
  hero_subtitle = 'A Rust Worker and Vite starter that uses GitHub as the public front door for source, docs, releases, and collaboration.',
  cta_primary = 'Open GitHub',
  cta_secondary = 'Open admin',
  updated_at = CURRENT_TIMESTAMP
WHERE id = 1;
