use leptos::prelude::*;

#[component]
pub fn Agitation() -> impl IntoView {
    let (hovered_side, set_hovered_side) = signal(Option::<&'static str>::None);

    view! {
        <section class="agitation animate-on-scroll" id="agitation">
            <div class="container">
                <h2 class="section-title">"Et si votre gestionnaire etait le maillon faible ?"</h2>
                <p class="section-subtitle">"La plupart des gestionnaires gardent la cle de VOS donnees sur LEURS serveurs."</p>

                <div class="agitation__comparison">
                    <div
                        class=move || {
                            let base = "agitation__col agitation__col--danger";
                            if hovered_side.get() == Some("danger") { format!("{} agitation__col--hovered", base) } else { base.to_string() }
                        }
                        on:mouseenter=move |_| set_hovered_side.set(Some("danger"))
                        on:mouseleave=move |_| set_hovered_side.set(None)
                    >
                        <div class="agitation__col-header">
                            <span class="agitation__col-icon">"✗"</span>
                            <h3>"Gestionnaire classique"</h3>
                        </div>
                        <ul class="agitation__list">
                            <li class="agitation__list-item agitation__list-item--bad">
                                "Une seule cle de chiffrement"
                            </li>
                            <li class="agitation__list-item agitation__list-item--bad">
                                "Stockee sur LEURS serveurs"
                            </li>
                            <li class="agitation__list-item agitation__list-item--bad">
                                "Si pirate → toutes vos donnees exposees"
                            </li>
                            <li class="agitation__list-item agitation__list-item--bad">
                                "Vous LEUR faites confiance"
                            </li>
                        </ul>
                    </div>

                    <div class="agitation__vs">"VS"</div>

                    <div
                        class=move || {
                            let base = "agitation__col agitation__col--success";
                            if hovered_side.get() == Some("success") { format!("{} agitation__col--hovered", base) } else { base.to_string() }
                        }
                        on:mouseenter=move |_| set_hovered_side.set(Some("success"))
                        on:mouseleave=move |_| set_hovered_side.set(None)
                    >
                        <div class="agitation__col-header">
                            <span class="agitation__col-icon agitation__col-icon--green">"✓"</span>
                            <h3>"SaladVault"</h3>
                        </div>
                        <ul class="agitation__list">
                            <li class="agitation__list-item agitation__list-item--good">
                                "Double verrouillage (deux cles)"
                            </li>
                            <li class="agitation__list-item agitation__list-item--good">
                                "Une moitie chez vous, une moitie sur le serveur"
                            </li>
                            <li class="agitation__list-item agitation__list-item--good">
                                "Si pirate → donnees inutilisables"
                            </li>
                            <li class="agitation__list-item agitation__list-item--good">
                                "Vous ne faites confiance a PERSONNE"
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}
