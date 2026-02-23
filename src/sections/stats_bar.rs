use leptos::prelude::*;

#[component]
pub fn StatsBar() -> impl IntoView {
    view! {
        <section class="stats-bar animate-on-scroll">
            <div class="container stats-bar__inner">
                <div class="stats-bar__item">
                    <span class="stats-bar__number">"24 milliards"</span>
                    <span class="stats-bar__label">"de mots de passe voles en 2024"</span>
                </div>
                <div class="stats-bar__divider"></div>
                <div class="stats-bar__item">
                    <span class="stats-bar__number">"80%"</span>
                    <span class="stats-bar__label">"des piratages exploitent des mots de passe faibles"</span>
                </div>
                <div class="stats-bar__divider"></div>
                <div class="stats-bar__item">
                    <span class="stats-bar__number">"1 sur 3"</span>
                    <span class="stats-bar__label">"reutilise le meme mot de passe partout"</span>
                </div>
            </div>
        </section>
    }
}
