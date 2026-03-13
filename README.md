# SaladVault Website

Site vitrine marketing pour le gestionnaire de mots de passe [SaladVault](https://saladvault.com).

Application 100% client-side (CSR) — aucun appel backend, aucune donnee collectee.

## Stack

- **Langage :** Rust (compile en WebAssembly)
- **Framework :** Leptos 0.8 (CSR)
- **Build :** Trunk
- **Style :** CSS custom properties, BEM-like

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2021)
- [Trunk](https://trunkrs.dev/) : `cargo install trunk`
- Target WASM : `rustup target add wasm32-unknown-unknown`

## Developpement

```bash
trunk serve
```

Serveur de dev avec hot reload sur `http://localhost:3000`.

## Build production

```bash
trunk build --release
```

Les fichiers statiques sont generes dans `dist/`.

## Structure

```
src/
  main.rs          Point d'entree
  app.rs           Composant racine
  sections/        Un fichier par section de la landing page
styles.css         Feuille de style principale
index.html         Template HTML (Trunk)
```

## Licence

[AGPL-3.0-or-later](LICENSE)
