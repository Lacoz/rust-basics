# Príklady v tomto workspace

Všetky tri sú **SSR + hydratácia**: HTML prvýkrát zostaví server (Axum + Leptos), potom rovnaká aplikácia na klientovi (WASM) pre interaktivitu. Build rieši **`cargo leptos`** (nie čisté `cargo run`, ak chceš WASM a statické súbory).

## Prehľad

Každý priečinok má vlastný **`README.md`** s popisom všetkých súborov a sekcií (`Cargo.toml`, `src/*`, `style/`, `public/`, metadata pre cargo-leptos).

| # | Priečinok | Dokumentácia | O čom to je |
|---|-----------|--------------|-------------|
| 1 | `01-hello-ssr` | [README.md](01-hello-ssr/README.md) | Minimálny shell: router, jedna stránka, bez interaktívneho stavu. |
| 2 | `02-counter` | [README.md](02-counter/README.md) | Reaktívny stav (`RwSignal`) — tlačidlá menia číslo. |
| 3 | `03-server-fn-example` | [README.md](03-server-fn-example/README.md) | `#[server]` funkcia volaná z UI — async na serveri, výsledok späť do komponentu. |

## Čo porovnať s tým, čo už poznáš

### `01-hello-ssr`

- **Symfony:** jedna „akcia“, ktorá vráti HTML — tu to delíš medzi `main.rs` (HTTP server, router) a `app.rs` (šablóna / komponenty).
- **FastAPI:** rozdiel je, že nevraciaš len JSON, ale celú stránku cez Leptos; stále ide o request → handler.
- **jQuery:** žiadny vlastný skript v šablóne — hydratácia prinesie WASM; na tejto stránke nemusíš nič klikať, ide o statický obsah.

### `02-counter`

- **jQuery:** namiesto `$('#x').text(n)` máš jednu hodnotu v signáli a v `view!` ju zobrazuješ; po kliknutí meníš signál.
- **Symfony / Twig:** Twig šablóna je väčšinou bez interaktívneho stavu po načítaní; tu stav žije v komponente na klientovi aj na serveri pri prvom renderi.
- **FastAPI:** stále nejde o API endpoint — stav je primárne UI vec (podobne ako keby si držal stav v prehliadači, ale deklarátívne).

### `03-server-fn-example`

- **FastAPI:** podobne ako keď máš `async def` endpoint, ktorý spracuje telo požiadavky — tu je to `#[server] async fn`, ktorú framework zavolá cez vlastný most (nie ručný `fetch` + JSON schéma, ktorú si píšeš celú sám).
- **Symfony:** blízke „internému“ JSON/AJAX endpointu pre jednu stránku, len s typovaným kontraktom medzi Rust kódom na klientovi a na serveri.
- **MySQL:** tento príklad stále **nepoužíva DB**; server funkcia len simuluje prácu servera (`tokio::time::sleep`). Ďalší krok mimo týchto dní by bol napojiť pool a SQL.

## Spustenie

```bash
cd examples/01-hello-ssr && cargo leptos watch
# alebo 02-counter, 03-server-fn-example — každý má vlastný port v Cargo.toml ([package.metadata.leptos] site-addr)
```
