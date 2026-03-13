use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::app::{scroll_to, section_ids};

#[component]
pub fn Hero() -> impl IntoView {
    let (connections, set_connections) = signal([false; 12]);

    // Randomly toggle connections every 2.5s
    Effect::new(move |_| {
        // Initial random state (~55% active)
        set_connections.update(|conns| {
            for conn in conns.iter_mut() {
                *conn = js_sys::Math::random() > 0.45;
            }
        });

        // Every tick, flip ~30% of connections for organic movement
        let cb = Closure::new(move || {
            set_connections.update(|conns| {
                for conn in conns.iter_mut() {
                    if js_sys::Math::random() < 0.3 {
                        *conn = !*conn;
                    }
                }
            });
        }) as Closure<dyn Fn()>;

        if let Some(window) = web_sys::window() {
            let _interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                2500,
            );
        }
        cb.forget();
    });

    let line_class = move |idx: usize| {
        if connections.get()[idx] {
            "hero-grid__line hero-grid__line--active"
        } else {
            "hero-grid__line"
        }
    };

    view! {
        <section class="hero" id="hero">
            <div class="hero__bg-grid"></div>

            // Network grid as background visual
            <div class="hero__bg-network" aria-hidden="true">
                <div class="hero-grid">
                    <svg class="hero-grid__lines" viewBox="0 0 420 420" fill="none">
                        <line x1="70" y1="70" x2="210" y2="70" class=move || line_class(0)/>
                        <line x1="210" y1="70" x2="350" y2="70" class=move || line_class(1)/>
                        <line x1="70" y1="210" x2="210" y2="210" class=move || line_class(2)/>
                        <line x1="210" y1="210" x2="350" y2="210" class=move || line_class(3)/>
                        <line x1="70" y1="350" x2="210" y2="350" class=move || line_class(4)/>
                        <line x1="210" y1="350" x2="350" y2="350" class=move || line_class(5)/>
                        <line x1="70" y1="70" x2="70" y2="210" class=move || line_class(6)/>
                        <line x1="210" y1="70" x2="210" y2="210" class=move || line_class(7)/>
                        <line x1="350" y1="70" x2="350" y2="210" class=move || line_class(8)/>
                        <line x1="70" y1="210" x2="70" y2="350" class=move || line_class(9)/>
                        <line x1="210" y1="210" x2="210" y2="350" class=move || line_class(10)/>
                        <line x1="350" y1="210" x2="350" y2="350" class=move || line_class(11)/>
                    </svg>
                    <div class="hero-grid__nodes">
                        <div class="hero-grid__node" style="--delay: 0s">
                            <div class="hero-grid__icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg></div>
                            <span class="hero-grid__label">"Extension"</span>
                        </div>
                        <div class="hero-grid__node" style="--delay: 0.1s">
                            <div class="hero-grid__icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg></div>
                            <span class="hero-grid__label">"Desktop"</span>
                        </div>
                        <div class="hero-grid__node" style="--delay: 0.2s">
                            <div class="hero-grid__icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg></div>
                            <span class="hero-grid__label">"Extension"</span>
                        </div>
                        <div class="hero-grid__node" style="--delay: 0.15s">
                            <div class="hero-grid__icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="5" y="2" width="14" height="20" rx="2"/><line x1="12" y1="18" x2="12.01" y2="18"/></svg></div>
                            <span class="hero-grid__label">"Mobile"</span>
                        </div>
                        <div class="hero-grid__node hero-grid__node--accent" style="--delay: 0.25s">
                            <div class="hero-grid__icon"><img src="public/favicon.svg" alt=""/></div>
                            <span class="hero-grid__label">"SaladVault"</span>
                        </div>
                        <div class="hero-grid__node" style="--delay: 0.3s">
                            <div class="hero-grid__icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z"/></svg></div>
                            <span class="hero-grid__label">"Web"</span>
                        </div>
                        <div class="hero-grid__node" style="--delay: 0.35s">
                            <div class="hero-grid__icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.78 7.78 5.5 5.5 0 0 1 7.78-7.78zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/></svg></div>
                            <span class="hero-grid__label">"Device Key"</span>
                        </div>
                        <div class="hero-grid__node" style="--delay: 0.4s">
                            <div class="hero-grid__icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 16 12 14 15 10 9 8 12 2 12"/><path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/></svg></div>
                            <span class="hero-grid__label">"Cloud Sync"</span>
                        </div>
                        <div class="hero-grid__node" style="--delay: 0.45s">
                            <div class="hero-grid__icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg></div>
                            <span class="hero-grid__label">"Saladiers"</span>
                        </div>
                    </div>
                </div>
            </div>

            // Centered content overlay
            <div class="container hero__inner">
                <div class="hero__badge">
                    <span class="hero__badge-dot"></span>
                    "Gestionnaire de mots de passe Zero-Knowledge"
                </div>

                <h1 class="hero__title">
                    "SaladVault"
                    <br/>
                    <span class="hero__title-accent">"Personne ne peut y toucher"</span>
                </h1>

                <p class="hero__subtitle">
                    "Le gestionnaire de mots de passe open source a double verrouillage."
                    <br/>
                    "Vous controlez 100% de vos donnees. Le serveur ne voit rien."
                </p>

                <div class="hero__actions">
                    <button class="btn btn--primary btn--lg btn--glow" on:click=move |_| scroll_to(section_ids::PRICING)>
                        "Proteger mes donnees"
                    </button>
                    <a href="https://github.com/salad-vault" target="_blank" rel="noopener" class="btn btn--outline btn--lg">
                        <svg class="btn__icon" viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
                            <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/>
                        </svg>
                        "Voir le code source"
                    </a>
                </div>

                // Feature cards
                <div class="hero__features">
                    <div class="hero__feature-card">
                        <div class="hero__feature-icon">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                            </svg>
                        </div>
                        <h3 class="hero__feature-title">"Zero-Knowledge"</h3>
                        <p class="hero__feature-desc">"Le serveur ne voit jamais vos donnees en clair"</p>
                    </div>
                    <div class="hero__feature-card">
                        <div class="hero__feature-icon">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                                <rect x="3" y="11" width="18" height="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                            </svg>
                        </div>
                        <h3 class="hero__feature-title">"Double Verrouillage"</h3>
                        <p class="hero__feature-desc">"Mot de passe + cle locale pour ouvrir vos donnees"</p>
                    </div>
                    <div class="hero__feature-card">
                        <div class="hero__feature-icon">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/>
                            </svg>
                        </div>
                        <h3 class="hero__feature-title">"100% Open Source"</h3>
                        <p class="hero__feature-desc">"Code auditable sous licence AGPL-3.0"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
