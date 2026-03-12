use leptos::prelude::*;

#[component]
pub fn PainPoints() -> impl IntoView {
    view! {
        <section class="pain-points animate-on-scroll" id="pain-points">
            <div class="container">
                <h2 class="section-title">"parce que ça n'arrive pas qu'aux autres."</h2>

                <div class="pain-points__grid">
                    // Card 1 - Data breach
                    <div class="pain-card">
                        <div class="pain-card__icon-badge">
                            <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                <path d="M24 4L6 12v12c0 11.1 7.7 21.5 18 24 10.3-2.5 18-12.9 18-24V12L24 4z" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                <line x1="14" y1="14" x2="34" y2="34" stroke="currentColor" stroke-width="2.5"/>
                                <line x1="34" y1="14" x2="14" y2="34" stroke="currentColor" stroke-width="2.5"/>
                            </svg>
                        </div>
                        <h3 class="pain-card__title">"Fuite de donnees"</h3>
                        <p class="pain-card__text">
                            "En 2024, des fuites massives ont expose "
                            <strong>"des centaines de millions de comptes"</strong>
                            ". Un seul mot de passe reutilise = tous vos comptes compromis en quelques minutes."
                        </p>
                        <span class="pain-card__link">"En savoir plus →"</span>
                        <div class="pain-card__glow"></div>
                    </div>

                    // Card 2 - Device theft
                    <div class="pain-card">
                        <div class="pain-card__icon-badge">
                            <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                <rect x="6" y="8" width="36" height="26" rx="3" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                <line x1="6" y1="38" x2="42" y2="38" stroke="currentColor" stroke-width="2.5"/>
                                <path d="M20 40h8" stroke="currentColor" stroke-width="2.5"/>
                                <path d="M18 18l12 8M30 18l-12 8" stroke="currentColor" stroke-width="2" opacity="0.6"/>
                            </svg>
                        </div>
                        <h3 class="pain-card__title">"Vol d'appareil"</h3>
                        <p class="pain-card__text">
                            "Les navigateurs stockent vos mots de passe avec une protection minimale. "
                            <strong>"Un voleur y accede en 30 secondes"</strong>
                            " avec un simple logiciel gratuit."
                        </p>
                        <span class="pain-card__link">"En savoir plus →"</span>
                        <div class="pain-card__glow"></div>
                    </div>

                    // Card 3 - Manager hacked
                    <div class="pain-card">
                        <div class="pain-card__icon-badge">
                            <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                <rect x="10" y="18" width="28" height="22" rx="3" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                <circle cx="24" cy="14" r="6" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                <path d="M19 30h10M19 35h6" stroke="currentColor" stroke-width="2" opacity="0.6"/>
                            </svg>
                        </div>
                        <h3 class="pain-card__title">"Gestionnaire pirate"</h3>
                        <p class="pain-card__text">
                            "Des gestionnaires populaires ont ete pirates. "
                            <strong>"25 millions de coffres chiffres voles"</strong>
                            ". Si la cle est sur leurs serveurs, vos donnees sont en danger."
                        </p>
                        <span class="pain-card__link">"En savoir plus →"</span>
                        <div class="pain-card__glow"></div>
                    </div>
                </div>
            </div>
        </section>
    }
}
