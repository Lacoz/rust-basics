# 03 — Server funkcia (`server_fn_example`)

Ukážka **server function**: funkcia označená `#[server]`, ktorá beží len na serveri (async Tokio), ale z UI ju voláš ako bežnú async operáciu s typovaným vstupom/výstupom — bez vlastného JSON API a ručného `fetch` v JavaScripte.

> **Názov crate-u** je `server_fn_example`, nie `server_fn`, aby nekolidoval s crates.io balíkom `server_fn`, ktorý Leptos používa interně.

## Štruktúra priečinka

```text
03-server-fn-example/
├── Cargo.toml
├── README.md
├── public/
├── style/
│   └── main.scss
└── src/
    ├── main.rs         # Axum + Leptos SSR (registruje aj endpointy pre server funkcie)
    ├── lib.rs          # mod app + hydrate()
    └── app.rs          # #[server] server_echo + shell + App + HomePage s ServerAction
```

## `Cargo.toml` — navyše oproti 01/02

| Položka | Účel |
|---------|------|
| **`serde`** | Server funkcie serializujú argumenty a návratové hodnoty (napr. `String`); Serde je potrebné pre generovaný kód makier. |

Zvyšok zodpovedá vzoru z [../01-hello-ssr/README.md](../01-hello-ssr/README.md).

Špecifické hodnoty v `[package.metadata.leptos]`:

- **`output-name`** = `server_fn_example` → CSS v `app.rs`: `/pkg/server_fn_example.css`
- **`site-addr`** = `127.0.0.1:3020`, **`reload-port`** = `3021`

## `src/main.rs`

Rovnaký pattern ako v predchádzajúcich príkladoch:

- `get_configuration(None)`, `generate_route_list(App)`, `LeptosRoutes`, `shell`, `file_and_error_handler`.

**`generate_route_list`** zoberie aj route-y vytvorené pre **server funkcie** (interné HTTP endpointy), takže netreba Axum handlery dopĺňať ručne pre každú `#[server]` funkciu.

Import: **`use server_fn_example::app::*`**.

## `src/lib.rs`

- **`pub mod app`**
- **`hydrate()`** — nutné pre interaktívne volanie server funkcie z prehliadača (WASM časť volá sieťový endpoint, ktorý zaregistruje server).

## `src/app.rs` — časti súboru odhora nadol

### 1. `#[server] pub async fn server_echo(input: String) -> Result<String, ServerFnError>`

- Kód tela funkcie sa **nekompiluje do WASM** — beží len na serveri.
- **`ServerFnError`** — štandardný typ chýb pre server funkcie.
- **`tokio::time::sleep`** — ilustrácia async práce (namiesto DB alebo HTTP volania von).

Makro vygeneruje typ (napr. **`ServerEcho`**) s poľami argumentov — ten istý typ používaš pri **`action.dispatch(ServerEcho { input: … })`**.

### 2. `shell(options)`

Rovnaká úloha ako v 01/02: plný HTML dokument, hydratácia, `App`.

### 3. `App`

Router, meta, štýly, jedna route na `/`.

### 4. `HomePage`

| Prvok | Účel |
|-------|------|
| **`RwSignal::new(String::from("Ahoj"))`** | Lokálny stav textového poľa (synchronizovaný s `<input>`). |
| **`ServerAction::<ServerEcho>::new()`** | Objekt na spustenie server funkcie a sledovanie výsledku / chyby. |
| **`prop:value` + `on:input:target`** | Obojsmerná väzba vstupu so signálom (`ev.target().value()`). |
| **`on:click` + `action.dispatch(ServerEcho { input: input.get() })`** | Odošle aktuálny text na server. |
| **Blok `{move || action.value().get() …}`** | Zobrazí `Ok` reťazec alebo text chyby po dokončení požiadavky. |

## `style/main.scss` a `public/`

Ako v ostatných príkladoch: globálne štýly a miesto pre statické súbory.

## Spustenie

```bash
cd examples/03-server-fn-example
cargo leptos watch
```

Adresa: **`http://127.0.0.1:3020`**.

## Porovnanie (krátko)

- **FastAPI / Symfony AJAX:** často vlastný `POST`, JSON schéma, parsovanie na klientovi. Tu je **jedna Rust funkcia** na oboch stranách kontraktu (generovaný most).
- **MySQL:** tento príklad **nepripája databázu**; `server_echo` je šablóna miesta, kde by si na serveri volal SQLx/Diesel.

Predchádzajúce príklady: [`../01-hello-ssr`](../01-hello-ssr), [`../02-counter`](../02-counter).
