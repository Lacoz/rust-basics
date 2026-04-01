# 02 — Counter

Rovnaký architektonický základ ako 01, ale na domovskej stránke je **reaktívny stav** (`RwSignal`) a tlačidlá menia číslo po hydratácii v prehliadači.

## Štruktúra priečinka

```text
02-counter/
├── Cargo.toml
├── README.md
├── public/
├── style/
│   └── main.scss
└── src/
    ├── main.rs         # Axum + Leptos SSR (identický vzor ako v 01, iný import crate-u)
    ├── lib.rs          # mod app + hydrate()
    └── app.rs          # shell, App, HomePage so signálmi a tlačidlami
```

## `Cargo.toml`

Rovnaký vzor ako v [../01-hello-ssr/README.md](../01-hello-ssr/README.md) (najmä sekcia **Cargo.toml**), s týmito rozdielmi:

| Pole | Hodnota | Poznámka |
|------|---------|----------|
| `name` | `counter` | Import v `main.rs`: `use counter::app::*`. |
| `[package.metadata.leptos] output-name` | `counter` | Musí sedieť s `href="/pkg/counter.css"` v `app.rs`. |
| `site-addr` | `127.0.0.1:3010` | Iný port ako 01, aby oba príklady mohli bežať naraz. |
| `reload-port` | `3011` | Live reload pre tento projekt. |

## `src/main.rs`

Logika je rovnaká ako v príklade 01:

- načítanie konfigurácie, `generate_route_list(App)`, Axum `Router` + `leptos_routes` + `shell`,
- obsluha statických súborov a chýb,
- `#[tokio::main] async fn main`.

Jediný špecifický rozdiel je **`use counter::app::*`** podľa názvu crate-u.

## `src/lib.rs`

- **`pub mod app`**
- **`hydrate()`** — po načítaní stránky pripojí WASM k už vyrenderovanému HTML; bez toho by tlačidlá po prvom SSR renderi nefungovali interaktívne.

## `src/app.rs` — čo kde je

| Symbol | Úloha |
|--------|--------|
| **`shell`** | Rovnaký účel ako v 01: dokument HTML, skripty hydratácie, `App` v `<body>`. |
| **`App`** | Meta, štýly, titulok, router s jednou route; CSS cesta `/pkg/counter.css`. |
| **`HomePage`** | Tu je učebná časť: |

**`RwSignal::new(0)`** — drží celočíselný stav. Čítanie v `view!` cez `{count}`; zápis cez **`count.write()`**.

**`let dec = move |_| …` / `let inc = move |_| …`** — closure s **`move`**: vlastní prístup k signálu. Po kliknutí sa aktualizuje signál a Leptos prekreslí závislé časti view (podobná idea ako reaktívny framework, nie imperatívny jQuery).

**`on:click=dec`** — väzba udalosti na handler.

## `style/main.scss` a `public/`

Rovnaká úloha ako v 01: globálne štýly a voliteľné statické assety.

## Spustenie

```bash
cd examples/02-counter
cargo leptos watch
```

Server počúva na **`http://127.0.0.1:3010`** (podľa `site-addr`).

## Porovnanie (krátko)

- **jQuery:** tam by si po kliknutí menil DOM ručne; tu meníš **dáta** (signál) a UI sleduje štruktúru v `view!`.
- **Symfony:** stále jedna „stránka“, ale interaktivita je v komponente, nie v samostatnom `.js` súbore s selektormi.

Predchádzajúci krok: [`../01-hello-ssr`](../01-hello-ssr). Ďalší: [`../03-server-fn-example`](../03-server-fn-example).
