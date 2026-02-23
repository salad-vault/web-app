use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container footer__inner">
                <div class="footer__cols">
                    <div class="footer__col">
                        <h4 class="footer__col-title">"Produit"</h4>
                        <ul class="footer__links">
                            <li><a href="#how-it-works" class="footer__link">"Fonctionnement"</a></li>
                            <li><a href="#pricing" class="footer__link">"Tarifs"</a></li>
                            <li><a href="#comparison" class="footer__link">"Comparatif"</a></li>
                            <li><a href="#faq" class="footer__link">"FAQ"</a></li>
                        </ul>
                    </div>
                    <div class="footer__col">
                        <h4 class="footer__col-title">"Securite"</h4>
                        <ul class="footer__links">
                            <li><a href="https://github.com" target="_blank" rel="noopener" class="footer__link">"Code source"</a></li>
                            <li><a href="#trust" class="footer__link">"Documentation crypto"</a></li>
                            <li><span class="footer__link footer__link--disabled">"Audits (bientot)"</span></li>
                        </ul>
                    </div>
                    <div class="footer__col">
                        <h4 class="footer__col-title">"Legal"</h4>
                        <ul class="footer__links">
                            <li><span class="footer__link footer__link--disabled">"Conditions d'utilisation"</span></li>
                            <li><span class="footer__link footer__link--disabled">"Politique de confidentialite"</span></li>
                            <li>
                                <span class="footer__link footer__warrant-canary" title="Aucune injonction gouvernementale recue a ce jour.">
                                    "Warrant Canary ✓"
                                </span>
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="footer__bottom">
                    <p class="footer__signature">
                        "Fait avec Rust 🦀 — Pour ceux qui ne font confiance a personne."
                    </p>
                    <p class="footer__copyright">
                        "© 2025 SaladVault. Open Source."
                    </p>
                </div>
            </div>
        </footer>
    }
}
