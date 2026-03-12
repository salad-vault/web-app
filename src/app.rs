use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::sections::navbar::Navbar;
use crate::sections::hero::Hero;
use crate::sections::stats_bar::StatsBar;
use crate::sections::pain_points::PainPoints;
use crate::sections::agitation::Agitation;
use crate::sections::dual_lock::DualLock;
use crate::sections::saladier_demo::SaladierDemo;
use crate::sections::how_it_works::HowItWorks;
use crate::sections::comparison::Comparison;
use crate::sections::trust::Trust;
use crate::sections::pricing::Pricing;
use crate::sections::faq::Faq;
use crate::sections::final_cta::FinalCta;
use crate::sections::footer::Footer;

/// Section IDs used for navigation anchors.
pub mod section_ids {
    pub const HOW_IT_WORKS: &str = "how-it-works";
    pub const TRUST: &str = "trust";
    pub const PRICING: &str = "pricing";
}

/// Height of the fixed navbar in pixels, used as scroll offset.
const NAVBAR_HEIGHT: f64 = 80.0;

/// Scroll to a section by its id.
pub fn scroll_to(id: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(el) = document.get_element_by_id(id) {
                let top = el.get_bounding_client_rect().top() + window.scroll_y().unwrap_or(0.0) - NAVBAR_HEIGHT;
                let opts = web_sys::ScrollToOptions::new();
                opts.set_top(top);
                opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                window.scroll_to_with_scroll_to_options(&opts);
            }
        }
    }
}

/// Setup IntersectionObserver for scroll-triggered animations.
/// Elements with class `animate-on-scroll` get `.visible` added when they enter the viewport.
pub fn setup_scroll_observer() {
    let closure = Closure::<dyn Fn(js_sys::Array, web_sys::IntersectionObserver)>::new(
        move |entries: js_sys::Array, _observer: web_sys::IntersectionObserver| {
            for i in 0..entries.length() {
                if let Some(entry) = entries.get(i).dyn_ref::<web_sys::IntersectionObserverEntry>() {
                    if entry.is_intersecting() {
                        let _ = entry.target().class_list().add_1("visible");
                    }
                }
            }
        },
    );

    let options = web_sys::IntersectionObserverInit::new();
    options.set_threshold(&JsValue::from_f64(0.1));

    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
        closure.as_ref().unchecked_ref(),
        &options,
    ) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let qsa = js_sys::Reflect::get(
                    &document,
                    &JsValue::from_str("querySelectorAll"),
                );
                if let Ok(func) = qsa {
                    let func = js_sys::Function::from(func);
                    let result = js_sys::Reflect::apply(
                        &func,
                        &document,
                        &js_sys::Array::of1(&JsValue::from_str(".animate-on-scroll")),
                    );
                    if let Ok(node_list) = result {
                        let length = js_sys::Reflect::get(&node_list, &JsValue::from_str("length"))
                            .unwrap_or(JsValue::from_f64(0.0))
                            .as_f64()
                            .unwrap_or(0.0) as u32;
                        for i in 0..length {
                            if let Ok(el) = js_sys::Reflect::get(&node_list, &JsValue::from_f64(i as f64)) {
                                if !el.is_undefined() && !el.is_null() {
                                    observer.observe(el.unchecked_ref::<web_sys::Element>());
                                }
                            }
                        }
                    }
                }
            }
        }
        // Keep the closure and observer alive for the lifetime of the page.
        // This is intentional: the observer must persist to detect scroll events.
        closure.forget();
        std::mem::forget(observer);
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Setup scroll observer after first render.
    Effect::new(move |_| {
        // Small delay to ensure DOM is populated.
        let cb = Closure::<dyn FnMut()>::new(move || {
            setup_scroll_observer();
        });
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                200,
            );
        }
        cb.forget();
    });

    view! {
        <Navbar/>
        <main>
            <Hero/>
            <StatsBar/>
            <PainPoints/>
            <Agitation/>
            <DualLock/>
            <SaladierDemo/>
            <HowItWorks/>
            <Comparison/>
            <Trust/>
            <Pricing/>
            <Faq/>
            <FinalCta/>
        </main>
        <Footer/>
    }
}
