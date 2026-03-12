use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::app::{scroll_to, section_ids};

#[component]
pub fn Navbar() -> impl IntoView {
    let (scrolled, set_scrolled) = signal(false);
    let (menu_open, set_menu_open) = signal(false);

    // Listen to scroll events to toggle navbar background.
    Effect::new(move |_| {
        let cb = Closure::<dyn FnMut()>::wrap(Box::new(move || {
            if let Some(window) = web_sys::window() {
                let y = window.scroll_y().unwrap_or(0.0);
                set_scrolled.set(y > 50.0);
            }
        }));
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    });

    let nav_class = move || {
        let mut cls = String::from("navbar");
        if scrolled.get() {
            cls.push_str(" navbar--scrolled");
        }
        cls
    };

    view! {
        <nav class=nav_class>
            <div class="navbar__inner">
                <a href="#" class="navbar__logo" on:click=move |e: web_sys::MouseEvent| {
                    e.prevent_default();
                    if let Some(window) = web_sys::window() {
                        window.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                }>
                    <span class="navbar__logo-icon">"🔒"</span>
                    <span class="navbar__logo-text">"SaladVault"</span>
                </a>

                <div class=move || if menu_open.get() { "navbar__links navbar__links--open" } else { "navbar__links" }>
                    <a href="#how-it-works" class="navbar__link" on:click=move |e: web_sys::MouseEvent| {
                        e.prevent_default();
                        set_menu_open.set(false);
                        scroll_to(section_ids::HOW_IT_WORKS);
                    }>"Fonctionnement"</a>
                    <a href="#trust" class="navbar__link" on:click=move |e: web_sys::MouseEvent| {
                        e.prevent_default();
                        set_menu_open.set(false);
                        scroll_to(section_ids::TRUST);
                    }>"Securite"</a>
                    <a href="#pricing" class="navbar__link" on:click=move |e: web_sys::MouseEvent| {
                        e.prevent_default();
                        set_menu_open.set(false);
                        scroll_to(section_ids::PRICING);
                    }>"Tarif"</a>
                </div>

                <a href="#pricing" class="navbar__cta btn btn--primary btn--sm" on:click=move |e: web_sys::MouseEvent| {
                    e.prevent_default();
                    scroll_to(section_ids::PRICING);
                }>"Telecharger"</a>

                <button
                    class="navbar__burger"
                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                    aria-label="Menu"
                >
                    <span class=move || if menu_open.get() { "navbar__burger-line navbar__burger-line--open" } else { "navbar__burger-line" }></span>
                </button>
            </div>
        </nav>
    }
}
