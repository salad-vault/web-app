use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Pricing() -> impl IntoView {
    let (clicked, set_clicked) = signal(false);

    let on_cta_click = move |_| {
        set_clicked.set(true);
        // Reset after animation.
        let cb = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            set_clicked.set(false);
        });
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                1500,
            );
        }
        cb.forget();
    };

    view! {
        <section class="pricing animate-on-scroll" id="pricing">
            <div class="container">
                <h2 class="section-title">"Gratuit. Pour de vrai."</h2>
                <p class="section-subtitle">"Pas de periode d'essai. Pas de carte bancaire. Pas de piege."</p>

                <div class="pricing__card-wrap">
                    <div class=move || if clicked.get() { "pricing__card pricing__card--clicked" } else { "pricing__card" }>
                        <div class="pricing__card-badge">"Recommande"</div>
                        <h3 class="pricing__plan-name">"Jardinier Amateur"</h3>
                        <div class="pricing__price">
                            <span class="pricing__price-amount">"0€"</span>
                            <span class="pricing__price-period">"/ pour toujours"</span>
                        </div>
                        <ul class="pricing__features">
                            <li class="pricing__feature">
                                <span class="pricing__feature-check">"✓"</span>
                                "Saladiers illimites en local"
                            </li>
                            <li class="pricing__feature">
                                <span class="pricing__feature-check">"✓"</span>
                                "Chiffrement XChaCha20-Poly1305"
                            </li>
                            <li class="pricing__feature">
                                <span class="pricing__feature-check">"✓"</span>
                                "Double verrouillage (Dual-Lock)"
                            </li>
                            <li class="pricing__feature">
                                <span class="pricing__feature-check">"✓"</span>
                                "Generateur de mots de passe"
                            </li>
                            <li class="pricing__feature">
                                <span class="pricing__feature-check">"✓"</span>
                                "Import / Export (CSV, XML)"
                            </li>
                            <li class="pricing__feature">
                                <span class="pricing__feature-check">"✓"</span>
                                "Kit de secours (phrase de recuperation)"
                            </li>
                        </ul>
                        <button class="btn btn--primary btn--lg btn--glow btn--full" on:click=on_cta_click>
                            "Telecharger gratuitement"
                        </button>
                        <span class="pricing__no-card">"Pas de carte bancaire requise"</span>
                    </div>
                </div>

                <p class="pricing__future">
                    "Besoin de synchronisation cloud ? "
                    <span class="pricing__future-link">"Les plans Pro arrivent bientot."</span>
                </p>
            </div>
        </section>
    }
}
