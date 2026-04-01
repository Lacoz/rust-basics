use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

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
        <Stylesheet id="leptos" href="/pkg/counter.css"/>
        <Title text="02 — Counter"/>
        <Router>
            <main>
                <Routes fallback=|| "Stránka nenájdená.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Reaktívny stav: namiesto jQuery `$('#n').text(x)` držíš hodnotu v signáli a UI sa viaže na ňu.
#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let dec = move |_| *count.write() -= 1;
    let inc = move |_| *count.write() += 1;

    view! {
        <h1>"Počítadlo"</h1>
        <p>"Hodnota: " <strong>{count}</strong></p>
        <button on:click=dec>"−1"</button>
        <button on:click=inc>"+1"</button>
    }
}
