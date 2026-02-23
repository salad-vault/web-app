use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::app::scroll_to;

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
            let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
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

    // Grid 420x420, cells 140px, centers at 70/210/350
    view! {
        <section class="hero" id="hero">
            <div class="hero__bg-grid"></div>
            <div class="container hero__inner">
                <div class="hero__content">
                    <h1 class="hero__title">
                        "Vos mots de passe,"
                        <br/>
                        <span class="hero__title-accent">"Personne ne peut y toucher"</span>
                    </h1>
                    <p class="hero__subtitle">
                        "Gardez le contrôle de vos données"
                    </p>
                    <div class="hero__actions">
                        <button class="btn btn--primary btn--lg btn--glow" on:click=move |_| scroll_to("pricing")>
                            "Protéger mes données"
                        </button>
                        <a href="https://github.com" target="_blank" rel="noopener" class="hero__secondary-link">
                            "Voir le code source →"
                        </a>
                    </div>
                </div>

                <div class="hero__visual">
                    <div class="hero-grid">
                        <svg class="hero-grid__lines" viewBox="0 0 420 420" fill="none">
                            // Horizontal connections (6)
                            <line x1="70" y1="70" x2="210" y2="70" class=move || line_class(0)/>
                            <line x1="210" y1="70" x2="350" y2="70" class=move || line_class(1)/>
                            <line x1="70" y1="210" x2="210" y2="210" class=move || line_class(2)/>
                            <line x1="210" y1="210" x2="350" y2="210" class=move || line_class(3)/>
                            <line x1="70" y1="350" x2="210" y2="350" class=move || line_class(4)/>
                            <line x1="210" y1="350" x2="350" y2="350" class=move || line_class(5)/>
                            // Vertical connections (6)
                            <line x1="70" y1="70" x2="70" y2="210" class=move || line_class(6)/>
                            <line x1="210" y1="70" x2="210" y2="210" class=move || line_class(7)/>
                            <line x1="350" y1="70" x2="350" y2="210" class=move || line_class(8)/>
                            <line x1="70" y1="210" x2="70" y2="350" class=move || line_class(9)/>
                            <line x1="210" y1="210" x2="210" y2="350" class=move || line_class(10)/>
                            <line x1="350" y1="210" x2="350" y2="350" class=move || line_class(11)/>
                        </svg>
                        <div class="hero-grid__nodes">
                            <div class="hero-grid__node" style="--delay: 0s"><img src="public/favicon.svg" alt=""/></div>
                            <div class="hero-grid__node" style="--delay: 0.1s"><img src="public/favicon.svg" alt=""/></div>
                            <div class="hero-grid__node" style="--delay: 0.2s"><img src="public/favicon.svg" alt=""/></div>
                            <div class="hero-grid__node" style="--delay: 0.15s"><img src="public/favicon.svg" alt=""/></div>
                            <div class="hero-grid__node hero-grid__node--accent" style="--delay: 0.25s"><img src="public/favicon.svg" alt=""/></div>
                            <div class="hero-grid__node" style="--delay: 0.3s"><img src="public/favicon.svg" alt=""/></div>
                            <div class="hero-grid__node" style="--delay: 0.35s"><img src="public/favicon.svg" alt=""/></div>
                            <div class="hero-grid__node" style="--delay: 0.4s"><img src="public/favicon.svg" alt=""/></div>
                            <div class="hero-grid__node" style="--delay: 0.45s"><img src="public/favicon.svg" alt=""/></div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
