# 01 — Hello SSR

Najmenší funkčný reťazec **Axum (HTTP server) + Leptos (SSR + hydratácia)**. Jedna route, statický obsah bez interaktívneho stavu v UI.

## Štruktúra priečinka

```text
01-hello-ssr/
├── Cargo.toml          # balík, features, cargo-leptos metadata
├── README.md           # táto dokumentácia
├── public/             # statické súbory → kopírujú sa do site-root pri build-e
├── style/
│   └── main.scss       # štýly; cargo-leptos ich spracuje a vygeneruje CSS do /pkg/
└── src/
    ├── main.rs         # binárka: len pri feature `ssr` — Tokio + Axum + Leptos routes
    ├── lib.rs          # knižnica: modul `app` + WASM vstup `hydrate` pri feature `hydrate`
    └── app.rs          # HTML shell, router, komponenty stránky
```

## `Cargo.toml` — čo ktorá časť znamená

| Sekcia | Účel |
|--------|------|
| `[package] name = "hello_ssr"` | Názov crate-u; musí sedieť s `use hello_ssr::…` v `main.rs`. |
| `[lib] crate-type = ["cdylib", "rlib"]` | `cdylib` = WASM pre prehliadač, `rlib` = knižnica pre serverový build. |
| `[dependencies]` | Verzie berie z koreňového workspace; `optional = true` u server-only / WASM-only závislostí. |
| `[features] hydrate` | Zapína WASM: `leptos/hydrate`, panic hook, `wasm-bindgen`. |
| `[features] ssr` | Zapína server: Axum, Tokio, `leptos_axum`, SSR režim Leptosu a routera/meta. |
| `[package.metadata.leptos]` | Inštrukcie pre **cargo-leptos**: výstupný názov WASM/JS, `site-root`, port, ktoré features použiť pre bin vs lib. |

Kľúčové kľúče v `metadata.leptos`:

- **`output-name`** — musí sedieť s cestou v `<Stylesheet href="/pkg/hello_ssr.css"/>` v `app.rs`.
- **`site-addr` / `reload-port`** — kde počúva server a live reload.
- **`bin-features = ["ssr"]`**, **`lib-features = ["hydrate"]`** — jeden build ide ako server, druhý ako WASM knižnica.

## `src/main.rs` — serverový vstup

Spustí sa len ak je zapnutá feature **`ssr`** (tak to nastavuje cargo-leptos pre binárku).

1. **`get_configuration(None)`** — načíta `Cargo.toml` metadata (adresa, cesty); v workspace vždy `None`, nie cesta k súboru.
2. **`generate_route_list(App)`** — zostaví zoznam ciest z Leptos routera (vrátane interných ciest pre server funkcie v iných príkladoch).
3. **`Router::new().leptos_routes(…, shell)`** — Axum dostane handlery pre stránky; `shell` obalí odpoveď do plného HTML dokumentu.
4. **`file_and_error_handler(shell)`** — fallback: statické súbory z `target/site` a pekná chyba 404.
5. **`axum::serve`** — klasický async HTTP server cez Tokio.

Ak **`ssr`** nie je zapnuté, `main` je prázdna — typické pri čistom WASM build-e knižnice.

## `src/lib.rs` — knižnica + WASM vstup

- **`pub mod app`** — verejný modul s UI logikou; server ho importuje ako `hello_ssr::app::*`.
- **`#[wasm_bindgen] pub fn hydrate()`** (len pri **`hydrate`**): po načítaní stránky v prehliadači zavolá **`hydrate_body(App)`**, aby sa statické HTML „oživilo“ rovnakým Rust kódom (eventy, signály v ďalších príkladoch).

## `src/app.rs` — čo kde je

| Symbol | Úloha |
|--------|--------|
| **`shell(options)`** | Šablóna celého HTML dokumentu: `<head>` s `AutoReload`, `HydrationScripts`, `MetaTags`; v `<body>` len `<App/>`. |
| **`App`** | Koreňová komponenta: meta kontext, odkaz na CSS z buildu, `Title`, **Leptos Router** s jednou route na `/`. |
| **`HomePage`** | Obsah domovskej stránky — tu čisto statický text (žiadne signály). |

Router (`StaticSegment("")`) zodpovedá ceste `/`. **`fallback`** na `Routes` zobrazí text pri neznámej pod-ceste.

## `style/main.scss`

Zdroj CSS pre cargo-leptos (môže byť aj `.sass`/`.scss`). Výsledok sa servuje ako `/pkg/hello_ssr.css` (názov z `output-name`).

## `public/`

Súbory sa kopírujú do `site-root` (`target/site`). Tu je prázdny priečinok s `.gitkeep`; môžeš pridať napr. `favicon.ico`.

## Spustenie

```bash
cd examples/01-hello-ssr
cargo leptos watch
```

Rýchla kontrola typov len servera:

```bash
cargo check -p hello_ssr --features ssr
```

## Súvislosti mimo tohto príkladu

- **Symfony:** `main.rs` ≈ „kernel + server“; `app.rs` ≈ layout + jedna „akcia“ bez DB.
- **Twig:** `view! { … }` je šablónový jazyk v Ruste, nie externý súbor.
- Ďalší krok v repozitári: [`../02-counter`](../02-counter) (reaktívny stav), [`../03-server-fn-example`](../03-server-fn-example) (volanie servera z UI).
