use leptos::prelude::*;

#[component]
pub fn Faq() -> impl IntoView {
    let (open_index, set_open_index) = signal(Option::<usize>::None);

    let toggle = move |index: usize| {
        set_open_index.update(|o| {
            if *o == Some(index) {
                *o = None;
            } else {
                *o = Some(index);
            }
        });
    };

    let items: Vec<(&str, &str)> = vec![
        (
            "Que se passe-t-il si je perds mon appareil ?",
            "A la creation de votre compte, SaladVault genere une phrase de recuperation de 24 mots. Cette phrase permet de regenerer votre cle locale sur un nouvel appareil. Imprimez-la et gardez-la en securite — c'est votre filet de secours."
        ),
        (
            "C'est vraiment gratuit ? Ou est le piege ?",
            "Le plan Jardinier Amateur est gratuit pour toujours, sans limites sur le nombre de Saladiers ou de Feuilles en local. Notre modele est freemium : les fonctionnalites avancees (sync cloud, partage de Saladiers) seront payantes, mais le coeur de l'application restera gratuit."
        ),
        (
            "Comment ca marche si vous ne connaissez pas mon email ?",
            "Votre email est hache (transforme en code irreversible) sur votre appareil AVANT d'etre envoye a notre serveur. Nous ne recevons qu'un identifiant anonyme. C'est le Blind Indexing : nous pouvons vous identifier sans savoir qui vous etes."
        ),
        (
            "Et si SaladVault disparait un jour ?",
            "SaladVault est 100% Open Source. Meme si l'entreprise disparait, le code reste public et utilisable. De plus, vos donnees sont stockees localement sur votre appareil — elles vous appartiennent, pas a nous."
        ),
        (
            "Mon mot de passe maitre est-il stocke quelque part ?",
            "Jamais. Votre mot de passe maitre n'est jamais stocke, ni en clair, ni sous forme chiffree. Il est utilise pour deriver une cle de chiffrement en RAM, puis immediatement efface de la memoire (grace a Zeroize). Personne — pas meme nous — ne peut le retrouver."
        ),
        (
            "Pourquoi vous faire confiance plutot qu'a Dashlane ?",
            "Ne nous faites pas confiance. Verifiez. Notre code est Open Source : chaque ligne est auditable. Notre architecture Dual-Lock fait que meme si nos serveurs sont compromis, vos donnees sont inutilisables sans votre cle locale. Nous n'avons techniquement pas la capacite de lire vos donnees."
        ),
    ];

    view! {
        <section class="faq animate-on-scroll" id="faq">
            <div class="container">
                <h2 class="section-title">"Vos questions."</h2>

                <div class="faq__list">
                    {items.into_iter().enumerate().map(|(i, (question, answer))| {
                        let is_open = move || open_index.get() == Some(i);
                        let item_class = move || {
                            if is_open() { "faq__item faq__item--open" }
                            else { "faq__item" }
                        };

                        view! {
                            <div class=item_class>
                                <button class="faq__question" on:click=move |_| toggle(i)>
                                    <span class="faq__question-text">{question}</span>
                                    <span class="faq__chevron">
                                        {move || if is_open() { "−" } else { "+" }}
                                    </span>
                                </button>
                                <div class="faq__answer">
                                    <p class="faq__answer-text">{answer}</p>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
