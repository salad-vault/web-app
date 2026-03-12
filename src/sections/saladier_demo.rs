use leptos::prelude::*;

#[component]
pub fn SaladierDemo() -> impl IntoView {
    let (saladiers, set_saladiers) = signal(Vec::<String>::new());
    let (input_value, set_input_value) = signal(String::new());
    let (breached_index, set_breached_index) = signal(Option::<usize>::None);
    let (breach_message, set_breach_message) = signal(Option::<String>::None);

    let try_add_saladier = move || {
        let name = input_value.get().trim().to_string();
        if !name.is_empty() && saladiers.get().len() < 4 {
            set_saladiers.update(|s| s.push(name));
            set_input_value.set(String::new());
            set_breached_index.set(None);
            set_breach_message.set(None);
        }
    };

    let add_saladier = move |_| {
        try_add_saladier();
    };

    let simulate_breach = move |_| {
        let list = saladiers.get();
        if list.len() >= 2 {
            // Pick a pseudo-random index based on list length.
            let idx = (js_sys::Math::random() * list.len() as f64).floor() as usize;
            set_breached_index.set(Some(idx));
            let safe_count = list.len() - 1;
            set_breach_message.set(Some(format!(
                "1 saladier compromis. Les {} autres sont intacts.",
                safe_count
            )));
        }
    };

    let reset_demo = move |_| {
        set_saladiers.set(Vec::new());
        set_breached_index.set(None);
        set_breach_message.set(None);
        set_input_value.set(String::new());
    };

    let on_input = move |ev: leptos::ev::Event| {
        use wasm_bindgen::JsCast;
        if let Some(target) = ev.target() {
            if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                set_input_value.set(input.value());
            }
        }
    };

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            try_add_saladier();
        }
    };

    let is_add_disabled = move || {
        input_value.get().trim().is_empty() || saladiers.get().len() >= 4
    };

    let is_breach_disabled = move || {
        saladiers.get().len() < 2 || breached_index.get().is_some()
    };

    let is_reset_disabled = move || {
        saladiers.get().is_empty()
    };

    let is_full = move || saladiers.get().len() >= 4;

    view! {
        <section class="saladier-demo animate-on-scroll" id="saladier-demo">
            <div class="container">
                <h2 class="section-title">"Ne mettez pas tous vos oeufs dans le meme panier."</h2>
                <p class="section-subtitle">"Chaque Saladier est un compartiment chiffre independamment. Essayez."</p>

                <div class="saladier-demo__workspace">
                    // Creation form
                    <div class="saladier-demo__form">
                        <input
                            type="text"
                            class="saladier-demo__input"
                            aria-label="Nom du Saladier"
                            placeholder="Nommez votre Saladier (Perso, Pro, Crypto...)"
                            prop:value=input_value
                            on:input=on_input
                            on:keydown=on_keydown
                        />
                        <button
                            class="btn btn--primary"
                            on:click=add_saladier
                            prop:disabled=is_add_disabled
                        >
                            "Creer le Saladier"
                        </button>
                    </div>

                    {move || if is_full() {
                        view! { <p class="saladier-demo__limit">"Maximum 4 Saladiers pour la demo."</p> }.into_any()
                    } else {
                        view! { <p></p> }.into_any()
                    }}

                    // Saladier grid
                    <div class="saladier-demo__grid">
                        {move || {
                            saladiers.get().iter().enumerate().map(|(i, name)| {
                                let is_breached = breached_index.get() == Some(i);
                                let card_class = if is_breached {
                                    "saladier-demo__card saladier-demo__card--breached"
                                } else if breached_index.get().is_some() {
                                    "saladier-demo__card saladier-demo__card--safe"
                                } else {
                                    "saladier-demo__card saladier-demo__card--idle"
                                };
                                let lock_icon = if is_breached { "🔓" } else { "🔒" };
                                let name = name.clone();
                                view! {
                                    <div class=card_class>
                                        <span class="saladier-demo__card-lock">{lock_icon}</span>
                                        <span class="saladier-demo__card-name">{name}</span>
                                        <span class="saladier-demo__card-status">
                                            {if is_breached { "Compromis" } else if breached_index.get().is_some() { "Intact" } else { "Protege" }}
                                        </span>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>

                    // Breach message
                    {move || breach_message.get().map(|msg| {
                        view! {
                            <p class="saladier-demo__breach-msg">{msg}</p>
                        }
                    })}

                    // Action buttons
                    <div class="saladier-demo__actions">
                        <button
                            class="btn btn--danger"
                            on:click=simulate_breach
                            prop:disabled=is_breach_disabled
                        >
                            "Simuler un piratage"
                        </button>
                        <button
                            class="btn btn--ghost"
                            on:click=reset_demo
                            prop:disabled=is_reset_disabled
                        >
                            "Reinitialiser"
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
