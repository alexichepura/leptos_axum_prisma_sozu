use crate::ssr_number::SsrNumber;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/leptos_axum_prisma_sozu.css"/> // id=leptos for cargo-leptos hot-reload
        <Title text="Leptos Axum Prisma Sozu boilerplate"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" ssr=SsrMode::Async view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    view! { cx,
        <h1>"Welcome to 100% rust fullstack web boilerplate!"</h1>
        <ul class="links">
            <li><a href="https://github.com/leptos-rs/leptos">"Leptos: github.com/leptos-rs/leptos"</a></li>
            <li><a href="https://github.com/tokio-rs/axum">"Axum: github.com/tokio-rs/axum"</a></li>
            <li><a href="https://github.com/Brendonovich/prisma-client-rust">"Prisma: github.com/Brendonovich/prisma-client-rust"</a></li>
            <li><a href="https://github.com/sozu-proxy/sozu">"Sozu: github.com/sozu-proxy/sozu"</a></li>
        </ul>
        <button on:click=on_click>"Click Me: " {count}</button>
        <SsrNumber/>
    }
}
