use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn SsrNumber(cx: Scope) -> impl IntoView {
    let ssr_number_data = create_blocking_resource(cx, || (), move |_| get_ssr_number(cx));
    view! { cx,
        <Suspense  fallback=|| ()>
            {move || ssr_number_data.with(cx, |ssr_number| {
                    ssr_number.clone().map(|ssr_number| {
                        let number = ssr_number.count.to_string();
                        let meta: TextProp = ssr_number.count.to_string().into();
                        view! { cx,
                            <p>"SSR#": {&number}</p>
                            <Meta name="ssr:number" content=meta/>
                        }
                    })
                })
            }
        </Suspense>
    }
}

#[server(GetSsrNumber, "/api")]
pub async fn get_ssr_number(cx: Scope) -> Result<SsrNumber, ServerFnError> {
    let prisma_client = crate::prisma::use_prisma(cx)?;
    let count = prisma_client.ssr().count(vec![]).exec().await;
    Ok(SsrNumber {
        count: count.unwrap() as usize,
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SsrNumber {
    pub count: usize,
}
