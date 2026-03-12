use leptos::prelude::*;

use crate::app::{scroll_to, section_ids};

#[component]
pub fn FinalCta() -> impl IntoView {
    view! {
        <section class="final-cta" id="final-cta">
            <div class="final-cta__bg-grid"></div>
            <div class="container final-cta__inner">
                <h2 class="final-cta__title">
                    "Vos donnees meritent mieux qu'un mot de passe recycle."
                </h2>
                <p class="final-cta__subtitle">
                    "Rejoignez ceux qui ont repris le controle."
                </p>
                <button class="btn btn--primary btn--lg btn--glow" on:click=move |_| scroll_to(section_ids::PRICING)>
                    "Telecharger SaladVault — Gratuit et Open Source"
                </button>
                <p class="final-cta__platforms">
                    "Windows · macOS · Linux · Pas de carte bancaire"
                </p>
            </div>
        </section>
    }
}
