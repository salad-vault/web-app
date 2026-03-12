use leptos::prelude::*;

#[component]
pub fn Trust() -> impl IntoView {
    let (expanded, set_expanded) = signal(Option::<usize>::None);

    let toggle = move |index: usize| {
        set_expanded.update(|e| {
            if *e == Some(index) {
                *e = None;
            } else {
                *e = Some(index);
            }
        });
    };

    let badges: Vec<(&str, &str, &str)> = vec![
        (
            "100% Open Source",
            "Notre code est public. Verifiez vous-meme qu'on ne ment pas. Chaque ligne est auditable par n'importe qui.",
            "code",
        ),
        (
            "XChaCha20-Poly1305",
            "Le meme chiffrement utilise par les systemes les plus securises au monde. Vos donnees sont du bruit pour un pirate.",
            "shield",
        ),
        (
            "Argon2id",
            "Votre mot de passe est renforce des milliards de fois avant d'etre utilise. Les attaques par force brute sont inutiles.",
            "lock",
        ),
        (
            "Zero Logs",
            "Nous ne savons pas qui vous etes. Pas d'email en clair, pas de nom, pas de trace. Meme sous contrainte legale, nous n'avons rien a donner.",
            "eye-off",
        ),
    ];

    view! {
        <section class="trust animate-on-scroll" id="trust">
            <div class="container">
                <h2 class="section-title">"Verifiez. Ne faites pas confiance."</h2>
                <p class="section-subtitle">"Cliquez sur un badge pour comprendre en termes simples."</p>

                <div class="trust__grid">
                    {badges.into_iter().enumerate().map(|(i, (title, desc, icon_type))| {
                        let is_expanded = move || expanded.get() == Some(i);
                        let card_class = move || {
                            if is_expanded() { "trust__badge trust__badge--expanded" }
                            else { "trust__badge" }
                        };

                        let icon_svg = match icon_type {
                            "code" => view! {
                                <svg viewBox="0 0 36 36" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                    <path d="M12 10L4 18l8 8M24 10l8 8-8 8M20 6l-4 24" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            }.into_any(),
                            "shield" => view! {
                                <svg viewBox="0 0 36 36" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                    <path d="M18 3L4 9v9c0 8.3 6 16.1 14 18 8-1.9 14-9.7 14-18V9L18 3z" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                    <path d="M12 18l4 4 8-8" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            }.into_any(),
                            "lock" => view! {
                                <svg viewBox="0 0 36 36" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                    <rect x="8" y="16" width="20" height="14" rx="3" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                    <path d="M12 16v-4a6 6 0 0112 0v4" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                    <circle cx="18" cy="24" r="2.5" fill="currentColor"/>
                                </svg>
                            }.into_any(),
                            _ /* eye-off */ => view! {
                                <svg viewBox="0 0 36 36" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                    <path d="M4 18s5-10 14-10 14 10 14 10-5 10-14 10S4 18 4 18z" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                    <circle cx="18" cy="18" r="4" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                    <line x1="6" y1="6" x2="30" y2="30" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/>
                                </svg>
                            }.into_any(),
                        };

                        view! {
                            <div class=card_class on:click=move |_| toggle(i)>
                                <div class="trust__badge-icon">{icon_svg}</div>
                                <h3 class="trust__badge-title">{title}</h3>
                                {move || if is_expanded() {
                                    view! { <p class="trust__badge-desc">{desc}</p> }.into_any()
                                } else {
                                    view! { <span class="trust__badge-hint">"En savoir plus"</span> }.into_any()
                                }}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
