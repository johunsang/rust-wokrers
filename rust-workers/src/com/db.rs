use wasm_bindgen::JsValue;
use worker::{D1Database, Result};

use crate::com::env::iso_now;
use crate::com::types::{CountRow, SiteSettings, SiteSettingsDto};

pub async fn d1_all<T>(db: &D1Database, sql: &str, params: &[JsValue]) -> Result<Vec<T>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let statement = if params.is_empty() {
        db.prepare(sql)
    } else {
        db.prepare(sql).bind(params)?
    };
    statement.all().await?.results::<T>()
}

pub async fn d1_first<T>(db: &D1Database, sql: &str, params: &[JsValue]) -> Result<Option<T>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let statement = if params.is_empty() {
        db.prepare(sql)
    } else {
        db.prepare(sql).bind(params)?
    };
    statement.first::<T>(None).await
}

pub async fn count_query(db: &D1Database, sql: &str) -> Result<i64> {
    Ok(d1_first::<CountRow>(db, sql, &[])
        .await?
        .map(|row| row.count)
        .unwrap_or(0))
}

pub async fn get_site_settings(db: &D1Database) -> Result<SiteSettings> {
    Ok(d1_first::<SiteSettings>(
        db,
        "SELECT id, brand, hero_label, hero_title, hero_subtitle, cta_primary, cta_secondary, updated_at FROM site_settings WHERE id = 1",
        &[],
    )
    .await?
    .unwrap_or_else(default_site_settings))
}

pub fn site_settings_dto(settings: SiteSettings) -> SiteSettingsDto {
    SiteSettingsDto {
        id: settings.id,
        brand: settings.brand,
        hero_label: settings.hero_label,
        hero_title: settings.hero_title,
        hero_subtitle: settings.hero_subtitle,
        cta_primary: settings.cta_primary,
        cta_secondary: settings.cta_secondary,
        updated_at: settings.updated_at,
    }
}

fn default_site_settings() -> SiteSettings {
    SiteSettings {
        id: 1,
        brand: "rust-wokrers".to_string(),
        hero_label: "Rust Worker + Vite starter".to_string(),
        hero_title: "A Cloudflare-native SaaS shell rendered at the edge.".to_string(),
        hero_subtitle:
            "Use full mode for landing/admin/CMS screens, or api-only mode for service deployments."
                .to_string(),
        cta_primary: "Open GitHub".to_string(),
        cta_secondary: "Open admin".to_string(),
        updated_at: iso_now(),
    }
}
