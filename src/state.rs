use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "ssr")] {

use axum::extract::FromRef;
use leptos::LeptosOptions;
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub prisma_client: std::sync::Arc<prisma_client::db::PrismaClient>
}

}}
