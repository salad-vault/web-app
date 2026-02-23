use leptos::prelude::*;

#[component]
pub fn HowItWorks() -> impl IntoView {
    let (active_step, set_active_step) = signal(0usize);

    let steps = vec![
        ("1", "Telechargez", "30 secondes. Windows, Mac ou Linux.", "download"),
        ("2", "Creez votre Potager", "Un mot de passe maitre + votre cle locale generee automatiquement.", "garden"),
        ("3", "Organisez vos Saladiers", "Perso, Pro, Crypto... chaque compartiment est isole et chiffre.", "folders"),
        ("4", "Dormez tranquille", "Vos donnees sont intouchables. Meme par nous.", "moon"),
    ];

    view! {
        <section class="how-it-works animate-on-scroll" id="how-it-works">
            <div class="container">
                <h2 class="section-title">"Pret en 3 minutes. Protege pour toujours."</h2>

                <div class="how-it-works__steps">
                    {steps.into_iter().enumerate().map(|(i, (num, title, desc, icon_type))| {
                        let is_active = move || active_step.get() == i;
                        let step_class = move || {
                            if is_active() { "how-it-works__step how-it-works__step--active" }
                            else { "how-it-works__step" }
                        };

                        let icon_svg = match icon_type {
                            "download" => view! {
                                <svg viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M20 5v22M12 20l8 7 8-7" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path d="M6 30v4a2 2 0 002 2h24a2 2 0 002-2v-4" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/>
                                </svg>
                            }.into_any(),
                            "garden" => view! {
                                <svg viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M20 36V20" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/>
                                    <path d="M20 20c0-8 8-12 14-10-2 6-8 10-14 10z" stroke="currentColor" stroke-width="2" fill="none"/>
                                    <path d="M20 24c0-6-6-10-12-8 2 5 7 8 12 8z" stroke="currentColor" stroke-width="2" fill="none"/>
                                    <rect x="6" y="34" width="28" height="3" rx="1.5" stroke="currentColor" stroke-width="2" fill="none"/>
                                </svg>
                            }.into_any(),
                            "folders" => view! {
                                <svg viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <rect x="4" y="8" width="14" height="12" rx="2" stroke="currentColor" stroke-width="2" fill="none"/>
                                    <rect x="22" y="8" width="14" height="12" rx="2" stroke="currentColor" stroke-width="2" fill="none"/>
                                    <rect x="4" y="24" width="14" height="12" rx="2" stroke="currentColor" stroke-width="2" fill="none"/>
                                    <rect x="22" y="24" width="14" height="12" rx="2" stroke="currentColor" stroke-width="2" fill="none"/>
                                    <circle cx="11" cy="14" r="2" fill="currentColor"/>
                                    <circle cx="29" cy="14" r="2" fill="currentColor"/>
                                    <circle cx="11" cy="30" r="2" fill="currentColor"/>
                                    <circle cx="29" cy="30" r="2" fill="currentColor"/>
                                </svg>
                            }.into_any(),
                            _ /* moon */ => view! {
                                <svg viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M32 24A14 14 0 0116 8a14 14 0 1016 16z" stroke="currentColor" stroke-width="2.5" fill="none"/>
                                    <circle cx="28" cy="10" r="1" fill="currentColor"/>
                                    <circle cx="34" cy="18" r="1.5" fill="currentColor"/>
                                    <circle cx="30" cy="6" r="0.8" fill="currentColor"/>
                                </svg>
                            }.into_any(),
                        };

                        view! {
                            <div class=step_class on:click=move |_| set_active_step.set(i)>
                                <div class="how-it-works__step-number">{num}</div>
                                <div class="how-it-works__step-icon">{icon_svg}</div>
                                <h3 class="how-it-works__step-title">{title}</h3>
                                <p class="how-it-works__step-desc">{desc}</p>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
