# Rust: Leptos + Axum (učebné príklady)

Jeden [Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) s malými webovými demami. Každý príklad má v `src/` menej ako ~200 riadkov Rustu, aby sa dal prečítať naraz.

### Prečo má projekt na disku gigabajty?

**Nie je to tvoj kód.** Priečinok [`target/`](https://doc.rust-lang.org/cargo/guide/build-cache.html) je **cache kompilácie**: stovky závislostí (Leptos, Tokio, Axum, WASM toolchain, …) sa prekladajú do **natívneho debug** buildu aj do **`wasm32-unknown-unknown`** (to často ide do `target/front/` pri `cargo-leptos`). Jedna sada `.rlib` / `.o` / debug symbolov je veľká; tri príklady zdieľajú časť závislostí, ale WASM a SSR stále narobia veľa súborov.

- Samotné zdrojáky + README v repozitári sú rádu **stoviek kilobajtov**.
- Po `cargo clean` v koreňovom adresári zmizne takmer celá veľkosť; pri ďalšom `cargo leptos watch` sa znova vytvorí (prvý build bude dlhší).
- `target/` je v [`.gitignore`](.gitignore) — do Gita sa necommituje, ak nepoužívaš `git add -f`.

```bash
cd /path/to/rust-basics
cargo clean
```

## Čo budeš potrebovať

- [Rust](https://www.rust-lang.org/tools/install) (stable; v repozitári je [`rust-toolchain.toml`](rust-toolchain.toml))
- Cieľ pre WASM: `rustup target add wasm32-unknown-unknown`
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos): `cargo install cargo-leptos --locked`

## Ako spustiť príklad

**Úplne bez JavaScriptu / WASM** (len Axum, ideálne na `curl`):

```bash
cd examples/00-axum-html
cargo run -p axum_html
# curl -s http://127.0.0.1:3090/text
```

**Leptos + Axum (SSR + hydratácia)** — z koreňa repozitára alebo z priečinka príkladu:

```bash
cd examples/01-hello-ssr
cargo leptos watch
```

Otvor v prehliadači adresu z výstupu (predvolene `http://127.0.0.1:3000` pre prvý Leptos príklad). Ďalšie Leptos demá majú porty `3010`, `3020`.

Na čistú kontrolu typov bez WASM buildu (rýchlejšie):

```bash
cargo check -p hello_ssr --features ssr
```

## Príklady

| Priečinok | Balík (crate) | Port / spustenie |
|-----------|---------------|------------------|
| [`examples/00-axum-html`](examples/00-axum-html) | `axum_html` | `3090`, `cargo run -p axum_html` |
| [`examples/01-hello-ssr`](examples/01-hello-ssr) | `hello_ssr` | 3000, `cargo leptos watch` |
| [`examples/02-counter`](examples/02-counter) | `counter` | 3010, `cargo leptos watch` |
| [`examples/03-server-fn-example`](examples/03-server-fn-example) | `server_fn_example` | 3020, `cargo leptos watch` |

V každom priečinku príkladu je **`README.md`** s dokumentáciou všetkých častí projektu (súbory, Cargo metadata, význam `main.rs` / `lib.rs` / `app.rs`).

Priečinok sa volá `03-server-fn-example`, lebo názov crate `server_fn` by kolidoval s crates.io balíkom `server_fn`, ktorý používa Leptos pod kapotou.

Ďalší prehľad a mapovanie na tvoje skúsenosti je v [`examples/README.md`](examples/README.md).

## Ak už poznáš… (Symfony, FastAPI, jQuery, MySQL)

Krátke mapovanie — nie náhrada za tutoriál:

- **Symfony routing / controller** — v týchto demách **Axum** `Router`: URL cesta ide na **handler** (`async fn`), podobná myšlienka ako „route → akcia“, len bez `front controller` v štýle starého `app.php`. Najjednoduchšie to vidíš v [`examples/00-axum-html`](examples/00-axum-html) (bez Leptosu).
- **Twig** — **Leptos** `view! { ... }` a `#[component]`: šablóna s logikou, ale všetko v Ruste a typovo prísnejšie ako v PHP.
- **FastAPI** (`@app.get`, `async def`, JSON) — Axum handlery sú tiež `async fn`; štruktúry na serializáciu rieši často **Serde** (analogicky k Pydantic, ale iný ekosystém). **Dependency injection** ako vo FastAPI tu nie je centrálny kontajner — závislosti typicky explicitne (`State`, konštruktory).
- **jQuery** (kliknutie → ručne zmeníš DOM) — v Leptose držíš **reaktívny stav** (napr. signály); UI sa viaže na stav, nie na postupné `$('#…').text(…)`.
- **Symfony / FastAPI interný `POST` + JSON** — v príklade 03 je **server funkcia**: jeden typovaný volací bod namiesto vlastného JSON API a ručného parsovania v JS (stále to nie je náhrada za verejné REST API — skôr „RPC“ v rámci aplikácie).
- **MySQL** — žiadny z príkladov **nepoužíva databázu** (držíme kód krátky). Typický ďalší krok v Ruste je napr. **SQLx** alebo **Diesel** s **Tokio** poolom — analogicky k PDO/Doctrine alebo SQLAlchemy.

Ďalšie zdroje: [The Rust Book](https://doc.rust-lang.org/book/), [Leptos Book](https://book.leptos.dev/).

## Nový príklad neskôr

1. Skopíruj napr. `examples/01-hello-ssr` do `examples/04-moj-nazov`.
2. V novom `Cargo.toml` zmeň `name`, `[package.metadata.leptos]` (`output-name`, `site-addr`, `reload-port`).
3. Pridaj cestu do `[workspace].members` v koreňovom [`Cargo.toml`](Cargo.toml).
4. Voliteľne dopíš riadok do [`examples/README.md`](examples/README.md) a pridaj vlastný **`README.md`** v priečinku príkladu (rovnaký štýl ako existujúce: štruktúra, `Cargo.toml`, každý súbor v `src/`).
