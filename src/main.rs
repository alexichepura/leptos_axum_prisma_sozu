use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "ssr")] {
    use axum::{
        body::Body as AxumBody,
        extract::{Extension, State},
        http::Request,
        response::{IntoResponse, Response},
        routing::{get},
        Router,
    };
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_axum_prisma_sozu::{app::App, fileserv::file_and_error_handler, state::AppState};
    use leptos_axum_prisma_sozu::ssr_number::GetSsrNumber;
    use std::sync::Arc;
    use prisma_client::db;

    #[tokio::main]
    async fn main() {
        simple_logger::init_with_env().expect("couldn't initialize logging");

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

        let _ = GetSsrNumber::register();

        // use axum::extract::{Path, RawQuery};
        // use http::HeaderMap;
        // use leptos_axum::handle_server_fns_with_context;
        // async fn server_fn_handler(
        //     State(app_state): State<AppState>,
        //     path: Path<String>,
        //     headers: HeaderMap,
        //     raw_query: RawQuery,
        //     request: Request<AxumBody>
        // ) -> impl IntoResponse {
        //     handle_server_fns_with_context(path, headers, raw_query, move |cx| {
        //         provide_context(cx, app_state.prisma_client.clone());
        //     }, request).await
        // }

        async fn leptos_routes_handler(
            State(app_state): State<AppState>,
            req: Request<AxumBody>,
        ) -> Response {
            let path = req.uri().path().to_string();
            let headers = req.headers();
            let user_agent: Option<String> = match headers.get("user-agent") {
                Some(ua_header) => Some(ua_header.to_str().unwrap().to_string()),
                _ => None
            };
            let prisma_client = app_state.prisma_client.clone();
            tokio::spawn(async move {
                // println!("path: {path}, ua: {}", user_agent.clone().unwrap());
                // microdelay to prevent ssr number get race condition
                std::thread::sleep(std::time::Duration::from_millis(20));
                let result = app_state.prisma_client.clone().ssr().create(path, vec![
                    db::ssr::SetParam::SetUserAgent(user_agent)
                ]).exec().await;
                if let Err(query_error) = result{
                    dbg!(query_error);
                }
            });
            let handler = leptos_axum::render_app_async_with_context(
                app_state.leptos_options.clone(),
                move |cx| {
                    provide_context(cx, prisma_client.clone());
                },
                |cx| view! { cx, <App/> },
            );
            handler(req).await.into_response()
        }

        let client = if let Ok(db_url) = std::env::var("DATABASE_URL") {
            db::new_client_with_url(db_url.as_str()).await
        } else {
            db::new_client().await
        };
        let prisma_client = Arc::new(client.unwrap());
        #[cfg(debug)]
        prisma_client._db_push(false).await.unwrap();

        let app_state = AppState {
            leptos_options: leptos_options.clone(),
            prisma_client
        };

        let app = Router::new()
            .route("/api/*fn_name", axum::routing::post(leptos_axum::handle_server_fns))
            // .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            .fallback(file_and_error_handler)
            .with_state(app_state)
            .layer(Extension(Arc::new(leptos_options.clone())));

        #[cfg(not(debug_assertions))]
        let app = {
            use tower_http::{compression::CompressionLayer, services::ServeDir};
            let pkg_path = "/".to_owned() + &leptos_options.site_pkg_dir;
            let pkg_dir = leptos_options.site_root.clone() + &pkg_path;
            let pkg_router = Router::new()
                .nest_service(
                    &pkg_path,
                    ServeDir::new(pkg_dir)
                        .precompressed_br()
                        .precompressed_deflate()
                        .precompressed_gzip()
                        .precompressed_zstd(),
                )
                .route_layer(CompressionLayer::new());
            app.merge(pkg_router).layer(CompressionLayer::new())
        };

        log!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
} else {
    pub fn main() {}
}}
