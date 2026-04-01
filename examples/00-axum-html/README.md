# 00 — Axum HTML (bez JavaScriptu)

Najmenší možný „web“ v tomto workspace: **Axum** vráti **HTML alebo plain text** priamo z `async` handlera. Funguje v **prehliadači aj cez `curl`**; **nepotrebuje JS** na klientovi ani `cargo-leptos`.

## Štruktúra

```text
00-axum-html/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs    # binárka: router, dve route, Tokio server
```

## `Cargo.toml`

| Položka | Účel |
|---------|------|
| `name = "axum_html"` | Názov crate-u a výstupnej binárky. |
| `[[bin]]` | Explicitne jedna spustiteľná binárka z `src/main.rs`. |
| `axum`, `tokio` | Z koreňového workspace; žiadny Leptos → menší build ako pri `01+`. |

## `src/main.rs`

| Časť | Účel |
|------|------|
| **`Router`** | Dve cesty: `/` (HTML), `/text` (plain text). |
| **`server_facts()`** | Dynamický obsah: čas, OS/arch z `std::env::consts`, verzia crate-u z `env!`. |
| **`plain_info`** | `Content-Type: text/plain` — vhodné na testovanie cez `curl`. |
| **`html_info`** | Jednoduchý HTML dokument; reťazec s faktami je pred vložením do `<pre>` escapovaný (`&`, `<`, `>`). |
| **`axum::serve`** | Štandardný Tokio HTTP server. |

**Port** `3090` je konštanta `ADDR` v `main.rs` (aby nekolidoval s Leptos príkladmi na 3000/3010/3020).

## Spustenie

```bash
cd examples/00-axum-html
cargo run -p axum_html
```

Test:

```bash
curl -s http://127.0.0.1:3090/text
curl -s http://127.0.0.1:3090/ | head
```

## Porovnanie

- **PHP `phpinfo()`:** podobná idea — jedna stránka s informáciami zo servera, bez frontend frameworku.
- **FastAPI:** jedna route vracajúca `HTMLResponse` / `PlainTextResponse` — tu Axum `Html` alebo vlastné hlavičky.
- **Ďalší krok v repozitári:** [`../01-hello-ssr`](../01-hello-ssr) pridáva Leptos (šablóny v Ruste, SSR, neskôr WASM).
