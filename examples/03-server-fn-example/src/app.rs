use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

/// Beží len na serveri; klient ju volá cez generovaný most (podobná idea ako vlastný JSON endpoint vo FastAPI).
#[server]
pub async fn server_echo(input: String) -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(40)).await;
    Ok(format!("Server dostal: \"{input}\" (async Tokio)"))
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="sk">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/server_fn_example.css"/>
        <Title text="03 — Server funkcia"/>
        <Router>
            <main>
                <Routes fallback=|| "Stránka nenájdená.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let input = RwSignal::new(String::from("Ahoj"));
    let action = ServerAction::<ServerEcho>::new();

    view! {
        <h1>"Server funkcia"</h1>
        <label>
            "Text: "
            <input
                type="text"
                prop:value=move || input.get()
                on:input:target=move |ev| input.set(ev.target().value())
            />
        </label>
        <button on:click=move |_| {
            action.dispatch(ServerEcho { input: input.get() });
        }>
            "Odoslať na server"
        </button>
        <p>
            {move || {
                action
                    .value()
                    .get()
                    .map(|r| match r {
                        Ok(s) => s.clone(),
                        Err(e) => format!("Chyba: {e}"),
                    })
                    .unwrap_or_default()
            }}
        </p>
    }
}
