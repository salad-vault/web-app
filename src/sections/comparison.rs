use leptos::prelude::*;

#[component]
pub fn Comparison() -> impl IntoView {
    // Feature rows: (label, saladvault, dashlane, onepassword, bitwarden)
    // Values: "yes", "no", "partial"
    let features: Vec<(&str, &str, &str, &str, &str)> = vec![
        ("Zero-Knowledge", "yes", "partial", "partial", "yes"),
        ("Double Verrouillage (Dual-Lock)", "yes", "no", "no", "no"),
        ("Compartimentalisation (Saladiers)", "yes", "no", "no", "no"),
        ("100% Open Source", "yes", "no", "no", "yes"),
        ("Aucune donnee personnelle stockee", "yes", "no", "no", "partial"),
        ("Gratuit (stockage local illimite)", "yes", "no", "no", "yes"),
    ];

    let render_cell = |value: &str| -> &str {
        match value {
            "yes" => "✓",
            "no" => "✗",
            _ => "~",
        }
    };

    let cell_class = |value: &str| -> &str {
        match value {
            "yes" => "comparison__cell comparison__cell--yes",
            "no" => "comparison__cell comparison__cell--no",
            _ => "comparison__cell comparison__cell--partial",
        }
    };

    view! {
        <section class="comparison animate-on-scroll" id="comparison">
            <div class="container">
                <h2 class="section-title">"Comparez. Decidez."</h2>

                <div class="comparison__table-wrapper">
                    <table class="comparison__table">
                        <thead>
                            <tr>
                                <th class="comparison__header-feature"></th>
                                <th class="comparison__header comparison__header--highlight">"SaladVault"</th>
                                <th class="comparison__header">"Dashlane"</th>
                                <th class="comparison__header">"1Password"</th>
                                <th class="comparison__header">"Bitwarden"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {features.into_iter().map(|(label, sv, dash, one, bit)| {
                                view! {
                                    <tr class="comparison__row">
                                        <td class="comparison__feature">{label}</td>
                                        <td class=format!("{} comparison__cell--sv", cell_class(sv))>{render_cell(sv)}</td>
                                        <td class=cell_class(dash)>{render_cell(dash)}</td>
                                        <td class=cell_class(one)>{render_cell(one)}</td>
                                        <td class=cell_class(bit)>{render_cell(bit)}</td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()}
                        </tbody>
                    </table>
                </div>
            </div>
        </section>
    }
}
