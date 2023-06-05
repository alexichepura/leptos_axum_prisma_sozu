use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos::{use_context, Scope, ServerFnError};
    use prisma_client::db::PrismaClient;
    use std::sync::Arc;

    // let prisma_client = use_prisma(cx)?;
    pub fn use_prisma(cx: Scope) -> Result<Arc<PrismaClient>, ServerFnError> {
        use_context::<Arc<PrismaClient>>(cx)
            .ok_or("Prisma missing.")
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
}}
