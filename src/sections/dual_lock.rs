use leptos::prelude::*;

#[component]
pub fn DualLock() -> impl IntoView {
    let (key_a, set_key_a) = signal(false);
    let (key_b, set_key_b) = signal(false);

    let is_unlocked = move || key_a.get() && key_b.get();

    let reset = move |_| {
        set_key_a.set(false);
        set_key_b.set(false);
    };

    let vault_class = move || {
        let mut cls = String::from("dual-lock__vault");
        if is_unlocked() {
            cls.push_str(" dual-lock__vault--open");
        } else if key_a.get() || key_b.get() {
            cls.push_str(" dual-lock__vault--partial");
        }
        cls
    };

    view! {
        <section class="dual-lock animate-on-scroll" id="dual-lock">
            <div class="container">
                <h2 class="section-title">"Le Double Verrouillage"</h2>
                <p class="section-subtitle">"Votre secret et le notre. Les deux sont necessaires."</p>

                <div class="dual-lock__demo">
                    <button
                        class=move || if key_a.get() { "dual-lock__key dual-lock__key--a dual-lock__key--inserted" } else { "dual-lock__key dual-lock__key--a" }
                        on:click=move |_| set_key_a.update(|v| *v = !*v)
                    >
                        <div class="dual-lock__key-icon">
                            <svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                <circle cx="12" cy="12" r="8" stroke="currentColor" stroke-width="2" fill="none"/>
                                <line x1="18" y1="18" x2="30" y2="30" stroke="currentColor" stroke-width="2"/>
                                <line x1="26" y1="30" x2="30" y2="26" stroke="currentColor" stroke-width="2"/>
                            </svg>
                        </div>
                        <span class="dual-lock__key-label">"Cle A"</span>
                        <span class="dual-lock__key-desc">"Votre appareil"</span>
                    </button>

                    <div class=vault_class>
                        <div class="dual-lock__vault-body">
                            <div class=move || if key_a.get() { "dual-lock__slot dual-lock__slot--filled" } else { "dual-lock__slot" }>
                                <div class="dual-lock__slot-hole"></div>
                            </div>
                            <div class="dual-lock__vault-icon">
                                {move || if is_unlocked() {
                                    view! {
                                        <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" class="dual-lock__icon-unlocked" aria-hidden="true">
                                            <path d="M34 20h-4v-6a6 6 0 0 0-12 0" stroke="var(--accent)" stroke-width="3" fill="none" stroke-linecap="round"/>
                                            <rect x="10" y="20" width="28" height="20" rx="4" stroke="var(--accent)" stroke-width="3" fill="none"/>
                                            <circle cx="24" cy="31" r="3" fill="var(--accent)"/>
                                        </svg>
                                    }.into_any()
                                } else {
                                    view! {
                                        <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" class="dual-lock__icon-locked" aria-hidden="true">
                                            <path d="M14 20v-6a10 10 0 0 1 20 0v6" stroke="var(--text-muted)" stroke-width="3" fill="none"/>
                                            <rect x="10" y="20" width="28" height="20" rx="4" stroke="var(--text-muted)" stroke-width="3" fill="none"/>
                                            <circle cx="24" cy="31" r="3" fill="var(--text-muted)"/>
                                        </svg>
                                    }.into_any()
                                }}
                            </div>
                            <div class=move || if key_b.get() { "dual-lock__slot dual-lock__slot--filled" } else { "dual-lock__slot" }>
                                <div class="dual-lock__slot-hole"></div>
                            </div>
                        </div>
                        <div class="dual-lock__vault-status">
                            {move || if is_unlocked() {
                                view! { <span class="dual-lock__status dual-lock__status--open">"Acces autorise ✓"</span> }.into_any()
                            } else if key_a.get() || key_b.get() {
                                view! { <span class="dual-lock__status dual-lock__status--partial">"Il manque une cle..."</span> }.into_any()
                            } else {
                                view! { <span class="dual-lock__status">"Inserez les deux cles"</span> }.into_any()
                            }}
                        </div>
                    </div>

                    <button
                        class=move || if key_b.get() { "dual-lock__key dual-lock__key--b dual-lock__key--inserted" } else { "dual-lock__key dual-lock__key--b" }
                        on:click=move |_| set_key_b.update(|v| *v = !*v)
                    >
                        <div class="dual-lock__key-icon">
                            <svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                <circle cx="12" cy="12" r="8" stroke="currentColor" stroke-width="2" fill="none"/>
                                <line x1="18" y1="18" x2="30" y2="30" stroke="currentColor" stroke-width="2"/>
                                <line x1="24" y1="30" x2="30" y2="24" stroke="currentColor" stroke-width="2"/>
                            </svg>
                        </div>
                        <span class="dual-lock__key-label">"Cle B"</span>
                        <span class="dual-lock__key-desc">"Votre mot de passe"</span>
                    </button>
                </div>

                {move || if is_unlocked() {
                    view! {
                        <div class="dual-lock__reset-wrap">
                            <button class="btn btn--ghost btn--sm" on:click=reset>"Reverrouiller"</button>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                <p class="dual-lock__explainer">
                    "Meme si un pirate vole nos serveurs, sans votre "
                    <strong>"Ingredient Secret"</strong>
                    " (la cle sur votre appareil), vos donnees sont du charabia."
                </p>
            </div>
        </section>
    }
}
